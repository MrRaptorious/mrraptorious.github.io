<script module>
  import type { ItemData } from "$lib/types";

  export type OpenWindow_t = {
    id: number;
    width: number;
    height: number;
    posX: number;
    posY: number;
    displayData: ItemData;
  };
</script>

<script lang="ts">
  interface Props {
    id: number;
    parent: HTMLElement;
    width: number;
    height: number;
    px: number;
    py: number;
    displayData: ItemData;
    closeWindow: (id: number) => void;
    updateWindowPosition: (id: number, px: number, py: number) => void;
    updateWindowSize: (id: number, width: number, height: number) => void;

  }

  let {
    id,
    parent,
    width,
    height,
    px,
    py,
    displayData,
    closeWindow,
    updateWindowPosition,
    updateWindowSize,
  }: Props = $props();
  let isdragging = $state(false);
  let off_x = 0;
  let off_y = 0;
  let self: HTMLElement;
  let handleBar: HTMLElement;

  function clamp(min: number, max: number, x: number) {
    return Math.min(max, Math.max(min, x));
  }

  function handleMouseDown(event: MouseEvent) {
    isdragging = true;
    off_x = event.clientX - px;
    off_y = event.clientY - py;

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  }

  function handleMouseMove(event: MouseEvent) {
    event.preventDefault();

    if (isdragging) {
      px = clamp(
        0,
        parent.clientWidth - self.clientWidth,
        event.clientX - off_x,
      );
      py = clamp(
        0,
        parent.clientHeight - self.clientHeight,
        event.clientY - off_y,
      );

    }
  }

  function handleMouseUp() {
    isdragging = false;

    // not needed, parent div handels update
    // populateWindowUpdate();

    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  }

  function populateWindowUpdate() {
    console.log("WindowUPDATE Populated");
    updateWindowPosition(id, px, py);
    updateWindowSize(id, self.clientWidth, self.clientHeight );
  }


</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  bind:this={self}
  class={"resize overflow-hidden absolute bg-red-500 min-h-15 min-w-15"}
  style="width: {width}px;
        height: {height}px;
        top: {py}px;
        left: {px}px;"
  role="region"
  onmouseup={() => populateWindowUpdate()}
>
  <!-- headder -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={handleBar}
    role="region"
    class=" h-8 w-full flex flex-row justify-end bg-gray-500"
    onmousedown={handleMouseDown}
    onmousemove={handleMouseMove}
    onmouseup={handleMouseUp}
  >
    <p class="flex-1">{displayData.title}</p>

    <!-- close button-->
    <button
      class="h-full bg-green-500 aspect-square"
      onclick={() => closeWindow(id)}>x</button
    >
    <div class="bg-pink-500"></div>
  </div>

  {#if isdragging}
    <div class="absolute top-8 left-0 w-full h-full z-10"></div>
  {/if}

  <!-- contents-->
  <!-- svelte-ignore slot_element_deprecated -->
  <div class="w-full h-full bg-pink-500">
    <iframe
      title={displayData.title}
      class="w-full h-full"
      src={displayData.link}
      frameborder="0"
    ></iframe>
  </div>
</div>

<style>
</style>
