<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';

const { state: version } = useAsyncState(getVersion, null, { onError: handleError });
</script>

<template>
  <div>
    <h1 v-if="version">Tauri Template {{ version }}</h1>
    <div>Hello World!</div>
  </div>
</template>
