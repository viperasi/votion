<template>
  <div class="app grid-bg">
    <div class="titlebar">
      <div class="title-drag" @mousedown="startDrag"></div>
      <div class="window-controls">
        <button class="btn ctrl" @click="minimize" title="最小化">
          <svg viewBox="0 0 24 24"><path d="M6 12h12"/></svg>
        </button>
        <button class="btn ctrl" @click="toggleMax" title="最大化/还原">
          <svg viewBox="0 0 24 24"><path d="M6 6h12v12H6z"/></svg>
        </button>
        <button class="btn ctrl" @click="close" title="关闭">
          <svg viewBox="0 0 24 24"><path d="M6 6l12 12M18 6L6 18"/></svg>
        </button>
      </div>
    </div>
    <aside class="sidebar">
      <div class="brand">
        <img src="/votion.svg" class="brand-icon" alt="Votion">
      </div>
      <nav class="side-nav">
        <router-link to="/chat" class="side-link btn" title="问答">
          <svg viewBox="0 0 24 24"><path d="M4 5h16v10H7l-3 4z"/></svg>
        </router-link>
        <router-link to="/editor" class="side-link btn" title="编辑器">
          <svg viewBox="0 0 24 24"><path d="M7 5h10l-6 6-4-4M5 19h14"/></svg>
        </router-link>
        <router-link to="/settings" class="side-link btn" title="设置">
          <svg viewBox="0 0 24 24"><path d="M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0-6l2 3 3 1-1 3 1 3-3 1-2 3-2-3-3-1 1-3-1-3 3-1 2-3z"/></svg>
        </router-link>
      </nav>
    </aside>
    <main class="main">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
async function minimize() { await appWindow.minimize() }
async function toggleMax() { const m = await appWindow.isMaximized(); if (m) await appWindow.unmaximize(); else await appWindow.maximize() }
async function close() { await appWindow.close() }
async function startDrag() { try { await appWindow.startDragging() } catch {} }
</script>
<style scoped>
.app{display:grid;grid-template-columns:64px 1fr;grid-template-rows:32px 1fr;height:100vh;border:1px solid var(--panel-border);border-radius:0;box-shadow:0 12px 24px rgba(123,97,255,.12);background:var(--panel-bg)}
.titlebar{grid-column:1/-1;display:grid;grid-template-columns:1fr auto;align-items:center;height:32px}
.title-drag{-webkit-app-region:drag;height:100%}
.window-controls{display:flex;gap:6px;padding:0 6px;-webkit-app-region:no-drag}
.ctrl{display:flex;align-items:center;justify-content:center;gap:0;width:22px;height:22px;padding:0;border-radius:6px}
.ctrl svg{width:12px;height:12px;display:block}
.sidebar{display:flex;flex-direction:column;align-items:center;gap:12px;padding:12px;border-right:1px solid var(--panel-border);background:var(--panel-bg);grid-row:2}
.brand{font-weight:700;color:var(--text-secondary);display:flex;flex-direction:column;align-items:center;gap:6px}
.brand-icon{width:44px;height:44px;border-radius:10px}
.side-nav{display:flex;flex-direction:column;gap:10px}
.side-link{display:flex;align-items:center;justify-content:center}
.side-link.router-link-active{transform:translateY(-1px);box-shadow:0 6px 16px rgba(123,97,255,.25);background:var(--btn-hover)}
.main{overflow:auto;min-width:0;grid-row:2}
</style>