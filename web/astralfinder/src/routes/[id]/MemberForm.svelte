<script lang="ts">
  import { updateParty, type Member } from "$lib/db.svelte";
  import { jobs } from "$lib/jobs";
  import MaterialSymbolsDelete from "~icons/material-symbols/delete";

  type Props = {
    member: Member;
    partyId: string;
  };

  let { member, partyId }: Props = $props();
  let jobString = $state(Object.keys(member.jobs).join(", "));
  let memberName = $state(member.name);
  let confidences = $derived(
    Object.keys(member.jobs).map((job) => ({
      name: job,
      confidence: member.jobs[job],
    }))
  );

  function nameChange() {
    updateParty(partyId, (party) => {
      party.members[member.id].name = memberName.trim();
      return party;
    });
  }

  function jobStringChange() {
    const jobList = jobString
      .split(",")
      .map((job) => job.trim().toLowerCase())
      .filter((job) => !!jobs[job]);

    updateParty(partyId, (party) => {
      party.members[member.id].jobs = jobList.reduce(
        (acc, job) => {
          acc[job] = member.jobs[job] ?? 1;
          return acc;
        },
        {} as Record<string, number>
      );

      return party;
    });
  }

  function confidenceChange(job: string, confidence: number) {
    updateParty(partyId, (party) => {
      party.members[member.id].jobs[job] = confidence;
      return party;
    });
  }

  function deleteMemberClick() {
    updateParty(partyId, (party) => {
      delete party.members[member.id];
      return party;
    });
  }
</script>

<div class="grid grid-cols-1 md:grid-cols-2 md:gap-y-8">
  <label for="{member.id}:name">
    <span class="font-medium"> Member Name </span>
    <!-- Hint -->
    <div class="text-sm text-slate-400">Member character name or nickname.</div>
  </label>

  <div class="mt-2 md:mt-0">
    <input
      type="text"
      id="{member.id}:name"
      name="{member.id}:name"
      class="w-full px-3 py-2 bg-slate-50 border border-slate-200 rounded-md focus:outline-none focus:ring focus:ring-slate-300 focus:ring-opacity-50"
      placeholder="Enter member name"
      bind:value={memberName}
      oninput={nameChange}
    />
  </div>

  <label for="{member.id}:jobs" class="mt-8 md:mt-0">
    <span class="font-medium"> Jobs </span>
    <!-- Hint -->
    <div class="text-sm text-slate-400">
      Member's main and secondary jobs. Use <code>,</code> for separate each job.
      Using job abbreviation. Case insensitive.
    </div>
  </label>

  <div class="mt-2 md:mt-0">
    <input
      type="text"
      id="{member.id}:jobs"
      name="{member.id}:jobs"
      class="w-full px-3 py-2 bg-slate-50 border border-slate-200 rounded-md focus:outline-none focus:ring focus:ring-slate-300 focus:ring-opacity-50"
      placeholder="Enter member jobs"
      bind:value={jobString}
      oninput={jobStringChange}
    />
    <!-- Example -->
    <div class="text-sm text-slate-400 mt-1">Example: pld, war, drk</div>
  </div>

  <div class="mt-8 md:mt-0">
    <span class="font-medium"> Jobs Confidence </span>
    <!-- Hint -->
    <div class="text-sm text-slate-400">
      Member's confidence level in their job.
    </div>
  </div>

  <div class="mt-2 md:mt-0">
    {#each confidences as con}
      <div class="grid grid-cols-[10em_1fr]">
        <label
          for="{member.id}:{con.name}:confidence"
          aria-label="member {jobs[con.name].name} confidence"
          class="font-medium flex items-center"
        >
          {jobs[con.name].name}
          <span
            role="status"
            class="px-2 py-1 bg-slate-200 text-xs rounded ml-2"
          >
            {con.confidence}
          </span>
        </label>
        <input
          type="range"
          id="{member.id}:{con.name}:confidence"
          name="{member.id}:{con.name}:confidence"
          class="w-full px-3 py-2 bg-slate-50 border border-slate-200 rounded-md focus:outline-none focus:ring focus:ring-slate-300 focus:ring-opacity-50"
          min="0.1"
          max="1"
          step="0.1"
          value={con.confidence}
          oninput={(e) =>
            confidenceChange(
              con.name,
              Number((e.target as HTMLInputElement).value)
            )}
        />
      </div>
    {/each}
  </div>

  <div class="col-span-2">
    <div class="flex flex-row justify-end">
      <button
        class="px-4 py-2 text-sm font-medium rounded cursor-pointer hover:bg-slate-200/70 flex flex-row items-center gap-2"
        onclick={deleteMemberClick}
      >
        <MaterialSymbolsDelete class="w-6 h-6 text-slate-600" />
        Remove Member
      </button>
    </div>
  </div>
</div>
