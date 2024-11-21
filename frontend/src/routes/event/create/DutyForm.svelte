<script lang="ts">
  import { css } from "$lib/styled-system/css";
  import { getTabState } from "$lib/components/tabs";

  const tab = getTabState();
</script>

<h1 class={css({ fontSize: "2xl" })}>Duty selection</h1>

<div class={css({ mt: "12" })}>
  <button onclick={tab.selectNextTab}>Next</button>
</div>
