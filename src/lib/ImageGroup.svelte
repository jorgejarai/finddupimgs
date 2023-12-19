<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import Image from './Image.svelte';

  export let group;

  const dispatch = createEventDispatcher();
</script>

<div class="flex w-full gap-2 border-b p-4 last:border-b-0">
  {#each group as img}
    <Image
      {img}
      on:click={({ detail: { img } }) => dispatch('click', { img })}
    />
  {/each}
</div>
