<script lang="ts">
  import * as Tabs from "$lib/components/tabs";
  import { css, cx } from "$lib/styled-system/css";
  import { container, flex, hstack, vstack } from "$lib/styled-system/patterns";
  import BaseLineAssignment from "virtual:icons/ic/baseline-assignment";
  import BaselineGroup from "virtual:icons/ic/baseline-group";
  import BaselineSchedule from "virtual:icons/ic/baseline-schedule";
  import BaselinePublish from "virtual:icons/ic/baseline-publish";
  import DutyForm from "./DutyForm.svelte";
  import type { Component } from "svelte";
  import type { SVGAttributes } from "svelte/elements";
  import type { Snapshot } from "./$types.js";

  let { data } = $props();

  const tabs = [
    { id: "duty", label: "Duty", unlocked: true },
    { id: "party", label: "Party", unlocked: true },
    { id: "schedule", label: "Schedule", unlocked: true },
    { id: "publish", label: "Publish", unlocked: true },
  ];

  const title: Record<
    string,
    { title: string; icon: Component<SVGAttributes<SVGSVGElement>> }
  > = {
    duty: {
      title: "Select duty for the event",
      icon: BaseLineAssignment,
    },
    party: {
      title: "Set up party for the event",
      icon: BaselineGroup,
    },
    schedule: {
      title: "When and where the event will be held?",
      icon: BaselineSchedule,
    },
    publish: {
      title: "Publish the event",
      icon: BaselinePublish,
    },
  };

  let selectedTab = $state("duty");
  export const snapshot: Snapshot<string> = {
    capture: () => selectedTab,
    restore: (value) => {
      selectedTab = value;
    },
  };

  let Icon = $derived(title[selectedTab].icon);
</script>

<div class={container({ mt: "6", maxW: "5xl" })}>
  <Tabs.Root bind:value={selectedTab}>
    <div>
      <h1 class={hstack({ fontSize: "2xl", alignItems: "center", gap: "4" })}>
        <span
          class={flex({
            display: "inline-flex",
            h: "12",
            w: "12",
            justify: "center",
            alignItems: "center",
            border: "1px solid {colors.slate.300}",
            borderRadius: "full",
            color: "slate.500",
          })}
        >
          <Icon class={css({ h: "6", w: "6" })} />
        </span>
        <span>
          {title[selectedTab].title}
        </span>
      </h1>
    </div>

    <Tabs.TabList
      aria-label="Create Event Tabs"
      class={hstack({
        mt: "10",
        w: "full",
        justify: "space-between",
        gap: "6",
      })}
    >
      {#each tabs as tab (tab.id)}
        <Tabs.Tab
          id={tab.id}
          disabled={!tab.unlocked}
          class={cx(
            "group",
            vstack({
              w: "full",
              gap: "1.5",
              cursor: "pointer",
            })
          )}
        >
          <div
            class={css({
              w: "full",
              h: "1.5",
              bg: "slate.300",
              _groupSelected: {
                bg: "blue.500",
              },
            })}
          ></div>
          <div
            class={css({
              w: "full",
              textAlign: "left",
              fontWeight: "semibold",
            })}
          >
            {tab.label}
          </div>
        </Tabs.Tab>
      {/each}
    </Tabs.TabList>

    <div class={css({ mt: "12" })}>
      <Tabs.Panel id="duty">
        <DutyForm {data} />
      </Tabs.Panel>

      <Tabs.Panel id="party">Party</Tabs.Panel>
      <Tabs.Panel id="schedule">Schedule</Tabs.Panel>
      <Tabs.Panel id="publish">Publish</Tabs.Panel>
    </div>
  </Tabs.Root>
</div>
