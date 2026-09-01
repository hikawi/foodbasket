<script setup lang="ts">
import { ref, onMounted } from "vue";
import { LucidePanelRightClose, LucidePanelRightOpen } from "lucide-vue-next";
import scopedFetch from "@/utils/fetcher";

defineOptions({
  name: "UsersSubpage",
});

interface StaffMember {
  id: string;
  name: string;
  avatarUrl: string | null;
  createdAt: string;
  updatedAt: string;
}

// Collapsible Properties Pane State
const isPropertiesOpen = ref(true);

// Data & Loading States
const staffList = ref<StaffMember[]>([]);
const isLoading = ref(true);
const fetchError = ref<string | null>(null);

const fetchStaff = async () => {
  isLoading.value = true;
  fetchError.value = null;

  try {
    const response = await scopedFetch({
      path: "/v1/staff",
      method: "GET",
    });

    if (!response.ok) {
      throw new Error(`Failed to fetch staff list (Status: ${response.status})`);
    }

    const data = await response.json();
    staffList.value = data;
  } catch (err: any) {
    fetchError.value = err.message ?? "An error occurred while fetching staff data";
  } finally {
    isLoading.value = false;
  }
};

onMounted(() => {
  fetchStaff();
});
</script>

<template>
  <div class="flex w-full h-full overflow-hidden gap-4 flex-row">
    <section class="w-90 h-full flex flex-col shrink-0 overflow-y-auto p-4 gap-4">
      <div class="flex items-center justify-between">
        <h2 class="text-headline font-bold">Staff Master</h2>
        <button
          type="button"
          @click="fetchStaff"
          class="px-2 py-1 text-caption1 bg-grouped-background-primary rounded hover:bg-grouped-background-secondary transition-colors"
        >
          Refresh
        </button>
      </div>

      <!-- Loading / Error States -->
      <div v-if="isLoading" class="text-caption1 text-label-secondary">Loading staff list...</div>

      <div v-else-if="fetchError" class="text-caption1 text-state-danger">
        {{ fetchError }}
      </div>

      <!-- JSON Raw Output for Master Pane -->
      <pre
        v-else
        class="text-caption2 bg-grouped-background-primary p-2 rounded overflow-x-auto whitespace-pre-wrap font-mono"
        >{{ staffList }}</pre
      >
    </section>

    <main class="flex-1 h-full flex flex-col min-w-0 overflow-y-auto p-4 gap-4">
      <div class="flex items-center justify-between">
        <h1 class="text-title-2 font-bold">Detail View</h1>

        <!-- Toggle Button for Properties Pane -->
        <button
          type="button"
          @click="isPropertiesOpen = !isPropertiesOpen"
          class="flex items-center gap-2 px-3 py-1.5 rounded bg-grouped-background-primary hover:bg-grouped-background-secondary transition-colors text-caption1"
        >
          <component
            :is="isPropertiesOpen ? LucidePanelRightClose : LucidePanelRightOpen"
            class="size-4"
          />
          <span>{{ isPropertiesOpen ? "Hide Properties" : "Show Properties" }}</span>
        </button>
      </div>

      <div class="p-4 bg-grouped-background-primary rounded">
        Select a staff member from the master list to inspect details.
      </div>
    </main>

    <aside v-if="isPropertiesOpen" class="w-90 h-full flex flex-col shrink-0 overflow-y-auto p-4">
      <h3 class="text-title-3 font-semibold mb-2">Properties</h3>
      <p class="text-caption1 text-label-secondary">
        Collapsible sidebar metadata and options panel.
      </p>
    </aside>
  </div>
</template>
