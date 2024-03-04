<script setup lang="ts">
import { useDarkMode } from 'manatsu';

const darkMode = useDarkMode();
</script>

<template>
  <m-scaffold>
    <template #top>
      <m-top-appbar>
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
