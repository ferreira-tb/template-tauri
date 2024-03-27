import { RouteName } from '@/router/route';
import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: RouteName.Home,
      component: () => import('../views/Home.vue')
    }
  ]
});
