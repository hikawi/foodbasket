<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";
import AvatarProfile from "../AvatarProfile.vue";
import { LucideArrowRight } from "lucide-vue-next";

const props = defineProps<{
  lang: Language;
}>();

const tl = computed(() => getTranslations(props.lang));
const loading = ref(true);
const error = ref("");
const tenants = ref<any[]>([]);
const domain = import.meta.env.PUBLIC_DOMAIN || ".foodbasket.app";

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
        tenants.value = json.data;
        // TODO: Add paging.
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
    <div v-else class="w-full flex flex-col">
      <template v-for="tenant in tenants" :key="tenant.id">
        <a
          class="flex group flex-row w-full items-center justify-between cursor-pointer p-1 hover:bg-violent-violet-600/20 rounded-xl pr-4"
          :href="`http://${tenant.slug}${domain}`"
        >
          <div class="w-full flex flex-row gap-2 items-center">
            <AvatarProfile :name="tenant.name" class="rounded-lg" :size="64" />
            <span class="font-semibold">{{ tenant.name }}</span>
          </div>

          <div class="group-hover:flex hidden flex-row gap-1 items-center">
            <span class="font-bold text-violent-violet-600 min-w-fit">{{ tl.index.go }}</span>
            <LucideArrowRight class="size-5 text-violent-violet-600" />
          </div>
        </a>
      </template>
    </div>

    <a class="text-violent-violet-600 text-left cursor-pointer" :href="`/${lang}/new`">
      {{ tl.index.createATenant }}
    </a>
  </div>
</template>
