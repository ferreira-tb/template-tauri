import { createMemoryHistory, createRouter } from 'vue-router';

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [{ path: '/', component: () => import('../views/HomeView.vue') }]
});
