// @ts-check
import { defineConfig } from "astro/config";

import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
  integrations: [vue(), sitemap()],

  vite: {
    plugins: [tailwindcss()],
  },
  server: {
    port: 3001,
  },
});
