<template>
  <section>
    <h2>设置</h2>
    <div>
      <label>笔记目录</label>
      <input v-model="notesDir" />
      <button @click="applyWatch">开始监听</button>
      <button @click="pickDir">选择目录</button>
      <button @click="reindex">重建索引</button>
    </div>
    <div>
      <h3>AI 配置</h3>
      <label>提供者</label>
      <select v-model="provider">
        <option value="openai">OpenAI</option>
        <option value="ollama">Ollama</option>
      </select>
      <div v-if="provider==='openai'">
        <label>OpenAI API Key</label>
        <input v-model="openaiApiKey" type="password" />
        <label>OpenAI Base URL</label>
        <input v-model="openaiBaseUrl" placeholder="可选，如 https://api.openai.com" />
        <label>大模型</label>
        <input v-model="openaiModel" placeholder="如 gpt-4o-mini" />
        <label>嵌入模型</label>
        <input v-model="openaiEmbedModel" placeholder="如 text-embedding-3-large" />
      </div>
      <div v-if="provider==='ollama'">
        <label>Ollama Base URL</label>
        <input v-model="ollamaBaseUrl" placeholder="如 http://localhost:11434" />
        <label>大模型</label>
        <input v-model="ollamaModel" placeholder="如 llama3.1:8b" />
        <label>嵌入模型</label>
        <input v-model="ollamaEmbedModel" placeholder="如 nomic-embed-text" />
      </div>
      <div>
        <label>温度</label>
        <input v-model.number="temperature" type="number" step="0.1" min="0" max="2" />
        <label>最大Tokens</label>
        <input v-model.number="maxTokens" type="number" min="1" />
        <label>系统提示</label>
        <textarea v-model="systemPrompt" placeholder="系统提示"></textarea>
      </div>
      <button @click="saveAi">保存AI配置</button>
      <div>
        <label>测试嵌入文本</label>
        <input v-model="testText" placeholder="输入一句话" />
        <button @click="doTestEmbed">测试嵌入</button>
        <span v-if="embedDim!==null">维度: {{ embedDim }}</span>
      </div>
      <div>
        <label>测试问答问题</label>
        <input v-model="testQuestion" placeholder="输入问题" />
        <button @click="doTestGenerate">测试问答</button>
        <pre v-if="testAnswer">{{ testAnswer }}</pre>
      </div>
    </div>
    <div>
      <h3>知识库配置</h3>
      <label>分块大小</label>
      <input v-model.number="chunkSize" type="number" min="1" />
      <label>分块重叠</label>
      <input v-model.number="chunkOverlap" type="number" min="0" />
      <label>检索Top-K</label>
      <input v-model.number="searchTopK" type="number" min="1" />
      <label>最小相似度</label>
      <input v-model.number="minSim" type="number" step="0.01" min="0" max="1" />
      <button @click="saveKb">保存知识库配置</button>
    </div>
    <div>
      <h3>MCP 配置</h3>
      <label>端点列表（逗号分隔）</label>
      <input v-model="mcpEndpoints" placeholder="如 http://localhost:4000,http://tools.local" />
      <button @click="saveMcp">保存MCP配置</button>
    </div>
    <div>
      <h3>Prompt 配置</h3>
      <label>用户提示模板</label>
      <textarea v-model="userPromptTemplate" placeholder="模板中可包含 {query} 与 {context}"></textarea>
      <button @click="savePrompt">保存Prompt配置</button>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'

const notesDir = ref('')
const provider = ref<'openai'|'ollama'>('openai')
const openaiApiKey = ref('')
const openaiBaseUrl = ref('')
const openaiModel = ref('')
const openaiEmbedModel = ref('')
const ollamaBaseUrl = ref('')
const ollamaModel = ref('')
const ollamaEmbedModel = ref('')
const temperature = ref<number>(0.7)
const maxTokens = ref<number>(1024)
const systemPrompt = ref('你是一个根据提供的参考内容进行回答的助理。')
const chunkSize = ref<number>(800)
const chunkOverlap = ref<number>(200)
const searchTopK = ref<number>(5)
const minSim = ref<number>(0.0)
const mcpEndpoints = ref('')
const userPromptTemplate = ref('请基于以下参考内容回答问题。\n\n问题:\n{query}\n\n参考:\n{context}')
const testText = ref('')
const embedDim = ref<number|null>(null)
const testQuestion = ref('')
const testAnswer = ref('')

