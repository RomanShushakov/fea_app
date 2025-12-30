import { defineConfig } from "vite";

export default defineConfig({
  server: {
    port: 5001,
  },
  build: {
    outDir: "../public",
    assetsDir: "./assets",
    rollupOptions: {
      input: {
        fea_service: "./fea_service.html",
      },
    },
  },
  worker: {
    format: "es",
  },
  plugins: [],
});
