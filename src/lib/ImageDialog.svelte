<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import {
    Dialog,
    DialogOverlay,
    DialogTitle,
  } from '@rgossiaux/svelte-headlessui';
  import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

  export let isOpen = false;
  export let path = '';

  $: imgUrl = convertFileSrc(path);
  $: fileName = decodeURIComponent(imgUrl).split('/').pop();

  let fileType = '';
  $: invoke('get_file_type', { path }).then((res: string) => {
    fileType = res;
  });

  let fileSize = 0;
  $: invoke('get_file_size', { path }).then((res: string) => {
    fileSize = parseInt(res);
  });

  let imageSize = { width: 0, height: 0 };
  $: invoke('get_image_size', { path }).then((res: number[]) => {
    imageSize.width = res[0];
    imageSize.height = res[1];
  });

  const dispatch = createEventDispatcher();

  const bytesToHumanReadable = (numBytes: number): string => {
    const kib = 1024;
    const mib = kib * 1024;
    const gib = mib * 1024;

    if (numBytes >= gib) {
      return (numBytes / gib).toFixed(2) + ' GiB';
    } else if (numBytes >= mib) {
      return (numBytes / mib).toFixed(2) + ' MiB';
    } else if (numBytes >= kib) {
      return (numBytes / kib).toFixed(2) + ' KiB';
    } else {
      return numBytes + ' B';
    }
  };

  const parseMimeType = (mimeType: string) => {
    switch (mimeType) {
      case 'image/jpeg':
        return 'JPEG image';
      case 'image/png':
        return 'PNG image';
      case 'image/bmp':
        return 'BMP image';
      case 'image/tiff':
        return 'TIFF image';
      case 'image/gif':
        return 'GIF image';
      case 'image/svg+xml':
        return 'SVG image';
      case 'image/webp':
        return 'WebP image';
      default:
        return 'Unknown image type';
    }
  };

  const handleClose = () => {
    dispatch('close');
  };
</script>

<Dialog open={isOpen} on:close={handleClose} as="div" class="relative z-10">
  <DialogOverlay>
    <div class="fixed inset-0 bg-black bg-opacity-25" />
  </DialogOverlay>

  <div class="fixed inset-0 overflow-y-auto">
    <div class="flex min-h-full items-center justify-center p-4 text-center">
      <div
        class="flex w-full transform flex-col items-center overflow-hidden rounded bg-white p-6 text-left align-middle shadow-xl transition-all"
      >
        <DialogTitle class="text-xl font-bold">{fileName}</DialogTitle>

        <img
          src={imgUrl}
          class="mx-auto mb-1 mt-8 max-h-[60vh] object-cover"
          alt={imgUrl}
        />

        <p class="mb-8 text-center">
          {parseMimeType(fileType)}, {imageSize.width} x {imageSize.height}&nbsp;px,
          {bytesToHumanReadable(fileSize)}
        </p>

        <!-- <button on:click={handleClose}>Delete</button> -->
        <button
          class="self-end rounded border bg-slate-300 px-4 py-1 font-semibold transition-colors hover:bg-slate-400"
          on:click={handleClose}>OK</button
        >
      </div>
    </div>
  </div></Dialog
>
