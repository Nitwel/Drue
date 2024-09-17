<script setup lang="ts">
import { ref } from 'vue';

const collections = ref<Record<string, any>>([])

async function loadCollections() {
  const response = await fetch('/api/collections')

  const data = await response.json();

  console.log(data)
  
  collections.value = data
}

loadCollections()
</script>

<template>
  <main>
    <h1>Home</h1>
    <div class="collections">
      <div class="collection" v-for="collection in collections" :key="collection.id">
        <h2>{{ collection.name }}</h2>
        <p>{{ collection.description }}</p>
        <button>Start</button>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>