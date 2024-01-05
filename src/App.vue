<script setup lang="ts">
import { MNavbar, MScaffold } from 'manatsu';
</script>

<template>
  <MScaffold>
    <template #header>
      <MNavbar title="Hello, Manatsu!" />
    </template>

    <RouterView #default="{ Component }">
      <component :is="Component" />
    </RouterView>
  </MScaffold>
</template>
