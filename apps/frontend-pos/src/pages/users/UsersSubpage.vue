<script setup lang="ts">
import { ref, onMounted } from "vue";
import { LucidePanelRightClose, LucidePanelRightOpen } from "lucide-vue-next";
import scopedFetch from "@/utils/fetcher";
import StaffMasterList from "@/components/users/StaffMasterList.vue";
import type { StaffMember } from "@/utils/types";

// Selection & Sidebar State
const selectedStaff = ref<StaffMember | null>(null);
const isPropertiesOpen = ref(true);

// API Fetch State
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

    const payload = await response.json();
    staffList.value = payload.data ?? [];
  } catch (err: any) {
    fetchError.value = err.message ?? "An error occurred while fetching staff data";
  } finally {
    isLoading.value = false;
  }
};

function handleSelect(item: StaffMember) {
  selectedStaff.value = item;
}

onMounted(() => {
  fetchStaff();
});
</script>

<template>
  <div class="flex w-full h-full overflow-hidden gap-4 flex-row">
    <section class="w-90 h-full flex flex-col shrink-0 overflow-y-auto gap-4">
      <StaffMasterList :items="staffList" @select="handleSelect" />
    </section>

    <main
      class="flex-1 h-full flex flex-col min-w-0 overflow-y-auto gap-4 bg-background-primary rounded-xl"
    >
      <div class="flex items-center justify-between border-b pb-3">
        <div>
          <h1 class="text-title2 font-semibold">
            {{ selectedStaff?.name ?? "No Selection" }}
          </h1>
          <p class="text-caption1 text-label-secondary font-mono">
            ID: {{ selectedStaff?.id ?? "N/A" }}
          </p>
        </div>

        <button
          type="button"
          @click="isPropertiesOpen = !isPropertiesOpen"
          class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-grouped-background-primary hover:bg-grouped-background-secondary transition-colors text-caption1"
        >
          <component
            :is="isPropertiesOpen ? LucidePanelRightClose : LucidePanelRightOpen"
            class="size-4"
          />
          <span>{{ isPropertiesOpen ? "Hide Details" : "Show Details" }}</span>
        </button>
      </div>

      <!-- Detail Barebones Raw JSON -->
      <div class="flex-1 min-h-0 overflow-y-auto">
        <h3 class="text-caption1 text-label-secondary uppercase tracking-wider mb-2">
          Raw Selected Payload
        </h3>
        <pre
          class="text-caption2 bg-grouped-background-primary p-3 rounded-lg overflow-x-auto whitespace-pre-wrap font-mono"
          >{{ selectedStaff ?? "Select a staff member from the left list." }}</pre
        >
      </div>
    </main>

    <!-- 3. Properties Pane (Collapsible Fixed 360px) -->
    <aside
      v-if="isPropertiesOpen"
      class="w-90 h-full flex flex-col shrink-0 overflow-y-auto bg-background-primary rounded-xl gap-3"
    >
      <h3 class="text-title3 font-semibold border-b pb-2">Staff Meta Properties</h3>

      <div v-if="selectedStaff" class="flex flex-col gap-2 text-caption1">
        <div>
          <span class="text-label-secondary block">Tenant ID:</span>
          <span class="font-mono text-caption2">{{ selectedStaff.tenant_id }}</span>
        </div>
        <div>
          <span class="text-label-secondary block">User ID:</span>
          <span class="font-mono text-caption2">{{ selectedStaff.user_id }}</span>
        </div>
        <div>
          <span class="text-label-secondary block">Created At:</span>
          <span>{{ selectedStaff.created_at }}</span>
        </div>
      </div>

      <div v-else class="text-caption1 text-label-secondary">No active item selected.</div>
    </aside>
  </div>
</template>
