<script lang="ts">
  import * as Tabs from "$lib/components/tabs";
  import { css, cx } from "$lib/styled-system/css";
  import { container, flex, hstack, vstack } from "$lib/styled-system/patterns";
  import Icon from "@iconify/svelte";
  import DutyForm from "./DutyForm.svelte";

  const tabs = [
    { id: "duty", label: "Duty", unlocked: true },
    { id: "party", label: "Party", unlocked: true },
    { id: "schedule", label: "Schedule", unlocked: true },
    { id: "publish", label: "Publish", unlocked: true },
  ];

  const title: Record<string, { title: string; icon: string }> = {
    duty: {
      title: "Select duty for the event",
      icon: "ic:baseline-assignment",
    },
    party: {
      title: "Set up party for the event",
      icon: "ic:baseline-group",
    },
    schedule: {
      title: "When and where the event will be held?",
      icon: "ic:baseline-schedule",
    },
    publish: {
      title: "Publish the event",
      icon: "ic:baseline-publish",
    },
  };

  let selectedTab = $state("duty");
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
          <Icon
            icon={title[selectedTab].icon}
            class={css({ w: "6", h: "6" })}
          />
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

    <div class={css({ mt: "6" })}>
      <Tabs.Panel id="duty">
        <DutyForm />
      </Tabs.Panel>

      <Tabs.Panel id="party">Party</Tabs.Panel>
      <Tabs.Panel id="schedule">Schedule</Tabs.Panel>
      <Tabs.Panel id="publish">Publish</Tabs.Panel>
    </div>
  </Tabs.Root>
</div>
