#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::{Arc, Mutex}};

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use tauri::Manager;
use std::collections::HashMap;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

struct AppState {
  db: Arc<Mutex<Connection>>,
  notes_dir: Arc<Mutex<Option<PathBuf>>>,
  watcher: Arc<Mutex<Option<RecommendedWatcher>>>,
  gen_cancel: Arc<Mutex<bool>>,
}

fn init_db() -> Result<Connection> {
  let base = tauri::api::path::home_dir().unwrap_or(std::env::current_dir().unwrap());
  let app_dir = base.join(".votion");
  std::fs::create_dir_all(&app_dir)?;
  let db_path = app_dir.join("votion.db");
  let conn = Connection::open(db_path)?;
  conn.execute_batch(r#"
    CREATE TABLE IF NOT EXISTS notes(
      id INTEGER PRIMARY KEY,
      path TEXT UNIQUE,
      title TEXT,
      created_at INTEGER,
      updated_at INTEGER,
      hash TEXT,
      tags TEXT
    );
    CREATE TABLE IF NOT EXISTS chunks(
      id INTEGER PRIMARY KEY,
      note_id INTEGER,
      seq INTEGER,
      content TEXT,
      content_hash TEXT,
      FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE
    );
    CREATE TABLE IF NOT EXISTS settings(
      key TEXT PRIMARY KEY,
      value TEXT,
      updated_at INTEGER
    );
  "#)?;
  conn.execute(
    "CREATE TABLE IF NOT EXISTS embeddings(
      id INTEGER PRIMARY KEY,
      chunk_id INTEGER UNIQUE,
      vector TEXT,
      dim INTEGER,
      FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE
    )",
    [],
  )?;
  let _ = conn.execute("ALTER TABLE notes ADD COLUMN tags TEXT", []);
  Ok(conn)
}

fn file_hash(content: &str) -> String {
  let mut h = Sha256::new();
  h.update(content.as_bytes());
  hex::encode(h.finalize())
}

fn read_file(path: &PathBuf) -> Result<String> {
  let c = std::fs::read_to_string(path)?;
  Ok(c)
}

fn parse_front_matter(content: &str) -> (Option<std::collections::HashMap<String, String>>, String) {
  let mut map = std::collections::HashMap::new();
  if content.starts_with("---\n") {
    if let Some(end) = content[4..].find("\n---") {
      let header = &content[4..4+end];
      for line in header.lines() {
        if let Some(pos) = line.find(':') {
          let key = line[..pos].trim().to_lowercase();
          let val = line[pos+1..].trim().to_string();
          map.insert(key, val);
        }
      }
      let body = content[4+end+4..].to_string();
      return (Some(map), body)
    }
  }
  (None, content.to_string())
}

fn parse_title_and_tags(content: &str) -> (String, Option<String>, String) {
  let (fm_opt, body) = parse_front_matter(content);
  let mut title = None;
  let mut tags = None;
  if let Some(fm) = &fm_opt {
    if let Some(t) = fm.get("title") { title = Some(t.clone()); }
    if let Some(ts) = fm.get("tags") { tags = Some(ts.clone()); }
  }
  let fallback = body.lines().find(|l| l.starts_with('#')).map(|l| l.trim().trim_start_matches('#').trim().to_string()).unwrap_or_else(|| "未命名".to_string());
  (title.unwrap_or(fallback), tags, body)
}

fn chunk_markdown(content: &str) -> Vec<String> {
  let mut chunks = Vec::new();
  let mut current = String::new();
  for line in content.lines() {
    if line.starts_with('#') && !current.is_empty() {
      chunks.push(current.clone());
      current.clear();
    }
    current.push_str(line);
    current.push('\n');
  }
  if !current.is_empty() { chunks.push(current); }
  chunks
}

