<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  tl: any;
  callback: string;
}>();

const name = ref("");
const email = ref("");
const password = ref("");
const error = ref("");

async function register() {
  try {
    error.value = "";
    const res = await fetch(`${import.meta.env.PUBLIC_API}/v1/auth/register`, {
      method: "POST",
      mode: "cors",
      credentials: "include",
      body: JSON.stringify({ name: name.value, email: email.value, password: password.value }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    switch (res.status) {
      case 400:
        error.value = "errorBadRequest";
        break;
      case 409:
        error.value = "errorAccountConflict";
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
      @submit.prevent="register"
      class="p-6 rounded-xl shadow-md bg-white w-full flex flex-col gap-4"
    >
      <h1 class="text-2xl font-bold text-center">{{ tl.register.title }}</h1>

      <div class="w-full flex flex-col gap-2">
        <label class="flex flex-col gap-1 w-full">
          {{ tl.register.name }}
          <input
            type="text"
            class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
            :placeholder="tl.register.placeholderName"
            v-model="name"
          />
        </label>

        <label class="flex flex-col gap-1 w-full">
          {{ tl.register.emailAddress }}
          <input
            type="email"
            class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
            :placeholder="tl.register.placeholderEmail"
            v-model="email"
          />
        </label>

        <label class="flex flex-col gap-1 w-full">
          {{ tl.register.password }}
          <input
            type="password"
            class="w-full rounded-md placeholder:text-black/50 bg-violent-violet-50 outline-none px-2 py-1 duration-200 hover:ring-2 hover:ring-violent-violet-300 focus:ring-2 focus:ring-violent-violet-600"
            v-model="password"
          />
        </label>

        <div class="flex flex-row w-full gap-3.5 items-center">
          <div aria-hidden class="w-full bg-black/20 rounded-full h-px"></div>
          <span class="text-black/20 min-w-fit">{{ tl.register.or }}</span>
          <div aria-hidden class="w-full bg-black/20 rounded-full h-px"></div>
        </div>

        <button
          class="p-4 w-full rounded-xl shadow-md flex items-center justify-center gap-3 duration-200 hover:bg-violent-violet-50"
          disabled
        >
          <!-- Google's SVG -->
          <svg
            viewBox="-0.5 0 48 48"
            version="1.1"
            class="size-6"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            fill="#000000"
          >
            <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
            <g id="SVGRepo_iconCarrier">
              <title>Google-color</title>
              <desc>Created with Sketch.</desc>
              <defs></defs>
              <g id="Icons" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                <g id="Color-" transform="translate(-401.000000, -860.000000)">
                  <g id="Google" transform="translate(401.000000, 860.000000)">
                    <path
                      d="M9.82727273,24 C9.82727273,22.4757333 10.0804318,21.0144 10.5322727,19.6437333 L2.62345455,13.6042667 C1.08206818,16.7338667 0.213636364,20.2602667 0.213636364,24 C0.213636364,27.7365333 1.081,31.2608 2.62025,34.3882667 L10.5247955,28.3370667 C10.0772273,26.9728 9.82727273,25.5168 9.82727273,24"
                      id="Fill-1"
                      fill="#FBBC05"
                    ></path>
                    <path
                      d="M23.7136364,10.1333333 C27.025,10.1333333 30.0159091,11.3066667 32.3659091,13.2266667 L39.2022727,6.4 C35.0363636,2.77333333 29.6954545,0.533333333 23.7136364,0.533333333 C14.4268636,0.533333333 6.44540909,5.84426667 2.62345455,13.6042667 L10.5322727,19.6437333 C12.3545909,14.112 17.5491591,10.1333333 23.7136364,10.1333333"
                      id="Fill-2"
                      fill="#EB4335"
                    ></path>
                    <path
                      d="M23.7136364,37.8666667 C17.5491591,37.8666667 12.3545909,33.888 10.5322727,28.3562667 L2.62345455,34.3946667 C6.44540909,42.1557333 14.4268636,47.4666667 23.7136364,47.4666667 C29.4455,47.4666667 34.9177955,45.4314667 39.0249545,41.6181333 L31.5177727,35.8144 C29.3995682,37.1488 26.7323182,37.8666667 23.7136364,37.8666667"
                      id="Fill-3"
                      fill="#34A853"
                    ></path>
                    <path
                      d="M46.1454545,24 C46.1454545,22.6133333 45.9318182,21.12 45.6113636,19.7333333 L23.7136364,19.7333333 L23.7136364,28.8 L36.3181818,28.8 C35.6879545,31.8912 33.9724545,34.2677333 31.5177727,35.8144 L39.0249545,41.6181333 C43.3393409,37.6138667 46.1454545,31.6490667 46.1454545,24"
                      id="Fill-4"
                      fill="#4285F4"
                    ></path>
                  </g>
                </g>
              </g>
            </g>
          </svg>

          {{ tl.register.registerWithGoogle }}
        </button>
      </div>

      <span
        class="w-full bg-state-danger/5 p-4 rounded-xl text-state-danger font-semibold"
        v-if="error"
      >
        {{ tl.register[error] }}
      </span>

      <button
        class="bg-violent-violet-600 rounded-xl p-4 text-white font-semibold duration-200 hover:bg-violent-violet-700"
        type="submit"
      >
        {{ tl.register.cta }}
      </button>
    </form>

    <a :href="`./login`" class="text-center underline text-chill-500">{{
      tl.register.hasAnAccount
    }}</a>
  </div>
</template>
