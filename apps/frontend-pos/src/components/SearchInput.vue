<script setup lang="ts">
import { LucideSearch } from "lucide-vue-next";

defineProps<{
  placeholder?: string;
}>();

const value = defineModel<string>();
</script>

<template>
  <div
    class="w-full flex items-center rounded-lg gap-2 py-1 bg-grouped-background-secondary pr-2 pl-3"
  >
    <LucideSearch
      class="size-4 pointer-events-none shrink-0 duration-200 transition-colors"
      :class="{ 'text-label-primary': value != '', 'text-label-secondary': value == '' }"
    />
    <input
      type="text"
      v-model="value"
      :placeholder="placeholder ?? $t('general.search')"
      class="w-full text-body text-label-primary placeholder:text-label-secondary outline-none border-none transition-colors"
    />
  </div>
</template>
