<script lang="ts">
  import { onMount } from "svelte";
  import init, {
    slime_init,
    slime_set_sim_data,
    slime_step,
  } from "$lib/wasm_processor";

  let canvas: HTMLCanvasElement;
  let isAnimating = false;
  let simulationStep = 0;
  let ctx : CanvasRenderingContext2D | null;


  onMount(async () => {
    try {
      await init();
      console.log("WASM module loaded successfully!");
    } catch (e) {
      console.error("Failed to load WASM module:", e);
    }
  });


// Die Hauptanimations-Loop
function animate() {
    const data = slime_step();
    simulationStep++;

    const array = new Uint32Array(canvas.width * canvas.height);

    for (let i = 0; i < canvas.width * canvas.height; i++) {
        //const pointData = (255 << 24) | (data[i] << 16) | (data[i] << 8) | data[i];
        //const pointData = (255 << 24) | (data[i] << 16) | (data[i] << 8)| (data[i]);
        // array[i] = pointData;
        array[i] = 0x0;
    }

    const iData = new ImageData(new Uint8ClampedArray(array.buffer), canvas.width, canvas.height);
    
    ctx?.putImageData(iData, 0, 0);

    if (isAnimating) {
        requestAnimationFrame(animate);
    }
}


// Funktion zum Starten der Simulation
function startSimulation() {
    if (!isAnimating) {
        isAnimating = true;
        requestAnimationFrame(animate);
    }
    else 
      stopSimulation();
}

// Funktion zum Anhalten der Simulation
function stopSimulation() {
    isAnimating = false;
}

  function initRandom() {
    ctx = canvas.getContext('2d');
    slime_set_sim_data(canvas.width, canvas.height, 2, 10, 50);
    slime_init();
  }

</script>

<div class="flex flex-box flex-col">
  <div class="flex flex-box flex-col p-5">
    <h1 class="text-4xl">Cellular Automaton</h1>
    <p>This is a cellular Automaton</p>
    <button
      on:click={initRandom}
      class="w-100 py-2 px-4 rounded-md text-white font-semibold transition-colors bg-blue-600"
    >
      Init
    </button>
    <button
      on:click={startSimulation}
      class="w-100 py-2 px-4 rounded-md text-white font-semibold transition-colors bg-blue-600"
    >
      Start
    </button>
  </div>

  <canvas id="cell_canvas" bind:this={canvas} class="w-full h-full outline">
  </canvas>
</div>
