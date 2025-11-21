<template>
  <section class="chat">
    <header class="chat-header">
      <div class="title">聊天</div>
      <div class="status">
        <span class="provider" v-if="provider">{{ providerLabel }}</span>
        <span class="model" v-if="model">{{ model }}</span>
        <span class="dot" v-if="streaming"></span>
      </div>
      <div class="tools">
        <button @click="copyAnswer" :disabled="!answer">复制</button>
        <button @click="clearAll">清空</button>
        <button @click="toggleRefs">{{ refsCollapsed ? '展开引用' : '收起引用' }}</button>
      </div>
    </header>
    <div class="chat-container">
      <aside class="refs-panel" v-if="refs.length && !refsCollapsed">
        <div class="panel-header">引用来源</div>
        <ul class="ref-list">
          <li v-for="r in refs" :key="r.id" class="ref-item" @click="openEditor(r.path)" :title="r.title">
            <div class="ref-title">{{ r.title }}</div>
            <div class="ref-meta">
              <span class="score">{{ formatScore(r.score) }}</span>
            </div>
            <div class="ref-preview">{{ r.preview }}</div>
          </li>
        </ul>
      </aside>
      <main class="chat-main">
        <div class="messages" ref="messagesRef">
          <div class="message user" v-if="questionSnapshot">
            <div class="avatar">我</div>
            <div class="bubble">{{ questionSnapshot }}</div>
          </div>
          <div class="message assistant" v-if="answer">
            <div class="avatar">AI</div>
            <div class="bubble" v-html="safeAnswerHtml"></div>
            <div class="typing" v-if="streaming"><span></span><span></span><span></span></div>
          </div>
        </div>
        <div class="input-bar">
          <textarea v-model="question" ref="inputRef" placeholder="请输入问题" @input="autoResize" />
          <div class="actions">
            <button @click="ask">提问</button>
            <button @click="askStream">流式提问</button>
            <button @click="cancel">取消</button>
          </div>
        </div>
      </main>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'
import MarkdownIt from 'markdown-it'
import DOMPurify from 'dompurify'
import { writeText } from '@tauri-apps/api/clipboard'

const question = ref('')
const questionSnapshot = ref('')
const answer = ref('')
let unlisten: (() => void) | null = null
const refs = ref<Array<{ id: number; title: string; path?: string; preview: string; score: number }>>([])
const router = useRouter()
const md = new MarkdownIt()
const safeAnswerHtml = computed(() => DOMPurify.sanitize(md.render(answer.value)))
const messagesRef = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLTextAreaElement | null>(null)
const streaming = ref(false)
const provider = ref('')
const model = ref('')
const refsCollapsed = ref(false)
const providerLabel = computed(() => provider.value === 'openai' ? 'OpenAI' : (provider.value === 'ollama' ? 'Ollama' : ''))

async function ask() {
  questionSnapshot.value = question.value
  await loadRefs()
  const res = await invoke<string>('generate_answer', { query: question.value })
  answer.value = res ?? ''
  await nextTick(); scrollToBottom()
}

async function askStream() {
  answer.value = ''
  questionSnapshot.value = question.value
  if (unlisten) { unlisten(); unlisten = null }
  await loadRefs()
  const un = await listen('votion://answer-stream', (ev: any) => {
    const t = (ev?.payload as any)?.token as string
    if (t) answer.value += t
    scrollToBottom()
  })
  unlisten = un
  streaming.value = true
  await listen('votion://answer-done', () => { streaming.value = false })
  await listen('votion://answer-cancelled', () => { streaming.value = false })
  await invoke('start_generate_stream', { query: question.value })
}

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
  el.style.height = Math.min(el.scrollHeight, 200) + 'px'
}

async function copyAnswer() { if (answer.value) await writeText(answer.value) }
function clearAll() { question.value = ''; questionSnapshot.value = ''; answer.value = ''; refs.value = [] }
function toggleRefs() { refsCollapsed.value = !refsCollapsed.value }

