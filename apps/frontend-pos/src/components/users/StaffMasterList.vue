<script setup lang="ts">
import { ref } from "vue";
import { LucidePlus, LucideGroup, LucideArrowDownWideNarrow } from "lucide-vue-next";
import SearchInput from "@/components/SearchInput.vue";
import type { StaffMember } from "@/utils/types";

const props = defineProps<{
  items: StaffMember[];
  selectedId?: string;
}>();

const emit = defineEmits<{
  (e: "select", item: StaffMember): void;
}>();

const searchQuery = ref("");

// Top bar action stubs
const add = () => {};
const showFilterOptions = () => {};
const showGroupingOptions = () => {};
</script>

<template>
  <div
    class="w-full h-full flex flex-col p-4 gap-2 bg-grouped-background-primary rounded-xl overflow-hidden"
  >
    <!-- Top Action Bar -->
    <div class="flex items-center justify-end shrink-0">
      <button
        type="button"
        @click="add"
        class="p-1 rounded-lg cursor-pointer text-label-primary hover:bg-grouped-background-secondary transition-colors"
        aria-label="Add user"
      >
        <LucidePlus class="size-6" />
      </button>

      <button
        type="button"
        @click="showFilterOptions"
        class="p-1 rounded-lg cursor-pointer text-label-primary hover:bg-grouped-background-secondary transition-colors"
        aria-label="Filter options"
      >
        <LucideArrowDownWideNarrow class="size-6" />
      </button>

      <button
        type="button"
        @click="showGroupingOptions"
        class="p-1 rounded-lg cursor-pointer text-label-primary hover:bg-grouped-background-secondary transition-colors"
        aria-label="Grouping options"
      >
        <LucideGroup class="size-6" />
      </button>
    </div>

    <!-- Search Input Component -->
    <SearchInput v-model="searchQuery" />

    <!-- List Content Area -->
    <div class="flex-1 min-h-0 overflow-y-auto flex flex-col gap-1 mt-1">
      <!-- Empty State -->
      <div
        v-if="!items || items.length === 0"
        class="h-full flex items-center justify-center p-4 cursor-pointer select-none"
        @click="add"
      >
        <span class="text-title3 text-label-secondary text-center">
          {{ $t("users.users.emptyList") }}
        </span>
      </div>

      <!-- Item List -->
      <button
        v-else
        v-for="item in items"
        :key="item.id"
        type="button"
        @click="emit('select', item)"
        class="w-full text-left p-3 rounded-lg transition-colors cursor-pointer"
        :class="[
          selectedId === item.id
            ? 'bg-grouped-background-secondary text-label-primary'
            : 'hover:bg-grouped-background-secondary/50 text-label-primary',
        ]"
      >
        <span class="text-body font-medium block truncate">{{ item.name }}</span>
      </button>
    </div>
  </div>
</template>
