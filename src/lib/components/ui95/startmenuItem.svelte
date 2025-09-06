<script lang="ts">
  import { removeAfterElement } from "$lib/util";

  interface StartmenuItemProps {
    id: number;
    text: string;
    image: any;
    currentPath: number[];
    clicked?: (data: any) => void;
    highlighted?: boolean;
    isFolder?: boolean;
    height?: number;
    data?: any;
  }
  let {
    id,
    text,
    image,
    currentPath = $bindable(),
    clicked = () => {},
    highlighted = false,
    isFolder = false,
    height = 60,
    data = null,
  }: StartmenuItemProps = $props();

  let showChildren = false;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  id="item"
  class="relative"
  onmouseenter={() => {
    currentPath.push(id);
  }}
  onmouseleave={() => {
    currentPath = removeAfterElement(currentPath, id);
  }}
>
  <button
    class:startmenuitem-highlighted={currentPath.find((x) => x == id)}
    class="startmenuitem w-full bg-red-50 flex flex-row items-center px-2 text-left h-full space-x-3"
    style="height: {height}px;"
    onclick={(e) => {
      e.stopPropagation();
      console.log(text);
      clicked(data);
    }}
  >
    <!-- svelte-ignore a11y_missing_attribute -->
    <img
      src={image}
      class="[image-rendering:pixelated] select-none"
      style="height: {height * 0.7}px;"
    />

    <p class="grow text-lg">{text}</p>
    {#if isFolder}
      <div class="arrow-right absolute right-3"></div>
    {/if}

    <!-- svelte-ignore slot_element_deprecated -->
    {#if currentPath.find((x) => x == id)}
      <slot />
    {/if}
  </button>
</div>

<style>
  .startmenuitem {
    background: var(--color-winlightgray-500);
    color: black;
  }
  .startmenuitem-highlighted {
    background: var(--color-winblue-500);
    color: var(--color-winwhite-500);
  }

  .arrow-right {
    content: "";
    width: 0;
    height: 0;
    border-top: 7px solid transparent;
    border-bottom: 7px solid transparent;
    border-left: 8px solid black;
  }
</style>
