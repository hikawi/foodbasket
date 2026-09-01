import { BookUser, Mail, UserLock, Users } from "lucide-vue-next";
import type { FunctionalComponent } from "vue";
import { pages, type PageRoute } from "./pages";

export const drills = ["users", "groups", "roles", "invitations"] as const;

export type DrillAxis = (typeof drills)[number];

/**
 * Drill is the leftmost bar on the desktop layout for master-detail.
 * These components make up that leftmost bar.
 */
export interface DrillComponent {
  id: DrillAxis;
  link: PageRoute;
  icon: FunctionalComponent;
  label: string;
}

export const USERS_DRILL_BAR: DrillComponent[] = [
  {
    id: "users",
    link: pages.usersMaster,
    icon: Users,
    label: "users.drill.users",
  },
  {
    id: "groups",
    link: pages.groups,
    icon: BookUser,
    label: "users.drill.groups",
  },
  {
    id: "roles",
    link: pages.roles,
    icon: UserLock,
    label: "users.drill.roles",
  },
  {
    id: "invitations",
    link: pages.invitations,
    icon: Mail,
    label: "users.drill.invitations",
  },
] as const;
