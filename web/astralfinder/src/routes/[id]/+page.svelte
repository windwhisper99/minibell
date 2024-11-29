<script lang="ts">
  import * as Tabs from "$lib/components/tabs";
  import { queryPartyById } from "$lib/db.svelte";
  import MemberList from "./MemberList.svelte";
  import TdesignMemberFilled from "~icons/tdesign/member-filled";
  import UisSchedule from "~icons/uis/schedule";
  import PartyForm from "./PartyForm.svelte";
  import IconamoonEditFill from "~icons/iconamoon/edit-fill";
  import MaterialSymbolsDownload from "~icons/material-symbols/download";
  import Scheduling from "./Scheduling.svelte";
  import type { Snapshot } from "./$types.js";

  let { data } = $props();

  let selectedTab = $state("party");
  const tabs = [
    { id: "party", label: "Party", icon: IconamoonEditFill },
    { id: "members", label: "Members", icon: TdesignMemberFilled },
    { id: "schedule", label: "Schedule", icon: UisSchedule },
  ];

  const party = queryPartyById(data.partyId);

  export const snapshot: Snapshot<string> = {
    capture: () => selectedTab,
    restore: (value) => (selectedTab = value),
  };
</script>

<div role="toolbar" class="border-b border-slate-100 shadow-xs">
  <div
    class="container mx-auto flex flex-row justify-between items-center h-12 px-2"
  >
    <span class="font-medium text-lg">
      {party.data?.name}
    </span>

    <div>
      <button
        type="button"
        class="hover:bg-slate-100 font-medium cursor-pointer px-3 py-1 rounded flex flex-row gap-1"
      >
        <MaterialSymbolsDownload class="w-6 h-6 text-slate-400" />
        Export
      </button>
    </div>
  </div>
</div>

<div class="px-2 py-6 container mx-auto">
  <Tabs.Root bind:value={selectedTab}>
    <Tabs.TabList
      aria-label="Party Scheduling Tabs"
      class="grid grid-cols-2 md:flex gap-2"
    >
      {#each tabs as tab (tab.id)}
        <Tabs.Tab
          id={tab.id}
          class="px-6 py-3 hover:bg-slate-100 rounded-md aria-selected:bg-slate-700 aria-selected:text-slate-100 cursor-pointer grow sm:grow-0 group"
        >
          <div class="font-semibold flex flex-row items-center gap-4">
            <tab.icon></tab.icon>
            {tab.label}
          </div>
        </Tabs.Tab>
      {/each}
    </Tabs.TabList>

    <div class="mt-6">
      <Tabs.Panel id="party">
        {#if party.data}
          <PartyForm party={party.data} />
        {/if}
      </Tabs.Panel>
      <Tabs.Panel id="members">
        {#if party.data}
          <MemberList
            members={Object.values(party.data.members)}
            partyId={data.partyId}
          />
        {/if}
      </Tabs.Panel>
      <Tabs.Panel id="schedule">
        {#if party.data}
          <Scheduling party={party.data} />
        {/if}
      </Tabs.Panel>
    </div>
  </Tabs.Root>
</div>
