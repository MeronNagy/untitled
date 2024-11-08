<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from '@tauri-apps/api/event';

  let x = $state(0);
  let y = $state(0);
  let last_x = $state(0);
  let last_y = $state(0);
  let key = $state('c');
  let last_key = $state("");

  async function click(event: Event) {
    event.preventDefault();
    await invoke("click", { x, y });
  }

  async function press(event: Event) {
    event.preventDefault();
    await invoke("press", { key });
  }

  onMount(async () => {
    // Start mouse tracking
    await invoke("start_tracking");

    // Listen for mouse movement events
    const unlisten = await listen('mouse-move', (event: any) => {
      last_x = event.payload.x;
      last_y = event.payload.y;
    });

    return () => {
      unlisten();
    };
  });
</script>

<main class="container">
  <h1>WIP</h1>

  <form class="row" onsubmit={click}>
    <input type="number" id="x-input" placeholder="x" bind:value={x} required/>
    <input type="number" id="y-input" placeholder="y" bind:value={y} required/>
    <button type="submit">Click</button>
  </form>
  <form class="row" onsubmit={press}>
    <input
            type="text"
            id="key-input"
            maxlength="1"
            placeholder="c"
            bind:value={key}
            required
            oninput={() => key = key.toLowerCase()}
            autocomplete="off"
    />
    <button type="submit">Press</button>
  </form>

  <div class="position-display">
    <h2>Current Mouse Position:</h2>
    <p>X: {last_x}, Y: {last_y}</p>
  </div>
  <div class="corner-image">
    <a href="https://meronnagy.github.io/untitled/">
      <img src="/images/00219-1214167366-transparent.png" alt="Documentation" />
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
  padding-top: 10vh;
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
button {
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
button {
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
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
@media (prefers-color-scheme: dark) {
  .position-display {
    background-color: #1f1f1f;
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
    background-color: #1f1f1f;
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
</style>
