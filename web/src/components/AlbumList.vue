<script setup>
import { store } from "@/store";

const props = defineProps(["filter"]);

let albums = store.albums;
if (props.filter) {
  albums = albums.filter((album) => {
    for (let id in album.artist_ids) {
      if (id == props.filter) {
        return true;
      }
    }
  });
}

</script>


<template>
  <thead>
  <tr>
    <th>Artist</th>
    <th>Album</th>
    <th>Year</th>

  </tr>
  </thead>
  <tbody>
  <tr v-for="album in albums" @click="store.add_album(album)">

    <th>{{ album.artist }}</th>
    <td>{{ album.title }}</td>
    <td>{{ album.year }}</td>

  </tr>
  </tbody>
</template>

<style scoped>

</style>
