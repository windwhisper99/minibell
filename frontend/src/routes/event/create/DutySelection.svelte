<script lang="ts">
  import { css } from "$lib/styled-system/css";
  import { grid, hstack, vstack } from "$lib/styled-system/patterns";
  import type { DutiesResult } from "./+page.server";
  import RightFill from "virtual:icons/mingcute/right-fill";

  let { duties: data }: { duties: DutiesResult } = $props();

  const categories = $derived(data.categories ?? []);
  const duties = $derived(data.duties ?? []);
</script>

<div class={css({ w: { base: "full", md: "2xl" }, mx: "auto" })}>
  <ul class={vstack({ gap: "2" })}>
    {#each categories as category}
      <li class={css({ w: "full" })}>
        <a
          href="/event/create?category={category.id}"
          class={hstack({
            justify: "space-between",
            color: "slate.800",
            cursor: "pointer",
            w: "full",
            px: "5",
            py: "4",
            borderRadius: "md",
            fontSize: "lg",
            fontWeight: "medium",
            bg: "slate.50",
            _hover: {
              bg: "slate.100",
            },
          })}
        >
          <span>
            {category.name}
          </span>

          <RightFill class={css({ h: "4", w: "4", color: "slate.500" })} />
        </a>
      </li>
    {/each}
  </ul>
</div>

<ul
  class={grid({
    gap: "4",
    columns: { base: 1, sm: 2, md: 3 },
  })}
>
  {#each duties as duty}
    <li class={css({ w: "full" })}>
      <a
        href="/event/create?duty={duty.id}"
        class={vstack({
          gap: "0",
          justify: "space-between",
          color: "slate.800",
          cursor: "pointer",
          w: "full",
          borderRadius: "md",
          fontSize: "md",
          fontWeight: "medium",
          bg: "slate.100",
          _hover: {
            bg: "slate.200",
          },
        })}
      >
        <img
          src={duty.image}
          alt={duty.name}
          class={css({ w: "full", borderRadius: "inherit" })}
        />
        <div class={css({ my: "2", mx: "3" })}>
          {duty.name}
        </div>
      </a>
    </li>
  {/each}
</ul>
