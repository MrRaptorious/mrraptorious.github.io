<script lang="ts">
  import { onMount } from "svelte";
  import { stutterFade } from "$lib/components/ui95/ui95Util";
  import { uiSettings } from "$lib/components/ui95/uiSettings.svelte.js";
  import StartmenuItem from "./startmenuItem.svelte";
  import ScreenSaver from "./screensaver.svelte";
  import type {
    DataManipulator,
    Program,
  } from "$lib/components/ui95/ui95types";
  import Speechbubble from "./speechbubble.svelte";

  import msg_information from "$lib/components/ui95/assets/msg_information.png";
  import start_icon from "$lib/components/ui95/assets/start_icon.png";
  import links_icon from "$lib/components/ui95/assets/links.ico";
  import programs_icon from "$lib/components/ui95/assets/programs.ico";
  import suspend_icon from "$lib/components/ui95/assets/suspend.ico";
  import documents_icon from "$lib/components/ui95/assets/documents.ico";
  import github_icon from "$lib/components/ui95/assets/githubPixelated.png";
  import linkedin_icon from "$lib/components/ui95/assets/linkedInPixelated.png";
  import forbidden_icon from "$lib/components/ui95/assets/forbidden.ico";
  import text_icon_smooth from "$lib/components/ui95/assets/font_smooth.png";
  import text_icon_pixelated from "$lib/components/ui95/assets/font_pixelated.png";

  interface HomebarProps {
    openPrograms: Program[];
    availablePrograms: Program[];
    dataManipulator: DataManipulator;
    openProgram: (toOpen: Program) => void;
  }
  let {
    openPrograms,
    availablePrograms,
    dataManipulator,
    openProgram,
  }: HomebarProps = $props();

  // ********** homebar *************
  let showStartMenu = $state(false);
  // svelte-ignore non_reactive_update
  let currentID = 1;
  let currentPath: number[] = $state([]);
  let screensaver = $state(false);

  // ********** info Bubble *************
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

  function closeStartMenu() {
    showStartMenu = false;
  }
</script>

<div
  class="text-text-500 mt-auto flex w-full p-[2px] box-border bg-winlightgray-500 space-x-2 z-9000 border-t-white border-3 h-12"
