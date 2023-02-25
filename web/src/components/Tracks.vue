<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import { store } from "@/store";

const props = defineProps(["query"]);

store.getTracks("?album=" + props.query.album);

</script>

<template>

  <div class="overflow-x-auto">
    <table class="table table-compact w-full">
      <thead>
      <tr>
        <th>Disc</th>
        <th>#</th>
        <th>Title</th>
        <th>Album</th>
        <th>Year</th>
        <th></th>


      </tr>
      </thead>
      <tbody>
      <tr v-for="track in store.tracks">

        <td>{{ track.disc_number }}</td>
        <td>{{ track.track_number }}</td>
        <td>{{ track.title }}</td>
        <th>{{ track.album[1] }}</th>
        <td>{{ track.year }}</td>
        <td>
          <FontAwesomeIcon icon="fa-solid fa-play"
                           @click="store.add_track(track.id)"
                           class="btn btn-ghost btn-sm" />
        </td>


      </tr>
      </tbody>
    </table>
  </div>

</template>
