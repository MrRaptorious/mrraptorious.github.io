<script lang="ts">
  import OpenWindow from "./openWindow.svelte";
  import type {
    Program,
    WindowEvents,
    WindowData,
    DataManipulator,
  } from "./ui95types";

  interface WindowRendererProps {
    openPrograms: Program[];
    availableArea: HTMLElement;
    windowEvents: WindowEvents;
    dataManipulator: DataManipulator;
  }

  let {
    openPrograms,
    availableArea,
    windowEvents,
    dataManipulator,
  }: WindowRendererProps = $props();

  // crate window data

  $effect(() => {
    openPrograms.forEach((p) => {
      if (!p.windowData) {
        let windowData: WindowData = {
          pos_x: availableArea.offsetWidth * 0.1,
          pos_y: availableArea.offsetHeight * 0.1,
          width: availableArea.offsetWidth * 0.8,
          height: availableArea.offsetHeight * 0.8,
          maximized: false,
          minimized: false,
          hasfocus: false,
          zindex: 0,
        };

        console.log("WINDOWDATA CREATED: " + p.id);
        dataManipulator.setWindowData(p.id, windowData);
        dataManipulator.setfocus(p.id);
      } else {
        console.log("WINDOWDATA FOUND" + p.id);
      }
    });
  });
</script>

{#each openPrograms as p (p.id)}
  {#if p.windowData}
    <OpenWindow
      windowData={p.windowData}
      {availableArea}
      {windowEvents}
      {dataManipulator}
      program={p}
    ></OpenWindow>
  {/if}
{/each}
