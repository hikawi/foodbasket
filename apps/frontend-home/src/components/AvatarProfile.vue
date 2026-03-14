<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  name: string;
  avatarUrl?: string;
}>();

const computedUrl = computed(
  () =>
    props.avatarUrl ||
    `https://ui-avatars.com/api/?name=${encodeURIComponent(props.name)}&background=random&size=32`,
);
</script>

<template>
  <img class="size-8 rounded-full object-cover object-center" :src="computedUrl" alt="" />
</template>