fn index_file_conn(db: &Arc<Mutex<Connection>>, path: PathBuf) -> Result<()> {
  let content = read_file(&path)?;
  let (title, tags, body) = parse_title_and_tags(&content);
  let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
  let hash = file_hash(&content);
  let conn = db.lock().unwrap();
  conn.execute(
    "INSERT INTO notes(path, title, created_at, updated_at, hash, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
     ON CONFLICT(path) DO UPDATE SET title=excluded.title, updated_at=excluded.updated_at, hash=excluded.hash, tags=excluded.tags",
    params![path.to_string_lossy(), title, now, now, hash, tags.unwrap_or_default()],
  )?;
  let note_id: i64 = conn.query_row("SELECT id FROM notes WHERE path=?1", params![path.to_string_lossy()], |r| r.get(0))?;
  conn.execute("DELETE FROM chunks WHERE note_id=?1", params![note_id])?;
  for (i, ch) in chunk_markdown(&body).into_iter().enumerate() {
    let chash = file_hash(&ch);
    conn.execute(
      "INSERT INTO chunks(note_id, seq, content, content_hash) VALUES (?1, ?2, ?3, ?4)",
      params![note_id, i as i64, ch, chash],
    )?;
    let chunk_id: i64 = conn.query_row(
      "SELECT id FROM chunks WHERE note_id=?1 AND seq=?2",
      params![note_id, i as i64],
      |r| r.get(0)
    )?;
    if let Some(vec) = compute_embedding_blocking(&conn, &ch)? {
      let dim = vec.len() as i64;
      let vec_json = serde_json::to_string(&vec).unwrap_or("[]".to_string());
      conn.execute(
        "INSERT INTO embeddings(chunk_id, vector, dim) VALUES (?1, ?2, ?3)
         ON CONFLICT(chunk_id) DO UPDATE SET vector=excluded.vector, dim=excluded.dim",
        params![chunk_id, vec_json, dim],
      )?;
    }
  }
  Ok(())
}

#[tauri::command]
fn get_notes(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
  let conn = state.db.lock().unwrap();
  let mut stmt = conn.prepare("SELECT id, path, title, updated_at, tags FROM notes ORDER BY updated_at DESC").map_err(|e| e.to_string())?;
  let rows = stmt.query_map([], |r| {
    let id: i64 = r.get(0)?;
    let path: String = r.get(1)?;
    let title: String = r.get(2)?;
    let updated_at: i64 = r.get(3)?;
    let tags: String = r.get(4).unwrap_or_default();
    Ok((id, path, title, updated_at, tags))
  }).map_err(|e| e.to_string())?;
  let mut items = Vec::new();
  for row in rows { let (id, path, title, updated_at, tags) = row.map_err(|e| e.to_string())?; items.push(serde_json::json!({"id": id, "path": path, "title": title, "updated_at": updated_at, "tags": tags})); }
  Ok(serde_json::json!({"items": items}))
}

#[tauri::command]
fn watch_notes(state: tauri::State<'_, AppState>, dir: String, app: tauri::AppHandle) -> Result<(), String> {
  let dir_path = PathBuf::from(dir);
  if !dir_path.exists() { std::fs::create_dir_all(&dir_path).map_err(|e| e.to_string())?; }
  {
    let mut d = state.notes_dir.lock().unwrap();
    *d = Some(dir_path.clone());
  }
  {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
    let _ = state.db.lock().unwrap().execute(
      "INSERT INTO settings(key, value, updated_at) VALUES('notes_dir', ?1, ?2)
       ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=excluded.updated_at",
      params![dir_path.to_string_lossy(), now]
    );
  }
  {
    for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
      let p = entry.path();
      if p.is_file() && p.extension().map(|x| x == "md").unwrap_or(false) {
        let _ = index_file_conn(&state.db, p.to_path_buf());
      }
    }
    let _ = app.emit_all("votion://index-updated", serde_json::json!({"type":"full-reindex"}));
  }
  let db = state.db.clone();
  let app_handle = app.clone();
  let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
    if let Ok(ev) = res {
      if let Some(p) = ev.paths.first() {
        let pbuf = p.to_path_buf();
        if pbuf.is_file() && pbuf.extension().map(|x| x == "md").unwrap_or(false) {
          let _ = index_file_conn(&db, pbuf.clone());
          let _ = app_handle.emit_all("votion://index-updated", serde_json::json!({"type":"file","path": pbuf.to_string_lossy()}));
        }
      }
    }
  }).map_err(|e| e.to_string())?;
  watcher.watch(&dir_path, RecursiveMode::Recursive).map_err(|e| e.to_string())?;
  {
    let mut w = state.watcher.lock().unwrap();
    *w = Some(watcher);
  }
  Ok(())
}

