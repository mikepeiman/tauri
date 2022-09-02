<script lang="ts">
  import './tailwind.css'
  import svelteLogo from './assets/svelte.svg'
  import metabrain3 from './assets/metabrain3.png'
  import metabrain4 from './assets/metabrain4.png'
  import metabrain5 from './assets/metabrain5.png'
  import Counter from './lib/Counter.svelte'
</script>
<svelte:head>
  <link href="/dist/output.css" rel="stylesheet">
</svelte:head>

<main  class="flex flex-col items-center justify-center w-screen h-screen">
  <div  class="flex flex-col items-center justify-center">

    <div class="flex">
      <a href="https://svelte.dev" target="_blank">
        <img src={metabrain4} class="logo svelte" alt="Svelte Logo" />
      </a>
      <a href="https://svelte.dev" target="_blank">
        <img src={metabrain5} class="logo svelte" alt="Svelte Logo" />
      </a>
    </div>
    <div class="flex font-sans text-cyan-400 text-6xl p-6">Welcome to MetaBrain</div>
  </div>
  <h1 class="text-orange-500 text-2xl text-fuchsia-400">An app built with Vite + Svelte</h1>

  <div class="card p-4">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank">SvelteKit</a>, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p>
</main>

<style>
  .logo {
    height: 12em;
    padding: 1.5em;
    will-change: filter;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>