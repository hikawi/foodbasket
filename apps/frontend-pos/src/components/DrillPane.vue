<script setup lang="ts">
import type { DrillComponent } from "@/utils/drills";
import { useI18n } from "vue-i18n";

const props = defineProps<{
  items: DrillComponent[];
  selectedId?: string;
}>();

const { t } = useI18n();
</script>

<template>
  <div
    class="h-full bg-grouped-background-primary flex flex-col items-center gap-1 p-1 shrink-0 select-none rounded-xl"
  >
    <router-link
      v-for="item in items"
      :key="item.id"
      :to="{ name: item.link }"
      class="size-16 flex flex-col items-center justify-center gap-0.5 rounded-xl transition-colors cursor-pointer"
      :class="[
        selectedId === item.id
          ? 'bg-grouped-background-secondary'
          : 'hover:bg-grouped-background-secondary/50',
      ]"
    >
      <component :is="item.icon" class="size-6 shrink-0" />
      <span class="text-caption1 text-center w-full">
        {{ t(item.label) }}
      </span>
    </router-link>
  </div>
</template>
