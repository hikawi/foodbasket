<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";
import AvatarProfile from "../AvatarProfile.vue";
import TenantNavigator from "./TenantNavigator.vue";

const props = defineProps<{
  lang: Language;
}>();

const tl = computed(() => getTranslations(props.lang));
const loading = ref(true);
const error = ref("");
const data = ref<{
  user: {
    id: string;
    email: string;
    createdAt: string;
    updatedAt: string;
  };
  tenantId?: string;
  branchId?: string;
  customerProfile?: {};
  staffProfile?: {};
  systemProfile?: {};
} | null>(null);

async function tryFetchMe() {
  loading.value = true;
  error.value = "";

  try {
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/auth/me`, {
      method: "GET",
      mode: "cors",
      credentials: "include",
    });

    if (res.ok) {
      const json = await res.json();
      data.value = json;
    } else {
      error.value = "errorUnauthorized";
    }
  } catch {
    error.value = "errorInternet";
  } finally {
    loading.value = false;
  }
}

onMounted(tryFetchMe);
</script>

<template>
  <div
    v-if="loading"
    class="w-full max-w-xl text-center px-8 py-4 rounded-xl shadow-md bg-white italic"
  >
    {{ tl.index.loading }}
  </div>
  <div
    v-else-if="error"
    class="w-full max-w-xl flex items-center flex-col gap-2 text-center text-state-danger px-8 py-4 rounded-xl shadow-md bg-white"
  >
    {{ tl.index[error as keyof typeof tl.index] }}

    <a v-if="error == 'errorUnauthorized'" :href="`/${lang}/login`" class="underline text-black">
      {{ tl.login.title }}
    </a>
  </div>
  <div v-else class="flex flex-col gap-8 w-full max-w-xl items-center">
    <div class="flex flex-col items-center mb-8">
      <div class="flex flex-row items-center gap-2">
        <AvatarProfile :name="data!.user.email" />
        <span class="text-title-3 text-white">{{ data?.user.email }}</span>
      </div>

      <a :href="`/${lang}/login`" class="text-subheadline text-neptune-300 hover:underline">{{
        tl.index.notYou
      }}</a>
    </div>

    <TenantNavigator :lang />
  </div>
</template>