#[tauri::command]
fn index_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
  let p = PathBuf::from(path);
  index_file_conn(&state.db, p).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_embeddings(state: tauri::State<'_, AppState>, query: String, top_k: Option<i64>) -> Result<serde_json::Value, String> {
  let conn = state.db.lock().unwrap();
  let has_embeddings: i64 = conn.query_row("SELECT COUNT(1) FROM embeddings", [], |r| r.get(0)).unwrap_or(0);
  if has_embeddings > 0 {
    if let Some(qvec) = compute_embedding_blocking(&conn, &query).map_err(|e| e.to_string())? {
      let mut stmt = conn.prepare("SELECT e.chunk_id, n.title, n.path, c.content, e.vector FROM embeddings e JOIN chunks c ON e.chunk_id=c.id JOIN notes n ON c.note_id=n.id").map_err(|e| e.to_string())?;
      let rows = stmt.query_map([], |r| {
        let id: i64 = r.get(0)?;
        let title: String = r.get(1)?;
        let path: String = r.get(2)?;
        let content: String = r.get(3)?;
        let vector_json: String = r.get(4)?;
        Ok((id, title, path, content, vector_json))
      }).map_err(|e| e.to_string())?;
      let mut scored: Vec<(i64, String, String, String, f32)> = Vec::new();
      let qnorm = vec_norm(&qvec);
      for row in rows {
        let (id, title, path, content, vjson) = row.map_err(|e| e.to_string())?;
        if let Ok(v) = serde_json::from_str::<Vec<f32>>(&vjson) {
          let sim = cosine_sim(&qvec, &v, qnorm);
          scored.push((id, title, path, content, sim));
        }
      }
      scored.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
      let k = top_k.unwrap_or(5) as usize;
      let items: Vec<serde_json::Value> = scored.into_iter().take(k).map(|(id, title, path, content, sim)| {
        serde_json::json!({"id": id, "title": title, "path": path, "content": content, "score": sim})
      }).collect();
      return Ok(serde_json::json!({"items": items}));
    }
  }
  let q = query.to_lowercase();
  let mut stmt = conn.prepare("SELECT c.id, n.title, n.path, c.content FROM chunks c JOIN notes n ON c.note_id=n.id").map_err(|e| e.to_string())?;
  let rows = stmt.query_map([], |r| {
    let id: i64 = r.get(0)?;
    let title: String = r.get(1)?;
    let path: String = r.get(2)?;
    let content: String = r.get(3)?;
    Ok((id, title, path, content))
  }).map_err(|e| e.to_string())?;
  let mut scored: Vec<(i64, String, String, String, i64)> = Vec::new();
  for row in rows {
    let (id, title, path, content) = row.map_err(|e| e.to_string())?;
    let score = content.to_lowercase().matches(&q).count() as i64;
    if score > 0 { scored.push((id, title, path, content, score)); }
  }
  scored.sort_by_key(|x| -x.4);
  let k = top_k.unwrap_or(5) as usize;
  let items: Vec<serde_json::Value> = scored.into_iter().take(k).map(|(id, title, path, content, sc)| {
    serde_json::json!({"id": id, "title": title, "path": path, "content": content, "score": sc as f32})
  }).collect();
  Ok(serde_json::json!({"items": items}))
}

