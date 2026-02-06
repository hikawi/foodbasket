// @ts-check
import { defineConfig } from "astro/config";

import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";

import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
  integrations: [vue(), sitemap()],

  vite: {
    plugins: [tailwindcss()],
  },

  server: {
    port: 3001,
  },

  adapter: node({
    mode: "standalone",
  }),
});