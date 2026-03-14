<script setup lang="ts">
import { onMounted, ref } from "vue";

defineProps<{
  tl: any;
}>();

const loading = ref(true);
const error = ref("");
const data = ref();

async function tryFetchMe() {
  loading.value = true;
  error.value = "";

  try {
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/auth/me`, {
      method: "GET",
      mode: "cors",
      credentials: "include",
    });

    switch (res.status) {
      case 404:
        error.value = "errorNotTenant";
        break;
      case 401:
        error.value = "errorUnauthorized";
        break;
      case 200:
        const json = await res.json();
        data.value = json;
        break;
    }
  } catch {
    error.value = "errorInternet";
  }

  loading.value = false;
}

onMounted(tryFetchMe);
</script>

<template>
  <p v-if="loading">{{ tl.index.loading }}</p>
  <p v-if="error">{{ tl.index[error] }}</p>
  <p v-else>{{ JSON.stringify(data) }}</p>
</template>
