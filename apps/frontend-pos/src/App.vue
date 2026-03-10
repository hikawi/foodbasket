<script setup lang="ts">
import { ref } from 'vue';
import PlaceholderProfile from './components/demo/PlaceholderProfile.vue';

const host = ref(window.location.host);
</script>

<template>
  <h1 class="">You did it! You should be on {{ host }}</h1>
  <PlaceholderProfile />
</template
