<script setup lang="ts">
import { useDarkMode, useInvoke } from 'manatsu';
import { Command } from './utils';

const darkMode = useDarkMode();
const { state: version } = useInvoke<string>(Command.Version);
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
