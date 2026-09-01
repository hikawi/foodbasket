<script setup lang="ts">
import CommonHeader from "@/components/CommonHeader.vue";
import { useProfileStore } from "@/stores/profile";

const profile = useProfileStore();
</script>

<template>
  <CommonHeader module="dashboard" />

  <p>{{ profile.profile }}</p>
</template>
