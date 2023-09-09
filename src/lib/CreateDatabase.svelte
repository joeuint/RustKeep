<script lang="ts">
  import Dialog from './Dialog.svelte';

  import { save, open } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import { getContext } from 'svelte';
  import zxcvbn from 'zxcvbn';
  import type { Writable } from 'svelte/store';
  import type { Alert } from '$types/Alert';
  import type { Database } from '$types/Database';
  import { goto } from '$app/navigation';

  const alerts = getContext<Writable<Alert>>('alerts');
  const database = getContext<Writable<Database>>('database');
  const passwordStore = getContext<Writable<string>>('password');

  function stringTime(seconds: number) {
    const minute = 60;
    const hour = minute * 60;
    const day = hour * 24;
    const month = day * 31;
    const year = month * 12;
    const century = year * 100;
    let base: number;
    let type: string;

    if (seconds < 1) {
      return 'less than a second';
    } else if (seconds < minute) {
      base = Math.round(seconds);
      type = 'second';
    } else if (seconds < hour) {
      base = Math.round(seconds / minute);
      type = 'minute';
    } else if (seconds < day) {
      base = Math.round(seconds / hour);
      type = 'hour';
    } else if (seconds < month) {
      base = Math.round(seconds / day);
      type = 'day';
    } else if (seconds < year) {
      base = Math.round(seconds / month);
      type = 'month';
    } else if (seconds < century) {
      base = Math.round(seconds / year);
      type = 'year';
    } else {
      return 'centries';
    }

    if (base != 1) {
      type += 's';
    }

    return base + ' ' + type;
  }

  let password: string = '';

  let entropy: string | number;

  let modal: HTMLDialogElement;

  async function newDatabase() {
    modal.showModal();
  }

  function updateEntropy() {
    setTimeout(() => {
      entropy = stringTime(zxcvbn(password).guesses / 1000);
      console.log(password);
    }, 10);
  }

  async function sendDatabaseFile() {
    const path = await save({
      filters: [
        {
          name: 'Rust Keep Database',
          extensions: ['rkdb'],
        },
      ],
    });

    if (path != null) {
      await invoke('create_database', { path: path, password: password });
      password = '';
      modal.close();
    }
  }

  async function openDatabaseFile() {
    const path = await open({
      filters: [
        {
          name: 'Rust Keep Database',
          extensions: ['rkdb'],
        },
      ],
    });

    if (path != null) {
      const data = await invoke<[string, ArrayBuffer]>('open_database', {
        path: path,
        password: password,
      }).catch((e) => {
        alerts.set({
          severity: 'error',
          msg: e,
        });

        return;
      });

      if (data == null) return;

      const parse: Database = JSON.parse(data[0]);

      passwordStore.set(password);
      database.set(parse);

      goto('/database');
    }
  }
</script>

<Dialog bind:modal title="Create Database">
  <p>Please enter a strong master password to encrypt all of your data!</p>
  <input type="password" on:input={updateEntropy} bind:value={password} />
  <p>
    <abbr title="Provided by zxcvbn at 1000 guesses/sec.">Est. Crack Time:</abbr
    >
    {entropy}
  </p>
  <div class="flex">
    <button
      type="submit"
      on:click={() => modal.close()}
      class="bg-red-700 p-1 w-1/3 text-lg font-semibold mx-auto rounded-lg"
      >Close</button
    >
    <button
      type="submit"
      on:click={sendDatabaseFile}
      class="bg-green-700 p-1 w-1/3 text-lg font-semibold mx-auto rounded-lg"
      >Create</button
    >
  </div>
</Dialog>

<div class="flex gap-x-5">
  <button
    type="submit"
    class="bg-neutral-800 p-2 rounded-lg font-semibold text-lg"
    on:click={openDatabaseFile}>Open Database</button
  >
  <button
    type="submit"
    class="bg-neutral-800 p-2 rounded-lg font-semibold text-lg"
    on:click={newDatabase}>New Database</button
  >
  <a
    href="/generator"
    type="submit"
    class="bg-neutral-800 p-2 rounded-lg font-semibold text-lg">Generate</a
  >
</div>
