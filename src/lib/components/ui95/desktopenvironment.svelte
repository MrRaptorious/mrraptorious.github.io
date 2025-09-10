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
  import { onMount } from "svelte";

  let { toOpen }: { toOpen?: string[] } = $props();

  // ************ Short URL Mapping ************
  const urlMapping = new Map<string, string>([
    ["dot", "dottify"],
    ["mob", "mobilitymodels"],
  ]);
  const itemdataToProgramID = new Map<string, number>();

  // ************ Programs ************
  import type { ItemData } from "$lib/types";
  import dottify_item from "../../experiments/dottify/item";
  import mobility_item from "../../experiments/mobility_models/item";
  import error_item from "../ui95/errorMessage/item";
  let experiments: ItemData[] = [dottify_item, mobility_item];
  // add to url mapping
  experiments.forEach((x) => urlMapping.set(x.id.toLowerCase(), x.id));
  // ************ end Programs ************

  // svelte-ignore non_reactive_update
  let availableArea: HTMLDivElement;

  // svelte-ignore non_reactive_update
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
    (itemdata: ItemData, index: number): Program => {
      // do some sideeffect ...
      itemdataToProgramID.set(itemdata.id.toLowerCase(), index);

      return {
        id: index,
        image: itemdata.image,
        title: itemdata.title,
        component: itemdata.component,
        windowData: null,
        windowHints: null,
      };
    },
  );

  // ************ Callbacks ************

  function openProgramByName(name: string) {
    console.log("try to open " + name);

    if (urlMapping.has(name)) {
      let itemID = urlMapping.get(name)?.toLowerCase();

      if (itemID && itemdataToProgramID.has(itemID)) {
        let programID = itemdataToProgramID.get(itemID);
        let program = availablePrograms.find((x) => x.id == programID);

        if (program) {
          openProgram(program);
        }
      }
    } else {
      console.log("could not find  " + name);
      let errorProg: Program = {
        id: 1 + Math.max(...availablePrograms.map((x) => x.id)),
        image: error_item.image,
        title: `Could not find Program "${name}"`,
        windowData: null,
        component: error_item.component,
        windowHints: { width: 400, height: 160 },
      };

      openProgram(errorProg);
    }
  }

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
    // debut
    let tmp = openPrograms.find((pid) => pid.id == id);
    console.log("SizeChanged: " + tmp?.title);
    if (tmp) {
      console.log("\tfrom: ", tmp.windowData?.width);
      console.log("\tto  : ", newWidth);
    }
    // enddebug

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

  onMount(() => {
    console.log("the component has mounted");
    // todo
    toOpen?.forEach((x) => {
      openProgramByName(x.toLowerCase());
    });
  });
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
