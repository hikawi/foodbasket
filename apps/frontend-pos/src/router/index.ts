import DashboardPage from "@/pages/DashboardPage.vue";
import MenusPage from "@/pages/MenusPage.vue";
import MorePage from "@/pages/MorePage.vue";
import OrdersPage from "@/pages/OrdersPage.vue";
import TablesPage from "@/pages/TablesPage.vue";
import UsersPage from "@/pages/UsersPage.vue";
import { pages } from "@/utils/pages";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: pages.dashboard,
      path: "/",
      component: DashboardPage,
    },
    {
      name: pages.orders,
      path: "/orders",
      component: OrdersPage,
    },
    {
      name: pages.tables,
      path: "/tables",
      component: TablesPage,
    },
    {
      name: pages.menus,
      path: "/menus",
      component: MenusPage,
    },
    {
      name: pages.users,
      path: "/users",
      component: UsersPage,
      redirect: { name: pages.usersMaster },
      children: [
        {
          name: pages.usersMaster,
          path: "/users/master",
          component: () => import("../pages/users/UsersSubpage.vue"),
        },
        {
          name: pages.groups,
          path: "/users/groups",
          component: () => import("../pages/users/GroupsSubpage.vue"),
        },
        {
          name: pages.roles,
          path: "/users/roles",
          component: () => import("../pages/users/RolesSubpage.vue"),
        },
        {
          name: pages.invitations,
          path: "/users/invitations",
          component: () => import("../pages/users/InvitationsSubpage.vue"),
        },
      ],
    },
    {
      name: pages.more,
      path: "/more",
      component: MorePage,
    },
  ],
});

export default router;
