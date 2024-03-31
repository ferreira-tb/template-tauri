import { RouteName } from '@/router/routes';
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
