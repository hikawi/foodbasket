<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  name: string;
  avatarUrl?: string;
  size?: number;
}>();

const actualSize = computed(() => props.size || 32);
const computedUrl = computed(
  () =>
    props.avatarUrl ||
    `https://ui-avatars.com/api/?name=${encodeURIComponent(props.name)}&background=random&size=${actualSize.value}`,
);
</script>

<template>
  <img class="rounded-full object-cover object-center" :src="computedUrl" alt="" />
</template>
