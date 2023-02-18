<script setup lang="ts">
import NavBarDesktop from "@/components/NavBarDesktop.vue";
import Player from "@/components/Player.vue";
import { store } from "@/store";

if (store.albums.length == 0) {
  store.getAlbums();
}
if (store.artists.length == 0) {
  store.getArtists();
}

</script>

<template>

  <NavBarDesktop />

  <Suspense>
    <RouterView />
  </Suspense>

  <Suspense>
    <Player v-if="store.currentTrack != null" />
  </Suspense>


</template>

<style scoped>

</style>
