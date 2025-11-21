<template>
  <section>
    <h2>搜索</h2>
    <div>
      <input v-model="query" placeholder="输入问题或关键词" />
      <button @click="doSearch">搜索</button>
    </div>
    <ul>
      <li v-for="item in results" :key="item.id" @click="openEditor(item.path)" :title="item.title">
        <strong>{{ item.title }}</strong>
        <p v-html="item.snippetHtml"></p>
      </li>
    </ul>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import DOMPurify from 'dompurify'

const query = ref('')
const results = ref<Array<{ id: number; title: string; snippetHtml: string; path?: string }>>([])
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
</script>