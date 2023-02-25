<script setup>
import { store } from "@/store";
import router from "@/router";

</script>

<template>
  <thead>
  <tr>
    <th>Artist</th>

  </tr>
  </thead>
  <tbody>
  <tr v-for="artist in store.artists" @click="router.push({ path: '/albums', query: { artist: artist.id }})
">

    <th>{{ artist.name }}</th>

  </tr>
  </tbody>
</template>

<style scoped>

</style>
