<script setup lang="ts">
import { computed } from 'vue';
import { MBrand, MButton, MNavbar, MScaffold, useDarkMode } from 'manatsu';

const darkMode = useDarkMode();
const darkModeLabel = computed(() => (darkMode.value ? 'Light' : 'Dark'));
</script>

<template>
  <MScaffold>
    <template #header>
      <MNavbar>
        <template #start>
          <MBrand>
            <template #title>Hello, Manatsu!</template>
          </MBrand>
        </template>

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
