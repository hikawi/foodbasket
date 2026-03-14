<script setup lang="ts">
import { computed, ref } from "vue";
import TenantTabSwitcher from "./TenantTabSwitcher.vue";
import type { Language } from "../../utils/i18n";
import YourTenantsWidget from "./YourTenantsWidget.vue";
import InvitationsWidget from "./InvitationsWidget.vue";

defineProps<{
  lang: Language;
}>();

const selected = ref<"your_tenants" | "invitations">("your_tenants");
const selectedComponent = computed(() =>
  selected.value == "your_tenants" ? YourTenantsWidget : InvitationsWidget,
);
const transitionName = computed(() =>
  selected.value === "your_tenants" ? "slide-right" : "slide-left",
);
</script>

<template>
  <div class="rounded-xl shadow-md flex flex-col w-full gap-8 p-6 overflow-x-hidden bg-white">
    <TenantTabSwitcher :selected :lang @select="(val) => (selected = val)" />

    <Transition :name="transitionName" mode="out-in">
      <KeepAlive>
        <component :is="selectedComponent" :key="selected" :lang />
      </KeepAlive>
    </Transition>
  </div>
</template>

<style>
.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: transform 0.2s ease;
}
.slide-left-enter-from {
  transform: translateX(100%);
}
.slide-left-leave-to {
  transform: translateX(-100%);
}
.slide-right-enter-from {
  transform: translateX(-100%);
}
.slide-right-leave-to {
  transform: translateX(100%);
}
</style>
