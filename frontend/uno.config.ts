import { defineConfig, presetUno, presetIcons, presetWebFonts } from "unocss";

export default defineConfig({
  content: {
    filesystem: ["./{src}/**/*.{html,js,jsx,ts,tsx}"],
  },
  presets: [
    presetUno(),
    presetIcons({
      cdn: "https://esm.sh/",
      collections: {
        ff14: {},
      },
    }),
    presetWebFonts(),
  ],
});
