<script lang="ts">
  export let kind: 'info' | 'warning' | 'error' | 'success';

  const getTitle = (str: string) => {
    let charArray = [...str];
    charArray[0] = charArray[0].toUpperCase();
    return charArray.join('');
  };

  const kindDisplay: string = getTitle(kind);
</script>

<div
  class="w-1/2 mx-auto mt-4 bg-red-600/40 p-2 rounded-lg absolute left-[25vw]"
>
  <h2 class="text-xl font-semibold">{kindDisplay}:</h2>
  <p><slot /></p>
</div>
