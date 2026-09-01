import {
  LucideEllipsis,
  LucideLayoutDashboard,
  LucideScrollText,
  LucideSquareMenu,
  LucideTable2,
  LucideUsers,
} from "lucide-vue-next";
import type { FunctionalComponent } from "vue";

export type Module = "dashboard" | "orders" | "tables" | "menus" | "users" | "more";

export interface ModuleConfig {
  module: Module;
  link: string;
  label: string;
  icon: FunctionalComponent;
}

export const DEFAULT_MODULES: Record<Module, Omit<ModuleConfig, "module">> = {
  dashboard: {
    link: "/",
    label: "navigationBar.dashboard",
    icon: LucideLayoutDashboard,
  },
  orders: {
    link: "/orders",
    label: "navigationBar.orders",
    icon: LucideScrollText,
  },
  tables: {
    link: "/tables",
    label: "navigationBar.tables",
    icon: LucideTable2,
  },
  menus: {
    link: "/menus",
    label: "navigationBar.menus",
    icon: LucideSquareMenu,
  },
  users: {
    link: "/users",
    label: "navigationBar.users",
    icon: LucideUsers,
  },
  more: {
    link: "/more",
    label: "navigationBar.more",
    icon: LucideEllipsis,
  },
};
