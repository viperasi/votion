<template>
  <section class="settings grid-bg">
    <h2>设置</h2>
    <div class="panel">
      <div class="panel-header">笔记目录</div>
      <div class="inline">
        <label>目录路径</label>
        <input class="input" v-model="notesDir" />
        <div class="actions">
          <button class="btn" @click="applyWatch">
            <svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></svg>
            开始监听
          </button>
          <button class="btn" @click="pickDir">
            <svg viewBox="0 0 24 24"><path d="M4 7h6l2 2h8v10H4z"/></svg>
            选择目录
          </button>
          <button class="btn" @click="reindex">
            <svg viewBox="0 0 24 24"><path d="M12 6a6 6 0 1 1-5.3 3H4l3-3 3 3H8.7A4 4 0 1 0 12 8"/></svg>
            重建索引
          </button>
        </div>
      </div>
    </div>

    <div class="panel">
      <div class="panel-header">AI 配置</div>
      <div class="inline">
        <label>提供者</label>
        <select class="input" v-model="provider">
          <option value="openai">OpenAI</option>
          <option value="ollama">Ollama</option>
        </select>
      </div>
      <div v-if="provider==='openai'" class="inline">
        <label>OpenAI API Key</label>
        <div class="input-with-btn">
          <input class="input" v-model="openaiApiKey" :type="showOpenaiKey ? 'text' : 'password'" />
          <button class="btn" @click="showOpenaiKey = !showOpenaiKey">
            <svg viewBox="0 0 24 24"><path d="M12 5c5 0 9 7 9 7s-4 7-9 7-9-7-9-7 4-7 9-7zm0 4a3 3 0 1 0 0 6 3 3 0 0 0 0-6"/></svg>
            {{ showOpenaiKey ? '隐藏' : '显示' }}
          </button>
        </div>
        <label>OpenAI Base URL</label>
        <input class="input" v-model="openaiBaseUrl" placeholder="可选，如 https://api.openai.com" />
        <label>大模型</label>
        <input class="input" v-model="openaiModel" placeholder="如 gpt-4o-mini" />
        <label>嵌入模型</label>
        <input class="input" v-model="openaiEmbedModel" placeholder="如 text-embedding-3-large" />
      </div>
      <div v-if="provider==='ollama'" class="inline">
        <label>Ollama Base URL</label>
        <input class="input" v-model="ollamaBaseUrl" placeholder="如 http://localhost:11434" />
        <label>大模型</label>
        <input class="input" v-model="ollamaModel" placeholder="如 llama3.1:8b" />
        <label>嵌入模型</label>
        <input class="input" v-model="ollamaEmbedModel" placeholder="如 nomic-embed-text" />
      </div>
      <div class="inline">
        <label>温度</label>
        <input class="input" v-model.number="temperature" type="number" step="0.1" min="0" max="2" />
        <label>最大Tokens</label>
        <input class="input" v-model.number="maxTokens" type="number" min="1" />
      </div>
      <div class="inline">
        <label>系统提示</label>
        <textarea class="input" v-model="systemPrompt" placeholder="系统提示"></textarea>
      </div>
      <div class="actions">
        <button class="btn" @click="saveAi">
          <svg viewBox="0 0 24 24"><path d="M5 5h14v10H5zM9 19h6"/></svg>
          保存AI配置
        </button>
      </div>
    </div>

    <div class="panel">
      <div class="panel-header">测试</div>
      <div class="inline">
        <label>测试嵌入文本</label>
        <input class="input" v-model="testText" placeholder="输入一句话" />
        <button class="btn" @click="doTestEmbed">
          <svg viewBox="0 0 24 24"><path d="M12 3v7l6 3-6 3v5"/></svg>
          测试嵌入
        </button>
        <span v-if="embedDim!==null">维度: {{ embedDim }}</span>
      </div>
      <div class="inline">
        <label>测试问答问题</label>
        <input class="input" v-model="testQuestion" placeholder="输入问题" />
        <button class="btn" @click="doTestGenerate">
          <svg viewBox="0 0 24 24"><path d="M5 12h10M9 8l4 4-4 4"/></svg>
          测试问答
        </button>
      </div>
      <pre v-if="testAnswer">{{ testAnswer }}</pre>
    </div>

    <div class="panel">
      <div class="panel-header">知识库配置</div>
      <div class="inline">
        <label>分块大小</label>
        <input class="input" v-model.number="chunkSize" type="number" min="1" />
        <label>分块重叠</label>
        <input class="input" v-model.number="chunkOverlap" type="number" min="0" />
        <label>检索Top-K</label>
        <input class="input" v-model.number="searchTopK" type="number" min="1" />
        <label>最小相似度</label>
        <input class="input" v-model.number="minSim" type="number" step="0.01" min="0" max="1" />
      </div>
      <div class="actions">
        <button class="btn" @click="saveKb">
          <svg viewBox="0 0 24 24"><path d="M6 6h12v12H6zM8 10h8M8 14h6"/></svg>
          保存知识库配置
        </button>
      </div>
    </div>

    <div class="panel">
      <div class="panel-header">MCP 配置</div>
      <div class="inline">
        <label>端点列表（逗号分隔）</label>
        <input class="input" v-model="mcpEndpoints" placeholder="如 http://localhost:4000,http://tools.local" />
      </div>
      <div class="actions">
        <button class="btn" @click="saveMcp">
          <svg viewBox="0 0 24 24"><path d="M4 7h16v10H4zM6 9h12"/></svg>
          保存MCP配置
        </button>
      </div>
    </div>

    <div class="panel">
      <div class="panel-header">Prompt 配置</div>
      <div class="inline">
        <label>用户提示模板</label>
        <textarea class="input" v-model="userPromptTemplate" placeholder="模板中可包含 {query} 与 {context}"></textarea>
      </div>
      <div class="actions">
        <button class="btn" @click="savePrompt">
          <svg viewBox="0 0 24 24"><path d="M5 5h14v14H5zM7 9h10M7 13h8"/></svg>
          保存Prompt配置
        </button>
      </div>
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
const showOpenaiKey = ref(false)
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
<style scoped>
.settings{padding:16px;color:var(--text-secondary)}
h2{color:var(--text-secondary)}
label{display:block;margin:6px 0;color:var(--text-muted)}
.panel{padding:12px;margin-bottom:12px}
.inline{display:grid;grid-template-columns:140px 1fr;gap:8px;align-items:center}
.actions{display:flex;gap:8px;margin-top:8px}
pre{background:var(--panel-bg);border:1px solid var(--panel-border);border-radius:8px;padding:10px;color:var(--text-primary)}
.input-with-btn{display:grid;grid-template-columns:1fr auto;gap:8px;align-items:center}
</style>