<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from '@tauri-apps/api/event';

  let x = $state(0);
  let y = $state(0);
  let last_x = $state(0);
  let last_y = $state(0);
  let key = $state('c');
  let last_key = $state('');
  let actionScriptInput = $state('LeftClick; X=3180; Y=2030\nLeftClick; X=3280; Y=2030');

  async function clickMouse(event: Event) {
    event.preventDefault();
    await invoke("mouse_click", { x, y });
  }

  async function clickKeyboard(event: Event) {
    event.preventDefault();
    await invoke("keyboard_click", { key });
  }

  function recordKey(event: KeyboardEvent) {
    last_key = event.key;
  }

  async function executeActionScript(event: Event) {
    event.preventDefault();
    await invoke("orchestrate", { script: actionScriptInput })
            .catch((error) => console.error(error))
  }

  onMount(async () => {
    window.addEventListener('keydown', recordKey);

    await invoke("mouse_listener");
    await invoke("keyboard_listener");

    const unlistenMouse = await listen('mouse-move', (event: any) => {
      last_x = event.payload.x;
      last_y = event.payload.y;
    });

    const unlistenKeyboard = await listen('key-click', (event: any) => {
      last_key = event.payload.key;
    });

    return () => {
      unlistenMouse();
      unlistenKeyboard();
    };
  });
</script>

<main class="container">
  <h1>untitled</h1>
  <div class="position-display">
    <h2>Current Mouse Position</h2>
    <p>X: {last_x}, Y: {last_y}</p>
  </div>
  <div class="position-display">
    <h2>Action Script</h2>
    <form class="row" onsubmit="{executeActionScript}">
      <textarea
              id="text-input"
              placeholder="Enter text here"
              bind:value={actionScriptInput}
              rows="4"
              class="text-area"
              autocomplete="off"
      ></textarea>
      <button type="submit">Execute</button>
    </form>
  </div>
  <h1>Debug / Testing</h1>
  <div class="position-display">
    <h2>Click on absolute coordinates</h2>
    <form class="row" onsubmit={clickMouse}>
      <input
              id="x-input"
              type="number"
              placeholder="x"
              bind:value={x}
              required
              autocomplete="off"
      />
      <input
              id="y-input"
              type="number"
              placeholder="y"
              bind:value={y}
              required
              autocomplete="off"
      />
      <button type="submit">Click</button>
    </form>
  </div>
  <div class="position-display">
    <h2>Simulate Keystroke</h2>
    <form class="row" onsubmit={clickKeyboard}>
      <input
              id="key-input"
              type="text"
              placeholder="c"
              maxlength="1"
              bind:value={key}
              required
              autocomplete="off"
      />
      <button type="submit">Click</button>
    </form>
  </div>
  <div class="position-display">
    <h2>Last Key Pressed</h2>
    <p>{last_key}</p>
  </div>
  <div class="corner-image">
    <img src="/images/00219-1214167366-transparent.png" alt="Documentation" />
  </div>
  <div class="sticky-top-left">
    <a href="https://meronnagy.github.io/untitled/" target="_blank">
      <img src="/icons/help-circle.svg" alt="Help icon" width="24" height="24">
    </a>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 2vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button,
  .text-area {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  .text-area {
    width: 80%;
    max-width: 600px;
    resize: vertical;
    min-height: 100px;
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button,
  .text-area {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button,
    .text-area {
      color: #ffffff;
      background-color: #0f0f0f98;
    }

    button:active {
      background-color: #0f0f0f69;
    }
  }
  .position-display {
    margin-top: 2rem;
    padding: 1rem;
    background-color: #f0f0f0;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  @media (prefers-color-scheme: dark) {
    .position-display {
      background-color: rgba(31, 31, 31, 0.8);
    }
  }
  .corner-image {
    position: fixed;
    top: 0;
    right: 0;
    z-index: -100;
    transform: scaleX(-1);
  }

  .corner-image img {
    max-width: 200px;
    height: auto;
  }

  .sticky-top-left {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1000;
    margin: 10px;
    cursor: pointer;
  }
</style>