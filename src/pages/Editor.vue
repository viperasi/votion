<template>
  <section class="editor grid-bg">
    <div class="toolbar">
      <input class="input" v-model="path" placeholder="文件路径" />
      <button class="btn" @click="openNote" title="打开">
        <svg viewBox="0 0 24 24"><path d="M4 7h6l2 2h8v10H4z"/></svg>
      </button>
      <button class="btn" @click="saveNote" title="保存">
        <svg viewBox="0 0 24 24"><path d="M5 5h14v10H5zM9 19h6"/></svg>
      </button>
      <button class="btn" @click="newNote" title="新建">
        <svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></svg>
      </button>
      <button class="btn" @click="deleteNote" title="删除">
        <svg viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></svg>
      </button>
      <button class="btn" @click="loadNotes" title="刷新列表">
        <svg viewBox="0 0 24 24"><path d="M12 6a6 6 0 1 1-5.3 3H4l3-3 3 3H8.7A4 4 0 1 0 12 8"/></svg>
      </button>
      <button class="btn" @click="renameNote" title="重命名">
        <svg viewBox="0 0 24 24"><path d="M5 19h14M7 5h10l-6 6-4-4"/></svg>
      </button>
      <button class="btn" @click="openInOS" title="在系统中打开">
        <svg viewBox="0 0 24 24"><path d="M14 3h7v7M21 3l-9 9M5 5h6v6H5z"/></svg>
      </button>
      <button class="btn" @click="duplicateNote" title="复制为...">
        <svg viewBox="0 0 24 24"><path d="M9 9h10v10H9zM5 5h10v2H7v10H5z"/></svg>
      </button>
      <input class="input" v-model="docTitle" placeholder="文档标题" />
      <button class="btn" @click="applyTitle" title="应用标题">
        <svg viewBox="0 0 24 24"><path d="M6 13l4 4 8-8"/></svg>
      </button>
      <button class="btn" @click="copyHtml" title="复制HTML">
        <svg viewBox="0 0 24 24"><path d="M8 6l-4 6 4 6M16 6l4 6-4 6"/></svg>
      </button>
      <input class="input" v-model="docTags" placeholder="文档标签（逗号分隔）" />
      <button class="btn" @click="applyTags" title="应用标签">
        <svg viewBox="0 0 24 24"><path d="M6 7h12v4H6zM6 13h8v4H6z"/></svg>
      </button>
      <button class="btn" @click="revealInFinder" title="在Finder中显示">
        <svg viewBox="0 0 24 24"><path d="M12 5c5 0 9 7 9 7s-4 7-9 7-9-7-9-7 4-7 9-7zm0 4a3 3 0 1 0 0 6 3 3 0 0 0 0-6"/></svg>
      </button>
      <button class="btn" @click="exportHtml" title="导出HTML">
        <svg viewBox="0 0 24 24"><path d="M6 6h12v12H6zM8 10h8M8 14h6"/></svg>
      </button>
      <button class="btn" @click="showPreview = !showPreview" :title="showPreview ? '隐藏预览' : '显示预览'">
        <svg viewBox="0 0 24 24"><path d="M12 5c5 0 9 7 9 7s-4 7-9 7-9-7-9-7 4-7 9-7zm0 4a3 3 0 1 0 0 6 3 3 0 0 0 0-6"/></svg>
      </button>
      <span class="dirty" v-if="dirty">未保存</span>
    </div>
    <div class="content" :style="{ gridTemplateColumns: showPreview ? '280px 1fr 1fr' : '280px 1fr' }">
      <aside class="sidebar">
        <h3 class="panel-header">笔记</h3>
        <input class="input" v-model="noteFilter" placeholder="筛选标题或路径" />
        <input class="input" v-model="tagFilter" placeholder="按标签筛选" />
        <select class="input" v-model="sortMode">
          <option value="path">按路径</option>
          <option value="updated">按更新时间</option>
        </select>
        <ul>
          <li v-for="n in treeList" :key="n.key" :class="n.type" :style="{ paddingLeft: (8 + n.depth * 12) + 'px' }" @click="onTreeClick(n)">
            <template v-if="n.type === 'dir'">{{ collapsed[n.path] ? '📁' : '📂' }} {{ n.name }}</template>
            <template v-else>{{ n.name }} <small v-if="n.tags">{{ n.tags }}</small></template>
          </li>
        </ul>
        <h3 class="panel-header">目录</h3>
        <input class="input" v-model="findQuery" placeholder="查找当前文档" />
        <ul>
          <li v-for="h in headings" :key="h.id" @click="scrollToHeading(h.id)" :class="['h' + h.level, h.id === activeHeadingId ? 'active' : '']">{{ h.text }}</li>
        </ul>
      </aside>
      <textarea class="input editor-text" v-model="content" ref="taRef" @scroll="onEditorScroll" />
      <div v-if="showPreview" class="preview" v-html="safeHighlightedHtml" ref="pvRef" @scroll="onPreviewScroll" @click="onPreviewClick"></div>
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
const findQuery = ref('')
function escapeReg(s: string) { return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') }
const safeHighlightedHtml = computed(() => {
  if (!findQuery.value) return safeHtml.value
  const re = new RegExp(escapeReg(findQuery.value), 'gi')
  const marked = html.value.replace(re, (m) => `<mark>${m}</mark>`)
  return DOMPurify.sanitize(marked)
})
const taRef = ref<HTMLTextAreaElement | null>(null)
const pvRef = ref<HTMLElement | null>(null)
const notes = ref<Array<{ id: number; path: string; title: string }>>([])
const route = useRoute()
const noteFilter = ref('')
const tagFilter = ref('')
const sortMode = ref<'path' | 'updated'>('path')
const noteMap = computed(() => {
  const m: Record<string, { title: string; tags: string; updated_at: number }> = {}
  for (const n of notes.value as any) { m[n.path] = { title: n.title, tags: (n.tags || ''), updated_at: n.updated_at } }
  return m
})
const filteredNotes = computed(() => {
  const f = noteFilter.value.trim().toLowerCase()
  const tf = tagFilter.value.trim().toLowerCase()
  if (!f) return notes.value
  return notes.value.filter(n => {
    const base = (n.title || '').toLowerCase().includes(f) || (n.path || '').toLowerCase().includes(f)
    if (!tf) return base
    const tags = (noteMap.value[n.path]?.tags || '').toLowerCase()
    return base && tags.includes(tf)
  })
})
const collapsed = ref<Record<string, boolean>>({})
const treeList = computed(() => {
  const dirs = new Set<string>()
  const items: Array<{ key: string; type: 'dir' | 'file'; name: string; path: string; depth: number; tags?: string; updated?: number }> = []
  const src = filteredNotes.value
  for (const n of src) {
    const parts = (n.path || '').split('/').filter(Boolean)
    let acc = ''
    for (let i = 0; i < parts.length - 1; i++) {
      acc = acc ? acc + '/' + parts[i] : parts[i]
      if (!dirs.has(acc)) {
        dirs.add(acc)
        items.push({ key: 'd:' + acc, type: 'dir', name: parts[i], path: acc, depth: i })
      }
    }
    const fileName = parts[parts.length - 1] || n.title || '未命名'
    const meta = noteMap.value[n.path]
    items.push({ key: 'f:' + n.path, type: 'file', name: fileName, path: n.path, depth: Math.max(parts.length - 1, 0), tags: meta?.tags, updated: meta?.updated_at })
  }
  items.sort((a, b) => {
    if (sortMode.value === 'updated') {
      const au = a.updated || 0, bu = b.updated || 0
      if (a.type === 'dir' && b.type === 'dir') return a.path.localeCompare(b.path)
      if (a.type === 'dir') return -1
      if (b.type === 'dir') return 1
      return bu - au
    }
    return a.path.localeCompare(b.path)
  })
  const visible: typeof items = []
  const hiddenSet = new Set<string>()
  for (const it of items) {
    if (it.type === 'dir') {
      visible.push(it)
      if (collapsed.value[it.path]) {
        hiddenSet.add(it.path + '/')
      }
    } else {
      let hide = false
      for (const prefix of hiddenSet) {
        if ((it.path + '/').startsWith(prefix)) { hide = true; break }
      }
      if (!hide) visible.push(it)
    }
  }
  return visible
})
function onTreeClick(n: { type: 'dir' | 'file'; path: string }) {
  if (n.type === 'dir') {
    collapsed.value[n.path] = !collapsed.value[n.path]
  } else {
    openFromList(n.path)
  }
}
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
const showPreview = ref(true)
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

onMounted(async () => {
  await loadNotes()
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
.editor{display:flex;flex-direction:column;height:100%}
.toolbar{display:flex;gap:8px;padding:8px;border-bottom:1px solid var(--panel-border);background:linear-gradient(90deg,rgba(92,225,230,.12),rgba(123,97,255,.1))}
.content{display:grid;grid-template-columns:280px 1fr 1fr;height:100%}
.sidebar{border-right:1px solid var(--panel-border);padding:8px;overflow:auto;background:var(--panel-bg)}
.sidebar ul { list-style: none; padding: 0; margin: 0; }
.sidebar li { cursor: pointer; padding: 6px 4px; border-radius: 4px; }
.sidebar li:hover{background:var(--btn-hover)}
.sidebar li.h2 { padding-left: 8px; }
.sidebar li.h3 { padding-left: 16px; }
.sidebar li.h4 { padding-left: 24px; }
.preview{padding:12px;border-left:1px solid var(--panel-border);overflow:auto;background:var(--panel-bg)}
.hljs{background:#0e1220}
.sidebar li.active { background: #e6f0ff; }
.toolbar input{ }
.dirty{color:var(--text-muted)}
.path{color:var(--text-secondary)}
.status { display: flex; align-items: center; gap: 8px; padding: 6px 10px; border-top: 1px solid #eee; color: #667085; font-size: 12px; }
.status { border-top: 1px solid var(--panel-border); color: var(--text-muted) }
.status .sep { color: var(--panel-border) }
.editor-text{height:100%;resize:none;font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas;line-height:1.6}
</style>