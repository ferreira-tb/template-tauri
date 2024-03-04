<script setup lang="ts">
import { useDarkMode } from 'manatsu';

const darkMode = useDarkMode();
</script>

<template>
  <m-scaffold default-border="none">
    <template #top-bar>
      <m-top-appbar title="Hello, Manatsu!">
        <template #end>
          <m-button variant="outlined" @click="$mana.toggleDarkMode()">
            {{ darkMode ? 'Light' : 'Dark' }}
          </m-button>
        </template>
      </m-top-appbar>
    </template>

    <template #default>
      <router-view />
    </template>
  </m-scaffold>
</template>
