<script setup lang="ts">
import { RouterLink } from "vue-router";
import { ref } from 'vue'

const searchText = ref('Hello World!')

</script>

<template>
  <div class="navbar bg-base-200">
    <div class="flex-1">
      <RouterLink class="btn btn-ghost normal-case text-xl" :to="{ name: 'home'}">
        Magpie
      </RouterLink>
    </div>
    <div class="form-control">
      <input type="text" placeholder="Search" class="input input-bordered" v-model="searchText" />
    </div>

  </div>
</template>

<style scoped>

</style>
