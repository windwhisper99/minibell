<script lang="ts">
  import { css } from "$lib/styled-system/css";
  import { container, hstack } from "$lib/styled-system/patterns";
  import Icon from "@iconify/svelte";
  import "../app.css";
  import Avatar from "$lib/components/Avatar.svelte";

  export let data;
</script>

<svelte:head>
  <title>Astral Bells</title>
</svelte:head>

<!-- Header -->
<header
  class={hstack({
    h: "16",
    borderBottom: "1px solid {colors.slate.200}",
    bg: "colors.white",
  })}
>
  <div
    class={container(
      hstack.raw({
        justify: "space-between",
        width: "full",
      })
    )}
  >
    <nav>
      <a class={css({ fontSize: "2xl", fontWeight: "semibold" })} href="/">
        Astral Bells
      </a>
    </nav>

    <div>
      {#if data.auth.member}
        {@const member = data.auth.member}
        <a class={hstack({})} href="/">
          <span class={css({ fontWeight: "medium" })}>{member.name}</span>
          <Avatar src={member.avatar} name={member.name} />
        </a>
      {:else}
        <a
          class={hstack({
            fontSize: "md",
            fontWeight: "medium",
            gap: "2",
            alignItems: "center",
            bg: "transparent",
            color: "slate.800",
            _hover: {
              color: "slate.800/80",
            },
          })}
          href={data.auth.authUrl}
        >
          <span> Sign Up </span>
          <Icon icon="logos:discord-icon" class={css({ w: "6", h: "6" })} />
        </a>
      {/if}
    </div>
  </div>
</header>

<div>
  <slot />
</div>
