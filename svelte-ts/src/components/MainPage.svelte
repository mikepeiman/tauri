<script>
      import Counter from "../lib/Counter.svelte";
      import metabrain5 from "../assets/metabrain5.png";
</script>


<div class="flex flex-col items-center justify-center">
    <div class="flex">
      <a href="https://svelte.dev" target="_blank">
        <img src={metabrain5} class="logo svelte" alt="Svelte Logo" />
      </a>
    </div>
    <div class="flex title text-cyan-800 text-6xl font-bold p-2">MetaBrain</div>
  </div>
  <h1 class="text-orange-500 text-2xl text-fuchsia-400">
    An app built with Vite + Svelte + Tauri
  </h1>

  <div class="card p-4">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank"
      >SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>

  