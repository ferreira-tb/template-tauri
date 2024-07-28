<script setup lang="ts">
import { showWindow } from '@/lib/commands';
import { setGlobalSensors } from '@/lib/sensors';

setGlobalSensors();

onMounted(() => {
  flushPromises().then(showWindow).catch(handleError);
});
</script>

<template>
  <main class="fixed inset-0 select-none">
    <RouterView #default="{ Component }">
      <template v-if="Component">
        <component :is="Component" />
      </template>
    </RouterView>
  </main>
</template>
