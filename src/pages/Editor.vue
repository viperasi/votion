<template>
  <section class="editor grid-bg">
    <div class="editor-header">
      <div class="header-left">
        <button class="btn" @click="newNote" title="新建" data-tip="新建笔记">
          <svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></svg>
        </button>
        <button class="btn" @click="openNote" title="打开" data-tip="打开文件">
          <svg viewBox="0 0 24 24"><path d="M4 7h6l2 2h8v10H4z"/></svg>
        </button>
        <button class="btn" @click="saveNote" title="保存" data-tip="保存">
          <svg viewBox="0 0 24 24"><path d="M5 5h14v10H5zM9 19h6"/></svg>
        </button>
        <button class="btn" @click="duplicateNote" title="复制" data-tip="复制副本">
          <svg viewBox="0 0 24 24"><path d="M9 9h10v10H9zM5 5h10v2H7v10H5z"/></svg>
        </button>
        <button class="btn" @click="renameNote" title="重命名" data-tip="重命名">
          <svg viewBox="0 0 24 24"><path d="M5 19h14M7 5h10l-6 6-4-4"/></svg>
        </button>
        <button class="btn" @click="deleteNote" title="删除" data-tip="删除">
          <svg viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></svg>
        </button>
        <button class="btn" @click="openInOS" title="打开所在位置" data-tip="打开所在位置">
          <svg viewBox="0 0 24 24"><path d="M14 3h7v7M21 3l-9 9M5 5h6v6H5z"/></svg>
        </button>
        <button class="btn" @click="copyHtml" title="复制HTML" data-tip="复制HTML">
          <svg viewBox="0 0 24 24"><path d="M8 6l-4 6 4 6M16 6l4 6-4 6"/></svg>
        </button>
        <button class="btn" @click="exportHtml" title="导出HTML" data-tip="导出HTML">
          <svg viewBox="0 0 24 24"><path d="M6 6h12v12H6zM8 10h8M8 14h6"/></svg>
        </button>
        <div class="sort-wrap">
          <button class="btn" @click="showSort = !showSort" title="排序" data-tip="排序">
            <svg viewBox="0 0 24 24"><path d="M6 7h12M6 12h8M6 17h4"/></svg>
          </button>
          <div v-if="showSort" class="sort-menu card">
            <button class="btn" @click="setSort('time_desc')">时间倒序</button>
            <button class="btn" @click="setSort('time_asc')">时间正序</button>
            <button class="btn" @click="setSort('name_desc')">名称倒序</button>
            <button class="btn" @click="setSort('name_asc')">名称正序</button>
          </div>
        </div>
        <button class="btn" @click="cycleViewMode" :title="viewMode==='both' ? '仅显示预览' : (viewMode==='preview' ? '仅显示编辑' : '同时显示预览')" data-tip="切换预览">
          <svg viewBox="0 0 24 24"><path d="M12 5c5 0 9 7 9 7s-4 7-9 7-9-7-9-7 4-7 9-7zm0 4a3 3 0 1 0 0 6 3 3 0 0 0 0-6"/></svg>
        </button>
      </div>
      <div class="header-center">
        <input class="input title-input" v-model="docTitle" placeholder="文档标题" />
        <button class="btn" @click="applyTitle" title="应用标题">
          <svg viewBox="0 0 24 24"><path d="M6 13l4 4 8-8"/></svg>
        </button>
      </div>
      
    </div>
    <div class="content" :style="{ gridTemplateColumns: viewMode==='both' ? (sidebarWidth + 'px 6px 1fr 1fr') : (sidebarWidth + 'px 6px 1fr') }">
      <aside class="sidebar">
        <ul>
          <li v-for="n in treeList" :key="n.key" class="file" @click="openFromList(n.path)">{{ n.name }}</li>
        </ul>
        
        
        <ul>
          <li v-for="h in headings" :key="h.id" @click="scrollToHeading(h.id)" :class="['h' + h.level, h.id === activeHeadingId ? 'active' : '']">{{ h.text }}</li>
        </ul>
      </aside>
      <div class="resizer" @mousedown="onResizerDown"></div>
      <textarea v-if="viewMode!=='preview'" class="input editor-text" v-model="content" ref="taRef" />
      <div v-if="viewMode!=='editor'" class="preview" v-html="safeHtml" ref="pvRef" @click="onPreviewClick"></div>
    </div>
    <div class="status">
      <span class="path">{{ path }}</span>
      <span class="sep">·</span>
      <span>字数 {{ charCount }} / 词数 {{ wordCount }}</span>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import MarkdownIt from 'markdown-it'
