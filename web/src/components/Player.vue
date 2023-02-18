<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { store } from "@/store";

const volume = ref(0.5);
const timestamp = ref(0);
const duration = ref(null);

const player: HTMLAudioElement = new Audio();
player.setAttribute("id", "audio");
player.setAttribute("src", "https://localhost:8080/tracks/" + store.currentTrack.id + "/play");
player.setAttribute("class", "hidden");
player.setAttribute("preload", "true");
player.addEventListener("timeupdate", updateProgress);
player.addEventListener("ended", next_track)

function updateProgress() {
  timestamp.value = player.currentTime;
}

function next_track() {
  store.next_track();
  player.currentTime = 0;
  player.setAttribute("src", "https://localhost:8080/tracks/" + store.currentTrack.id + "/play");
  if (store.playing) {
    player.play();
  }

}

watchEffect(async () => {
  if (store.playing) {
    await player.play();
  } else {
    await player.pause();
  }
});

function prettyTimestamp(time: number) {
  return parseInt(time / 60) + ":" + parseInt(time % 60);
}

watchEffect(async () => {
  setVolume(volume.value);
});

function stringifyArtist(artists: any[]) {
  let result = [];
  for (let [k, v] of artists.entries()) {
    result.push(v);
  }
  return result.toString();
}

function setVolume(vol: number) {
  player.volume = vol;
}

</script>

<template>
  <div class="bottom-0 sticky w-screen bg-base-200 py-6 px-4 flex flex-row align-center justify-between z-20">
    <div class="flex flex-row gap-4">
      <div>
        <img :src="store.currentTrack.art" alt="album art" class="h-12 w-12" />
      </div>
      <div class="block">
        <p>{{ store.currentTrack.title }}</p>
        <p>{{ stringifyArtist(store.currentTrack.album_artist) }} - {{ store.currentTrack.album }}
          ({{ store.currentTrack.year || "unknown" }})</p>
        <p>{{ prettyTimestamp(player.currentTime) }} / {{ prettyTimestamp(player.duration) }}</p>
      </div>
    </div>

    <div class="w-1/3">
      <div class="text-center flex flex-row justify-center gap-2 py-2">
        <font-awesome-icon icon="fa-solid fa-backward" class="btn btn-ghost btn-sm" />
        <font-awesome-icon @click="store.togglePlaying()" v-if="store.playing" icon="fa-solid fa-pause"
                           class="btn btn-ghost btn-sm" />
        <font-awesome-icon @click="store.togglePlaying()" v-if="!store.playing" icon="fa-solid fa-play"
                           class="btn btn-ghost btn-sm" />
        <font-awesome-icon icon="fa-solid fa-forward" class="btn btn-ghost btn-sm" @click="next_track()" />
      </div>
      <input id="timebar" type="range" min="0" :max="player.duration"
             v-on:change="(e) => player.currentTime = e.target.value"
             :value="timestamp" class="range range-sm" />
    </div>

    <div>
      <input type="range" :min="0" max="1" step="0.1" v-model="volume" class="range range-success" />
      {{ volume }}
    </div>
  </div>


</template>


<style scoped>

</style>
