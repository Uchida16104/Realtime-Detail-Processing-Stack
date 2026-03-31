<template>
  <main class="min-h-screen p-6 lg:p-10">
    <div class="mx-auto max-w-7xl space-y-6">
      <section class="rounded-3xl border border-slate-800 bg-slate-900/70 p-6 shadow-2xl">
        <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
          <div>
            <h1 class="text-3xl font-bold tracking-tight text-white">Realtime Detail Processing Stack</h1>
            <p class="mt-2 max-w-3xl text-sm text-slate-300">
              Vue frontend, Rust backend, SSE event stream, external language processors, downloadable logs.
            </p>
          </div>
          <div class="flex flex-wrap gap-2 text-xs">
            <span class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-3 py-1 text-emerald-300">Backend: {{ backendUrl }}</span>
            <span class="rounded-full border border-sky-500/40 bg-sky-500/10 px-3 py-1 text-sky-300">SSE connected: {{ connected ? 'yes' : 'no' }}</span>
          </div>
        </div>
      </section>

      <section class="grid gap-6 lg:grid-cols-3">
        <article class="rounded-3xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-lg font-semibold text-white">Realtime analysis</h2>
          <textarea v-model="payload" class="mt-3 h-44 w-full rounded-2xl border border-slate-700 bg-slate-950 p-3 text-sm text-slate-100 outline-none" />
          <div class="mt-3 flex flex-wrap gap-2">
            <button class="rounded-2xl bg-indigo-500 px-4 py-2 text-sm font-medium text-white" @click="runAnalyze">Analyze</button>
            <button class="rounded-2xl bg-slate-700 px-4 py-2 text-sm font-medium text-white" @click="runNetworkCheck">Network Check</button>
            <button class="rounded-2xl bg-slate-700 px-4 py-2 text-sm font-medium text-white" @click="runSecurityCheck">Security Check</button>
          </div>
          <div class="mt-4 rounded-2xl border border-slate-800 bg-slate-950 p-3 text-xs text-slate-300">
            <div><span class="text-slate-500">Status:</span> {{ lastResult.status }}</div>
            <div class="mt-2 whitespace-pre-wrap break-words text-slate-200">{{ lastResult.message }}</div>
          </div>
        </article>

        <article class="rounded-3xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-lg font-semibold text-white">Realtime file processing</h2>
          <div class="mt-3 flex items-center gap-3">
            <input type="file" class="block w-full text-sm text-slate-300 file:mr-4 file:rounded-2xl file:border-0 file:bg-slate-700 file:px-4 file:py-2 file:text-white" @change="onFileSelect" />
          </div>
          <div class="mt-3 rounded-2xl border border-slate-800 bg-slate-950 p-3 text-xs text-slate-300">
            <div><span class="text-slate-500">Filename:</span> {{ selectedFileName }}</div>
            <div class="mt-2 whitespace-pre-wrap break-words text-slate-200">{{ fileResult }}</div>
          </div>
          <div class="mt-4 flex flex-wrap gap-2">
            <button class="rounded-2xl bg-emerald-500 px-4 py-2 text-sm font-medium text-white" @click="runFileCheck">File Check</button>
            <a :href="`${backendUrl}/api/logs/latest.json`" class="rounded-2xl bg-slate-700 px-4 py-2 text-sm font-medium text-white">Download latest log</a>
          </div>
        </article>

        <article class="rounded-3xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-lg font-semibold text-white">Mermaid topology</h2>
          <div ref="mermaidHost" class="mt-3 overflow-auto rounded-2xl border border-slate-800 bg-slate-950 p-3 text-xs"></div>
        </article>
      </section>

      <section class="grid gap-6 lg:grid-cols-2">
        <article class="rounded-3xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-lg font-semibold text-white">Live stream</h2>
          <div class="mt-3 max-h-80 space-y-2 overflow-auto rounded-2xl border border-slate-800 bg-slate-950 p-3 text-xs">
            <div v-for="(item, index) in events" :key="index" class="rounded-xl border border-slate-800 bg-slate-900 p-3">
              <div class="flex items-center justify-between gap-3">
                <span class="font-semibold text-sky-300">{{ item.kind }}</span>
                <span class="text-slate-500">{{ item.timestamp }}</span>
              </div>
              <div class="mt-1 text-slate-300">{{ item.status }}</div>
              <pre class="mt-2 overflow-auto whitespace-pre-wrap text-slate-400">{{ JSON.stringify(item.detail, null, 2) }}</pre>
            </div>
          </div>
        </article>

        <article class="rounded-3xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-lg font-semibold text-white">Operational notes</h2>
          <div class="mt-3 space-y-3 text-sm leading-7 text-slate-300">
            <p>HTMX, hyperscript, and Alpine.js are loaded in the page shell for lightweight interaction and progressive enhancement.</p>
            <p>Rust owns the request/response path and writes every successful action to a JSON log file that can be downloaded from the backend.</p>
            <p>Missing Zig, Mojo, Java, or native toolchains do not stop the app; the backend falls back to deterministic Rust logic and records the fallback in the log.</p>
          </div>
        </article>
      </section>
    </div>
  </main>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, reactive, ref } from 'vue'
