<script lang="ts">
  import Image from './Image.svelte';

  export let group;
</script>

<div class="flex w-full gap-2 border-b p-4 last:border-b-0">
  {#each group as img}
    <Image {img} />
  {/each}
</div>
