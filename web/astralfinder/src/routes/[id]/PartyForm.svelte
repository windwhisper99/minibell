<script lang="ts">
  import { goto } from "$app/navigation";
  import { deleteParty, updateParty, type Party } from "$lib/db.svelte";
  import { combination } from "$lib/jobs";
  import MaterialSymbolsDelete from "~icons/material-symbols/delete";

  type Props = {
    party: Party;
  };

  let { party }: Props = $props();

  function nameChange(event: Event) {
    const name = (event.target as HTMLInputElement).value;

    updateParty(party.id, (party) => {
      party.name = name;
      return party;
    });
  }

  function combinationChange(event: Event) {
    const id = (event.target as HTMLSelectElement).value;

    updateParty(party.id, (party) => {
      party.combination = id;
      return party;
    });
  }

  async function deletePartyClick() {
    await deleteParty(party.id);
    goto("/");
  }
</script>

<div class="bg-white shadow-md rounded-lg p-4 md:p-6">
  <div class="grid grid-cols-1 md:grid-cols-2 md:gap-y-8">
    <label for="partyname">
      <span class="font-medium">Party Name</span>
      <div class="text-sm text-slate-400">
        Can name the party after the content you're planning to run or any other
        theme.
      </div>
    </label>

    <div class="mt-2 md:mt-0">
      <input
        type="text"
        id="partyname"
        class="w-full px-3 py-2 bg-slate-50 border border-slate-200 rounded-md focus:outline-none focus:ring focus:ring-slate-300 focus:ring-opacity-50"
        bind:value={party.name}
        oninput={nameChange}
      />
    </div>

    <label for="partycombination" class="mt-6 md:mt-0">
      <span class="font-medium">Party Combination</span>
      <div class="text-sm text-slate-400">Using roles template for FF14.</div>
    </label>

    <div class="mt-2 md:mt-0">
      <select
        class="w-full px-3 py-2 bg-slate-50 border border-slate-200 rounded-md focus:outline-none focus:ring focus:ring-slate-300 focus:ring-opacity-50"
        value={party.combination}
        onchange={combinationChange}
      >
        {#each Object.entries(combination) as [id, com]}
          <option value={id}>{com.name}</option>
        {/each}
      </select>

      <div class="text-sm text-slate-400 mt-1">
        {combination[party.combination].description}
      </div>
    </div>

    <div class="md:col-span-2 flex flex-row justify-end mt-12">
      <button
        class="px-4 py-2 bg-red-700 text-white text-sm font-medium rounded shadow-xs cursor-pointer hover:bg-red-700/90 flex flex-row gap-1 items-center"
        onclick={deletePartyClick}
      >
        <MaterialSymbolsDelete class="w-6 h-6 text-white" />
        Delete Party
      </button>
    </div>
  </div>
</div>
