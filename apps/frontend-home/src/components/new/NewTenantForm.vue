<script setup lang="ts">
import { computed, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";
import { LucideChevronLeft } from "lucide-vue-next";
import slugify from "slugify";

const props = defineProps<{
  lang: Language;
}>();

const tl = computed(() => getTranslations(props.lang));

const tenantName = ref("");
const tenantSlug = ref("");
const roleName = ref("");

const loading = ref(false);
const error = ref<keyof typeof tl.value.index | "">("");

async function createTenant() {
  loading.value = true;
  error.value = "";

  // Normalize.
  tenantName.value = tenantName.value.trim();
  tenantSlug.value = slugify(tenantSlug.value, { lower: true, strict: true, trim: true });
  roleName.value = roleName.value.trim();

  try {
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/tenants`, {
      method: "POST",
      mode: "cors",
      credentials: "include",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify({
        name: tenantName.value,
        slug: tenantSlug.value,
        policyName: roleName.value,
      }),
    });

    switch (res.status) {
      case 400:
        error.value = "errorNewTenantBadRequest";
        break;
      case 401:
        error.value = "errorUnauthorized";
        break;
      case 403:
        error.value = "errorSlugForbidden";
        break;
      case 409:
        error.value = "errorSlugTaken";
        break;
    }
  } catch {
    error.value = "errorInternet";
  }
  loading.value = false;
}
</script>

<template>
  <div class="w-full max-w-xl p-6 rounded-xl bg-white shadow-md flex flex-col gap-4">
    <div class="flex flex-row items-center gap-2">
      <a :href="`/${lang}`">
        <LucideChevronLeft :size="24" class="size-6" />
      </a>
      <h1 class="font-semibold text-xl">{{ tl.index.createATenant }}</h1>
    </div>

    <hr class="rounded-full w-full border-black/20" />

    <label class="flex flex-col gap-1 w-full">
      {{ tl.index.tenantName }}
      <input
        type="text"
        class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
        :placeholder="tl.index.placeholderTenantName"
        v-model="tenantName"
      />
    </label>

    <label class="flex flex-col gap-1 w-full">
      {{ tl.index.tenantSlug }}
      <input
        type="text"
        class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
        :placeholder="tl.index.placeholderTenantSlug"
        v-model="tenantSlug"
      />
    </label>

    <label class="flex flex-col gap-1 w-full">
      {{ tl.index.roleName }}
      <input
        type="text"
        class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
        :placeholder="tl.index.placeholderRoleName"
        v-model="roleName"
      />
    </label>

    <span
      class="w-full bg-state-danger/5 p-4 rounded-xl text-state-danger font-semibold"
      v-if="error"
    >
      {{ tl.index[error] }}
    </span>

    <button
      class="bg-violent-violet-600 cursor-pointer rounded-xl p-4 text-white font-semibold duration-200 hover:bg-violent-violet-700 disabled:opacity-50"
      @click="createTenant"
      :disabled="loading"
    >
      {{ loading ? tl.index.loading : tl.index.createATenant }}
    </button>
  </div>
</template>
