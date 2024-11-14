<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from '@tauri-apps/api/event';
  import Modal from './Modal.svelte';
  import CommandPanel from "./CommandPanel.svelte";

  let showErrorModal = $state(false);
  let errorMessage = $state('');

  let x = $state(0);
  let y = $state(0);
  let lastRecordedX = $state(0);
  let lastRecordedY = $state(0);
  let key = $state('c');
  let lastRecordedKey = $state('');
  let actionScriptInput = $state('LeftClick; X=3180; Y=2030; Delay=1000\nLeftClick; X=3280; Y=2030; Delay=1000');
  let isExecuting = $state(false);
  let executeButtonText = $derived.by(() => {
    if (isExecuting) {
      return "Stop\n ALT+S";
    }
    return "Execute";
  })
  let replayCount = $state(1);

  async function clickMouse(event: Event) {
    event.preventDefault();
    await invoke("mouse_click", { x, y });
  }

  async function clickKeyboard(event: Event) {
    event.preventDefault();
    await invoke("keyboard_click", { key });
  }

  async function handleExecuteButtonClick(event: Event) {
    event.preventDefault();
    if (isExecuting) {
      await invoke("interrupt_orchestration");
    } else {
      isExecuting = true;
      await invoke("orchestrate", { script: actionScriptInput, replayCount: replayCount })
              .catch((error) => {
                errorMessage = error;
                showErrorModal = true;
              })
              .finally(() => {
                isExecuting = false;
              })
      ;
    }
  }

  async function recordActions(event: Event) {
    event.preventDefault();
    await invoke("record_actions")
            .catch((error) => {
              errorMessage = error;
              showErrorModal = true;
            })
    ;
  }

  onMount(async () => {
    await invoke("mouse_listener");
    await invoke("keyboard_listener");

    const unlistenMouse = await listen('mouse-move', (event: any) => {
      lastRecordedX = event.payload.x;
      lastRecordedY = event.payload.y;
    });

    const unlistenKeyboard = await listen('key-click', (event: any) => {
      lastRecordedKey = event.payload.key;
    });

    return () => {
      unlistenMouse();
      unlistenKeyboard();
    };
  });
</script>

<main class="container">
  <h1>untitled</h1>
  <CommandPanel title={"Action Script"}>
    <form onsubmit="{handleExecuteButtonClick}">
      <div>
        <textarea
                id="text-input"
                bind:value={actionScriptInput}
                rows="4"
                class="text-area"
                autocomplete="off"
        ></textarea>
      </div>
      <div>
        <label for="replay-input">Replay Count:</label>
        <input
                id="replay-input"
                type="number"
                bind:value={replayCount}
                required
                autocomplete="off"
                style="width: 4rem"
                min="1"
        />
        <button type="submit">{executeButtonText}</button>
        <button onclick={recordActions}>Record</button>
      </div>
    </form>
  </CommandPanel>
  <h1>Debug / Info</h1>
  <CommandPanel title={"Current Mouse Position"}>
    <p>X: {lastRecordedX}, Y: {lastRecordedY}</p>
  </CommandPanel>
  <CommandPanel title={"Last Key Pressed"}>
    <p>{lastRecordedKey}</p>
  </CommandPanel>
  <h1>Debug / Testing</h1>
  <CommandPanel title={"Click on absolute coordinates"}>
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
  </CommandPanel>
  <CommandPanel title={"Simulate Keystroke"}>
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
  </CommandPanel>
</main>

<Modal bind:showModal={showErrorModal} title={"Something went wrong."}>
  <p>{errorMessage}</p>
</Modal>

<style>
  .container {
    margin: 0;
    padding-top: 2vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }
</style>
