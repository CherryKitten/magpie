<script setup>
import AlbumList from "@/components/AlbumList.vue";
import ArtistList from "@/components/ArtistList.vue";
import Tracks from "@/components/Tracks.vue";

const props = defineProps(["view", "query"]);

</script>

<template>
  <div class="min-h-screen">
    <div class="overflow-x-auto">
      <table class="table table-compact w-full">
        <ArtistList v-if="props.view === 'artists'" />
        <AlbumList v-if="props.view === 'albums'" :query="props.query" />
        <Tracks v-if="props.view === 'tracks'" :query="props.query" />
      </table>
    </div>
  </div>
</template>

<style scoped>

</style>
