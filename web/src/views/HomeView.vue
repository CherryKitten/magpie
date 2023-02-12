<script setup lang="ts">
const props = defineProps(["searchText"]);

</script>

<template>
  <main>
    <h1>
      {{ props.searchText }}
    </h1>

  </main>
</template>
