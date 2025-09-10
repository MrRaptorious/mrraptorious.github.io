<script lang="ts">
  import type { WindowEvents } from "../ui95types";
  import forbidden from "../assets/msg_warning.ico";

  let {
    windowEvents,
    programID,
  }: { windowEvents: WindowEvents; programID: number } = $props();
</script>

<div class="flex flex-col w-full h-full space-y-3 items-center">
  <div class="flex space-x-6 m-3 justify-center items-center">
    <img src={forbidden} class="w-10 h-10" alt="error icon" />
    <p>
      The program was not found.<br />Only open programs from the desktop.
    </p>
  </div>
  <div class="flex justify-center h-8 w-30">
    <button
      class="flex items-center justify-center h-full w-full button3d leading-5"
      onclick={() => {
        windowEvents.onClose(programID);
      }}
    >
      <div class="h-full">Ok</div>
    </button>
  </div>
</div>

<style>
  @import "../assets/ui95.css";
</style>

