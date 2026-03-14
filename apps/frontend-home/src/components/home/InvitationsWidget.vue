<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";

const props = defineProps<{
  lang: Language;
}>();

const tl = computed(() => getTranslations(props.lang));
const loading = ref(true);
const error = ref("");
const invitations = ref<any[]>([]);

async function tryFetchInvitations() {
  loading.value = true;
  error.value = "";
  try {
    invitations.value = [];
  } catch {
    error.value = "errorInternet";
  }
  loading.value = false;
}

onMounted(tryFetchInvitations);
</script>

<template>
  <p class="italic" v-if="loading">{{ tl.index.loading }}</p>
  <p class="text-state-danger" v-else-if="error">{{ tl.index[error as keyof typeof tl.index] }}</p>
  <div class="flex flex-col gap-8 w-full" v-else>
    <div class="min-h-40 flex items-center justify-center italic" v-if="invitations.length == 0">
      {{ tl.index.noInvitations }}
    </div>
    <div v-else></div>
  </div>
</template>
