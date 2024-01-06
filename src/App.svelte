<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';

  import ImageDialog from './lib/ImageDialog.svelte';
  import ImageGroup from './lib/ImageGroup.svelte';

  let selectedPath = '';
  let output = [];

  let selectedImages = [];

  let openDialog = false;
  let dialogPath = '';

  let loading = true;
  let takingTooMuch = false;

  const handleBrowse = async () => {
    const newPath = (await open({
      directory: true,
      multiple: false,
    })) as string | null;

    if (newPath) {
      selectedPath = newPath;
    }
  };

  $: if (selectedPath) {
    loading = true;
    takingTooMuch = false;
    invoke('find_duplicates', { path: selectedPath }).then((res: string) => {
      const tooMuchTimeout = setTimeout(() => {
        takingTooMuch = true;
      }, 30_000);

      output = JSON.parse(res);
      loading = false;

      clearTimeout(tooMuchTimeout);
    });
  }
</script>

<div class="flex h-screen flex-col overflow-hidden">
  <header class="flex w-full items-center gap-4 bg-slate-200 p-4">
    <span class="cursor-default select-none text-xl">finddupimgs</span>
    <div class="flex flex-grow items-center gap-2">
      <span class="cursor-default select-none">Searching in:</span>
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

  {#if loading}
    <main class="flex flex-grow flex-col items-center justify-center gap-2">
      <span class="cursor-default select-none text-gray-500">Loading...</span>
      {#if takingTooMuch}
        <span class="cursor-default select-none text-gray-500">
          This is taking a while...
        </span>
      {/if}
    </main>
  {:else}
    <main class="flex flex-grow flex-col overflow-y-scroll">
      {#each output as group}
        <ImageGroup
          {group}
          on:click={({ detail: { img } }) => {
            openDialog = true;
            dialogPath = img;
          }}
        />
      {/each}
    </main>
  {/if}

  {#if selectedImages.length > 0}
    <div class="flex items-center justify-between bg-red-300 p-3">
      {#if selectedImages.length == 1}
        <span class="cursor-default select-none text-red-900"
          >1 image selected</span
        >
      {:else if selectedImages.length > 1}
        <span class="cursor-default select-none text-red-900"
          >{selectedImages.length} images selected</span
        >
      {/if}

      <button
        class="cursor-default select-none rounded bg-red-400 px-4 py-1 text-sm font-bold transition-colors hover:bg-red-500"
      >
        Delete
      </button>
    </div>
  {/if}

  <footer class="w-screen bg-slate-100 px-2 py-1 text-sm">
    {#if selectedPath != ''}
      {#if output.length == 1}
        <span class="cursor-default select-none">
          1 group of duplicates found
        </span>
      {:else if output.length > 1}
        <span class="cursor-default select-none">
          {output.length} groups of duplicates found
        </span>
      {:else}
        <span class="cursor-default select-none">No duplicates found</span>
      {/if}
    {:else}
      <span class="cursor-default select-none"
        >Choose a directory to search</span
      >
    {/if}
  </footer>

  <ImageDialog
    isOpen={openDialog}
    path={dialogPath}
    on:close={() => (openDialog = false)}
  />
</div>
