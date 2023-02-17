<script setup lang="ts">
import { ref, watchEffect } from "vue";


const tracklist = ref(null);
const playing = ref(false);
const volume = ref(0.5);
const currentTrack = ref(null);
const timestamp = ref(0);
const duration = ref(null);

tracklist.value = await (await fetch(`https://localhost:8080/albums/10`)).json();

if (tracklist.value != null) {
  tracklist.value = tracklist.value[0]["track_ids"];

  currentTrack.value = await (await fetch(`https://localhost:8080/tracks/` + tracklist.value[0])).json();
  currentTrack.value = currentTrack.value[0];
}

const player: HTMLAudioElement = new Audio();
player.setAttribute("id", "audio");
player.setAttribute("src", "https://localhost:8080/tracks/" + currentTrack.value.id + "/play");
player.setAttribute("class", "hidden");
player.setAttribute("preload", "true");
player.addEventListener("timeupdate", updateProgress);

function updateProgress() {
  timestamp.value = player.currentTime;

}

watchEffect(async () => {
  if (playing.value) {
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
        <p>{{ stringifyArtist(currentTrack.album_artist) }} - {{ currentTrack.album }}
          ({{ currentTrack.year || "unknown" }})</p>
        <p>{{ prettyTimestamp(player.currentTime) }} / {{ prettyTimestamp(player.duration) }}</p>
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
