<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/tauri';

  export let img;

  let imgUrl = convertFileSrc(img);

  $: filename = decodeURIComponent(imgUrl).split('/').pop();

  const dispatch = createEventDispatcher();

  const handleClick = () => {
    dispatch('click', { img });
  };
</script>

<div
  class="flex cursor-pointer flex-col items-center p-2 transition-colors hover:bg-gray-100"
  on:click={handleClick}
  on:keyup={handleClick}
>
  <img src={imgUrl} class="mb-1 h-36 w-36 object-cover" alt={imgUrl} />
  <span
    title={filename}
    class="w-36 cursor-default select-none break-all text-center"
    >{filename}</span
  >
</div>
