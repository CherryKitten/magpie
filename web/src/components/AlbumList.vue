<script setup>
import { store } from "@/store";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import router from "@/router";

const props = defineProps(["query"]);

store.getAlbums("?artist=" + props.query.artist);

</script>


<template>
  <thead>
  <tr>
    <th>Artist</th>
    <th>Album</th>
    <th>Year</th>
    <th></th>

  </tr>
  </thead>
  <tbody>
  <tr v-for="album in store.albums">

    <th>{{ album.artist }}</th>
    <td @click="router.push({ path: '/tracks', query: { album: album.id }})">
      {{ album.title }}
    </td>
    <td>{{ album.year }}</td>
    <td>
      <FontAwesomeIcon icon="fa-solid fa-play"
                       @click="store.add_album(album)"
                       class="btn btn-ghost btn-sm" />
    </td>

  </tr>
  </tbody>
</template>

<style scoped>

</style>