import DOMPurify from 'dompurify'
import anchor from 'markdown-it-anchor'
import taskLists from 'markdown-it-task-lists'
import { invoke } from '@tauri-apps/api/tauri'
import { save } from '@tauri-apps/api/dialog'
import hljs from 'highlight.js'
import { useRoute } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { open as openShell } from '@tauri-apps/api/shell'
import { writeText } from '@tauri-apps/api/clipboard'

const md = new MarkdownIt({
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return '<pre class="hljs"><code>' + hljs.highlight(str, { language: lang, ignoreIllegals: true }).value + '</code></pre>'
      } catch (__) {}
    }
    return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>'
  }
})
md.use(anchor).use(taskLists)
const path = ref('')
const content = ref('')
const html = computed(() => md.render(content.value))
const safeHtml = computed(() => DOMPurify.sanitize(html.value))
 
const taRef = ref<HTMLTextAreaElement | null>(null)
const pvRef = ref<HTMLElement | null>(null)
const notes = ref<Array<{ id: number; path: string; title: string }>>([])
const notesDir = ref('')
const sortMode = ref<'time_desc'|'time_asc'|'name_desc'|'name_asc'>('name_asc')
const showSort = ref(false)
const route = useRoute()
 
const noteMap = computed(() => {
  const m: Record<string, { title: string; tags: string; updated_at: number }> = {}
  for (const n of notes.value as any) { m[n.path] = { title: n.title, tags: (n.tags || ''), updated_at: n.updated_at } }
  return m
})
const filteredNotes = computed(() => notes.value)
const treeList = computed(() => {
  const items = filteredNotes.value.map(n => {
    const parts = (n.path || '').split('/').filter(Boolean)
    const fileName = parts[parts.length - 1] || n.title || '未命名'
    const updated = noteMap.value[n.path]?.updated_at || 0
    return { key: 'f:' + n.path, name: fileName, path: n.path, updated }
  })
  return items.sort((a, b) => {
    if (sortMode.value === 'time_desc') return (b.updated || 0) - (a.updated || 0)
    if (sortMode.value === 'time_asc') return (a.updated || 0) - (b.updated || 0)
    if (sortMode.value === 'name_desc') return b.name.localeCompare(a.name)
    return a.name.localeCompare(b.name)
  })
})
 
const originalContent = ref('')
const dirty = computed(() => content.value !== originalContent.value)
let saveTimer: any = null
watch(content, () => {
  if (!path.value) return
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => { if (dirty.value) saveNote() }, 1000)
})
const headings = computed(() => {
  const tokens = md.parse(content.value, {})
  const hs: Array<{ id: string; text: string; level: number }> = []
  for (let i = 0; i < tokens.length; i++) {
    const t = tokens[i]
    if (t.type === 'heading_open') {
      const lvl = Number(t.tag?.replace('h', '') || 0)
      const inline = tokens[i + 1]
      const text = inline?.content || ''
      const id = (t.attrs?.find((a) => a[0] === 'id')?.[1]) || ''
      if (id) hs.push({ id, text, level: lvl })
    }
  }
  return hs
})
const activeHeadingId = ref('')
const viewMode = ref<'both'|'editor'|'preview'>('both')
const sidebarWidth = ref(280)
let startX = 0
let startW = 0
function onResizerDown(e: MouseEvent) {
  startX = e.clientX; startW = sidebarWidth.value
  document.addEventListener('mousemove', onResizerMove)
  document.addEventListener('mouseup', onResizerUp)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}
