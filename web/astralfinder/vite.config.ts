import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import icons from "unplugin-icons/vite";
// import wasm from "vite-plugin-wasm";

export default defineConfig({
  server: {
    fs: {
      allow: ["../../crates/astralfinder-wasm/pkg"],
    },
  },
  plugins: [
    sveltekit(),
    tailwindcss(),
    icons({ compiler: "svelte", autoInstall: true }),
    // wasm(),
  ],
});
