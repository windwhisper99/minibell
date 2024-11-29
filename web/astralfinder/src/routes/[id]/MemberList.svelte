<script lang="ts">
  import MingcuteRightFill from "~icons/mingcute/right-fill";
  import IcBaselinePlus from "~icons/ic/baseline-plus";
  import * as Accordion from "$lib/components/accordion";
  import MemberForm from "./MemberForm.svelte";
  import { addMemberToParty, type Member } from "$lib/db.svelte";
  import JobBadge from "$lib/components/JobBadge.svelte";

  type Props = {
    partyId: string;
    members: Member[];
  };
  let { members, partyId }: Props = $props();
  let selected = $state("");

  async function addMember() {
    const id = await addMemberToParty(partyId, "New Member");
    selected = id;
  }
</script>

<div class="shadow rounded-lg">
  <Accordion.Root bind:value={selected}>
    {#each members as member}
      <Accordion.Header
        id={member.id}
        class="py-4 px-6 hover:bg-slate-100/20 first:rounded-t-lg cursor-pointer flex flex-row justify-between items-center w-full group"
      >
        <div>
          <p class="text-xl font-medium text-left">{member.name}</p>
          <p class="mt-3 flex flex-row gap-1">
            {#each Object.keys(member.jobs) as job}
              <JobBadge {job} />
            {/each}
          </p>
        </div>

        <MingcuteRightFill class="w-6 h-6 text-slate-400" />
      </Accordion.Header>

      <Accordion.Panel
        id={member.id}
        class="px-6 py-4 border-b border-slate-200/50"
      >
        <MemberForm {member} {partyId}></MemberForm>
      </Accordion.Panel>
    {/each}
  </Accordion.Root>

  <button
    class="px-6 py-4 text-slate-600 font-medium hover:bg-slate-100/20 rounded-b-lg cursor-pointer w-full flex flex-row justify-center gap-2"
    onclick={addMember}
  >
    <IcBaselinePlus class="w-6 h-6 text-slate-400" />
    Add Member
  </button>
</div>
