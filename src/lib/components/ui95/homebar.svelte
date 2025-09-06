<script lang="ts">
  import { onMount } from "svelte";
  import type {
    DataManipulator,
    Program,
  } from "$lib/components/ui95/ui95types";
  import start_icon from "$lib/components/ui95/assets/start_icon.png";
  import msg_information from "$lib/components/ui95/assets/msg_information.ico";
  import Speechbubble from "./speechbubble.svelte";

  interface HomebarProps {
    openPrograms: Program[];
    dataManipulator: DataManipulator;
  }
  let { openPrograms, dataManipulator }: HomebarProps = $props();

  let time = $state(new Date());
  let showBubble = $state(true);

  function hiddeBubble() {
    showBubble = false;
  }

  onMount(() => {
    const interval = setInterval(() => {
      time = new Date();
    }, 1000 * 60);

    return () => {
      clearInterval(interval);
    };
  });
</script>

<div
  class="text-text-500 mt-auto flex w-full p-1 box-border bg-winlightgray-500 space-x-2"
>
  <!-- start button -->
  <button
    class="w-25 text-black flex space-x-2 button3d text-xl font-bold justify-center"
  >
    <img
      src={start_icon}
      alt="svelte windows logo"
      class="[image-rendering:pixelated] p-[3px]"
    />
    <p>Home</p>
  </button>

  <!-- open windows -->
  <div class=" h-full flex space-x-1 overflow-hidden flex-1 min-w-0">
    {#each openPrograms as program}
      <button
        class="h-full flex-1 max-w-60 flex flex-row justify-left items-center {program
          .windowData?.hasfocus
          ? 'element3d-inv taskbarbutton-focused'
          : 'element3d taskbarbutton'}"
        onclick={() => {
          console.log("clicked button taskbar" + program.windowData?.hasfocus);
          dataManipulator.setfocus(program.id);
        }}
      >
        <img
          src={program.image}
          class="p-1 h-full [image-rendering:pixelated]"
          alt={program.title}
        />
        <p class="text-black truncate">{program.title}</p>
      </button>
    {/each}
  </div>

  <!-- time -->
  <div
    class="px-4 text-black flex content-center items-center justify-center space-x-3 button3dInvert ml-auto"
  >
    <div class="flex justify-center space-x-1">
      <div class="flex justify-center">
        <button
          onclick={() => {
            showBubble = true;
          }}
          class="relative"
        >
          <img
            src={msg_information}
            alt="svelte windows logo"
            class="[image-rendering:pixelated] aspect-square"
          />

          {#if showBubble}
            <Speechbubble right_offset={-40} right_offset_unit={"px"}>
              <svelte:fragment>
                <div>
                  <p class="text-left mx-1 whitespace-nowrap p-2">
                    <b>Hello!</b> <br />
                    This is my hobby project where I show things I wanted to explore.
                    Feel free to look around.<br />
                    <b> Double click</b> any Desktop Icon to see some Interesting
                    things!
                  </p>
                  <hr class="mx-20 mt-2 border-wininfo-200" />
                  <button
                    class="w-20 border-1 border-wininfo-200 rounded-md m-2 hover:border-2 box-border px-[1px] py-[1px] hover:p-0 negative-push"
                    onclick={(event) => {
                      event.stopPropagation();
                      hiddeBubble();
                    }}
                  >
                    Ok
                  </button>
                </div>
              </svelte:fragment>
            </Speechbubble>
          {/if}
        </button>
      </div>
    </div>
    <p>
      {time.toLocaleTimeString("de-DE", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      })}
    </p>
  </div>
</div>

<style>
  @import "../ui95/assets/ui95.css";

  .negative-push:active {
    border: var(--color-wininfo-200) solid;
    border-right-width: 1px;
    border-left-width: 3px;
    border-bottom-width: 1px;
  }
</style>