function onResizerMove(e: MouseEvent) {
  const dx = e.clientX - startX
  sidebarWidth.value = Math.max(200, Math.min(600, startW + dx))
}
function onResizerUp() {
  document.removeEventListener('mousemove', onResizerMove)
  document.removeEventListener('mouseup', onResizerUp)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}
const charCount = computed(() => content.value.length)
const wordCount = computed(() => content.value.trim().split(/\s+/).filter(Boolean).length)

async function openNote() {
  const r = await invoke<any>('get_note', { path: path.value })
  content.value = r?.content ?? ''
  originalContent.value = content.value
  const fm = parseFrontMatter(content.value)
  docTags.value = fm.tags
  docTitle.value = fm.title
}

async function saveNote() {
  await invoke('save_note', { path: path.value, content: content.value })
  if (path.value) await invoke('index_note', { path: path.value })
}

async function loadNotes() {
  const r = await invoke<any>('get_notes')
  notes.value = r?.items ?? []
}

function openFromList(p: string) {
  path.value = p
  openNote()
}
function setSort(m: 'time_desc'|'time_asc'|'name_desc'|'name_asc') { sortMode.value = m; showSort.value = false }

function onEditorScroll(e: Event) {
  const ta = taRef.value, pv = pvRef.value
  if (!ta || !pv) return
  const ratio = ta.scrollTop / (ta.scrollHeight - ta.clientHeight || 1)
  pv.scrollTop = ratio * (pv.scrollHeight - pv.clientHeight)
}

function onPreviewScroll(e: Event) {
  const ta = taRef.value, pv = pvRef.value
  if (!ta || !pv) return
  const ratio = (pv.scrollTop) / (pv.scrollHeight - pv.clientHeight || 1)
  ta.scrollTop = ratio * (ta.scrollHeight - ta.clientHeight)
  const ids = headings.value.map(h => h.id)
  for (let i = 0; i < ids.length; i++) {
    const el = pv.querySelector(`[id="${ids[i]}"]`) as HTMLElement | null
    if (el && el.offsetTop <= pv.scrollTop + 8) {
      activeHeadingId.value = ids[i]
    }
  }
}

function scrollToHeading(id: string) {
  const pv = pvRef.value
  if (!pv) return
  const el = pv.querySelector(`[id="${id}"]`) as HTMLElement | null
  if (el) {
    pv.scrollTop = el.offsetTop
  }
}

function onPreviewClick(e: MouseEvent) {
  const pv = pvRef.value
  if (!pv) return
  const target = (e.target as HTMLElement)
  const a = target.closest('a') as HTMLAnchorElement | null
  if (!a) return
  const href = a.getAttribute('href') || ''
  if (!href) return
  if (/^https?:\/\//i.test(href)) return
  const [filePart, hash] = href.split('#')
  let baseDir = ''
  if (path.value) {
    const idx = path.value.lastIndexOf('/')
    baseDir = idx >= 0 ? path.value.slice(0, idx) : ''
  }
  let targetPath = filePart
  if (filePart && !/^\//.test(filePart)) {
    targetPath = baseDir ? `${baseDir}/${filePart}` : filePart
  }
  if (targetPath) {
    path.value = targetPath
    openNote()
    if (hash) setTimeout(() => scrollToHeading(hash), 50)
  } else if (hash) {
    scrollToHeading(hash)
  }
  e.preventDefault()
}

function cycleViewMode() {
  if (viewMode.value === 'both') viewMode.value = 'preview'
  else if (viewMode.value === 'preview') viewMode.value = 'editor'
  else viewMode.value = 'both'
}

onMounted(async () => {
  await loadNotes()
  try {
    const s = await invoke<any>('get_settings')
    if (s && s['notes_dir']) notesDir.value = s['notes_dir']
  } catch {}
  const qp = route.query?.path as string | undefined
  if (qp) {
    path.value = qp
    await openNote()
  }
  const un = await listen('votion://index-updated', async (_ev) => {
    await loadNotes()
    if (path.value) await openNote()
  })
  onBeforeUnmount(() => { un() })
  window.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
      e.preventDefault(); saveNote()
    }
  })
  window.addEventListener('beforeunload', (e) => {
    if (dirty.value) { e.preventDefault(); (e as any).returnValue = '' }
  })
})

