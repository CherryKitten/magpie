<script setup>
import { store } from "@/store";
import AlbumList from "@/components/AlbumList.vue";
import ArtistList from "@/components/ArtistList.vue";

const props = defineProps(["view", "query"]);

</script>

<template>
  <div class="min-h-screen">
    <div class="overflow-x-auto">
      <table class="table table-compact w-full">
        <ArtistList v-if="props.view === 'artists'" />
        <AlbumList v-if="props.view === 'albums'" :filter="props.query.artist" />
      </table>
    </div>
  </div>
</template>

<style scoped>

</style>
