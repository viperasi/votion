<template>
  <div class="app">
    <header class="header">
      <h1>Votion</h1>
      <nav>
        <router-link to="/">首页</router-link>
        <router-link to="/editor">编辑器</router-link>
        <router-link to="/chat">问答</router-link>
        <router-link to="/settings">设置</router-link>
      </nav>
    </header>
    <main class="main">
      <router-view />
    </main>
  </div>
</template>

<style scoped>
.app { display: flex; flex-direction: column; height: 100vh; }
.header { display: flex; align-items: center; gap: 16px; padding: 12px; border-bottom: 1px solid #eee; }
.header a { margin-right: 8px; }
.main { flex: 1; overflow: auto; }
</style>