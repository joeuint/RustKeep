<script lang="ts">
  import { goto } from '$app/navigation';
  import type { Database } from '$types/Database';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import type { DatabaseEntry } from '$types/Database';
  import { invoke } from '@tauri-apps/api/tauri';
  import Dialog from '$lib/Dialog.svelte';

  export let modal: HTMLDialogElement;

  const database = getContext<Writable<Database>>('database');
  if (!$database) {
    goto('/');
  }

  const test_entry: DatabaseEntry = {
    creation: 289329,
    name: 'Twitch',
    password: 'password',
    url: 'twitch.tv',
    username: 'popcornstriker',
    uuid: '2d46160e-8230-4012-89a6-081170abac01',
  };

  async function submitEntry(entry: DatabaseEntry) {
    modal.showModal();
    database.set(
      await invoke('add_entry', {
        database: $database,
        entry: entry,
      })
    );
  }
</script>

{#if $database}
  <Dialog bind:modal title="Entry Wizard">hi</Dialog>
  <div class="grid grid-cols-3 pt-10 pb-10 px-4 h-[calc(100vh-3rem)] gap-x-8">
    <div
      id="sidebar"
      class="bg-neutral-800 sm:w-5/6 rounded-lg p-2 overflow-y-scroll"
    >
      <h2 class="sm:text-2xl text-xl font-bold text-center mb-4">Entries</h2>
      <div id="entries">
        {#each $database.root.entries as entry}
          <hr class="border-neutral-600" />
          <div class="flex-col my-4">
            <div class="font-semibold sm:text-xl text-lg ellipsis">
              {entry.name}
            </div>
            <div
              class="font-medium text-gray-200/80 sm:text-base text-sm ellipsis"
            >
              {entry.username}
            </div>
          </div>
        {/each}
      </div>
    </div>
    <div id="overview">
      <h2 class="text-3xl font-bold text-center mb-6">
        RustKeep {$database.version}
      </h2>
      <div class="flex justify-center">
        <button
          class="bg-neutral-800 p-2 rounded-lg font-semibold text-lg"
          on:click={() => {
            submitEntry(test_entry);
          }}>Add Entry</button
        >
      </div>
    </div>
  </div>
{/if}