>
  <!-- start button -->
  <div class="w-25 text-black flex justify-center">
    <button
      class:button3d-inv={showStartMenu}
      class:button3d={!showStartMenu}
      class="w-25 text-black flex space-x-2 text-xl font-bold justify-center"
      onclick={(e) => {
        e.stopPropagation();
        if (showStartMenu) {
          showStartMenu = false;
          document.removeEventListener("click", closeStartMenu);
        } else {
          showStartMenu = true;
          document.addEventListener("click", closeStartMenu);
        }
      }}
    >
      <div class="flex w-full mx-1">
        <img
          src={start_icon}
          alt="svelte windows logo"
          class="[image-rendering:pixelated] m-[2px]"
        />
        <p
          class="flex h-full items-center text-center leading-0 font-bold text-2xl"
        >
          Start
        </p>
      </div>
    </button>

    {#if showStartMenu}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="absolute bottom-11 text-black border speech-bubble left-1 w-60 element3d flex flex-row bg-wingray-500"
        onclick={(e) => {
          e.stopPropagation();
        }}
      >
        <div
          class="[writing-mode:vertical-rl] flex w-10 bg-winblue-500 justify-end items-end text-white"
        >
          <p class="text-rotated text-3xl px-5">PortfoliOS</p>
        </div>
        <div class="grow">
          <StartmenuItem
            id={currentID++}
            bind:currentPath
            text={"Programs"}
            image={programs_icon}
            isFolder={true}
            height={55}
          >
            <div class="absolute" style="right: 0px; top: 0px;">
              <div class="absolute flex element3d flex-col min-w-50">
                {#each availablePrograms as program}
                  <StartmenuItem
                    id={currentID++}
                    text={program.title}
                    image={program.image}
                    bind:currentPath
                    height={30}
                    data={program.id}
                    clicked={() => {
                      openProgram(program);

                      showStartMenu = false;
                    }}
                    highlighted={true}
                  ></StartmenuItem>
                {/each}
              </div>
            </div>
          </StartmenuItem>

          <StartmenuItem
            id={currentID++}
            text={"Documents"}
            image={documents_icon}
            bind:currentPath
            isFolder={true}
            height={60}
          >
            <div class="absolute" style="right: 0px; top: 0px; ">
              <div class="absolute flex element3d flex-col min-w-50">
                <StartmenuItem
                  id={currentID++}
                  text={"Empty"}
                  image={forbidden_icon}
                  bind:currentPath
                  height={30}
                ></StartmenuItem>
              </div>
            </div>
          </StartmenuItem>

          <StartmenuItem
            id={currentID++}
            text={"Links"}
            image={links_icon}
            bind:currentPath
            isFolder={true}
            height={60}
          >
            <div class="absolute" style="right: 0px; top: 0px; ">
              <div class="absolute flex element3d flex-col min-w-50">
                <StartmenuItem
                  id={currentID++}
                  text={"GitHub"}
                  image={github_icon}
                  bind:currentPath
                  height={30}
                  data={"https://github.com/MrRaptorious"}
                  clicked={() => {
                    window
                      ?.open("https://github.com/MrRaptorious", "_blank")
                      ?.focus();
                    showStartMenu = false;
                  }}
                  highlighted={true}
                ></StartmenuItem>
                <StartmenuItem
                  id={currentID++}
                  text={"Linkedin"}
                  image={linkedin_icon}
                  bind:currentPath
                  height={30}
                  data={"https://www.linkedin.com/in/lorenz-braun-023284160/"}
                  clicked={() => {
                    window
                      ?.open(
                        "https://www.linkedin.com/in/lorenz-braun-023284160/",
                        "_blank",
                      )
                      ?.focus();
                    showStartMenu = false;
                  }}
                  highlighted={true}
                ></StartmenuItem>
              </div>
            </div>
          </StartmenuItem>

          <hr id="startmenuHR" />

          <StartmenuItem
            id={currentID++}
            text={"Suspend"}
            image={suspend_icon}
            bind:currentPath
            clicked={() => {
              screensaver = true;
            }}
            height={60}
          ></StartmenuItem>
        </div>
      </div>
    {/if}
  </div>

  <div class="vline h-full"></div>

  <!-- open windows -->
  <div class="flex h-full space-x-1 overflow-hidden flex-1 min-w-0">
    {#each openPrograms as program (program.id)}
      <button
        class="flex h-full flex-1 max-w-60 flex-row {program.windowData
          ?.hasfocus
          ? 'element3d-inv taskbarbutton-focused'
          : 'element3d taskbarbutton'}"
        onclick={() => {
          dataManipulator.setfocus(program.id);
        }}
      >
        <img
          src={program.image}
          class="p-1 h-full [image-rendering:pixelated]"
          alt={program.title}
        />
        <p
          class="flex items-center text-black h-full truncate text-[15pt] text-center leading-0 pb-[2px]"
        >
          {program.title}
        </p>
      </button>
    {/each}
  </div>

  <div class="vline h-full"></div>
  <!-- time -->
  <div
    class="px-4 h-full text-black flex content-center items-center justify-center space-x-3 element3d-inv"
    style="border-top-color:  var(--color-gray-500); border-left-color: var(--color-gray-500);"
  >
    <div class="flex h-full justify-center space-x-1 my-1">
      <button
        onclick={() => {
          uiSettings.fontPixelated = !uiSettings.fontPixelated;
        }}
        class="relative"
      >
        <img
          src={uiSettings.fontPixelated
            ? text_icon_smooth
            : text_icon_pixelated}
          alt="svelte windows logo"
          class="[image-rendering:pixelated] size-6"
        />
      </button>

      <div class="flex justify-center">
        <button
          onclick={() => {
            showBubble = true;
          }}
          class="relative"
        >
          <!-- Add 1px margin because icon does not look aligned even if it is -->
          <img
            src={msg_information}
            alt="svelte windows logo"
            class="[image-rendering:pixelated] size-8 mt-[1px]"
          />

          {#if showBubble}
            <div out:stutterFade>
              <Speechbubble
                right_offset={-40}
                right_offset_unit={"px"}
                time={1000 * 40}
                close={hiddeBubble}
              >
                <svelte:fragment>
                  <div>
                    <p class="text-left mx-1 whitespace-nowrap p-2">
                      <b>Hello!</b> <br />
                      This is my hobby project/portfolio where I show things I wanted
                      to explore or read about.<br />Feel free to look around.
                      <b> Double click</b> any Desktop Icon to see some
                      Interesting things! <br /> <br />
                      <b>Tipp</b>: If it's too hard to read the pixelated text
                      inside a window, click the [T] icon in the tray.
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
            </div>
          {/if}
        </button>
      </div>
    </div>
    <p class="text-[16pt] leading-0">
      {time.toLocaleTimeString("de-DE", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      })}
    </p>
  </div>
</div>
{#if screensaver}
  <div in:stutterFade out:stutterFade={{ delay: 200 }}>
    <ScreenSaver
      deactivate={() => {
        screensaver = false;
      }}
    ></ScreenSaver>
  </div>
{/if}

<style>
  @import "$lib/components/ui95/assets/ui95.css";

  .negative-push:active {
    border: var(--color-wininfo-200) solid;
    border-right-width: 1px;
    border-left-width: 3px;
    border-bottom-width: 1px;
  }
  .text-rotated {
    transform: rotate(180deg);
  }
  #startmenuHR {
    border: 2px solid transparent;
    border-top-color: var(--color-wingray-500);
    border-bottom-color: var(--color-winlightgray-200);
  }
</style>
