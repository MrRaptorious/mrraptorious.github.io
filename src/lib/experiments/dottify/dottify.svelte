<script lang="ts">
  import { onMount } from "svelte";
  import init, { init_logging, dottify } from "$lib/wasm_processor";
  import Scrollbar from "$lib/components/ui95/scrollbar.svelte";

  let wasmLoaded = false;
  let file: File | null = null;
  let processedImageUrl: string | null = null;
  let error: string | null = null;
  let isLoading: boolean = false;
  let dotSize = 6;

  onMount(async () => {
    try {
      await init();
      wasmLoaded = true;
      console.log("WASM module loaded successfully!");
      init_logging();
    } catch (e) {
      console.error("Failed to load WASM module:", e);
      error = "Error while loading Wasm.";
    }
  });

  function handleFileUpload(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      file = target.files[0];
      processedImageUrl = null;
      error = null;
    }
  }

  async function processImageFromWASM() {
    if (!file) {
      error = "Choose Image first!";
      return;
    }
    if (!wasmLoaded) {
      error = "WASM-Modul not loaded yet";
      return;
    }

    isLoading = true;
    error = null;
    processedImageUrl = null;

    try {
      const arrayBuffer = await file.arrayBuffer();
      const byteArray = new Uint8Array(arrayBuffer);

      let outputFormat = "png";
      if (file.type === "image/jpeg") {
        outputFormat = "jpeg";
      } else if (file.type === "image/gif") {
        outputFormat = "gif";
      }

      console.time("dottify");
      const processedBytes = dottify(byteArray, outputFormat, dotSize);
      console.timeEnd("dottify");
      const blob = new Blob([processedBytes], {
        type: `image/${outputFormat}`,
      });
      processedImageUrl = URL.createObjectURL(blob);
    } catch (e: any) {
      error = `Error while working with the Image: ${e.message || e}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="flex mx-[1px]">
  <div class="min-h-screen p-8 font-sans space-y-10 overflow-hidden" >
    <h1 class="text-3xl font-bold text-center mb-8">Dottify Image</h1>

    <p class="">
      With image dottification you can create an stilistic image made from
      colorful dots spaced evenly on a grid, mimicing the original image.
    </p>
    <p class="">
      This is my first project ever working with WASM. Just try it out!
    </p>

    <div
      class="flex flex-col md:flex-col justify-center items-center gap-4 mb-8 p-6 rounded-lg shadow-md"
    >
      <div
        class="flex flex-col md:flex-row justify-center items-center gap-4 mb-8 p-6 rounded-lg shadow-md"
      >
        <h3 class="text-xl">Image:</h3>
        <input
          type="file"
          on:change={handleFileUpload}
          accept="image/png, image/jpeg, image/gif"
          class="py-2 px-4 border border-gray-300 rounded-md bg-gray-50 text-gray-700"
        />

        <h3 class="text-xl">DotSize:</h3>
        <input
          type="range"
          id="numberInput"
          bind:value={dotSize}
          on:change={processImageFromWASM}
          min="2"
          max="100"
          step="2"
          defaultValue="20"
        />
      </div>
      <button
        on:click={processImageFromWASM}
        disabled={!file || !wasmLoaded || isLoading}
        class="py-2 px-4 rounded-md text-white font-semibold
             transition-colors duration-200 ease-in-out
             {!file || !wasmLoaded || isLoading
          ? 'bg-gray-400 cursor-not-allowed'
          : 'bg-blue-600 hover:bg-blue-700'}"
      >
        {#if isLoading}
          Editing Image...
        {:else if !wasmLoaded}
          Load WASM...
        {:else if !file}
          Coose Image first!
        {:else}
          Dottify Image!
        {/if}
      </button>
    </div>
    {#if error}
      <p
        class="text-red-700 bg-red-100 border border-red-400 rounded-md p-3 text-center mb-4"
      >
        {error}
      </p>
    {/if}

    {#if processedImageUrl}
      <h2 class="text-2xl font-semibold text-center mb-4">Your Image:</h2>
      <div class="flex justify-center">
        <!-- svelte-ignore a11y_img_redundant_alt -->
        <img
          src={processedImageUrl}
          alt="Dottified Image"
          class="max-w-full h-auto border border-gray-300 rounded-lg shadow-lg"
        />
      </div>
    {/if}
  </div>
</main>