async function newNote() {
  const s = await invoke<any>('get_settings')
  const base = (s && s['notes_dir']) ? s['notes_dir'] : ''
  const p = await save({ defaultPath: base ? `${base}/新笔记.md` : undefined })
  if (typeof p === 'string' && p) {
    await invoke('create_note', { path: p, content: `# 新笔记\n\n` })
    path.value = p
    await openNote()
    await loadNotes()
  }
}

async function deleteNote() {
  if (!path.value) return
  await invoke('delete_note', { path: path.value })
  path.value = ''
  content.value = ''
  await loadNotes()
}

async function renameNote() {
  if (!path.value) return
  const baseIdx = path.value.lastIndexOf('/')
  const baseDir = baseIdx >= 0 ? path.value.slice(0, baseIdx) : ''
  const p = await save({ defaultPath: baseDir ? `${baseDir}/重命名.md` : undefined })
  if (typeof p === 'string' && p && p !== path.value) {
    await invoke('rename_note', { oldPath: path.value, newPath: p })
    path.value = p
    await openNote()
    await loadNotes()
  }
}

async function duplicateNote() {
  if (!path.value) return
  const baseIdx = path.value.lastIndexOf('/')
  const baseDir = baseIdx >= 0 ? path.value.slice(0, baseIdx) : ''
  const p = await save({ defaultPath: baseDir ? `${baseDir}/副本.md` : undefined })
  if (typeof p === 'string' && p) {
    await invoke('save_note', { path: p, content: content.value })
    await invoke('index_note', { path: p })
    await loadNotes()
  }
}

async function openInOS() {
  if (!path.value) return
  await openShell(path.value)
}

async function exportHtml() {
  const s = await save({ defaultPath: '导出.html' })
  if (typeof s === 'string' && s) {
    const doc = `<!doctype html><html><head><meta charset="utf-8"><title>${path.value}</title></head><body>${safeHtml.value}</body></html>`
    await invoke('save_note', { path: s, content: doc })
  }
}

const docTags = ref('')
const docTitle = ref('')
function parseFrontMatter(txt: string): { title: string; tags: string } {
  if (txt.startsWith('---\n')) {
    const idx = txt.indexOf('\n---', 4)
    if (idx > 0) {
      const header = txt.slice(4, idx)
      const lines = header.split('\n')
      let title = ''
      let tags = ''
      for (const line of lines) {
        const p = line.indexOf(':')
        if (p > -1) {
          const k = line.slice(0, p).trim().toLowerCase()
          const v = line.slice(p + 1).trim()
          if (k === 'title') title = v
          if (k === 'tags') tags = v
        }
      }
      return { title, tags }
    }
  }
  return { title: '', tags: '' }
}
function applyTitle() {
  const t = docTitle.value.trim()
  if (!t) return
  if (content.value.startsWith('---\n')) {
    const idx = content.value.indexOf('\n---', 4)
    if (idx > 0) {
      const header = content.value.slice(4, idx)
      const lines = header.split('\n')
      let found = false
      const newHeader = lines.map(l => {
        const p = l.indexOf(':')
        if (p > -1) {
          const k = l.slice(0, p).trim().toLowerCase()
          if (k === 'title') { found = true; return `title: ${t}` }
        }
        return l
      }).join('\n') + (found ? '' : `\ntitle: ${t}`)
      content.value = `---\n${newHeader}\n---` + content.value.slice(idx + 4)
    }
  } else {
    content.value = `---\ntitle: ${t}\n---\n` + content.value
  }
  saveNote()
}
function applyTags() {
  const t = docTags.value.trim()
  if (!t) return
  if (content.value.startsWith('---\n')) {
    const idx = content.value.indexOf('\n---', 4)
    if (idx > 0) {
      const header = content.value.slice(4, idx)
      const lines = header.split('\n')
      let found = false
      const newHeader = lines.map(l => {
        const p = l.indexOf(':')
        if (p > -1) {
          const k = l.slice(0, p).trim().toLowerCase()
          if (k === 'tags') { found = true; return `tags: ${t}` }
        }
        return l
      }).join('\n') + (found ? '' : `\ntags: ${t}`)
      content.value = `---\n${newHeader}\n---` + content.value.slice(idx + 4)
    }
  } else {
    content.value = `---\ntags: ${t}\n---\n` + content.value
  }
  saveNote()
}

