<script lang="ts">
  import { goto } from '$app/navigation';
  import type { Database } from '$types/Database';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  const database = getContext<Writable<Database>>('database');
  if (!$database) {
    goto('/');
  }
</script>

{#if $database}
  <div class="flex items-center flex-col">
    <h2 class="text-3xl font-bold pt-10">RustKeep {$database.version}</h2>
    <h3 class="text-2xl font-semibold pt-5">Entries</h3>
  </div>
{/if}
