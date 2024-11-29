<script lang="ts">
  import type { Snippet } from "svelte";
  import TablerHeartFilled from "~icons/tabler/heart-filled";
  import "../app.css";

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <title>Astral Finder</title>

  <meta
    name="description"
    content="A tool for organize and schedule group contents for FF14."
  />
</svelte:head>

<div class="min-h-screen flex flex-col justify-between">
  <header class="h-14 shadow-sm flex flex-row sticky top-0 bg-white">
    <nav
      class="container px-2 flex flex-row items-center justify-between mx-auto"
    >
      <a href="/">
        <h1 class="text-xl md:text-2xl font-semibold text-slate-700">
          Astral Finder
        </h1>
      </a>

      <div class="grow flex flex-row justify-end">
        <a href="/faq" class="font-medium hover:underline">FAQ</a>
      </div>
    </nav>
  </header>

  <main class="mb-auto">{@render children?.()}</main>

  <footer>
    <div
      class="container mx-auto flex flex-col justify-center items-center h-14 px-2 my-2"
    >
      <p class="text-sm text-slate-500">
        Made with
        <TablerHeartFilled class="w-5 h-5 text-red-500 inline" />
        by
        <a
          href="https://liribell.com"
          target="_blank"
          class="hover:underline font-medium"
        >
          Liri Bell
        </a>
      </p>

      <p class="text-xs text-slate-400 text-center mt-1">
        All data is stored locally on your device using <a
          href="https://en.wikipedia.org/wiki/Indexed_Database_API"
          target="_blank"
          class="hover:underline"
        >
          IndexedDB
        </a>. No server is used.
      </p>
    </div>
  </footer>
</div>