onMounted(async () => {
  const s = await invoke<any>('get_settings')
  provider.value = s?.provider || ''
  model.value = provider.value === 'openai' ? (s?.openai_model || '') : (provider.value === 'ollama' ? (s?.ollama_model || '') : '')
})
</script>

<style scoped>
.chat { display: flex; flex-direction: column; height: 100%; background: #f6f8fb; }
.chat-header { display: grid; grid-template-columns: 1fr auto auto; align-items: center; gap: 12px; padding: 12px 16px; background: linear-gradient(90deg, #eef3ff, #f6f8fb); border-bottom: 1px solid #e6eaf0; }
.title { font-weight: 700; font-size: 16px; }
.status { display: flex; align-items: center; gap: 8px; color: #667085; }
.provider, .model { padding: 4px 8px; border-radius: 999px; background: #fff; border: 1px solid #e6eaf0; }
.dot { width: 8px; height: 8px; border-radius: 50%; background: #4f46e5; animation: pulse 1s infinite; }
@keyframes pulse { 0% { opacity: .2 } 50% { opacity: 1 } 100% { opacity: .2 } }
.tools { display: flex; gap: 8px; }
.tools button { padding: 6px 10px; border: 1px solid #d0d5dd; border-radius: 6px; background: #fff; }
.tools button:hover { background: #f2f4f7; }
.chat-container { display: grid; grid-template-columns: 320px 1fr; width: 100%; height: calc(100% - 54px); }
.refs-panel { border-right: 1px solid #e6eaf0; padding: 12px; overflow: auto; background: #fff; }
.panel-header { font-weight: 600; margin-bottom: 8px; }
.ref-list { list-style: none; padding: 0; margin: 0; }
.ref-item { padding: 10px; border-radius: 8px; cursor: pointer; border: 1px solid transparent; }
.ref-item:hover { background: #f7f9fc; border-color: #e6eaf0; }
.ref-title { font-weight: 600; color: #1f2937; }
.ref-meta { font-size: 12px; color: #667085; margin-top: 6px; }
.ref-preview { font-size: 13px; color: #475467; margin-top: 6px; }
.chat-main { display: grid; grid-template-rows: 1fr auto; }
.messages { overflow: auto; padding: 16px; }
.message { display: grid; grid-template-columns: 40px 1fr; gap: 10px; margin-bottom: 14px; align-items: flex-start; }
.avatar { width: 32px; height: 32px; border-radius: 50%; background: #e3e7ff; display: flex; align-items: center; justify-content: center; font-size: 12px; color: #333; box-shadow: 0 1px 2px rgba(0,0,0,.06); }
.message.user .avatar { background: #ffd7d7; }
.bubble { background: #fff; border: 1px solid #e6eaf0; border-radius: 12px; padding: 12px 14px; line-height: 1.7; box-shadow: 0 1px 2px rgba(0,0,0,.04); }
.message.assistant .bubble { background: #f8fafc; }
.typing { display: flex; gap: 4px; padding: 6px 12px; }
.typing span { width: 6px; height: 6px; border-radius: 50%; background: #9aa5b1; animation: blink 1s infinite; }
.typing span:nth-child(2) { animation-delay: .2s }
.typing span:nth-child(3) { animation-delay: .4s }
@keyframes blink { 0% { opacity: .2 } 50% { opacity: 1 } 100% { opacity: .2 } }
.input-bar { border-top: 1px solid #e6eaf0; padding: 12px; display: grid; grid-template-columns: 1fr auto; gap: 10px; background: #fff; }
.input-bar textarea { width: 100%; min-height: 48px; max-height: 200px; resize: none; border: 1px solid #d0d5dd; border-radius: 10px; padding: 10px; background: #f9fafb; }
.actions { display: flex; gap: 8px; }
.actions button { padding: 8px 12px; border: 1px solid #d0d5dd; border-radius: 8px; background: #fff; }
.actions button:hover { background: #f2f4f7; }
.score { margin-left: 8px; color: #667085; }
</style>