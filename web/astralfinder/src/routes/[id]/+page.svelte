<script lang="ts">
  import * as Tabs from "$lib/components/tabs";
  import { queryPartyById } from "$lib/db.svelte";
  import MemberList from "./MemberList.svelte";
  import TdesignMemberFilled from "~icons/tdesign/member-filled";
  import UisSchedule from "~icons/uis/schedule";

  let { data } = $props();

  let selectedTab = $state("members");
  const tabs = [
    { id: "members", label: "Members", icon: TdesignMemberFilled },
    { id: "schedule", label: "Schedule", icon: UisSchedule },
  ];

  const party = queryPartyById(data.partyId);
</script>

<Tabs.Root bind:value={selectedTab}>
  <Tabs.TabList aria-label="Party Scheduling Tabs" class="flex flex-row gap-2">
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
    <Tabs.Panel id="members">
      {#if party.data}
        <MemberList
          members={Object.values(party.data.members)}
          partyId={data.partyId}
        />
      {/if}
    </Tabs.Panel>
    <Tabs.Panel id="schedule">Schedule</Tabs.Panel>
  </div>
</Tabs.Root>