import mermaid from 'mermaid'
import { getBackendUrl, postJson } from './lib/api'
import type { StreamEvent } from './lib/types'

type ResultBox = {
  status: string
  message: string
}

type ApiResponse = {
  status: string
  message: string
  detail: Record<string, unknown>
}

const backendUrl = getBackendUrl()
const events = ref<StreamEvent[]>([])
const connected = ref(false)
const payload = ref(`{"target":"demo","scope":"realtime","mode":"analysis"}`)
const selectedFile = ref<File | null>(null)
const selectedFileName = computed(() => selectedFile.value?.name || 'none')
const fileResult = ref('No file processed yet.')
const lastResult = reactive<ResultBox>({
  status: 'idle',
  message: 'Ready.'
})
const mermaidHost = ref<HTMLElement | null>(null)
let source: EventSource | null = null

async function renderMermaid() {
  if (!mermaidHost.value) return
  mermaid.initialize({ startOnLoad: false, theme: 'dark' })
  const sourceText = `
flowchart LR
  A[Frontend] --> B[Rust API]
  B --> C[Processors]
  C --> D[Logs]
`
  const id = `m${Date.now()}`
  const { svg } = await mermaid.render(id, sourceText)
  mermaidHost.value.innerHTML = svg
}

function pushEvent(event: StreamEvent) {
  events.value.unshift(event)
  if (events.value.length > 40) {
    events.value.pop()
  }
}

function connectStream() {
  if (source) {
    source.close()
  }
  source = new EventSource(`${backendUrl}/api/stream`)
  source.onopen = () => {
    connected.value = true
  }
  source.onerror = () => {
    connected.value = false
  }
  source.onmessage = (message) => {
    try {
      const parsed = JSON.parse(message.data) as StreamEvent
      pushEvent(parsed)
    } catch {
      pushEvent({
        kind: 'stream',
        status: 'invalid message',
        timestamp: new Date().toISOString(),
        detail: { raw: message.data }
      })
    }
  }
}

async function runAnalyze() {
  try {
    const response = await postJson<ApiResponse>('/api/analyze', {
      mode: 'detail-analysis',
      data: payload.value
    })
    lastResult.status = response.status
    lastResult.message = response.message
    pushEvent({
      kind: 'analysis',
      status: response.status,
      timestamp: new Date().toISOString(),
      detail: response.detail
    })
  } catch (error) {
    lastResult.status = 'error'
    lastResult.message = error instanceof Error ? error.message : 'Unknown error'
  }
}

async function runNetworkCheck() {
  try {
    const response = await postJson<ApiResponse>('/api/network/check', {
      host: 'example.com',
      port: 443
    })
    lastResult.status = response.status
    lastResult.message = response.message
    pushEvent({
      kind: 'network',
      status: response.status,
      timestamp: new Date().toISOString(),
      detail: response.detail
    })
  } catch (error) {
    lastResult.status = 'error'
    lastResult.message = error instanceof Error ? error.message : 'Unknown error'
  }
}

async function runSecurityCheck() {
  try {
    const response = await postJson<ApiResponse>('/api/security/check', {
      input: payload.value
    })
    lastResult.status = response.status
    lastResult.message = response.message
    pushEvent({
      kind: 'security',
      status: response.status,
      timestamp: new Date().toISOString(),
      detail: response.detail
    })
  } catch (error) {
    lastResult.status = 'error'
    lastResult.message = error instanceof Error ? error.message : 'Unknown error'
  }
}

async function runFileCheck() {
  if (!selectedFile.value) {
    fileResult.value = 'Select a file first.'
    return
  }

  const text = await selectedFile.value.text()
  const response = await postJson<ApiResponse>('/api/file/check', {
    name: selectedFile.value.name,
    size: selectedFile.value.size,
    content: text.slice(0, 12000)
  })

  fileResult.value = `${response.status}: ${response.message}`
  pushEvent({
    kind: 'file',
    status: response.status,
    timestamp: new Date().toISOString(),
    detail: response.detail
  })
}

function onFileSelect(event: Event) {
  const input = event.target as HTMLInputElement
  selectedFile.value = input.files?.[0] || null
}

onMounted(async () => {
  await renderMermaid()
  connectStream()
})

onUnmounted(() => {
  if (source) source.close()
})
</script>
