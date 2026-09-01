import DashboardPage from "@/pages/DashboardPage.vue";
import MenusPage from "@/pages/MenusPage.vue";
import MorePage from "@/pages/MorePage.vue";
import OrdersPage from "@/pages/OrdersPage.vue";
import TablesPage from "@/pages/TablesPage.vue";
import UsersPage from "@/pages/UsersPage.vue";
import { routes } from "@/utils/routes";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: routes.dashboard,
      path: "/",
      component: DashboardPage,
    },
    {
      name: routes.orders,
      path: "/orders",
      component: OrdersPage,
    },
    {
      name: routes.tables,
      path: "/tables",
      component: TablesPage,
    },
    {
      name: routes.menus,
      path: "/menus",
      component: MenusPage,
    },
    {
      name: routes.users,
      path: "/users",
      component: UsersPage,
    },
    {
      name: routes.more,
      path: "/more",
      component: MorePage,
    },
  ],
});

export default router;
