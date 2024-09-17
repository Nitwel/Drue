<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute()
const collection = ref<Record<string, any> | null>(null)
const current_word_index = ref(0)

const word = computed(() => {
    if (collection.value) {
        return collection.value.words[current_word_index.value]
    }
    return null
})

async function loadCollections() {
  const response = await fetch(`/api/collections/${route.params.id}`)

  const data = await response.json();
  
  collection.value = data
}

function randomIndex() {
    current_word_index.value = Math.floor(Math.random() * collection.value?.words.length)
}

loadCollections()
</script>


<template>
  <div class="collection">
    <h1>{{ collection?.name }}</h1>
    <p>{{ collection?.description }}</p>

    <div class="word">{{ word?.word }}</div>
  </div>
</template>

<style scoped>

</style>
