<script lang="ts">
  import "./tailwind.css";
  import svelteLogo from "./assets/svelte.svg";
  import metabrain3 from "./assets/metabrain3.png";
  import metabrain4 from "./assets/metabrain4.png";
  import metabrain5 from "./assets/metabrain5.png";
  import Counter from "./lib/Counter.svelte";

  import { onMount, beforeUpdate, afterUpdate } from "svelte";
  import {
    register,
    registerAll,
    unregister,
    isRegistered,
  } from "@tauri-apps/api/globalShortcut";
  import {
    getCurrent as getCurrentWindow,
    appWindow,
    LogicalSize,
    LogicalPosition,
    PhysicalSize,
    PhysicalPosition,
  } from "@tauri-apps/api/window";
  // import * as w from "@tauri-apps/api/window";
  import App from "./App.svelte";
  let shortcut = "CommandOrControl+Shift+Space";

  const setHotkey = async () => {
    await unregister(shortcut);
    const _isRegistered = await isRegistered(shortcut);
    if (!_isRegistered) {
      await register(shortcut, async () => {
        const isVisible = await appWindow.isVisible();
        console.log(isVisible, "go");
        if (!isVisible) {
          appWindow.focus = true;
          let focused = await appWindow.setFocus()
          console.log(`🚀 ~ file: App.svelte ~ line 34 ~ awaitregister ~ focused`, focused)
          let ontop = await appWindow.setAlwaysOnTop(true);
          console.log(`🚀 ~ file: App.svelte ~ line 36 ~ awaitregister ~ ontop `, ontop )
          await appWindow.setSize(new LogicalSize(900, 100))
          // await appWindow.setPosition(new PhysicalPosition(200, -600));
          appWindow.center()
          return appWindow.show();
        }
        appWindow.hide();
      });
      return appWindow.hide();
    }
    console.log("globalShortcut already registered");
  };

  afterUpdate(() => {
    console.log(`🚀 ~ file: App.svelte ~ line 81 ~ afterUpdate ~ afterUpdate`);
  });

  onMount(async () => {
    setHotkey();
    appWindow.setTitle("MetaBrain")
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