#[tauri::command]
fn generate_answer(state: tauri::State<'_, AppState>, query: String) -> Result<String, String> {
  let res = search_embeddings(state.clone(), query.clone(), Some(5))?;
  let items = res.get("items").and_then(|v| v.as_array()).cloned().unwrap_or_default();
  let mut combined = String::new();
  for it in items {
    if let Some(c) = it.get("content").and_then(|x| x.as_str()) { combined.push_str(c); combined.push('\n'); }
  }
  let conn = state.db.lock().unwrap();
  if let Ok(opt) = generate_with_provider(&conn, &query, &combined) { if let Some(ans) = opt { return Ok(ans); } }
  let ans = format!("问题:\n{}\n\n参考:\n{}", query, combined);
  Ok(ans)
}

#[tauri::command]
fn get_note(path: String) -> Result<serde_json::Value, String> {
  let p = PathBuf::from(path);
  let content = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
  Ok(serde_json::json!({"content": content}))
}

#[tauri::command]
fn save_note(path: String, content: String) -> Result<(), String> {
  std::fs::write(PathBuf::from(path), content).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
  let conn = state.db.lock().unwrap();
  let mut stmt = conn.prepare("SELECT key, value FROM settings").map_err(|e| e.to_string())?;
  let rows = stmt.query_map([], |r| {
    let k: String = r.get(0)?;
    let v: String = r.get(1)?;
    Ok((k, v))
  }).map_err(|e| e.to_string())?;
  let mut map = serde_json::Map::new();
  for row in rows { let (k, v) = row.map_err(|e| e.to_string())?; map.insert(k, serde_json::Value::String(v)); }
  Ok(serde_json::Value::Object(map))
}

#[tauri::command]
fn update_settings(state: tauri::State<'_, AppState>, kv: serde_json::Value) -> Result<(), String> {
  let obj = kv.as_object().ok_or_else(|| "invalid settings".to_string())?;
  let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
  let mut conn = state.db.lock().unwrap();
  let tx = conn.transaction().map_err(|e| e.to_string())?;
  for (k, v) in obj.iter() {
    let val = v.as_str().unwrap_or("");
    tx.execute(
      "INSERT INTO settings(key, value, updated_at) VALUES(?1, ?2, ?3)
       ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=excluded.updated_at",
      params![k, val, now]
    ).map_err(|e| e.to_string())?;
  }
  tx.commit().map_err(|e| e.to_string())?;
  Ok(())
}

