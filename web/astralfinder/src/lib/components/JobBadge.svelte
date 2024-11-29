<script lang="ts">
  import { jobs } from "$lib/jobs";

  type Props = {
    job: string;
  };

  const roleColors: Record<string, string> = {
    tank: "bg-blue-200",
    pure_healer: "bg-green-200",
    shield_healer: "bg-green-200",
    melee: "bg-red-200",
    ranged: "bg-red-200",
    caster: "bg-red-200",
  };

  let { job }: Props = $props();
</script>

<span
  class="uppercase px-2 py-1 text-xs font-semibold rounded-sm {roleColors[
    jobs[job].role
  ]}"
  role="status"
>
  {job}
</span>
