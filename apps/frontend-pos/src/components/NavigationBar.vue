<script setup lang="ts">
import {
  LucideEllipsis,
  LucideLayoutDashboard,
  LucideScrollText,
  LucideSquareMenu,
  LucideTable2,
  LucideUsers,
} from "lucide-vue-next";

const props = defineProps<{
  enabledModules: {
    module: string;
    link: string;
    label: string;
  }[];
  selected?: string;
}>();

const icons: Record<string, any> = {
  dashboard: LucideLayoutDashboard,
  orders: LucideScrollText,
  tables: LucideTable2,
  menus: LucideSquareMenu,
  users: LucideUsers,
  more: LucideEllipsis,
};
</script>

<template>
  <div class="p-1 rounded-full bg-grouped-background-primary grid grid-flow-col auto-cols-fr gap-1">
    <router-link
      v-for="module in enabledModules"
      :key="module.link"
      :to="module.link"
      class="px-2 py-1 rounded-full flex flex-col items-center transition-colors"
      :class="{ 'bg-grouped-background-secondary': selected === module.module }"
    >
      <component :is="icons[module.module]" class="text-label-primary size-6" />
      <span class="text-caption1-emphasized">
        {{ module.label }}
      </span>
    </router-link>
  </div>
</template>
