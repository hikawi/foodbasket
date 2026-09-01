<script setup lang="ts">
import { DEFAULT_MODULES, type Module, type ModuleConfig } from "@/utils/modules";
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    enabledModules?: (Module | (Partial<ModuleConfig> & { module: Module }))[];
    selected?: Module;
  }>(),
  {
    enabledModules: () => Object.keys(DEFAULT_MODULES) as Module[],
  },
);

const resolvedModules = computed<ModuleConfig[]>(() => {
  return props.enabledModules.map((item) => {
    if (typeof item === "string") {
      return {
        module: item,
        ...DEFAULT_MODULES[item],
      };
    }

    return {
      ...DEFAULT_MODULES[item.module],
      ...item,
    };
  });
});
</script>

<template>
  <div
    class="p-1 rounded-full min-w-2xl bg-grouped-background-primary grid grid-flow-col auto-cols-fr gap-1"
  >
    <router-link
      v-for="item in resolvedModules"
      :key="item.link"
      :to="item.link"
      class="px-2 py-1 rounded-full flex flex-col items-center transition-colors"
      :class="{
        'bg-grouped-background-secondary': selected === item.module,
        'hover:bg-grouped-background-secondary/50': selected !== item.module,
      }"
    >
      <component :is="item.icon" class="text-label-primary size-6" />
      <span class="text-caption1-emphasized">
        {{ $t(item.label) }}
      </span>
    </router-link>
  </div>
</template>
