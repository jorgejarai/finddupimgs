<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/tauri';

  export let img;

  let imgUrl = convertFileSrc(img);

  $: filename = decodeURIComponent(imgUrl).split('/').pop();
</script>

<div
  class="flex cursor-pointer flex-col items-center p-2 transition-colors hover:bg-gray-100"
>
  <img src={imgUrl} class="mb-1 h-36 w-36 object-cover" alt={imgUrl} />
  <span title={filename} class="w-36 break-all text-center">{filename}</span>
</div>
