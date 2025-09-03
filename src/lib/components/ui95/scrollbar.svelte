<script lang="ts">
  import {clamp} from '$lib/util'
  let {scrollCallback, grabberSize} : {scrollCallback:(fraction:number)=> void, grabberSize:number} = $props();

  let mouseDown = $state(false);
  let posY = $state(0);
  let offset = $state(0);
  let container: HTMLDivElement;
  let grabber: HTMLButtonElement;

  function handleMouseDown(event: MouseEvent) {
    offset = event.clientY;
    mouseDown = true;
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  }

  function handleMouseMove(event: MouseEvent) {
    if (mouseDown) {
      const boundingboxParent = container.getBoundingClientRect();
      const boundingboxGrabber = grabber.getBoundingClientRect();
      posY = clamp(0, boundingboxParent.height - boundingboxGrabber.height, event.clientY - boundingboxParent.top);

      let relative = posY / (boundingboxParent.height - grabberSize);
      scrollCallback(relative);
    }
  }

  function handleMouseUp() {
    mouseDown = false;
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  }
</script>


<div class="flex flex-col w-full h-full ditherd">
  <!-- Arrow Top -->
  <!-- svelte-ignore a11y_consider_explicit_label -->
  <button class="button3d h-5 flex justify-center items-center">
    <div class="upbutton"></div>
  </button>

  <!-- mitddle -->
  <div class="flex-1" bind:this={container}>
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button
      bind:this={grabber}
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      onmouseup={handleMouseUp}
      class="w-full"
      style="position: relative;
      top: {posY}px;
      border-left: 1px solid white;
      border-top: 1px solid white;
      border-right: 1px solid black;
      border-bottom: 1px solid black;
      height: {grabberSize}px;
      background-color: #d4d4d4"></button
    >
  </div>

  <!-- Arrow Bottom -->
  <!-- svelte-ignore a11y_consider_explicit_label -->
  <button class="button3d h-5 flex justify-center items-center">
    <div class="downbutton"></div>
  </button>
</div>

<style>
  .ditherd {
    background-image: repeating-conic-gradient(
      from 0deg at 50% 50%,
      #ddd 0deg 90deg,
      #aaa 90deg 180deg
    );
    background-size: 4px 4px;
  }

  .upbutton {
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;

    border-bottom: 5px solid black;
  }

  .downbutton {
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;

    border-top: 5px solid black;
  }
</style>
