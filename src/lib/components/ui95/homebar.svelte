<script lang="ts">
  import {onMount} from 'svelte';
  import start_icon from '$lib/components/ui95/assets/start_icon.png'

  let time = $state(new Date());

  onMount(()=>{
    const interval = setInterval(() => {
      time = new Date();
    }, 1000 * 60);

    return () => {
      clearInterval(interval);
    }
  });
  
</script>

<div id="bar" class="text-text-500 mt-auto flex w-full p-1 box-border">
  <!-- start button -->
  <button class="w-25 text-black flex space-x-2 button3d text-xl font-bold justify-center">
    <img src="{start_icon}" alt="svelte windows logo" class="[image-rendering:pixelated] p-[3px]">
    <p>Home</p>
  </button>

  <!-- open windows -->
  <div class="flex-1 h-full"></div>

  <!-- time -->
  <div class="w-15 text-black flex content-center items-center justify-center button3dInvert">
    <p>{time.toLocaleTimeString('de-DE', {hour: '2-digit', minute: '2-digit', hour12: false})}</p>
  </div>
</div>

<style>
  @import '../ui95/assets/ui95.css';
  #bar {
    background-color: #c3c3c3
  }
</style>