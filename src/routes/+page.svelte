<script lang="ts">
  import CreateDatabase from '$lib/CreateDatabase.svelte';
</script>

<main class="h-screen flex-col flex items-center pt-20">
  <h1 class="text-4xl font-bold mb-10">Welcome to RustKeep!</h1>
  <CreateDatabase />
  <a href="/database">Database</a>
</main>

<style>
</style>
