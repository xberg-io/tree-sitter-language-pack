<script lang="ts">
  import Badge from "./Badge.svelte";
  import Icon from "./Icon.svelte";

  export let start = 0;
  let count = start;
  function increment() {
    count += 1;
  }
</script>

{#snippet badge(label)}
  <span class="badge">{label}</span>
{/snippet}

{#snippet row(item)}
  <li>
    <Icon name="dot" />
    {@render badge(item)}
  </li>
{/snippet}

<button on:click={increment}>
  {@render badge(count)}
</button>

<ul>
  {#each [1, 2, 3] as item}
    {@render row(item)}
  {/each}
</ul>

<Badge label="total" value={count} />
