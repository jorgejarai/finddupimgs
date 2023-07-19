<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';

  import ImageGroup from './lib/ImageGroup.svelte';

  let selectedPath = '';
  let output = [];

  let selectedImages = [];

  const handleBrowse = async () => {
    selectedPath = (await open({
      directory: true,
      multiple: false,
    })) as string;
  };

  $: selectedPath,
    invoke('find_duplicates', { path: selectedPath }).then((res: string) => {
      output = JSON.parse(res);
    });
</script>

<div class="flex h-screen flex-col overflow-hidden">
  <header class="flex w-full items-center gap-4 bg-slate-200 p-4">
    <span class="text-xl">finddupimgs</span>
    <div class="flex flex-grow items-center gap-2">
      <span>Searching in:</span>
      <input
        type="text"
        class="flex-grow rounded-md border border-gray-400 p-1"
        readonly
        value={selectedPath}
      />
    </div>
    <button
      class="rounded border bg-slate-300 px-4 py-1 text-sm font-bold transition-colors hover:bg-slate-400"
      on:click={handleBrowse}
    >
      Browse...
    </button>
  </header>

  <main class="flex flex-grow flex-col overflow-y-scroll">
    {#each output as group}
      <ImageGroup {group} />
    {/each}
  </main>

  {#if selectedImages.length > 0}
    <div class="flex items-center justify-between bg-red-300 p-3">
      {#if selectedImages.length == 1}
        <span class="text-red-900">1 image selected</span>
      {:else if selectedImages.length > 1}
        <span class="text-red-900">{selectedImages.length} images selected</span
        >
      {/if}

      <button
        class="rounded bg-red-400 px-4 py-1 text-sm font-bold transition-colors hover:bg-red-500"
      >
        Delete
      </button>
    </div>
  {/if}

  <footer class="w-screen bg-slate-100 px-2 py-1 text-sm">
    {#if selectedPath != ''}
      {#if output.length == 1}
        <span> 1 group of duplicates found </span>
      {:else if output.length > 1}
        <span>
          {output.length} groups of duplicates found
        </span>
      {:else}
        <span>No duplicates found</span>
      {/if}
    {:else}
      <span>Choose a directory to search</span>
    {/if}
  </footer>
</div>
