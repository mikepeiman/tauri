<script lang="ts">
  import "./tailwind.css";
  import metabrain5 from "./assets/metabrain5.png";
  import MainPage from "./components/MainPage.svelte";

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
    WebviewWindow,
  } from "@tauri-apps/api/window";
  // import * as w from "@tauri-apps/api/window";
  import App from "./App.svelte";
  let shortcut = "CommandOrControl+Shift+Space";

  const setWindowProperties = async () => {
    const window = await getCurrentWindow();

    const logicalSize: LogicalSize = {
      width: 800,
      height: 600,
    };
    const logicalPosition: LogicalPosition = {
      x: 0,
      y: 0,
    };
    const physicalSize: PhysicalSize = {
      width: 800,
      height: 600,
    };
    const physicalPosition: PhysicalPosition = {
      x: 0,
      y: 0,
    };
    const decorated = await appWindow.isDecorated();
    console.log(
      `🚀 ~ file: App.svelte ~ line 49 ~ setWindowProperties ~ decorated`,
      decorated
    );
    await appWindow.setDecorations(false);
  };

  const setHotkey = async () => {
    await unregister(shortcut);
    const _isRegistered = await isRegistered(shortcut);
    if (!_isRegistered) {
      await register(shortcut, async () => {
        const isVisible = await appWindow.isVisible();
        console.log(isVisible, "go");
        if (!isVisible) {
          appWindow.focus = true;
          let focused = await appWindow.setFocus();
          console.log(
            `🚀 ~ file: App.svelte ~ line 34 ~ awaitregister ~ focused`,
            focused
          );
          let ontop = await appWindow.setAlwaysOnTop(true);
          console.log(
            `🚀 ~ file: App.svelte ~ line 36 ~ awaitregister ~ ontop `,
            ontop
          );
          await appWindow.setSize(new LogicalSize(900, 100));
          // await appWindow.setPosition(new PhysicalPosition(200, -600));
          appWindow.center();
          await appWindow.requestUserAttention();
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
    const webview = new WebviewWindow("super-input");
    webview.once("tauri://created", function () {
      console.log(`🚀 ~ file: App.svelte ~ line 34 ~ created`, webview);
    });
  });

  function addEventListeners() {
    console.log(`🚀 ~ file: App.svelte ~ line 81 ~ addEventListeners`);
    const webview = new WebviewWindow("super-input");
    webview.once("tauri://created", function () {
      console.log(`🚀 ~ file: App.svelte ~ line 34 ~ created`, webview);
    });
    // document
    //   .getElementById("titlebar-minimize")
    //   .addEventListener("click", () => appWindow.minimize());
    // document
    //   .getElementById("titlebar-maximize")
    //   .addEventListener("click", () => appWindow.toggleMaximize());
    document
      .getElementById("titlebar-close")
      .addEventListener("click", () => appWindow.close());
  }

  onMount(async () => {
    setHotkey();
    setWindowProperties();
    appWindow.setTitle("MetaBrain");
    addEventListeners();
  });
</script>

<svelte:head>
  <link href="/dist/output.css" rel="stylesheet" />
</svelte:head>

<main
  class="flex flex-col items-center justify-center w-full h-full  bg-blue-300 overflow-hidden border-2 border-cyan-300">
  <div data-tauri-drag-region class="titlebar rounded-t-md">
    <!-- <div class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div> -->
    <div class="titlebar-button " id="titlebar-close">
      <img
      src="https://api.iconify.design/mdi-settings.svg"
      alt="close"
      class="text-white" />
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div> 
  </div>
  <!-- <MainPage /> -->
  <div class="flex h-24 w-full bg-black"><input type="text" /></div>
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
    /* color: #888; */
  }
  .titlebar {
    height: 30px;
    background: #329ea3;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
  }
  .titlebar-button:hover {
    background: #5bbec3;
  }
</style>
