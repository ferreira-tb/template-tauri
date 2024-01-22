import 'manatsu/style';
import './assets/style.css';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createManatsu, registerComponents } from 'manatsu';
import App from './App.vue';
import { router } from './router';

const app = createApp(App);
const pinia = createPinia();
const manatsu = createManatsu();

app.use(router);
app.use(pinia);
app.use(manatsu);

registerComponents(app);

router
  .push('/')
  .then(() => router.isReady())
  .then(() => app.mount('#app'))
  .catch((err: unknown) => console.error(err));