async function copyHtml() {
  await writeText(safeHtml.value)
}

async function revealInFinder() {
  if (!path.value) return
  const idx = path.value.lastIndexOf('/')
  const dir = idx >= 0 ? path.value.slice(0, idx) : path.value
  await openShell(dir)
}
</script>

<style scoped>
.editor{display:grid;grid-template-rows:auto 1fr;height:100%}
.editor-header{display:grid;grid-template-columns:auto 1fr;gap:8px;align-items:center;padding:8px;border-bottom:1px solid var(--panel-border);background:linear-gradient(90deg,rgba(92,225,230,.12),rgba(123,97,255,.1))}
.editor-header{position:sticky;top:0;z-index:5}
.header-left{position:relative}
.header-left,.header-right{display:flex;gap:8px;align-items:center}
.header-center{display:flex;gap:8px;align-items:center}
.title-input{min-width:220px}
.tags-input{min-width:180px}
.path-input{min-width:220px}
.content{display:grid;grid-template-columns:280px 1fr 1fr;height:100%;overflow:hidden;min-width:0}
.sidebar{border-right:1px solid var(--panel-border);padding:8px;overflow-y:auto;overflow-x:hidden;background:var(--panel-bg)}
.sidebar ul { list-style: none; padding: 0; margin: 0; }
.sidebar li { cursor: pointer; padding: 6px 4px; border-radius: 4px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis }
.sidebar li:hover{background:var(--btn-hover)}
.sidebar li.h2 { padding-left: 8px; }
.sidebar li.h3 { padding-left: 16px; }
.sidebar li.h4 { padding-left: 24px; }
.sort-wrap{position:relative;display:inline-block}
.sort-menu{position:absolute;top:36px;right:0;left:auto;display:flex;flex-direction:column;gap:6px;padding:8px;min-width:160px;z-index:10}
.preview{padding:12px;border-left:1px solid var(--panel-border);overflow:auto;background:var(--panel-bg);height:100%;min-width:0}
.hljs{background:#0e1220}
.sidebar li.active { background: #e6f0ff; }
.toolbar input{ }
.dirty{color:var(--text-muted)}
.path{color:var(--text-secondary)}
.status { display: flex; align-items: center; gap: 8px; padding: 6px 10px; border-top: 1px solid #eee; color: #667085; font-size: 12px; }
.status { border-top: 1px solid var(--panel-border); color: var(--text-muted) }
.status .sep { color: var(--panel-border) }
.editor-text{height:100%;resize:none;font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas;line-height:1.6;min-width:0}
.editor-header .btn{position:relative}
.editor-header .btn[data-tip]::after{content:attr(data-tip);position:absolute;bottom:-30px;left:50%;transform:translateX(-50%);background:var(--panel-bg);border:1px solid var(--panel-border);color:var(--text-secondary);border-radius:6px;padding:4px 8px;white-space:nowrap;font-size:12px;opacity:0;pointer-events:none;transition:opacity .15s ease}
.editor-header .btn:hover::after{opacity:1}
</style>
.resizer{width:6px;height:100%;cursor:col-resize;background:var(--panel-border)}
.resizer:hover{background:var(--accent-end)}