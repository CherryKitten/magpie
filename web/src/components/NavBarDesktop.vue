<template>
  <div class="navbar bg-base-200">
    <div class="flex-1">
      <a class="btn btn-ghost normal-case text-xl">Magpie</a>
    </div>
    <div class="form-control">
      <input type="text" placeholder="Search" class="input input-bordered" />
    </div>
    <ul class="ml-20 p-3 shadow bg-base-100 rounded-box">
      <router-link :to="{ name: 'tracks'}">
        Tracks
      </router-link>

    </ul>
  </div>
</template>

<script>
import { RouterLink } from "vue-router";

export default {
  name: "NavBarDesktop"
};
</script>

<style scoped>

</style>
