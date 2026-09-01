import {
  LucideEllipsis,
  LucideLayoutDashboard,
  LucideScrollText,
  LucideSquareMenu,
  LucideTable2,
  LucideUsers,
} from "lucide-vue-next";
import type { FunctionalComponent } from "vue";
import { pages } from "./pages";

export type Module = "dashboard" | "orders" | "tables" | "menus" | "users" | "more";

export interface ModuleConfig {
  module: Module;
  link: string;
  label: string;
  icon: FunctionalComponent;
}

export const DEFAULT_MODULES: Record<Module, Omit<ModuleConfig, "module">> = {
  dashboard: {
    link: pages.dashboard,
    label: "navigationBar.dashboard",
    icon: LucideLayoutDashboard,
  },
  orders: {
    link: pages.orders,
    label: "navigationBar.orders",
    icon: LucideScrollText,
  },
  tables: {
    link: pages.tables,
    label: "navigationBar.tables",
    icon: LucideTable2,
  },
  menus: {
    link: pages.menus,
    label: "navigationBar.menus",
    icon: LucideSquareMenu,
  },
  users: {
    link: pages.users,
    label: "navigationBar.users",
    icon: LucideUsers,
  },
  more: {
    link: pages.more,
    label: "navigationBar.more",
    icon: LucideEllipsis,
  },
};
