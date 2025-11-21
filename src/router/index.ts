import { createRouter, createWebHashHistory } from 'vue-router'
import Editor from '../pages/Editor.vue'
import Chat from '../pages/Chat.vue'
import Settings from '../pages/Settings.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: Chat },
    { path: '/chat', component: Chat },
    { path: '/editor', component: Editor },
    { path: '/settings', component: Settings }
  ]
})

export default router