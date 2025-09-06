<script lang="ts">
  import type {
    Program,
    WindowData,
    WindowEvents,
    DataManipulator,
  } from "./ui95types";
  import Homebar from "./homebar.svelte";
  import Desktop from "./desktop.svelte";
  import Windowrenderer from "./windowrenderer.svelte";

  // ************ Programs ************
  import type { ItemData } from "$lib/types";
  import dottify_item from "../../experiments/dottify/item";
  import mobility_item from "../../experiments/mobility_models/item";
  let experiments: ItemData[] = [dottify_item, mobility_item];
  // ************ end Programs ************

  let availableArea: HTMLDivElement;

  let availablePrograms: Program[] = [];
  let openPrograms: Program[] = $state([]);

  let windowEvents: WindowEvents = {
    onClose: closeProgram,
    onChangeSize: changeSize,
    onChangePosition: changePosition,
  };

  let dataManipulator: DataManipulator = {
    setWindowData: setWindowData,
    setfocus: setfocus,
  };

  availablePrograms = experiments.map(
    (itemdata: ItemData, index: number): Program => ({
      id: index,
      image: itemdata.image,
      title: itemdata.title,
      component: itemdata.component,
      windowData: null,
    }),
  );

  // ************ Callbacks ************
  function openProgram(toOpen: Program): void {
    console.log("openProgram called (id: " + toOpen.id + ")");

    let program = openPrograms.find((p) => p.id === toOpen.id);

    if (program) {
      setfocus(toOpen.id);
    } else {
      openPrograms = [...openPrograms, toOpen];
    }
  }

  function closeProgram(id: number): void {
    console.log(openPrograms);
    openPrograms = openPrograms.filter((program: Program) => program.id != id);
  }

  function changeSize(id: number, newWidth: number, newHeight: number): void {
    console.log("SizeChanged: " + id);
    openPrograms = openPrograms.map((p) => {
      if (p.id == id) {
        return {
          ...p,
          windowData: { ...p.windowData!, width: newWidth, height: newHeight },
        };
      }
      return p;
    });
  }

  function changePosition(id: number, newPosX: number, newPosY: number): void {
    console.log("PositionChanged: " + id);
    openPrograms = openPrograms.map((p) => {
      if (p.id === id) {
        return {
          ...p,
          windowData: { ...p.windowData!, pos_x: newPosX, pos_y: newPosY },
        };
      } else {
        return {
          ...p,
        };
      }
    });
  }

  function setWindowData(id: number, windowData: WindowData): void {
    console.log("setWindowData called");

    openPrograms = openPrograms.map((program) => {
      if (program.id === id) {
        return { ...program, windowData: windowData };
      }
      return program;
    });
  }

  function setfocus(id: number): void {
    console.log("focused called, new focus: " + id);

    let max_z: number = Math.max(
      0,
      ...openPrograms
        .filter((p) => p.windowData?.zindex)
        .map((p) => p.windowData!.zindex),
    );

    openPrograms = openPrograms.map((p) => {
      if (p.id === id) {
        return {
          ...p,
          windowData: { ...p.windowData!, hasfocus: true, zindex: ++max_z },
        };
      } else {
        return {
          ...p,
          windowData: { ...p.windowData!, hasfocus: false },
        };
      }
    });
  }
</script>

<div class="h-full w-full flex flex-col overflow-hidden">
  <!-- Desktop  -->
  <div class="w-full h-full overflow-hidden" bind:this={availableArea}>
    <Desktop {availablePrograms} {openProgram}></Desktop>
  </div>

  <!-- Windows -->
  <Windowrenderer
    {openPrograms}
    {windowEvents}
    {availableArea}
    {dataManipulator}
  ></Windowrenderer>

  <!-- Homebar -->
  <Homebar {openPrograms} {dataManipulator} {availablePrograms} {openProgram}
  ></Homebar>
</div>
