<script setup lang="ts">
import { computed } from 'vue';
import { MButton, MNavbar, MScaffold, useDarkMode } from 'manatsu';

const darkMode = useDarkMode();
const darkModeLabel = computed(() => (darkMode.value ? 'Light' : 'Dark'));
</script>

<template>
  <MScaffold>
    <template #header>
      <MNavbar title="Hello, Manatsu!">
        <template #end>
          <MButton variant="outlined" @click="$mana.toggleDarkMode()">
            {{ darkModeLabel }}
          </MButton>
        </template>
      </MNavbar>
    </template>

    <RouterView #default="{ Component }">
      <template v-if="Component">
        <Transition mode="out-in">
          <KeepAlive>
            <Suspense>
              <component :is="Component" />
              <template #fallback>Loading...</template>
            </Suspense>
          </KeepAlive>
        </Transition>
      </template>
    </RouterView>
  </MScaffold>
</template>
