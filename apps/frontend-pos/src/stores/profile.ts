import scopedFetch, { routes } from "@/utils/fetcher";
import { defineStore } from "pinia";
import { ref } from "vue";
import type { ErrorResponse } from "../utils/error";

interface Profile {
  id: string;
  name: string;
  avatarUrl: string | null;
  createdAt: string;
  updatedAt: string;
}

interface UserProfile {
  user: {
    id: string;
    email: string;
    createdAt: string;
    updatedAt: string;
  };
  tenantId: string;
  branchId: string | null;
  customerProfile: Profile | null;
  staffProfile: Profile | null;
  systemProfile: Profile | null;
}

export const useProfileStore = defineStore("profile", () => {
  const profile = ref<UserProfile>();
  const error = ref<ErrorResponse>();
  const loading = ref(false);

  async function fetchProfile() {
    if (loading.value) {
      return;
    }

    error.value = undefined;
    loading.value = true;
    profile.value = undefined;

    await scopedFetch({
      path: routes.authMe,
      method: "GET",
    })
      .then(async (res) => {
        if (res.status == 200) {
          profile.value = await res.json();
        } else {
          const json = await res.json();
          error.value = json;
        }
        loading.value = false;
      })
      .catch((err) => {
        loading.value = false;
        console.log(err);
      });
  }

  return { profile, loading, error, fetchProfile };
});
