<script setup lang="ts">
import { computed } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";

type Selection = "your_tenants" | "invitations";

const props = defineProps<{
  selected: Selection;
  lang: Language;
}>();

defineEmits<{
  select: [selection: Selection];
}>();

const tl = computed(() => getTranslations(props.lang));
const tabs = computed<{ label: string; selection: Selection }[]>(() => [
  {
    label: tl.value.index.yourTenants,
    selection: "your_tenants",
  },
  {
    label: tl.value.index.invitations,
    selection: "invitations",
  },
]);
</script>

<template>
  <div class="w-full flex flex-row gap-5 items-center border-b border-black/20">
    <template v-for="tab in tabs" :key="tab.selection">
      <button
        class="flex px-2 py-1 border-b cursor-pointer"
        :class="{
          'border-transparent duration-200 hover:border-violent-violet-600':
            selected !== tab.selection,
          'border-violent-violet-600': selected === tab.selection,
        }"
        @click="$emit('select', tab.selection)"
      >
        {{ tab.label }}
      </button>
    </template>
  </div>
</template>
