<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";

const props = defineProps<{
  lang: Language;
}>();

const tl = computed(() => getTranslations(props.lang));
const loading = ref(true);
const error = ref("");
const tenants = ref<any[]>([]);

async function tryFetchTenants() {
  loading.value = true;
  error.value = "";
  try {
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/tenants`, {
      method: "GET",
      mode: "cors",
      credentials: "include",
    });

    switch (res.status) {
      case 401:
        error.value = "errorUnauthorized";
        break;
      case 200:
        const json = await res.json();
        console.log(json);
        break;
    }
  } catch {
    error.value = "errorInternet";
  }
  loading.value = false;
}

onMounted(tryFetchTenants);
</script>

<template>
  <p class="italic" v-if="loading">{{ tl.index.loading }}</p>
  <p class="text-state-danger" v-else-if="error">{{ tl.index[error as keyof typeof tl.index] }}</p>
  <div class="flex flex-col gap-8 w-full" v-else>
    <div class="min-h-40 flex items-center justify-center italic" v-if="tenants.length == 0">
      {{ tl.index.noTenants }}
    </div>
    <div v-else></div>

    <a class="text-violent-violet-600 text-left cursor-pointer" :href="`/${lang}/new`">
      {{ tl.index.createATenant }}
    </a>
  </div>
</template>
