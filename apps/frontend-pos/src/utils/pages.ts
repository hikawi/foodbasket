export const pages = {
  dashboard: "dashboard",
  tables: "tables",
  orders: "orders",
  menus: "menus",
  users: "users",
  usersMaster: "users-users",
  groups: "users-groups",
  roles: "users-roles",
  invitations: "users-invitations",
  more: "more",
} as const;

export type PageRoute = (typeof pages)[keyof typeof pages];
