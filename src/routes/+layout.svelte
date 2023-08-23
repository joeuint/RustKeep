<script lang="ts">
  import '../app.css';
  import NavHeader from '$lib/NavHeader.svelte';
  import AlertComponent from '$lib/AlertComponent.svelte';
  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Alert } from '$types/Alert';
  import type { Database } from '$types/Database';
  import { getCurrent } from '@tauri-apps/api/window';
  import { confirm } from '@tauri-apps/api/dialog';

  const alerts = writable<Alert>();
  const database = writable<Database>();
  const unsaved = writable<boolean>(false);

  setContext('alerts', alerts);
  setContext('database', database);
  setContext('unsaved', unsaved);

  // Ask the user if they want to quit with unsaved changes
  getCurrent().onCloseRequested(async (event) => {
    if ($unsaved) {
      const quitAnyways = await confirm(
        'Your changes are not saved. Are you sure you would like to quit?',
        {
          title: 'Unsaved Database',
          type: 'warning',
        }
      );
      if (!quitAnyways) {
        event.preventDefault();
      }
    }
  });
</script>

<NavHeader />
{#if $alerts}
  <AlertComponent kind={$alerts.severity}>{$alerts.msg}</AlertComponent>
{/if}
<slot />
