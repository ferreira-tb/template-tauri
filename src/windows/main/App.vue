<script setup lang="ts"></script>

<template>
  <main>
    <router-view #default="{ Component }">
      <template v-if="Component">
        <component :is="Component" />
      </template>
    </router-view>
  </main>
</template>
