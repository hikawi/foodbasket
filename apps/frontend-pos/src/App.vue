<script setup lang="ts">
import { onMounted, ref } from 'vue';

const host = ref(window.location.host);
</script>

<template>
  <h1 class="">You did it! You should be on *.pos.foodbasket.app {{ host }}</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>

  <div>Hi</div>
</template
