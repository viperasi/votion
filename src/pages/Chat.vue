<template>
  <section class="chat grid-bg">
    <header class="chat-header">
      <div class="title">聊天 <span class="model-badge" v-if="model">{{ providerLabel }} · {{ model }}</span></div>
      <div class="status">
        <span class="dot" v-if="streaming"></span>
      </div>
      <div class="tools">
        <button class="btn" @click="copyAnswer" :disabled="!answer">
          <svg viewBox="0 0 24 24"><path d="M9 9h10v10H9zM5 5h10v2H7v10H5z"/></svg>
          复制
        </button>
        <button class="btn" @click="clearAll">
          <svg viewBox="0 0 24 24"><path d="M6 6h12M8 9h8M10 12h6M12 15h4"/></svg>
          清空
        </button>
        <button class="btn" @click="toggleRefs">
          <svg viewBox="0 0 24 24"><path d="M6 7h12v4H6zM6 13h12v4H6z"/></svg>
          {{ refsCollapsed ? '展开引用' : '收起引用' }}
        </button>
      </div>
    </header>
    <div class="chat-container" :class="{ full: refsCollapsed }">
      <aside class="refs-panel" v-if="refs.length && !refsCollapsed">
        <div class="panel-header">引用来源</div>
        <ul class="ref-list">
          <li v-for="r in refs" :key="r.id" class="ref-item card" @click="openEditor(r.path)" :title="r.title">
            <div class="ref-title">{{ r.title }}</div>
            <div class="ref-meta">
              <span class="ref-path" v-if="r.path">{{ r.path }}</span>
              <span class="score">{{ formatScore(r.score) }}</span>
            </div>
            <div class="ref-preview">{{ r.preview }}</div>
          </li>
        </ul>
      </aside>
      <main class="chat-main">
        <div class="messages" ref="messagesRef">
          <div class="message" v-for="(m,i) in convo" :key="i" :class="m.role">
            <div class="avatar">{{ m.role==='user' ? '我' : 'AI' }}</div>
            <div class="bubble">
              <div v-if="m.role==='assistant'" v-html="safeRender(m.content)"></div>
              <div v-else>{{ m.content }}</div>
              <div class="thinking" v-if="streaming && i===currentAiIndex"><span class="spinner"></span><span>正在思考… {{ providerLabel }} · {{ model }}</span></div>
            </div>
          </div>
        </div>
      </main>
      <div class="input-bar">
        <div class="compose">
          <textarea class="compose-input" v-model="question" ref="inputRef" placeholder="请输入问题" @input="autoResize" @keydown="onKeydownInput" />
        </div>
        <div class="compose-actions">
          <div class="actions-left">
            <button class="btn symbol-btn" @click="insertSymbol('@')" title="@ 提及">@</button>
            <button class="btn symbol-btn" @click="insertSymbol('#')" title="# 主题">#</button>
          </div>
          <div class="actions-right">
            <button class="btn btn-primary send-btn" @click="send" :disabled="!question.trim()" title="发送">
              <svg viewBox="0 0 24 24"><path d="M4 12l16-8-8 16-2-6z"/></svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import DOMPurify from 'dompurify'
import { writeText } from '@tauri-apps/api/clipboard'
import { save as saveDialog } from '@tauri-apps/api/dialog'

const question = ref('')
const questionSnapshot = ref('')
const answer = ref('')
let unlisten: (() => void) | null = null
const refs = ref<Array<{ id: number; title: string; path?: string; preview: string; score: number }>>([])
const router = useRouter()
const md = new MarkdownIt({
  highlight: function (str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return '<pre class="hljs"><code>' + hljs.highlight(str, { language: lang, ignoreIllegals: true }).value + '</code></pre>'
      } catch {}
    }
    return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>'
  }
})
const safeAnswerHtml = computed(() => DOMPurify.sanitize(md.render(answer.value)))
function safeRender(t: string) { return DOMPurify.sanitize(md.render(t)) }
const messagesRef = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLTextAreaElement | null>(null)
const streaming = ref(false)
const provider = ref('')
const model = ref('')
const refsCollapsed = ref(false)
const providerLabel = computed(() => provider.value === 'openai' ? 'OpenAI' : (provider.value === 'ollama' ? 'Ollama' : ''))
const streamMode = ref(true)
const thinking = ref(false)
const convo = ref<Array<{ role: 'user' | 'assistant'; content: string }>>([])
const currentAiIndex = ref<number|null>(null)

async function ask(q?: string) {
  questionSnapshot.value = q ?? question.value
  await loadRefs()
  const res = await invoke<string>('generate_answer', { query: questionSnapshot.value })
  answer.value = res ?? ''
  thinking.value = false
  await nextTick(); scrollToBottom()
}

