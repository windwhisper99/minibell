<script lang="ts" module>
  function normalizeName(name: string) {
    const [first, last] = name.split(" ");

    if (first && last) {
      return `${first[0]}${last[0]}`;
    }

    return first[0];
  }
</script>

<script lang="ts">
  import { css, type Styles } from "$lib/styled-system/css";
  import { createAvatar } from "@melt-ui/svelte";

  let {
    src,
    name = "BM",
    css: overrideCss = {},
  }: {
    src: string;
    name?: string;
    css?: Styles;
  } = $props();

  let fbIcon = $derived(normalizeName(name));

  const {
    elements: { image, fallback },
  } = createAvatar({ src });
</script>

<div
  class={css(
    {
      display: "flex",
      w: "12",
      h: "12",
      rounded: "full",
      fontSize: "2xl",
      fontWeight: "medium",
      color: "slate.500",
      bg: "slate.200",
      alignItems: "center",
      justifyContent: "center",
    },
    overrideCss
  )}
>
  <img
    {...$image}
    use:image
    alt={`${name} avatar`}
    class={css({ h: "full", w: "full", rounded: "inherit" })}
  />
  <span {...$fallback} use:fallback>
    {fbIcon}
  </span>
</div>
