<script lang="ts">
  import { queryParties, type Party } from "$lib/db.svelte";
  import { dateTime } from "$lib/format";
  import UilCreateDashboard from "~icons/uil/create-dashboard";
  import MaterialSymbolsUpload from "~icons/material-symbols/upload";
  import { combination } from "$lib/jobs";

  const parties = queryParties();
</script>

{#snippet card(party: Party)}
  <a href="/{party.id}">
    <div
      class="border-slate-200 shadow-xs p-4 rounded-sm border hover:border-slate-300"
    >
      <p class="font-semibold text-lg">
        {party.name}
      </p>

      <p class="text-sm mt-1 text-slate-600">
        {combination[party.combination].name} - {dateTime(party.created_at)}
      </p>
    </div>
  </a>
{/snippet}

<div class="px-2 py-8 container mx-auto">
  <div class="flex flex-row gap-2">
    <a
      class="px-4 py-2 bg-slate-700 text-white text-sm font-medium rounded shadow-xs cursor-pointer hover:bg-slate-700/90 flex flex-row items-center gap-2"
      href="/create"
      data-sveltekit-preload-data="off"
    >
      <UilCreateDashboard class="w-6 h-6 text-white" />
      New Party
    </a>
    <button
      class="px-4 py-2 text-sm font-medium rounded cursor-pointer hover:bg-slate-200/70 flex flex-row items-center gap-2"
    >
      <MaterialSymbolsUpload class="w-6 h-6 text-slate-600" />
      Upload
    </button>
  </div>

  {#if parties.data}
    {#if parties.data.length === 0}
      <p class="text-slate-600 mt-6">Don't have any parties yet.</p>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-6">
        {#each parties.data as party}
          {@render card(party)}
        {/each}
      </div>
    {/if}
  {/if}
</div>
