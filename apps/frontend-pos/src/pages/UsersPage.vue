<script setup lang="ts">
import CommonHeader from "@/components/CommonHeader.vue";
import DrillPane from "@/components/DrillPane.vue";
import MasterDetailLayout from "@/components/MasterDetailLayout.vue";
import { type DrillAxis, USERS_DRILL_BAR } from "@/utils/drills";
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const drillAxis = ref<DrillAxis>();

onMounted(() => {
  drillAxis.value = USERS_DRILL_BAR.find((v) => v.link == route.name)?.id || "users";
});

watch(route, (val) => {
  drillAxis.value = USERS_DRILL_BAR.find((v) => v.link == val.name)?.id || "users";
});
</script>

<template>
  <CommonHeader module="users" />

  <MasterDetailLayout>
    <template #drill>
      <DrillPane :items="USERS_DRILL_BAR" :selectedId="drillAxis" />
    </template>

    <template #default>
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component" :key="route.name" />
        </keep-alive>
      </router-view>
    </template>
  </MasterDetailLayout>
</template>
