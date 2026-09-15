<script setup lang="ts">
import { computed, ref } from "vue";
import { getTranslations, type Language } from "../../utils/i18n";
import Google from "../icons/Google.vue";

const props = defineProps<{
  lang: Language;
  callback: string;
}>();

const tl = computed(() => getTranslations(props.lang));

const email = ref("");
const password = ref("");
const error = ref("");

async function login() {
  try {
    error.value = "";
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/auth/login`, {
      method: "POST",
      mode: "cors",
      credentials: "include",
      body: JSON.stringify({ email: email.value, password: password.value }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    switch (res.status) {
      case 400:
        error.value = "errorBadRequest";
        break;
      case 403:
        error.value = "errorWrongPassword";
        break;
      case 404:
        error.value = "errorAccountDoesntExist";
        break;
      case 500:
        error.value = "errorServer";
        break;
    }

    if (res.ok) {
      window.location.href = props.callback;
    }
  } catch {
    error.value = "errorInternet";
  }
}
</script>

<template>
  <div class="max-w-xl flex flex-col gap-4 items-center justify-center w-full">
    <form
      @submit.prevent="login"
      class="p-6 rounded-xl shadow-md bg-white w-full flex flex-col gap-4"
    >
      <h1 class="text-2xl font-bold text-center">{{ tl.login.title }}</h1>

      <div class="w-full flex flex-col gap-2">
        <label class="flex flex-col gap-1 w-full">
          {{ tl.login.emailAddress }}
          <input
            type="email"
            class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
            maxlength="255"
            :placeholder="tl.login.placeholderEmail"
            v-model="email"
          />
        </label>

        <label class="flex flex-col gap-1 w-full">
          {{ tl.login.password }}
          <input
            type="password"
            maxlength="255"
            class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
            v-model="password"
          />
        </label>

        <div class="flex flex-row w-full gap-3.5 items-center">
          <div aria-hidden class="w-full bg-black/20 rounded-full h-px"></div>
          <span class="text-black/20 min-w-fit">{{ tl.login.or }}</span>
          <div aria-hidden class="w-full bg-black/20 rounded-full h-px"></div>
        </div>

        <button
          class="p-4 w-full rounded-xl shadow-md flex items-center cursor-pointer justify-center gap-3 duration-200 hover:bg-violent-violet-50"
          disabled
        >
          <Google class="size-6" />

          {{ tl.login.loginWithGoogle }}
        </button>
      </div>

      <span
        class="w-full bg-state-danger/5 p-4 rounded-xl text-state-danger font-semibold"
        v-if="error"
      >
        {{ tl.login[error as keyof typeof tl.login] }}
      </span>

      <button
        class="bg-violent-violet-600 cursor-pointer rounded-xl p-4 text-white font-semibold duration-200 hover:bg-violent-violet-700"
        type="submit"
      >
        {{ tl.login.cta }}
      </button>
    </form>

    <a :href="`./register`" class="text-center underline text-chill-500">{{
      tl.login.noAccount
    }}</a>
  </div>
</template>
