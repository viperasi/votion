<template>
  <div class="app grid-bg">
    <aside class="sidebar">
      <div class="brand">Votion</div>
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

<style scoped>
.app{display:grid;grid-template-columns:64px 1fr;height:100vh}
.sidebar{display:flex;flex-direction:column;align-items:center;gap:12px;padding:12px;border-right:1px solid var(--panel-border);background:var(--panel-bg)}
.brand{font-weight:700;color:var(--text-secondary)}
.side-nav{display:flex;flex-direction:column;gap:10px}
.side-link{display:flex;align-items:center;justify-content:center}
.side-link.router-link-active{transform:translateY(-1px);box-shadow:0 6px 16px rgba(123,97,255,.25);background:var(--btn-hover)}
.main{overflow:auto}
</style>