<script>
  let { name, image, dblclick, isSelected, element = $bindable() } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="w-25 h-25"
  onmousedown={(e) => {
    //e.stopPropagation();
  }}
>
  <button
    id="outer"
    bind:this={element}
    class:active-selection={isSelected}
    class="w-full h-full flex flex-col items-center justify-center"
    ondblclick={dblclick}
  >
    <img
      src={image}
      class="w-14 h-14 [image-rendering:pixelated] select-none"
      ondragstart={(e) => e.preventDefault()}
      alt={name}
    />
    <p class="leading-none">{name}</p>
  </button>
</div>

<style>
  butten {
    border: black 1px solid;
    border-style: none;
  }

  button:focus {
    border: black 2px solid;
    border-style: dotted;
  }

  .active-selection {
    border: black 2px solid;
    border-style: dotted;
  }
</style>
