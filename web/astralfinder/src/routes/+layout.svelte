<script lang="ts">
  import type { Snippet } from "svelte";
  import "../app.css";

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <title>Astral Finder</title>
</svelte:head>

<header class="h-14 shadow-sm flex flex-row">
  <div class="container mx-auto flex flex-row px-2">
    <nav class="flex flex-row items-center justify-center">
      <a href="/">
        <h1 class="text-xl md:text-2xl font-semibold text-slate-700">
          Astral Finder
        </h1>
      </a>
    </nav>
  </div>
</header>

<main class="container mx-auto my-8 px-2">{@render children?.()}</main>
