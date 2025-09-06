<script lang="ts">
  import type { Program } from "./ui95types";
  import DesktopIcon from "./desktop_icon.svelte";

  interface DesktopProps {
    availablePrograms: Program[];
    openProgram: (toOpen: Program) => void;
  }

  let { availablePrograms, openProgram }: DesktopProps = $props();

  let isDrawing = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let endX = $state(0);
  let endY = $state(0);

  let iconElements: HTMLElement[] = $state([]);
  let selectedProgramIds = $state<Set<number>>(new Set());

  function handleMouseDown(event: MouseEvent) {
    if (event.button !== 0) return;

    isDrawing = true;
    startX = event.clientX;
    startY = event.clientY;
    endX = startX;
    endY = startY;
    selectedProgramIds = new Set(); // Auswahl beim Start zurücksetzen

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  }

  function handleMouseMove(event: MouseEvent) {
    if (isDrawing) {
      endX = event.clientX;
      endY = event.clientY;
      updateSelection();
    }
  }

  function handleMouseUp() {
    isDrawing = false;
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  }

  function updateSelection() {
    const newSelectedIds = new Set<number>();
    const selectBoxRect = {
      left: Math.min(startX, endX),
      top: Math.min(startY, endY),
      right: Math.max(startX, endX),
      bottom: Math.max(startY, endY),
    };

    iconElements.forEach((iconEl, index) => {
      if (!iconEl) return;
      const iconRect = iconEl.getBoundingClientRect();

      const isOverlapping = !(
        selectBoxRect.right < iconRect.left ||
        selectBoxRect.left > iconRect.right ||
        selectBoxRect.bottom < iconRect.top ||
        selectBoxRect.top > iconRect.bottom
      );

      if (isOverlapping) {
        newSelectedIds.add(availablePrograms[index].id);
      }
    });

    selectedProgramIds = newSelectedIds;
  }
</script>

<div
  class="w-full h-full flex bg-winteal-500 relative"
  onmousedown={handleMouseDown}
  role="main"
>
  <div class="space-y-1 p-10 z-0">
    {#each availablePrograms as program, index}
      <DesktopIcon
        name={program.title}
        image={program.image}
        dblclick={() => openProgram(program)}
        isSelected={selectedProgramIds.has(program.id)}
        bind:element={iconElements[index]}
      ></DesktopIcon>
    {/each}
  </div>

  {#if isDrawing}
    <div
      class="select-box"
      style="
        top: {Math.min(startY, endY)}px;
        left: {Math.min(startX, endX)}px;
        width: {Math.abs(endX - startX)}px;
        height: {Math.abs(endY - startY)}px;
      "
    ></div>
  {/if}
</div>

<style>
  .select-box {
    position: absolute;
    border: 1px dashed black;
    background-color: transparent;
    pointer-events: none;
    z-index: 5;
  }
</style>
