<script setup lang="ts">
import { MBrand, MButton, MNavbar, MScaffold, useDarkMode } from 'manatsu';

const darkMode = useDarkMode();

function toggleDarkMode() {
  darkMode.value = !darkMode.value;
}
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
          <MButton outlined @click="toggleDarkMode">Toggle Dark Mode</MButton>
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
