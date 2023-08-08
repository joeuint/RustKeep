<script lang="ts">
  import type { Database } from '$types/Database';
  import { invoke } from '@tauri-apps/api/tauri';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  let passType: 'phrase' | 'word';
  let generatedType: 'phrase' | 'word';
  let generated_pass: string = '';
  let passLength: number = 1;

  async function getPass() {
    switch (passType) {
      case 'phrase':
        const phrase = await invoke<Array<string>>('gen_passphrase', {
          amount: passLength,
        });
        if (typeof phrase == 'object' && phrase) {
          generated_pass = phrase.join(' ');
          generatedType = passType;
        } else {
          console.error('You royally screwed up if you see this 💀.');
        }
        break;
      case 'word':
        generated_pass = await invoke<string>('gen_password', {
          amount: passLength,
        });
        generatedType = passType;
        break;
      default:
        console.error('Unkown pass type');
        break;
    }
  }

  const database = getContext<Writable<Database>>('database');
</script>

<main class="flex-col flex items-center pt-20">
  <h1 class="text-4xl font-bold mb-10">Password Generator</h1>
  <form on:submit|preventDefault={getPass}>
    <div class="flex items-center gap-x-20">
      <fieldset class="flex flex-col gap-y-4">
        <label>
          <input
            type="radio"
            name="passtype"
            id="password"
            class="mr-2"
            value="word"
            bind:group={passType}
            checked
          />
          Password
        </label>
        <label
          ><input
            type="radio"
            name="passtype"
            id="passphrase"
            class="mr-2"
            value="phrase"
            bind:group={passType}
          />Passphrase</label
        >
      </fieldset>
      <input
        type="number"
        name="length"
        id="length"
        max="256"
        min="1"
        bind:value={passLength}
      />
    </div>
    <div class="flex justify-center">
      <button
        type="submit"
        class="bg-green-700 p-2 rounded-lg font-semibold text-lg mt-10"
        >Generate</button
      >
    </div>
  </form>
  <div class="flex justify-center px-10">
    <div
      class="bg-neutral-800 mt-8 p-4 rounded-lg font-semibold text-lg text-center {generatedType ==
      'word'
        ? 'break-all'
        : ''}"
    >
      {generated_pass}
    </div>
  </div>
</main>
