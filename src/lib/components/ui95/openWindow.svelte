<script module>
  import { clamp } from "$lib/util";
</script>

<script lang="ts">
  import type { Program, WindowData, WindowEvents } from "./ui95types";

  interface OpenWindowProps {
    windowData: WindowData;
    availableArea: HTMLElement;
    windowEvents: WindowEvents;
    program: Program;
  }

  let { windowData, availableArea, windowEvents, program }: OpenWindowProps =
    $props();

  let isdragging = $state(false);
  let off_x = 0;
  let off_y = 0;

  let currentWidth = $state(windowData.width);
  let currentHeight = $state(windowData.height);
  let currentPosX = $state(windowData.pos_x);
  let currentPosY = $state(windowData.pos_y);

  let self: HTMLElement;
  let handleBar: HTMLElement;

  function handleMouseDown(event: MouseEvent) {
    isdragging = true;

    off_x = event.clientX - currentPosX;
    off_y = event.clientY - currentPosY;

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  }

  function handleMouseMove(event: MouseEvent) {
    if (isdragging) {
      currentPosX = clamp(
        0,
        availableArea.clientWidth - self.clientWidth,
        event.clientX - off_x,
      );
      currentPosY = clamp(
        0,
        availableArea.clientHeight - self.clientHeight,
        event.clientY - off_y,
      );
    }
  }

  function handleMouseUp() {
    isdragging = false;

    windowEvents.onChangePosition(program.id, currentPosX, currentPosY);
    windowEvents.onChangeSize(program.id, currentWidth, currentHeight);

    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  }

  function propagateUpdates() {
    windowEvents.onChangePosition(program.id, currentPosX, currentPosY);
    windowEvents.onChangeSize(program.id, currentWidth, currentHeight);
    console.log("current width: ", currentWidth);
    console.log("window width: ", windowData.width);
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  bind:this={self}
  id="openWindow"
  class={"resize overflow-hidden absolute min-h-15 min-w-15 flex flex-col"}
  style="width: {windowData.width}px;
        height: {windowData.height}px;
        top: {currentPosY}px;
        left: {currentPosX}px;"
  role="region"
  onmouseup={() => {
    propagateUpdates();
  }}
  bind:clientWidth={currentWidth}
  bind:clientHeight={currentHeight}
>
  <!-- headder -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    bind:this={handleBar}
    role="region"
    id="windowHead"
    class=" h-8 w-full flex flex-row justify-end items-center"
    onmousedown={handleMouseDown}
    onmousemove={handleMouseMove}
    onmouseup={handleMouseUp}
  >
    <p
      class="h-full w-full flex-1 text-white flex justify-left items-center m-1 mx-2 overflow-hidden whitespace-nowrap break-keep"
    >
      {program.title}
    </p>

    <!-- close button-->
    <button
      id="closeButton"
      class="h-[calc(100%-5px)] aspect-square mx-1
              grid place-items-center
              text-black text-lg font-bold
              button3d"
      onclick={() => windowEvents.onClose(program.id)}>X</button
    >
  </div>

  <!-- contents-->
  <div class="flex-1 w-full flex flex-row overflow-hidden">
    <div class="flex-1 overflow-auto">
      <!-- svelte-ignore svelte_component_deprecated -->
      <svelte:component this={program.component}></svelte:component>
    </div>
  </div>
</div>

<style>
  @import "./assets/ui95.css";

  #closeButton {
    border-width: 2px;
    background-color: #c3c3c3;
    font-size: large;
  }

  #windowHead {
    background-color: #000082;
    margin: 2px;
  }

  #openWindow {
    background-color: #c3c3c3;
    border: 2px solid;
    border-top-color: white;
    border-left-color: white;
    border-right-color: black;
    border-bottom-color: black;
  }
</style>
