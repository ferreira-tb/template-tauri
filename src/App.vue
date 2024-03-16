<script setup lang="ts">
import { inject } from 'vue';
import { symbols } from 'manatsu';

const darkMode = inject(symbols.darkMode);
const version = useInvoke<string | null>('Version', null);
</script>

<template>
  <m-scaffold>
    <template #top>
      <m-top-appbar>
        <template #start>
          <div class="flex items-center gap-2">
            <span>Tauri Template for Manatsu</span>
            <span v-if="version">v{{ version }}</span>
          </div>
        </template>
        <template #end>
          <m-button variant="outlined" @click="$mana.toggleDarkMode()">
            {{ darkMode ? 'Light' : 'Dark' }}
          </m-button>
        </template>
      </m-top-appbar>
    </template>

    <template #default>
      <router-view #default="{ Component }">
        <template v-if="Component">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </template>
      </router-view>
    </template>
  </m-scaffold>
</template>
