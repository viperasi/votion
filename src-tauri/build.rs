fn main() {
  let _ = std::fs::create_dir_all("icons");
  const ICON_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGMAAQAABQABDQottAAAAABJRU5ErkJggg==";
  if std::fs::read("icons/icon.png").is_err() {
    if let Ok(bytes) = base64::decode(ICON_B64) { let _ = std::fs::write("icons/icon.png", bytes); }
  }
  tauri_build::build()
}