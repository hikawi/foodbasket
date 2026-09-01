<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useProfileStore } from "./stores/profile";
import { errorCode } from "./utils/error";

const domain = import.meta.env.VITE_PUBLIC_DOMAIN;

const { locale } = useI18n();
const profileStore = useProfileStore();

const lang = computed(() => locale.value.split("-")[0]);
const loginUrl = computed(
  () => `${domain}/${lang}/login?callback=${encodeURIComponent(window.location.href)}`,
);

onMounted(async () => {
  await profileStore.fetchProfile();
});
</script>

<template>
  <!-- Fullscreen Loading View -->
  <div
    v-if="profileStore.loading"
    class="min-h-screen flex items-center justify-center bg-background-primary text-label-primary"
  >
    <p class="italic text-title-3">{{ $t("general.loading") }}</p>
  </div>

  <!-- Full-Canvas Application View -->
  <div
    class="flex flex-col w-full min-h-screen bg-background-primary text-label-primary gap-4 p-3"
    v-else-if="profileStore.profile?.tenantId !== undefined && profileStore.profile?.staffProfile"
  >
    <router-view></router-view>
  </div>

  <!-- Fullscreen Container for Auth & Error Cards -->
  <main
    v-else
    class="min-h-screen h-fit flex items-center justify-center text-label-primary bg-background-primary"
  >
    <div class="p-6 rounded-xl bg-white shadow-md max-w-xl w-full flex flex-col gap-4">
      <!-- Unauthorized Check -->
      <p
        v-if="profileStore.error && profileStore.error.code == errorCode.unauthorized"
        class="text-title-2"
      >
        Oh no, you're not logged in.
        <a :href="loginUrl" class="text-accent-primary font-semibold">Want to?</a>
      </p>

      <!-- Unregistered Tenant Check -->
      <div
        v-else-if="profileStore.error && profileStore.error.code == errorCode.unknownTenant"
        class="flex flex-col gap-2 items-center w-full text-center"
      >
        <p class="text-headline text-state-danger">{{ $t("general.errorUnknownTenant") }}</p>
        <a :href="`${domain}/${lang}/start`" class="text-accent-primary underline">{{
          $t("general.backHome")
        }}</a>
      </div>

      <!-- No Staff Profile Fallback -->
      <div v-else class="flex flex-col gap-2 items-center w-full text-center">
        <p class="text-headline">{{ $t("general.errorNoStaffProfile") }}</p>
        <p class="text-subheadline">{{ $t("general.errorNoStaffProfileDesc") }}</p>
        <a :href="`${domain}/${lang}/start`">{{ $t("general.backHome") }}</a>
      </div>
    </div>
  </main>
</template>
