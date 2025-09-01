<script>
  let { name, image, dblclick } = $props();
</script>

<div class="w-25 h-25">
  <button id="outer" class="w-full h-full flex flex-col items-center justify-center" ondblclick={dblclick}>
    <p class="bg-pink-500 w-15 h-15 text-center">{image}</p>
    <p class="bg-pink-200">{name}</p>
  </button>
</div>

<style>
  #outer{
    border: black 1px solid ;
    border-style: none;
  }

  #outer:focus{
    border: black 1px solid ;
    border-style: dotted;
  }
  
</style>