<script lang="ts">
  export let start = 0;
  let count = start;
  function increment() {
    count += 1;
  }
</script>

{#snippet badge(label)}
  <span>{label}</span>
{/snippet}

<button on:click={increment}>
  {@render badge(count)}
</button>
