<script lang="ts">
  import { type } from '@tauri-apps/api/os';
  import { open, confirm } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import type { Writable } from 'svelte/store';
  import type { Database } from '$types/Database';
  import { getContext } from 'svelte';
  import { goto } from '$app/navigation';

  const database = getContext<Writable<Database | null>>('database');
  const passwordStore = getContext<Writable<string | null>>('password');
  const unsaved = getContext<Writable<boolean>>('unsaved');

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

        await invoke('save_database', {
          path: path,
          database: JSON.stringify($database),
          password: $passwordStore,
        });

        unsaved.set(false);
      }
    }

    if (e.key == 'l') {
      if ($unsaved) {
        const quitAnyways = await confirm(
          'Your changes are not saved. Are you sure you would like to lock the database?',
          {
            title: 'Unsaved Database',
            type: 'warning',
          }
        );

        if (!quitAnyways) {
          return;
        }
      }

      database.set(null);
      passwordStore.set(null);
      unsaved.set(false);

      goto('/');
    }
  }
</script>

<slot />

<svelte:window on:keydown={keyHandler} />
