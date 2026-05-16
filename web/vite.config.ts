import { defineConfig } from "vite";

export default defineConfig({
  root: ".",
  publicDir: "public",
  server: { port: 5173 },
  build: { target: "es2022", outDir: "dist", sourcemap: true },
});
