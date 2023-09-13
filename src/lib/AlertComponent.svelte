<script lang="ts">
  import type { Alert } from '$types/Alert';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  export let kind: 'info' | 'warning' | 'error' | 'success';

  const alerts = getContext<Writable<Alert | null>>('alerts');

  const getTitle = (str: string) => {
    let charArray = [...str];
    charArray[0] = charArray[0].toUpperCase();
    return charArray.join('');
  };

  const kindDisplay: string = getTitle(kind);
</script>

<div
  class="w-1/2 mx-auto mt-1 bg-red-600/40 p-2 rounded-lg absolute left-[25vw]"
>
  <h2 class="text-xl font-semibold">{kindDisplay}:</h2>
  <p>
    <slot /><br /><button
      on:click={() => {
        alerts.set(null);
      }}
      class="underline">Click to dismiss</button
    >
  </p>
</div>