fn main() {
  let db = init_db().expect("db");
  let state = AppState { db: Arc::new(Mutex::new(db)), notes_dir: Arc::new(Mutex::new(None)), watcher: Arc::new(Mutex::new(None)), gen_cancel: Arc::new(Mutex::new(false)) };
  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![watch_notes, index_note, search_embeddings, generate_answer, get_note, save_note, get_settings, update_settings, get_notes, create_note, delete_note, rename_note, test_embedding, test_generate, start_generate_stream, cancel_generate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn create_note(state: tauri::State<'_, AppState>, path: String, content: Option<String>) -> Result<(), String> {
  let p = PathBuf::from(&path);
  if let Some(parent) = p.parent() { std::fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
  let initial = content.unwrap_or_else(|| "# 新笔记\n\n".to_string());
  std::fs::write(&p, initial).map_err(|e| e.to_string())?;
  index_file_conn(&state.db, p).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
  let p = PathBuf::from(&path);
  if p.exists() { std::fs::remove_file(&p).map_err(|e| e.to_string())?; }
  let conn = state.db.lock().unwrap();
  conn.execute("DELETE FROM notes WHERE path=?1", params![path]).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
fn rename_note(state: tauri::State<'_, AppState>, old_path: String, new_path: String) -> Result<(), String> {
  let op = PathBuf::from(&old_path);
  let np = PathBuf::from(&new_path);
  if let Some(parent) = np.parent() { std::fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
  std::fs::rename(&op, &np).map_err(|e| e.to_string())?;
  let conn = state.db.lock().unwrap();
  conn.execute("UPDATE notes SET path=?1 WHERE path=?2", params![np.to_string_lossy(), op.to_string_lossy()]).map_err(|e| e.to_string())?;
  drop(conn);
  index_file_conn(&state.db, np).map_err(|e| e.to_string())
}

#[tauri::command]
fn start_generate_stream(state: tauri::State<'_, AppState>, app: tauri::AppHandle, query: String) -> Result<(), String> {
  {
    let mut c = state.gen_cancel.lock().unwrap();
    *c = false;
  }
  let res = search_embeddings(state.clone(), query.clone(), Some(5)).map_err(|e| e.to_string())?;
  let items = res.get("items").and_then(|v| v.as_array()).cloned().unwrap_or_default();
  let mut combined = String::new();
  for it in items {
    if let Some(c) = it.get("content").and_then(|x| x.as_str()) { combined.push_str(c); combined.push('\n'); }
  }
  let db = state.db.clone();
  let cancel = state.gen_cancel.clone();
  std::thread::spawn(move || {
    let conn = db.lock().unwrap();
    let s = read_settings(&conn);
    let provider = s.get("provider").cloned().unwrap_or_default();
    if provider == "openai" {
      let key = s.get("openai_api_key").cloned().unwrap_or_default();
      let base = s.get("openai_base_url").cloned().unwrap_or_else(|| "https://api.openai.com".to_string());
      let model = s.get("openai_model").cloned().unwrap_or_default();
      if !key.is_empty() && !model.is_empty() {
        let client = Client::new();
        let url = format!("{}/v1/chat/completions", base.trim_end_matches('/'));
        let body = serde_json::json!({"model": model, "stream": true, "messages": [{"role":"system","content":"你是一个根据提供的参考内容进行回答的助理。"},{"role":"user","content": format!("请基于以下参考内容回答问题。\n\n问题:\n{}\n\n参考:\n{}", query, combined)}]});
        if let Ok(resp) = client.post(&url).header(AUTHORIZATION, format!("Bearer {}", key)).header(CONTENT_TYPE, "application/json").json(&body).send() {
          use std::io::BufRead;
          let mut reader = std::io::BufReader::new(resp);
          let mut line = String::new();
          loop {
            let n = reader.read_line(&mut line).ok().unwrap_or(0);
            if n == 0 { let _ = app.emit_all("votion://answer-done", serde_json::json!({})); break; }
            if *cancel.lock().unwrap() { let _ = app.emit_all("votion://answer-cancelled", serde_json::json!({})); break; }
            if let Some(payload) = line.strip_prefix("data: ") {
              if payload.trim() == "[DONE]" { let _ = app.emit_all("votion://answer-done", serde_json::json!({})); break; }
              if let Ok(v) = serde_json::from_str::<serde_json::Value>(payload) {
                if let Some(t) = v.get("choices").and_then(|c| c.get(0)).and_then(|i| i.get("delta")).and_then(|d| d.get("content")).and_then(|x| x.as_str()) {
                  let _ = app.emit_all("votion://answer-stream", serde_json::json!({"token": t}));
                }
              }
            }
            line.clear();
          }
        }
        return;
      }
    } else if provider == "ollama" {
      let base = s.get("ollama_base_url").cloned().unwrap_or_else(|| "http://localhost:11434".to_string());
      let model = s.get("ollama_model").cloned().unwrap_or_default();
      if !model.is_empty() {
        let client = Client::new();
        let url = format!("{}/api/generate", base.trim_end_matches('/'));
        let body = serde_json::json!({"model": model, "prompt": format!("请基于以下参考内容回答问题。\n\n问题:\n{}\n\n参考:\n{}", query, combined), "stream": true});
        if let Ok(resp) = client.post(&url).header(CONTENT_TYPE, "application/json").json(&body).send() {
          use std::io::BufRead;
          let mut reader = std::io::BufReader::new(resp);
          let mut line = String::new();
          loop {
            let n = reader.read_line(&mut line).ok().unwrap_or(0);
            if n == 0 { let _ = app.emit_all("votion://answer-done", serde_json::json!({})); break; }
            if *cancel.lock().unwrap() { let _ = app.emit_all("votion://answer-cancelled", serde_json::json!({})); break; }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) {
              if let Some(t) = v.get("response").and_then(|x| x.as_str()) { let _ = app.emit_all("votion://answer-stream", serde_json::json!({"token": t})); }
              if v.get("done").and_then(|x| x.as_bool()).unwrap_or(false) { let _ = app.emit_all("votion://answer-done", serde_json::json!({})); break; }
            }
            line.clear();
          }
        }
        return;
      }
    }
    for ch in combined.chars() {
      if *cancel.lock().unwrap() { let _ = app.emit_all("votion://answer-cancelled", serde_json::json!({})); return; }
      let _ = app.emit_all("votion://answer-stream", serde_json::json!({"token": ch.to_string()}));
      std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let _ = app.emit_all("votion://answer-done", serde_json::json!({}));
  });
  Ok(())
}

#[tauri::command]
fn cancel_generate(state: tauri::State<'_, AppState>) -> Result<(), String> {
  let mut c = state.gen_cancel.lock().unwrap();
  *c = true;
  Ok(())
}

#[tauri::command]
fn test_embedding(state: tauri::State<'_, AppState>, text: String) -> Result<serde_json::Value, String> {
  let conn = state.db.lock().unwrap();
  match compute_embedding_blocking(&conn, &text) {
    Ok(Some(v)) => Ok(serde_json::json!({"dim": v.len()})),
    Ok(None) => Ok(serde_json::json!({"dim": 0})),
    Err(e) => Err(e.to_string())
  }
}

#[tauri::command]
fn test_generate(state: tauri::State<'_, AppState>, query: String) -> Result<serde_json::Value, String> {
  let conn = state.db.lock().unwrap();
  match generate_with_provider(&conn, &query, "") {
    Ok(Some(ans)) => Ok(serde_json::json!({"answer": ans})),
    Ok(None) => Ok(serde_json::json!({"answer": ""})),
    Err(e) => Err(e.to_string())
  }
}

fn read_settings(conn: &Connection) -> HashMap<String, String> {
  let mut map = HashMap::new();
  if let Ok(mut stmt) = conn.prepare("SELECT key, value FROM settings") {
    if let Ok(rows) = stmt.query_map([], |r| { let k: String = r.get(0)?; let v: String = r.get(1)?; Ok((k, v)) }) {
      for row in rows { if let Ok((k, v)) = row { map.insert(k, v); } }
    }
  }
  map
}

fn compute_embedding_blocking(conn: &Connection, text: &str) -> Result<Option<Vec<f32>>> {
  let s = read_settings(conn);
  let provider = s.get("provider").map(|x| x.to_string()).unwrap_or_default();
  if provider == "openai" {
    let key = s.get("openai_api_key").cloned().unwrap_or_default();
    let base = s.get("openai_base_url").cloned().unwrap_or_else(|| "https://api.openai.com".to_string());
    let model = s.get("openai_embed_model").cloned().unwrap_or_default();
    if key.is_empty() || model.is_empty() { return Ok(None); }
    let client = Client::new();
    let url = format!("{}/v1/embeddings", base.trim_end_matches('/'));
    let body = serde_json::json!({"model": model, "input": text});
    let resp = client.post(&url).header(AUTHORIZATION, format!("Bearer {}", key)).header(CONTENT_TYPE, "application/json").json(&body).send()?;
    if !resp.status().is_success() { return Ok(None); }
    let v: serde_json::Value = resp.json()?;
    if let Some(arr) = v.get("data").and_then(|d| d.get(0)).and_then(|i| i.get("embedding")).and_then(|e| e.as_array()) {
      let mut out = Vec::with_capacity(arr.len());
      for x in arr { if let Some(f) = x.as_f64() { out.push(f as f32); } }
      return Ok(Some(out));
    }
    Ok(None)
  } else if provider == "ollama" {
    let base = s.get("ollama_base_url").cloned().unwrap_or_else(|| "http://localhost:11434".to_string());
    let model = s.get("ollama_embed_model").cloned().unwrap_or_default();
    if model.is_empty() { return Ok(None); }
    let client = Client::new();
    let url = format!("{}/api/embeddings", base.trim_end_matches('/'));
    let body = serde_json::json!({"model": model, "prompt": text});
    let resp = client.post(&url).header(CONTENT_TYPE, "application/json").json(&body).send()?;
    if !resp.status().is_success() { return Ok(None); }
    let v: serde_json::Value = resp.json()?;
    if let Some(arr) = v.get("embedding").and_then(|e| e.as_array()) {
      let mut out = Vec::with_capacity(arr.len());
      for x in arr { if let Some(f) = x.as_f64() { out.push(f as f32); } }
      return Ok(Some(out));
    }
    Ok(None)
  } else {
    Ok(None)
  }
}

fn vec_norm(v: &Vec<f32>) -> f32 { (v.iter().map(|x| x * x).sum::<f32>()).sqrt() }
fn cosine_sim(a: &Vec<f32>, b: &Vec<f32>, an: f32) -> f32 {
  if a.len() != b.len() { return 0.0; }
  let bn = vec_norm(b);
  if an == 0.0 || bn == 0.0 { return 0.0; }
  let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
  dot / (an * bn)
}

fn generate_with_provider(conn: &Connection, query: &str, context: &str) -> Result<Option<String>> {
  let s = read_settings(conn);
  let provider = s.get("provider").map(|x| x.to_string()).unwrap_or_default();
  let prompt = format!("请基于以下参考内容回答问题。\n\n问题:\n{}\n\n参考:\n{}", query, context);
  if provider == "openai" {
    let key = s.get("openai_api_key").cloned().unwrap_or_default();
    let base = s.get("openai_base_url").cloned().unwrap_or_else(|| "https://api.openai.com".to_string());
    let model = s.get("openai_model").cloned().unwrap_or_default();
    if key.is_empty() || model.is_empty() { return Ok(None); }
    let client = Client::new();
    let url = format!("{}/v1/chat/completions", base.trim_end_matches('/'));
    let body = serde_json::json!({
      "model": model,
      "messages": [
        {"role": "system", "content": "你是一个根据提供的参考内容进行回答的助理。"},
        {"role": "user", "content": prompt}
      ]
    });
    let resp = client.post(&url).header(AUTHORIZATION, format!("Bearer {}", key)).header(CONTENT_TYPE, "application/json").json(&body).send()?;
    if !resp.status().is_success() { return Ok(None); }
    let v: serde_json::Value = resp.json()?;
    if let Some(txt) = v.get("choices").and_then(|c| c.get(0)).and_then(|i| i.get("message")).and_then(|m| m.get("content")).and_then(|t| t.as_str()) { return Ok(Some(txt.to_string())); }
    Ok(None)
  } else if provider == "ollama" {
    let base = s.get("ollama_base_url").cloned().unwrap_or_else(|| "http://localhost:11434".to_string());
    let model = s.get("ollama_model").cloned().unwrap_or_default();
    if model.is_empty() { return Ok(None); }
    let client = Client::new();
    let url = format!("{}/api/generate", base.trim_end_matches('/'));
    let body = serde_json::json!({"model": model, "prompt": prompt, "stream": false});
    let resp = client.post(&url).header(CONTENT_TYPE, "application/json").json(&body).send()?;
    if !resp.status().is_success() { return Ok(None); }
    let v: serde_json::Value = resp.json()?;
    if let Some(txt) = v.get("response").and_then(|t| t.as_str()) { return Ok(Some(txt.to_string())); }
    Ok(None)
  } else {
    Ok(None)
  }
}