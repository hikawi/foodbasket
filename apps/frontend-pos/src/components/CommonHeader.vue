<script setup lang="ts">
import { ref } from "vue";
import { LucideChevronDown, LucideGlobe, LucideExternalLink, LucideMenu } from "lucide-vue-next";
import NavigationBar from "./NavigationBar.vue";
import { useI18n } from "vue-i18n";
import type { Module } from "@/utils/modules";

const domain = import.meta.env.VITE_PUBLIC_DOMAIN ?? "https://google.com";
const { locale } = useI18n();
const isMobileMenuOpen = ref(false);

defineProps<{
  module: Module;
}>();

const toggleLanguage = () => {
  locale.value = locale.value === "en-US" ? "ja-JP" : "en-US";
};
</script>

<template>
  <header class="w-full px-4 py-2 bg-background-primary">
    <!-- Desktop Layout: auto-expanding center column with balanced sides -->
    <div class="hidden xl:grid grid-cols-[1fr_auto_1fr] items-center w-full gap-4">
      <div class="flex items-center gap-4 justify-start min-w-0">
        <div class="p-1 rounded-full bg-background-secondary shrink-0">
          <img
            src="https://via.placeholder.com/56"
            alt="Logo"
            class="size-12 rounded-full object-cover"
          />
        </div>

        <a
          :href="domain"
          target="_blank"
          rel="noopener noreferrer"
          class="px-4 py-2 rounded-lg bg-grouped-background-primary text-label-primary text-body font-medium transition-colors hover:bg-accent-primary/20 truncate"
        >
          Placeholder
        </a>
      </div>

      <div class="flex justify-center max-w-full overflow-x-auto">
        <NavigationBar :selected="module" />
      </div>

      <div class="flex items-center gap-4 justify-end">
        <button
          type="button"
          @click="toggleLanguage"
          class="flex items-center gap-2 px-3 py-2 rounded-lg text-label-primary transition-colors hover:bg-grouped-background-primary shrink-0"
        >
          <LucideGlobe class="size-5" />
          <span class="text-caption1 uppercase">{{ locale.split("-")[0] }}</span>
        </button>

        <div
          class="flex items-center pl-4 pr-1 py-1 gap-1 rounded-full bg-grouped-background-primary cursor-pointer transition-colors hover:bg-accent-primary/20 shrink-0"
        >
          <LucideChevronDown class="size-6 text-label-primary" />

          <div class="flex flex-col justify-center text-left">
            <span class="text-body text-label-primary leading-tight">John Doe</span>
            <span class="text-caption2 text-label-primary leading-tight">Store Manager</span>
          </div>

          <img
            src="https://via.placeholder.com/48"
            alt="User Avatar"
            class="size-12 rounded-full object-cover ml-3"
          />
        </div>
      </div>
    </div>

    <!-- Mobile Responsive Layout: Uniform 5-button row -->
    <div class="flex xl:hidden items-center justify-between w-full">
      <div class="flex items-center gap-2">
        <button
          type="button"
          class="size-10 p-0.5 rounded-full bg-background-secondary flex items-center justify-center shrink-0 overflow-hidden"
        >
          <img
            src="https://via.placeholder.com/40"
            alt="Logo"
            class="size-full rounded-full object-cover"
          />
        </button>

        <a
          :href="domain"
          target="_blank"
          rel="noopener noreferrer"
          class="size-10 rounded-full bg-grouped-background-primary flex items-center justify-center text-label-primary shrink-0 hover:bg-accent-primary/20 transition-colors"
          aria-label="Open storefront"
        >
          <LucideExternalLink class="size-5" />
        </a>
      </div>

      <!-- Mobile Right Group (3 Buttons) -->
      <div class="flex items-center gap-2">
        <!-- 3. Navigation Menu Toggle -->
        <button
          type="button"
          @click="isMobileMenuOpen = !isMobileMenuOpen"
          class="size-10 rounded-full bg-grouped-background-primary flex items-center justify-center text-label-primary shrink-0 hover:bg-accent-primary/20 transition-colors"
          aria-label="Toggle navigation menu"
        >
          <LucideMenu class="size-5" />
        </button>

        <!-- 4. Language Switcher -->
        <button
          type="button"
          @click="toggleLanguage"
          class="size-10 rounded-full bg-grouped-background-primary flex items-center justify-center text-label-primary shrink-0 hover:bg-accent-primary/20 transition-colors"
          aria-label="Switch language"
        >
          <LucideGlobe class="size-5" />
        </button>

        <!-- 5. User Profile Avatar -->
        <button
          type="button"
          class="size-10 rounded-full bg-grouped-background-primary flex items-center justify-center text-label-primary shrink-0 hover:bg-accent-primary/20 transition-colors overflow-hidden"
          aria-label="Open profile settings"
        >
          <img
            src="https://via.placeholder.com/40"
            alt="User Avatar"
            class="size-full rounded-full object-cover"
          />
        </button>
      </div>
    </div>
  </header>
</template>
