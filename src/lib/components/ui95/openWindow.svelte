<script module>
  import type { ItemData } from "$lib/types";
  import { clamp } from "$lib/util";
  import Scrollbar from "./scrollbar.svelte";
  import { onMount } from "svelte";

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

  let grabberSize: number = 80;

  let scrollableHeight = 0;
  let contentContainer: HTMLDivElement;

  function scrollCallback(fraction: number) {
    if (contentContainer) {
      const totalScrollHeight =
        contentContainer.scrollHeight - contentContainer.clientHeight;
      const newScrollPosition = totalScrollHeight * fraction;
      contentContainer.scrollTo({ top: newScrollPosition, behavior: "smooth" });
    }
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
    updateWindowPosition(id, px, py);
    updateWindowSize(id, self.clientWidth, self.clientHeight);
  }

  function handleNativeScroll() {
    if (contentContainer) {
      const scrollPosition = contentContainer.scrollTop;
      const totalScrollHeight =
        contentContainer.scrollHeight - contentContainer.clientHeight;

    }
  }

  onMount(() => {
    if (contentContainer) {
      scrollableHeight = contentContainer.scrollHeight;
    }
  });
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  bind:this={self}
  id="openWindow"
  class={"resize overflow-hidden absolute min-h-15 min-w-15 flex flex-col"}
  style="width: {width}px;
        height: {height}px;
        top: {py}px;
        left: {px}px;"
  role="region"
  onmouseup={() => populateWindowUpdate()}
        onscroll={handleNativeScroll}
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
      {displayData.title}
    </p>

    <!-- close button-->
    <button
      id="closeButton"
      class="h-[calc(100%-5px)] aspect-square mx-1
              grid place-items-center
              text-black text-lg font-bold
              button3d"
      onclick={() => closeWindow(id)}>X</button
    >
  </div>

  <!-- {#if isdragging}
    <div class="absolute top-8 left-0 w-full h-full z-10"></div>
  {/if} -->

  <!-- contents-->
  <div class="flex-1 w-full flex flex-row overflow-hidden">
    <div
      class="flex-1 overflow-auto"
      bind:this={contentContainer}
    >
      <!-- svelte-ignore svelte_component_deprecated -->
      <svelte:component this={displayData.component}></svelte:component>
    </div>
    <!-- <div class="w-6 h-full">
      <Scrollbar {scrollCallback} {grabberSize}></Scrollbar>
    </div> -->
  </div>
</div>

<style>
  @import "./assets/ui95.css";

  .scrollContainer {
    scrollbar-width: none;
  }

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
