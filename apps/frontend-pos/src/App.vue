<script setup lang="ts">
import { computed, ref } from 'vue';
import PlaceholderProfile from './components/demo/PlaceholderProfile.vue';
import { useI18n } from 'vue-i18n';
import NavigationBar from './components/NavigationBar.vue';

const host = ref(window.location.host);
const { t } = useI18n();

const modules = computed(() => [{
  module: "dashboard",
  link: "/",
  label: t("navigationBar.dashboard"),
},
{
  module: "orders",
  link: "/orders",
  label: t("navigationBar.orders"),
},
{
  module: "tables",
  link: "/tables",
  label: t("navigationBar.tables"),
},
{
  module: "menus",
  link: "/menus",
  label: t("navigationBar.menus"),
},
{
  module: "users",
  link: "/users",
  label: t("navigationBar.users"),
},
{
  module: "more",
  link: "/misc",
  label: t("navigationBar.more"),
}]);
</script>

<template>
  <h1 class="">You did it! You should be on {{ host }}</h1>
  <PlaceholderProfile />
  <NavigationBar :enabledModules="modules" />
</template
