import scopedFetch, { routes } from "@/utils/fetcher";
import { defineStore } from "pinia";
import { ref } from "vue";

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
  const loading = ref(false);

  function fetchProfile() {
    if (loading.value) {
      return;
    }

    loading.value = true;
    profile.value = undefined;

    scopedFetch({
      path: routes.authMe,
      method: "GET",
    })
      .then(async (res) => {
        if (res.status == 200) {
          profile.value = await res.json();
        }
        loading.value = false;
      })
      .catch((err) => {
        loading.value = false;
      });
  }

  return { profile, loading, fetchProfile };
});
