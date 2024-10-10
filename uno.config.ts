import {
  defineConfig,
  definePreset,
  presetUno,
  presetIcons,
  presetWebFonts,
} from "unocss";

const presetDemo = definePreset(() => {
  return {
    name: "demo",
    rules: [],
  };
});

export default defineConfig({
  content: {
    filesystem: ["./{js,src,templates}/**/*.{html,js,ts,rs}"],
  },
  presets: [
    presetUno(),
    presetIcons({
      cdn: "https://esm.sh/",
    }),
    presetWebFonts(),
    presetDemo(),
  ],
});
