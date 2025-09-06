<script lang="ts">
  import type { Program } from "./ui95types";
  import DesktopIcon from "./desktop_icon.svelte";

  interface DesktopProps {
    availablePrograms: Program[];
    openProgram: (toOpen: Program) => void;
  }

  let { availablePrograms, openProgram }: DesktopProps = $props();
</script>

<div class="w-full h-full flex bg-winteal-500">
  <div class="space-y-1 p-10">
    {#each availablePrograms as program}
      <DesktopIcon
        name={program.title}
        image={program.image}
        dblclick={() => openProgram(program)}
      ></DesktopIcon>
    {/each}
  </div>
</div>
