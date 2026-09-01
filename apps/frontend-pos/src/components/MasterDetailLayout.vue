<script setup lang="ts">
import { computed } from "vue";
import type { DrillComponent } from "@/utils/drills";

const props = withDefaults(
  defineProps<{
    /** Enable/disable the leftmost DrillPane */
    showDrillBar?: boolean;
    /** Modules to enable inside the DrillPane */
    enabledModules?: DrillComponent[];
    /** Currently selected module in the DrillPane */
    selectedModule?: DrillComponent;
    /** Enable/disable the rightmost PropertiesPane */
    showPropertiesPane?: boolean;
  }>(),
  {
    showDrillBar: true,
    showPropertiesPane: true,
  },
);
</script>

<template>
  <div class="w-full h-full flex flex-row overflow-hidden gap-4">
    <aside v-if="showDrillBar" class="h-full flex shrink-0 overflow-y-auto">
      <slot name="drill">
        <div>Drill</div>
      </slot>
    </aside>

    <slot />
  </div>
</template>
