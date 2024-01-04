<script setup lang="ts">
import { MNavbar, MScaffold } from 'manatsu';
import MainView from './views/MainView.vue';
</script>

<template>
  <MScaffold>
    <template #header>
      <MNavbar title="Hello, Manatsu!" />
    </template>

    <MainView />
  </MScaffold>
</template>
