<script>
  let { name, image, dblclick } = $props();
</script>

<div class="w-25 h-25">
  <button id="outer" class="w-full h-full flex flex-col items-center justify-center" ondblclick={dblclick}>
    <img src={image} class="w-14 h-14 [image-rendering:pixelated]" alt="{name}">
    <p class="leading-none">{name}</p>
  </button>
</div>

<style>
  #outer{
    border: black 1px solid ;
    border-style: none;
  }

  #outer:focus{
    border: black 2px solid ;
    border-style: dotted;
  }
  
</style>