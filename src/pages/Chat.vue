<template>
  <section>
    <h2>智能问答</h2>
    <div>
      <textarea v-model="question" placeholder="请输入问题" />
      <button @click="ask">提问</button>
      <button @click="askStream">流式提问</button>
      <button @click="cancel">取消</button>
    </div>
    <div class="answer">
      <pre>{{ answer }}</pre>
    </div>
    <div class="refs" v-if="refs.length">
      <h3>引用来源</h3>
      <ul>
        <li v-for="r in refs" :key="r.id" @click="openEditor(r.path)" :title="r.title">
          <strong>{{ r.title }}</strong>
          <span class="score">相似度: {{ formatScore(r.score) }}</span>
          <p>{{ r.preview }}</p>
        </li>
      </ul>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'

const question = ref('')
const answer = ref('')
let unlisten: (() => void) | null = null
const refs = ref<Array<{ id: number; title: string; path?: string; preview: string; score: number }>>([])
const router = useRouter()

async function ask() {
  await loadRefs()
  const res = await invoke<string>('generate_answer', { query: question.value })
  answer.value = res ?? ''
}

async function askStream() {
  answer.value = ''
  if (unlisten) { unlisten(); unlisten = null }
  await loadRefs()
  const un = await listen('votion://answer-stream', (ev: any) => {
    const t = (ev?.payload as any)?.token as string
    if (t) answer.value += t
  })
  unlisten = un
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
</script>

<style scoped>
.answer { white-space: pre-wrap; border: 1px solid #eee; padding: 8px; margin-top: 8px; }
.refs { margin-top: 12px; }
.refs ul { list-style: none; padding: 0; }
.refs li { cursor: pointer; padding: 6px; border-radius: 4px; }
.refs li:hover { background: #f5f5f5; }
.score { margin-left: 8px; color: #666; }
</style>