async function applyWatch() {
  await invoke('watch_notes', { dir: notesDir.value })
  await invoke('update_settings', { kv: { notes_dir: notesDir.value } })
}

onMounted(async () => {
  const s = await invoke<any>('get_settings')
  if (s && s['notes_dir']) notesDir.value = s['notes_dir']
  if (s && s['provider']) provider.value = s['provider']
  if (s && s['openai_api_key']) openaiApiKey.value = s['openai_api_key']
  if (s && s['openai_base_url']) openaiBaseUrl.value = s['openai_base_url']
  if (s && s['openai_model']) openaiModel.value = s['openai_model']
  if (s && s['openai_embed_model']) openaiEmbedModel.value = s['openai_embed_model']
  if (s && s['ollama_base_url']) ollamaBaseUrl.value = s['ollama_base_url']
  if (s && s['ollama_model']) ollamaModel.value = s['ollama_model']
  if (s && s['ollama_embed_model']) ollamaEmbedModel.value = s['ollama_embed_model']
  if (s && s['temperature']) temperature.value = Number(s['temperature'])
  if (s && s['max_tokens']) maxTokens.value = Number(s['max_tokens'])
  if (s && s['system_prompt']) systemPrompt.value = s['system_prompt']
  if (s && s['chunk_size']) chunkSize.value = Number(s['chunk_size'])
  if (s && s['chunk_overlap']) chunkOverlap.value = Number(s['chunk_overlap'])
  if (s && s['search_top_k']) searchTopK.value = Number(s['search_top_k'])
  if (s && s['min_sim']) minSim.value = Number(s['min_sim'])
  if (s && s['mcp_endpoints']) mcpEndpoints.value = s['mcp_endpoints']
  if (s && s['user_prompt_template']) userPromptTemplate.value = s['user_prompt_template']
})

async function pickDir() {
  const d = await open({ directory: true, multiple: false })
  if (typeof d === 'string' && d) {
    notesDir.value = d
  }
}

async function reindex() {
  if (!notesDir.value) return
  await invoke('watch_notes', { dir: notesDir.value })
}

async function saveAi() {
  const kv: Record<string,string> = {
    provider: provider.value,
    openai_api_key: openaiApiKey.value,
    openai_base_url: openaiBaseUrl.value,
    openai_model: openaiModel.value,
    openai_embed_model: openaiEmbedModel.value,
    ollama_base_url: ollamaBaseUrl.value,
    ollama_model: ollamaModel.value,
    ollama_embed_model: ollamaEmbedModel.value,
    temperature: String(temperature.value),
    max_tokens: String(maxTokens.value),
    system_prompt: systemPrompt.value
  }
  await invoke('update_settings', { kv })
}

async function doTestEmbed() {
  const r = await invoke<any>('test_embedding', { text: testText.value })
  embedDim.value = r?.dim ?? 0
}

async function doTestGenerate() {
  const r = await invoke<any>('test_generate', { query: testQuestion.value })
  testAnswer.value = r?.answer ?? ''
}

async function saveKb() {
  const kv: Record<string,string> = {
    chunk_size: String(chunkSize.value),
    chunk_overlap: String(chunkOverlap.value),
    search_top_k: String(searchTopK.value),
    min_sim: String(minSim.value)
  }
  await invoke('update_settings', { kv })
}

async function saveMcp() {
  const kv: Record<string,string> = { mcp_endpoints: mcpEndpoints.value }
  await invoke('update_settings', { kv })
}

async function savePrompt() {
  const kv: Record<string,string> = { user_prompt_template: userPromptTemplate.value }
  await invoke('update_settings', { kv })
}
</script>