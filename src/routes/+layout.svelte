<script lang="ts">
  import '../app.css';
  import NavHeader from '$lib/NavHeader.svelte';
  import AlertComponent from '$lib/AlertComponent.svelte';
  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Alert } from '$types/Alert';
  import type { Database } from '$types/Database';

  const alerts = writable<Alert>();
  const database = writable<Database>();

  setContext('alerts', alerts);
  setContext('database', database);
</script>

<NavHeader />
{#if $alerts}
  <AlertComponent kind={$alerts.severity}>{$alerts.msg}</AlertComponent>
{/if}
<slot />
