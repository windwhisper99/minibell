<script lang="ts">
  import JobBadge from "$lib/components/JobBadge.svelte";
  import { schedule, type Party } from "$lib/db.svelte";
  import { sortCombination } from "$lib/jobs";
  import BxsRocket from "~icons/bxs/rocket";

  type Props = {
    party: Party;
  };

  let { party }: Props = $props();

  let combinations = $derived.by(() => {
    if (!party.combinations) return;
    return party.combinations.map((com) => sortCombination(com));
  });

  async function startScheduling() {
    schedule(party.id);
  }
</script>

<div>
  <button
    class="px-4 py-2 bg-slate-700 text-white text-sm font-medium rounded shadow-xs cursor-pointer hover:bg-slate-700/90 flex flex-row items-center gap-2"
    onclick={startScheduling}
  >
    <BxsRocket class="w-6 h-6 text-slate-200" />
    Start Scheduling
  </button>
</div>

{#if combinations}
  <div class="mt-4">
    There are {combinations.length} possible combinations.
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-6">
    {#each combinations as com}
      <div class="shadow rounded-lg">
        <div class="p-4">
          <ul class="flex flex-col gap-y-2">
            {#each com.assigned as assign}
              {@const member = party.members[assign.id]}
              {#if member}
                <li class="flex flex-row items-center gap-2">
                  <span class="font-medium">{member.name}</span>
                  <JobBadge job={assign.job} />
                </li>
              {/if}
            {/each}
          </ul>
        </div>

        <div class="border-t border-slate-100 px-4 py-2">
          <span class="font-medium"> Score: </span>
          <span class="text-lg font-bold">
            {Math.round(com.score * 100) / 100}
          </span>
        </div>
      </div>
    {/each}
  </div>
{/if}