async function askStream(q?: string) {
  answer.value = ''
  questionSnapshot.value = q ?? question.value
  if (unlisten) { unlisten(); unlisten = null }
  await loadRefs()
  const un = await listen('votion://answer-stream', (ev: any) => {
    const t = (ev?.payload as any)?.token as string
    if (t) answer.value += t
    if (thinking.value) thinking.value = false
    if (currentAiIndex.value !== null) {
      convo.value[currentAiIndex.value].content = answer.value
    }
    scrollToBottom()
  })
  unlisten = un
  streaming.value = true
  await listen('votion://answer-done', () => { streaming.value = false; thinking.value = false; currentAiIndex.value = null })
  await listen('votion://answer-cancelled', () => { streaming.value = false; thinking.value = false; currentAiIndex.value = null })
  await invoke('start_generate_stream', { query: questionSnapshot.value })
}

function toggleStreamMode() { streamMode.value = !streamMode.value }
async function send() {
  const q = question.value.trim()
  if (!q) return
  // append messages and set assistant placeholder index
  convo.value.push({ role: 'user', content: q })
  convo.value.push({ role: 'assistant', content: '' })
  currentAiIndex.value = convo.value.length - 1
  thinking.value = true
  if (streamMode.value) { await askStream(q) } else { await ask(q); if (currentAiIndex.value !== null) { convo.value[currentAiIndex.value].content = answer.value; currentAiIndex.value = null } }
  question.value = ''
  autoResize()
}
function onKeydownInput(e: KeyboardEvent) { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send() } }

async function cancel() {
  await invoke('cancel_generate')
}

onBeforeUnmount(() => { if (unlisten) unlisten() })

async function loadRefs() {
  const r = await invoke<any>('search_embeddings', { query: question.value, topK: 5 })
  refs.value = (r?.items ?? []).map((x: any) => ({ id: x.id, title: x.title ?? '片段', path: x.path, preview: (x.content || '').slice(0, 140), score: Number(x.score ?? 0) }))
}

function formatScore(s: number) { if (!s) return '0.000'; return (Math.round(s * 1000) / 1000).toFixed(3) }

function openEditor(path?: string) { if (path) router.push({ path: '/editor', query: { path } }) }

function scrollToBottom() {
  const el = messagesRef.value
  if (!el) return
  el.scrollTop = el.scrollHeight
}

function autoResize() {
  const el = inputRef.value
  if (!el) return
  el.style.height = 'auto'
  el.style.height = Math.min(el.scrollHeight, 160) + 'px'
}

async function copyAnswer() { if (answer.value) await writeText(answer.value) }
function clearAll() { question.value = ''; questionSnapshot.value = ''; answer.value = ''; refs.value = [] }
function toggleRefs() { refsCollapsed.value = !refsCollapsed.value }

onMounted(async () => {
  const s = await invoke<any>('get_settings')
  provider.value = s?.provider || ''
  model.value = provider.value === 'openai' ? (s?.openai_model || '') : (provider.value === 'ollama' ? (s?.ollama_model || '') : '')
})

async function regenerate() {
  if (!questionSnapshot.value) return
  question.value = questionSnapshot.value
  await askStream()
}

async function exportMarkdown() {
  if (!answer.value) return
  const p = await saveDialog({ defaultPath: '回答.md' })
  if (typeof p === 'string' && p) {
    await invoke('save_note', { path: p, content: answer.value })
  }
}

function insertSymbol(sym: string) {
  const el = inputRef.value
  if (!el) return
  const start = el.selectionStart ?? question.value.length
  const end = el.selectionEnd ?? start
  const before = question.value.slice(0, start)
  const after = question.value.slice(end)
  const spacer = start > 0 && !/\s/.test(question.value[start - 1]) ? ' ' : ''
  question.value = before + spacer + sym + ' ' + after
  nextTick(() => {
    const pos = (before + spacer + sym + ' ').length
    el.selectionStart = el.selectionEnd = pos
    autoResize()
  })
}
</script>

