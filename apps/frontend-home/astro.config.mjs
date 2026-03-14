// @ts-check
import { defineConfig } from "astro/config";

import vue from "@astrojs/vue";
import tailwindcss from "@tailwindcss/vite";
import node from "@astrojs/node";
import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
  integrations: [vue(), sitemap()],
  output: "server",
  vite: {
    plugins: [tailwindcss()],
  },
  server: {
    port: 3000,
  },
  i18n: {
    defaultLocale: "en",
    locales: ["en", "ja"],
    routing: {
      prefixDefaultLocale: true,
      fallbackType: "redirect",
    },
  },
  trailingSlash: "never",
  adapter: node({
    mode: "standalone",
  }),
  site: "https://foodbasket.app",
});
