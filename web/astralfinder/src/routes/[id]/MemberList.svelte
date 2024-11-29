<script lang="ts">
  import MingcuteRightFill from "~icons/mingcute/right-fill";
  //   import MingcutePlusFill from "~icons/mingcute/plus-fill";
  import IcBaselinePlus from "~icons/ic/baseline-plus";
</script>

{#snippet jobTab(name: string)}
  <span
    class="uppercase px-2 py-1 text-xs font-semibold bg-slate-200 rounded-sm"
  >
    {name}
  </span>
{/snippet}

{#snippet item(label: string)}
  <li
    class="py-4 px-6 border-b border-slate-200/50 hover:bg-slate-100/20 first:rounded-t-lg cursor-pointer flex flex-row justify-between items-center"
  >
    <div>
      <p class="text-xl font-medium">{label}</p>
      <p class="mt-3 flex flex-row gap-1">
        {@render jobTab("war")}
        {@render jobTab("pld")}
        {@render jobTab("whm")}
      </p>
    </div>

    <div>
      <MingcuteRightFill class="w-6 h-6 text-slate-400" />
    </div>
  </li>
{/snippet}

<div class="shadow rounded-lg">
  <ul>
    {@render item("Member A")}
    {@render item("Member B")}
    {@render item("Member C")}
  </ul>

  <button
    class="px-6 py-4 text-slate-600 font-medium hover:bg-slate-100/20 rounded-b-lg cursor-pointer w-full flex flex-row justify-center gap-2"
  >
    <IcBaselinePlus class="w-6 h-6 text-slate-400" />
    Add Member
  </button>
</div>
