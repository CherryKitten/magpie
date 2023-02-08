<script setup lang="ts">

async function get_tracks(): Promise<Array<Object>> {
  const response = await fetch("https://localhost:8080/tracks");
  return await response.json();

}

const tracks = await get_tracks();

</script>

<template>

  <div class="overflow-x-auto">
    <table class="table table-compact w-full">
      <thead>
      <tr>
        <th>Album</th>
        <th>Year</th>
        <th>Disc</th>
        <th>#</th>
        <th>Title</th>

      </tr>
      </thead>
      <tbody>
      <tr v-for="track in tracks">

        <th>{{ track.album }}</th>
        <td>{{ track.year }}</td>
        <td>{{ track.disc_number }}</td>
        <td>{{ track.track_number }}</td>
        <td>{{ track.title }}</td>

      </tr>
      </tbody>
    </table>
  </div>

</template>
