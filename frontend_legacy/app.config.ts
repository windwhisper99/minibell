import { defineConfig } from "@solidjs/start/config";
import unocssPlugin from "unocss/vite";

export default defineConfig({
  ssr: true,
  vite: {
    plugins: [unocssPlugin()],
  },
});
