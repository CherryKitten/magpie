<script setup>
import { store } from "@/store";

const albums = await (await fetch(store.API + "albums")).json();

</script>

<template>
  <div class="min-h-screen">
    <ul>
      <li v-for="album in albums" @click="store.add_album(album)">
        {{ album.title }} {{ album.id }}

        {{ album.track_ids }}
      </li>
    </ul>

    TEST:
    {{ store.playlist }}
  </div>
</template>

<style scoped>

</style>
