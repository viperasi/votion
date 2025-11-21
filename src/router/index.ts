import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../pages/Home.vue'
import Editor from '../pages/Editor.vue'
import Chat from '../pages/Chat.vue'
import Settings from '../pages/Settings.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/editor', component: Editor },
    { path: '/chat', component: Chat },
    { path: '/settings', component: Settings }
  ]
})

export default router