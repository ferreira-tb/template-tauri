<script setup lang="ts">
import { MButton, MScaffold, MTopAppbar, useDarkMode } from 'manatsu';

const darkMode = useDarkMode();
</script>

<template>
  <MScaffold default-border="none">
    <template #top-bar>
      <MTopAppbar title="Hello, Manatsu!">
        <template #end>
          <MButton variant="outlined" @click="$mana.toggleDarkMode()">
            {{ darkMode ? 'Light' : 'Dark' }}
          </MButton>
        </template>
      </MTopAppbar>
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
