<script setup lang="ts">
import NavBarDesktop from "@/components/NavBarDesktop.vue";
import Player from "@/components/Player.vue";
import Body from "@/components/Body.vue";
import { store } from "@/store";

let searchText = "";
</script>

<template>

  <NavBarDesktop />
  <Suspense>
    <Body />
  </Suspense>

  <Suspense>
    <Player v-if="store.currentTrack != null" />
  </Suspense>


</template>

<style scoped>

</style>
