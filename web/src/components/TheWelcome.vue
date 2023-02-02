<script setup lang="ts">

async function get_tracks() {
  const response = await fetch("https://localhost:8000");
  return await response.json();

}

function buildArtistList(artists: any[]) {
  if (artists.length == 1) {
    return artists[0];
  } else {
    let temp = "";
    artists.forEach((artist: string, i: number) => {
      if (i === 0) {
        temp = temp + artist;
      } else {
        temp = temp + ", " + artist;
      }
    });
    return temp;
  }
}

const tracks = await get_tracks();

</script>

<template>

  <ul>
    <li v-for="track in tracks">
      {{ buildArtistList(track.albumartists) }} - {{ track.album }} - {{ track.title }}
    </li>
  </ul>


</template>