<style scoped>
.chat{display:flex;flex-direction:column;height:100%}
.chat{overflow-x:hidden}
.chat-header{display:grid;grid-template-columns:1fr auto auto;align-items:center;gap:12px;padding:12px 16px;background:linear-gradient(90deg,rgba(92,225,230,.16),rgba(123,97,255,.12));border-bottom:1px solid var(--panel-border)}
.title{font-weight:700;font-size:16px;color:var(--text-secondary)}
.model-badge{margin-left:8px;padding:4px 8px;border-radius:999px;background:var(--card-bg);border:1px solid var(--card-border);font-size:12px;color:var(--text-secondary)}
.status{display:flex;align-items:center;gap:8px;color:var(--text-muted)}
.provider,.model{padding:4px 8px;border-radius:999px;background:var(--card-bg);border:1px solid var(--card-border)}
.dot { width: 8px; height: 8px; border-radius: 50%; background: #4f46e5; animation: pulse 1s infinite; }
@keyframes pulse { 0% { opacity: .2 } 50% { opacity: 1 } 100% { opacity: .2 } }
.tools { display: flex; gap: 8px; }
.chat-container{display:grid;grid-template-columns:minmax(260px,28%) 1fr;grid-template-rows:1fr auto;width:100%;height:100%;flex:1;min-height:0}
.chat-container.full{grid-template-columns:1fr}
.refs-panel{border-right:1px solid var(--panel-border);padding:12px;overflow:auto;background:var(--panel-bg)}
.panel-header{font-weight:600;margin-bottom:8px;color:var(--text-secondary)}
.ref-list{list-style:none;padding:0;margin:0}
.ref-item{padding:10px;border-radius:8px;cursor:pointer}
.ref-title{font-weight:600;color:var(--text-secondary)}
.ref-meta{font-size:12px;color:var(--text-muted);margin-top:6px}
.ref-path{color:var(--text-muted);margin-right:8px}
.ref-preview{font-size:13px;color:var(--text-primary);margin-top:6px}
.chat-main { display: grid; grid-template-rows: 1fr; min-height: 0 }
.messages { overflow: auto; padding: 16px; }
.message { display: grid; grid-template-columns: 40px 1fr; gap: 10px; margin-bottom: 14px; align-items: flex-start; }
.avatar { width: 32px; height: 32px; border-radius: 50%; background: #e3e7ff; display: flex; align-items: center; justify-content: center; font-size: 12px; color: #333; box-shadow: 0 1px 2px rgba(0,0,0,.06); }
.message.user .avatar { background: #ffd7d7; }
.bubble{background:var(--card-bg);border:1px solid var(--panel-border);border-radius:12px;padding:12px 14px;line-height:1.7;box-shadow:0 1px 2px rgba(0,0,0,.04);color:var(--text-primary)}
.message.assistant .bubble{background:var(--panel-bg)}
.typing { display: flex; gap: 4px; padding: 6px 12px; }
.typing span { width: 6px; height: 6px; border-radius: 50%; background: #9aa5b1; animation: blink 1s infinite; }
.typing span:nth-child(2) { animation-delay: .2s }
.typing span:nth-child(3) { animation-delay: .4s }
@keyframes blink { 0% { opacity: .2 } 50% { opacity: 1 } 100% { opacity: .2 } }
.input-bar{border-top:1px solid var(--panel-border);padding:12px;background:var(--panel-bg);grid-column:1/-1;box-sizing:border-box;overflow-x:hidden;position:sticky;bottom:0;z-index:5;display:grid;grid-template-columns:1fr;grid-template-rows:auto auto;gap:8px}
.compose{display:flex;align-items:center;background:var(--input-bg);border:1px solid var(--input-border);border-radius:5px;padding:0 8px}
.compose-input{flex:1;min-height:44px;max-height:160px;line-height:1.7;border:none;outline:none;background:transparent;color:var(--text-primary);resize:none;padding:10px 12px}
.symbol-btn{height:36px;min-width:36px;border-radius:999px;background:transparent;border-color:transparent;color:var(--text-primary)}
.send-btn{height:36px;border-radius:999px}
.compose-actions{display:flex;align-items:center;justify-content:space-between}
.actions-left,.actions-right{display:flex;gap:8px;align-items:center}
.thinking{display:flex;align-items:center;gap:8px;color:var(--text-muted)}
.spinner{width:14px;height:14px;border-radius:50%;border:2px solid rgba(123,97,255,.35);border-top-color:#7b61ff;animation:spin 1s linear infinite}
@keyframes spin{to{transform:rotate(360deg)}}
.compose-left{display:flex;gap:8px;align-items:center}
.compose-right{display:flex;gap:8px;align-items:center;justify-content:flex-end}
.score{margin-left:8px;color:var(--text-muted)}
textarea.input{width:100%;resize:none;min-height:44px;max-height:160px;line-height:1.7;border-radius:12px;transition:border-color .15s ease, box-shadow .15s ease;display:block;box-sizing:border-box}
textarea.input:focus{outline:none;border-color:#7b61ff;box-shadow:0 0 0 2px rgba(123,97,255,.25)}
textarea.input::placeholder{color:var(--text-muted)}
</style>