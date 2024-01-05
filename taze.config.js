import { defineConfig } from 'taze';

export default defineConfig({
  force: true,
  write: true,
  includeLocked: true,
  install: true,
  packageMode: {
    vue: 'ignore',
    '/manatsu/': 'major'
  }
});
