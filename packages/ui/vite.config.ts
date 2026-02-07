import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import dts from "vite-plugin-dts";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [
    vue(),
    dts({
      insertTypesEntry: true,
      rollupTypes: true,
    }),
    tailwindcss(),
  ],
  build: {
    lib: {
      entry: "src/index.ts",
      name: "",
      fileName: (format) => `index.${format === "es" ? "js" : "cjs"}`,
      formats: ["es"],
    },
    rollupOptions: {
      external: ["vue"],
      output: {
        globals: { vue: "Vue" },
      },
    },
    target: "esnext",
    sourcemap: true,
  },
});
