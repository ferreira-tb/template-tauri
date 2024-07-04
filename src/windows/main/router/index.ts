import { Route } from './routes';
import { createMemoryHistory, createRouter } from 'vue-router';

export { Route };

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/',
      name: Route.Home,
      component: () => import('../views/Home.vue')
    }
  ]
});

function push(to: Route) {
  return function () {
    void router.push({ name: to });
  };
}

export const navigate = {
  home: push(Route.Home)
} as const;
