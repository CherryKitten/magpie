<script setup lang="ts">
import { ref } from "vue";

const currentTrack = {
  title: "This is a test song for testing",
  album: "Vuetiful sounds",
  year: 2023,
  track_number: "2",
  disc_number: "1",
  artist: "Sammy",
  albumartist: "Sammy",
  art: "./favicon.ico"
};

const timestamp = 30;
let playing = ref(false);

const volume = ref(100);

function togglePlaying() {
  playing.value = !playing.value;
}
</script>

<template>
  <div class="bottom-0 sticky w-screen bg-base-200 py-6 px-4 flex flex-row align-center justify-between">
    <div class="flex flex-row gap-4">
      <div>
        <img :src="currentTrack.art" alt="album art" class="h-12 w-12" />
      </div>
      <div class="block">
        <p>{{ currentTrack.title }}</p>
        <p>{{ currentTrack.albumartist }} - {{ currentTrack.album }} ({{ currentTrack.year || "unknown" }})</p>
        <p>TODO: Timestamp</p>
      </div>
    </div>

    <div class="w-1/3">
      <div class="text-center flex flex-row justify-center gap-2 py-2">
        <font-awesome-icon icon="fa-solid fa-backward" class="btn btn-ghost btn-sm" />
        <font-awesome-icon @click="togglePlaying" v-if="playing" icon="fa-solid fa-pause"
                           class="btn btn-ghost btn-sm" />
        <font-awesome-icon @click="togglePlaying" v-if="!playing" icon="fa-solid fa-play"
                           class="btn btn-ghost btn-sm" />
        <font-awesome-icon icon="fa-solid fa-forward" class="btn btn-ghost btn-sm" />
      </div>
      <input type="range" min="0" max="100" :value="timestamp" class="range range-sm" />
    </div>

    <div>
      <input type="range" min="0" max="100" :value="volume" class="range range-success" />
    </div>
  </div>
</template>


<style scoped>

</style>
