export const routes = {
  dashboard: "dashboard",
  tables: "tables",
  orders: "orders",
  menus: "menus",
  users: "users",
  more: "more",
} as const;

export type PageRoute = (typeof routes)[keyof typeof routes];
