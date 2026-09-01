<script setup lang="ts">
import { onMounted } from "vue";
import { useProfileStore } from "./stores/profile";

const profileStore = useProfileStore();
const loginUrl = `${import.meta.env.VITE_PUBLIC_DOMAIN}/en/login?callback=${encodeURIComponent(window.location.href)}`;

onMounted(async () => {
  profileStore.fetchProfile();
});
</script>

<template>
  <main
    class="min-h-screen h-fit flex items-center justify-center text-label-primary bg-background-primary"
  >
    <div class="p-6 rounded-xl bg-white shadow-md max-w-xl w-full flex flex-col gap-4">
      <p v-if="profileStore.loading" class="italic text-title-3">{{ $t("general.loading") }}</p>
      <template v-else-if="profileStore.profile">
        <template v-if="profileStore.profile.staffProfile"> </template>
        <template v-else>
          <p>Nice profile</p>
        </template>
      </template>
      <p v-else class="text-title-2">
        Oh no, you're not logged in.
        <a :href="loginUrl" class="text-accent-primary font-semibold">Want to?</a>
      </p>
    </div>
  </main>
</template>
