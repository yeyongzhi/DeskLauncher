import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/layout/MainLayout.vue'),
      children: [
        { path: '', name: 'home', component: () => import('@/views/Home/index.vue'), meta: { title: '启动台' } },
        { path: 'hardware', name: 'hardware', component: () => import('@/views/Hardware/index.vue'), meta: { title: '硬件信息' } },
        { path: 'software', name: 'software', component: () => import('@/views/Software/index.vue'), meta: { title: '已安装软件' } },
        { path: 'settings', name: 'settings', component: () => import('@/views/Settings/index.vue'), meta: { title: '设置' } },
      ],
    },
  ],
})

export default router
