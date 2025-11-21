<template>
  <section class="home grid-bg">
    <header class="home-header">
      <div class="title">主页</div>
      <div class="tools">
        <button class="btn" @click="refreshRecent">
          <svg viewBox="0 0 24 24"><path d="M12 6a6 6 0 1 1-5.3 3H4l3-3 3 3H8.7A4 4 0 1 0 12 8"/></svg>
          刷新最近
        </button>
      </div>
    </header>
    <div class="home-container">
      <main class="search-panel">
        <div class="search-bar">
          <input class="input" v-model="query" placeholder="输入问题或关键词" />
          <button class="btn" @click="doSearch">
            <svg viewBox="0 0 24 24"><path d="M11 4a7 7 0 1 1 0 14 7 7 0 0 1 0-14zm8 14l-4-4"/></svg>
            搜索
          </button>
        </div>
        <div class="results">
          <div class="result card" v-for="item in results" :key="item.id" @click="openEditor(item.path)" :title="item.title">
            <div class="result-title">{{ item.title }}</div>
            <div class="result-snippet" v-html="item.snippetHtml"></div>
          </div>
        </div>
        <div class="quick-actions">
          <button class="btn" @click="createNote">
            <svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></svg>
            新建笔记
          </button>
          <button class="btn" @click="openSettings">
            <svg viewBox="0 0 24 24"><path d="M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0-6l2 3 3 1-1 3 1 3-3 1-2 3-2-3-3-1 1-3-1-3 3-1 2-3z"/></svg>
            打开设置
          </button>
          <button class="btn" @click="goChat">
            <svg viewBox="0 0 24 24"><path d="M4 5h16v10H7l-3 4z"/></svg>
            进入问答
          </button>
          <button class="btn" @click="goEditor">
            <svg viewBox="0 0 24 24"><path d="M7 5h10l-6 6-4-4M5 19h14"/></svg>
            进入编辑器
          </button>
        </div>
      </main>
      <aside class="recent-panel">
        <div class="panel-header">最近活动</div>
        <ul class="recent-list">
          <li class="recent-item card" v-for="r in recent" :key="r.path" @click="openEditor(r.path)">
            <div class="recent-head">
              <svg viewBox="0 0 24 24"><path d="M5 4h10l4 4v12H5z"/></svg>
              <div class="recent-title">{{ r.title || r.path }}</div>
            </div>
            <div class="recent-meta">
              <span class="recent-path">{{ r.path }}</span>
              <span>{{ formatTime(r.updated_at) }}</span>
            </div>
          </li>
        </ul>
      </aside>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import DOMPurify from 'dompurify'

const query = ref('')
const results = ref<Array<{ id: number; title: string; snippetHtml: string; path?: string }>>([])
const recent = ref<Array<{ path: string; title: string; updated_at: number }>>([])
const router = useRouter()

async function doSearch() {
  const r = await invoke<any>('search_embeddings', { query: query.value, topK: 10 })
  results.value = (r?.items ?? []).map((x: any) => ({ id: x.id, title: x.title ?? '片段', snippetHtml: highlight(x.content, query.value), path: x.path }))
}

function openEditor(path?: string) {
  if (path) router.push({ path: '/editor', query: { path } })
}

function highlight(text: string, q: string): string {
  const esc = (s: string) => s.replace(/[&<>"]/g, (ch) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[ch] as string))
  const t = esc(text)
  if (!q) return DOMPurify.sanitize(t)
  const re = new RegExp(q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi')
  const html = t.replace(re, (m) => `<mark>${m}</mark>`)
  return DOMPurify.sanitize(html)
}

async function refreshRecent() {
  const r = await invoke<any>('get_notes')
  const items = (r?.items ?? []) as Array<{ path: string; title: string; updated_at: number }>
  recent.value = items.slice(0, 10)
}

function openSettings() { router.push({ path: '/settings' }) }
function goChat() { router.push({ path: '/chat' }) }
function goEditor() { router.push({ path: '/editor' }) }

async function createNote() {
  const s = await invoke<any>('get_settings')
  const base = (s && s['notes_dir']) ? s['notes_dir'] : ''
  const path = base ? `${base}/新笔记.md` : `新笔记.md`
  await invoke('save_note', { path, content: `# 新笔记\n\n` })
  await invoke('index_note', { path })
  await refreshRecent()
}

function formatTime(ts: number) { try { return new Date(ts * 1000).toLocaleString() } catch { return '' } }

refreshRecent()
</script>
<style scoped>
.home{display:flex;flex-direction:column;height:100%}
.home-header{display:grid;grid-template-columns:1fr auto;align-items:center;gap:12px;padding:12px 16px;background:linear-gradient(90deg,rgba(92,225,230,.16),rgba(123,97,255,.12));border-bottom:1px solid var(--panel-border)}
.title{font-weight:700;font-size:16px;color:var(--text-secondary)}
.home-container{display:grid;grid-template-columns:1fr 320px;height:calc(100% - 54px)}
.search-panel{padding:16px;overflow:auto}
.search-bar{display:grid;grid-template-columns:1fr auto;gap:8px}
.results{margin-top:16px;display:grid;gap:10px}
.result{padding:10px 12px;cursor:pointer}
.result-title{font-weight:600;color:var(--text-secondary)}
.result-snippet{font-size:13px;color:var(--text-primary);margin-top:6px}
.quick-actions{margin-top:20px;display:flex;gap:10px}
.recent-panel{border-left:1px solid var(--panel-border);padding:12px;background:var(--panel-bg)}
.panel-header{font-weight:600;margin-bottom:8px;color:var(--text-secondary)}
.recent-list{list-style:none;padding:0;margin:0}
.recent-item{padding:8px;border-radius:8px;cursor:pointer}
.recent-head{display:flex;align-items:center;gap:8px}
.recent-item svg{width:16px;height:16px;display:block;overflow:visible;stroke:currentColor;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;fill:none;color:var(--text-secondary)}
.recent-path{color:var(--text-muted);margin-right:8px}
.recent-title{font-weight:600;color:var(--text-secondary)}
.recent-meta{font-size:12px;color:var(--text-muted);margin-top:4px}
</style>
