<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <nav>
      <RouterLink to="/">Home</RouterLink>
      <RouterLink to="/discover">Discover</RouterLink>
    </nav>
  </header>

  <RouterView />
</template>

<style scoped lang="scss">

</style>
