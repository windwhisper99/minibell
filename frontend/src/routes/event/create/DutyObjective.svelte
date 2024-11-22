<script lang="ts">
  import { css } from "$lib/styled-system/css";
  import { flex, vstack } from "$lib/styled-system/patterns";
  import type { Duty } from "./+page.server";

  let { duty }: { duty: Duty } = $props();
</script>

<div
  class={flex({
    alignItems: "start",
    gap: {
      base: "6",
      md: "4",
    },
    direction: {
      base: "column",
      md: "row",
    },
  })}
>
  <!-- Duty information -->
  <div
    class={vstack({
      w: { base: "full", md: "xs", lg: "md" },
      gap: "2",
    })}
  >
    <img
      src={duty.image}
      alt={duty.name}
      class={css({ w: "full", borderRadius: "inherit" })}
    />

    <!-- Body -->
    <div>
      <h2 class={css({ fontSize: "lg", fontWeight: "medium" })}>{duty.name}</h2>
    </div>
  </div>

  <div class={css({ flexGrow: 1, w: "full" })}>
    <div class={css({ px: "4", py: "3", borderWidth: "1", w: "full" })}>
      Edit duty objective here
    </div>
  </div>
</div>
