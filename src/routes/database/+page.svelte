<script lang="ts">
  import { goto } from '$app/navigation';
  import type { Database } from '$types/Database';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import type { DatabaseEntry } from '$types/Database';
  import { invoke } from '@tauri-apps/api/tauri';
  import { type } from '@tauri-apps/api/os';
  import Dialog from '$lib/Dialog.svelte';
  import { open } from '@tauri-apps/api/dialog';

  let modal: HTMLDialogElement;

  const password = getContext<Writable<string | null>>('password');

  const database = getContext<Writable<Database | null>>('database');
  if (!$database) {
    goto('/');
  }

  async function keyHandler(e: KeyboardEvent) {
    // User is on Mac
    const isMacOS = (await type()) == 'Darwin';

    if ((isMacOS && e.metaKey) || (!isMacOS && e.ctrlKey)) {
      if (e.key == 's') {
        const path = await open({
          filters: [
            {
              name: 'Rust Keep Database',
              extensions: ['rkdb'],
            },
          ],
        });

        console.log(path, $password);
        await invoke('save_database', {
          path: path,
          database: JSON.stringify($database),
          password: $password,
        });
      }

      if (e.key == 'l') {
        database.set(null);
        password.set(null);

        goto('/');
      }
    }
  }

  const current_entry: DatabaseEntry = {
    creation: 0,
    name: '',
    password: '',
    url: '',
    username: '',
    uuid: '',
  };

  async function submitEntry(entry: DatabaseEntry) {
    database.set(
      await invoke('add_entry', {
        database: $database,
        entry: entry,
      })
    );
  }
</script>

{#if $database}
  <Dialog bind:modal title="Entry Wizard">
    <form class="flex flex-col gap-y-2">
      <p class="text-lg">Name:</p>
      <input
        bind:value={current_entry.name}
        type="text"
        name="name"
        id="entryName"
      />
      <p class="text-lg">Username:</p>
      <input
        bind:value={current_entry.username}
        type="text"
        name="username"
        id="entryUsername"
      />
      <p class="text-lg">URL:</p>
      <input
        bind:value={current_entry.url}
        type="text"
        name="url"
        id="entryUrl"
      />
      <p class="text-lg">Password:</p>
      <input
        bind:value={current_entry.password}
        type="password"
        name="password"
        id="entryPassword"
        class="mb-4"
      />
      <input
        type="submit"
        value="Add"
        class="bg-neutral-700 p-2 text-lg rounded-lg font-semibold cursor-pointer"
        on:click={() => {
          submitEntry(current_entry);
        }}
      />
      <button
        class="bg-red-700 p-2 text-lg rounded-lg font-semibold"
        on:click={() => {
          modal.close();
        }}>Close</button
      >
    </form>
  </Dialog>
  <div class="grid grid-cols-3 pt-10 pb-10 px-4 h-[100vh] pb-20 gap-x-8">
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
            modal.showModal();
          }}>Add Entry</button
        >
      </div>
    </div>
  </div>
{/if}

<svelte:window on:keydown={keyHandler} />
