<script lang="ts">
  export let title: String;
  export let modal: HTMLDialogElement;
</script>

<dialog bind:this={modal}>
  <div class="p-4">
    <h1 class="text-3xl font-bold mb-2">
      {title}
    </h1>
    <div class="flex flex-col gap-y-2">
      <slot />
    </div>
  </div>
</dialog>

<style scoped>
  dialog {
    @apply bg-neutral-800 text-neutral-200 rounded-lg;
  }
</style>
