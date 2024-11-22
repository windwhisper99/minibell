<script lang="ts">
  import { css } from "$lib/styled-system/css";
  import * as Breadcrumb from "$lib/components/breadcrumb";
  import HomeFill from "virtual:icons/iconamoon/home-fill";
  import type { PageData } from "./$types";
  import { hstack } from "$lib/styled-system/patterns";
  import DutySelection from "./DutySelection.svelte";
  import DutyObjective from "./DutyObjective.svelte";

  let { data }: { data: PageData } = $props();

  const breadCrumbItemCss = hstack({
    color: "slate.800",
    cursor: "pointer",
    _hover: {
      textDecoration: "underline",
    },
    _currentPage: {
      fontWeight: "semibold",
    },
  });

  let breadcrumbs = $derived.by<
    { id: string; name: string; current: boolean }[]
  >(() => {
    if (data.duties) {
      const breadcrumbs = data.duties.breadcrumbs;

      return breadcrumbs.map((category, index) => ({
        id: category.id,
        name: category.name,
        current: index === breadcrumbs.length - 1,
      }));
    } else if (data.duty) {
      const duty = data.duty;

      return [
        ...data.duty.breadcrumbs.map((category) => ({
          id: category.id,
          name: category.name,
          current: false,
        })),
        {
          id: duty.duty.id,
          name: duty.duty.shortName ?? duty.duty.name,
          current: true,
        },
      ];
    } else {
      return [];
    }
  });
</script>

<Breadcrumb.Root aria-label="Duty breadscrumb" class={css({ my: "6" })}>
  <Breadcrumb.Item
    href="/event/create"
    class={breadCrumbItemCss}
    home
    current={breadcrumbs.length === 0}
  >
    <HomeFill class={css({ h: "4", w: "4", color: "slate.500" })} />
    Duty
  </Breadcrumb.Item>

  {#each breadcrumbs as bc (bc.id)}
    <Breadcrumb.Item
      href={`/event/create?category=${bc.id}`}
      class={breadCrumbItemCss}
      current={breadcrumbs.length === 1
        ? true
        : breadcrumbs[breadcrumbs.length - 1].id === bc.id}
    >
      {bc.name}
    </Breadcrumb.Item>
  {/each}
</Breadcrumb.Root>

<div class={css({ mt: "8" })}>
  {#if data.duties}
    <DutySelection duties={data.duties} />
  {:else if data.duty}
    <DutyObjective duty={data.duty.duty} />
  {/if}
</div>
