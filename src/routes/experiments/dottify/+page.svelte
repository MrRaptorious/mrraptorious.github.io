<script lang="ts">
  import { onMount } from "svelte";
  import init, { gray_image, init_logging, dottify } from "$lib/wasm_processor";

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
      error = "Fehler beim Laden des WASM-Moduls.";
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

      console.time('dottify')
      const processedBytes = dottify(byteArray, outputFormat, dotSize);
      console.timeEnd('dottify')
      const blob = new Blob([processedBytes], {
        type: `image/${outputFormat}`,
      });
      processedImageUrl = URL.createObjectURL(blob);

    } catch (e: any) {
      error = `Fehler bei der Bildverarbeitung: ${e.message || e}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="min-h-screen bg-gray-100 p-8 font-sans">
  <h1 class="text-3xl font-bold text-gray-800 text-center mb-8">
    SvelteKit & Rust WASM Bildverarbeitung
  </h1>

  <div
    class="flex flex-col md:flex-row justify-center items-center gap-4 mb-8 p-6 bg-white rounded-lg shadow-md"
  >
    <input
      type="file"
      on:change={handleFileUpload}
      accept="image/png, image/jpeg, image/gif"
      class="py-2 px-4 border border-gray-300 rounded-md bg-gray-50 text-gray-700"
    />

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
        Edited Image via Wasm!
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
    <h2 class="text-2xl font-semibold text-gray-700 text-center mb-4">
      Your Image:
    </h2>
    <div class="flex justify-center">
      <img
        src={processedImageUrl}
        alt="Verarbeitetes Bild von WASM"
        class="max-w-full h-auto border border-gray-300 rounded-lg shadow-lg"
      />
    </div>
  {/if}
</div>
