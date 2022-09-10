<script lang="ts">
  import "./tailwind.css";
  import svelteLogo from "./assets/svelte.svg";
  import metabrain3 from "./assets/metabrain3.png";
  import metabrain4 from "./assets/metabrain4.png";
  import metabrain5 from "./assets/metabrain5.png";
  import Counter from "./lib/Counter.svelte";

  import { onMount } from "svelte";
  import {
    register as registerShortcut,
    registerAll,
  } from "@tauri-apps/api/globalShortcut";
  import { getCurrent as getCurrentWindow } from "@tauri-apps/api/window";
  import App from "./App.svelte";
  let shortcut = "CmdOrControl+Alt+Space";

  async function toggleVisability() {
    const currentWindow = getCurrentWindow();
    console.log(`🚀 ~ file: App.svelte ~ line 20 ~ toggleVisability ~ currentWindow`, currentWindow)
    // currentWindow.isVisible() ? currentWindow.hide() : currentWindow.show();
    let windowState = await currentWindow.isVisible()
    console.log(`🚀 ~ file: App.svelte ~ line 23 ~ toggleVisability ~ windowState`, windowState)
    
    // windowState.then(() => {
    //   console.log(`🚀 ~ file: App.svelte ~ line 20 ~ toggleVisability ~ currentWindow.isVisible(): `, currentWindow.isVisible())
    windowState ? currentWindow.hide() : currentWindow.show();
    // }) 
    console.log(`show/hide window`)
  }

  onMount(async () => {
    await toggleVisability
    registerShortcut(shortcut, toggleVisability);
  });
</script>

<svelte:head>
  <link href="/dist/output.css" rel="stylesheet" />
</svelte:head>

<main class="flex flex-col items-center justify-center w-screen h-screen">
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
</main>

<style>
  .title {
    font-family: "Montserrat", sans-serif;
    font-weight: 700;
  }
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
