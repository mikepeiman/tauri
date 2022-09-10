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
    registerAll, unregister, isRegistered
  } from "@tauri-apps/api/globalShortcut";
  import { getCurrent as getCurrentWindow, appWindow } from "@tauri-apps/api/window";
  // import * as w from "@tauri-apps/api/window";
  import App from "./App.svelte";
  let shortcut = "CmdOrControl+Alt+Space";
  
  async function toggleVisability() {
    const currentWindow = getCurrentWindow();
    // const currentWindow = getCurrentWindow();
    // console.log(`🚀 ~ file: App.svelte ~ line 20 ~ toggleVisability ~ currentWindow`, currentWindow)
    // currentWindow.isVisible() ? currentWindow.hide() : currentWindow.show();
    let windowState = await currentWindow.isVisible();
    let windowFocus = await currentWindow.focus();
    let windowCenter = await currentWindow.center();
    console.log(
      `🚀 ~ file: App.svelte ~ line 23 ~ toggleVisability ~ windowState`,
      windowState
    );
    console.log(
      `🚀 ~ file: App.svelte ~ line 25 ~ toggleVisability ~ windowFocus`,
      windowFocus
    );
    console.log(
      `🚀 ~ file: App.svelte ~ line 27 ~ toggleVisability ~ windowCenter`,
      windowCenter
    );

    // windowState.then(() => {
    //   console.log(`🚀 ~ file: App.svelte ~ line 20 ~ toggleVisability ~ currentWindow.isVisible(): `, currentWindow.isVisible())
    windowState ? currentWindow.hide() : showWindow();
    // })
    console.log(`show/hide window`);
  }

  const setHotkey = async ()=> {
    // const appWindow = getCurrentWindow();
    await unregister('CommandOrControl+Shift+Space');
    const _isRegistered = await isRegistered('CommandOrControl+Shift+Space');
    if (!_isRegistered) {
      await register('CommandOrControl+Shift+Space', async () => {
        const isVisible = await appWindow.isVisible();
        console.log(isVisible,"go")
        if (!isVisible){
          appWindow.focus = true
          console.log(`🚀 ~ file: App.svelte ~ line 57 ~ awaitregister ~ appWindow.focus`, appWindow.focus)
          appWindow.alwaysOnTop = true
          console.log(`🚀 ~ file: App.svelte ~ line 59 ~ awaitregister ~ appWindow.alwaysOnTop`, appWindow.alwaysOnTop)
          // appWindow.center()
          return appWindow.show();
          
        }
        appWindow.hide();
        console.log(`🚀 ~ file: App.svelte ~ line 65 ~ awaitregister ~ appWindow`, appWindow)
        console.log(`🚀 ~ file: App.svelte ~ line 57 ~ awaitregister ~ appWindow.focus`, appWindow.focus)
        console.log(`🚀 ~ file: App.svelte ~ line 59 ~ awaitregister ~ appWindow.alwaysOnTop`, appWindow.alwaysOnTop)
      });
      return appWindow.hide();
    }
    console.log("globalShortcut already registered")
  }


  function showWindow() {
    getCurrentWindow().show();
    getCurrentWindow().focus();
    getCurrentWindow().center();
  }
afterUpdate(() => {
console.log(`🚀 ~ file: App.svelte ~ line 81 ~ afterUpdate ~ afterUpdate`)

  appWindow.listen("show", ({ event, payload }) => { 
    console.log(`🚀 ~ file: App.svelte ~ line 86 ~ appWindow.listen ~ payload`, payload)
    console.log(`🚀 ~ file: App.svelte ~ line 89 ~ appWindow.listen ~ event`, event)

    
  });
  console.log(`🚀 ~ file: App.svelte ~ line 89 ~ appWindow.listen ~ appWindow`, appWindow)

})

  onMount(async () => {
    // const appWindow = await getCurrentWindow();
    appWindow.setTitle = true
    appWindow.title = "New title"
    
    setHotkey();

    // console.log(`🚀 ~ file: App.svelte ~ line 15 ~ windowOptions`, windowOptions)
    // console.log(`🚀 ~ file: App.svelte ~ line 19 ~ w`, w)
    // await toggleVisability;
    // registerShortcut(shortcut, toggleVisability);
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
