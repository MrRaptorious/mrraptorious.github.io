<script lang="ts">
  import { onMount } from "svelte";
  import init, { init_logging, dottify } from "$lib/wasm_processor";
  import banner from "./cenna_banner.png";

  // svelte-ignore non_reactive_update
  let wasmLoaded = $state(false);
  // svelte-ignore non_reactive_update
  let file: File | null = $state(null);
  // svelte-ignore non_reactive_update
  let processedImageUrl: string | null = $state(null);
  // svelte-ignore non_reactive_update
  let error: string | null = $state(null);
  // svelte-ignore non_reactive_update
  let isLoading: boolean = $state(false);
  // svelte-ignore non_reactive_update
  let dotSize = $state(6);
  let dotPadding = $state(1);

  // hovering effect
  let hoveredNumber: number | null = $state(null);
  const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  const xIndex = $derived(
    hoveredNumber !== null ? Math.floor((hoveredNumber - 1) / 3) : null,
  );
  const yIndex = $derived(
    hoveredNumber !== null ? (hoveredNumber - 1) % 3 : null,
  );

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
      const processedBytes = dottify(
        byteArray,
        outputFormat,
        dotSize,
        dotPadding,
      );
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
  <div class="min-h-screen p-8 space-y-10 overflow-hidden">
    <!-- TEXT -->
    <div class="space-y-10">
      <img src={banner} />
      <h1 class="text-3xl font-bold text-center mb-8">Image dottification</h1>
      <section>
        <!-- prettier-ignore -->
        <p class="">
          The other day I saw a <a
            class="link"
            href="https://www.youtube.com/watch?v=YOwOAG3OQk8">Youtube video</a
          >
          in which the creator explained how to "Turn a Photo into a Color Dot Pattern
          in Photoshop".
          The visuals caught my eye, but eight and a half minutes for
          each image?
          (I am clearly ignoring the possibility to create a Photoshop
          template in which you only have to swap images and have <b>many</b>
          more possibilities for image compositing right at your fingertips).
          <br/>
          So what's hindering me from creating a much worse and less feature-rich
          pice of software in ten times the time I would need to buy, install
          and use Photoshop?
          Thats right, nothing! So let's get started and learn something.
        </p>
        <!-- prettier-ignore -->
        <p>
          I wanted to learn something new and heard of this magical
          &#10024;WASM&#10024; which I wanted to try out.
          <a
            class="link text-xl"
            href="#fn01"
            role="">*</a
          >
          <br />
          The only thing I knew was that WASM is compiled code running in the browser
          and that Rust can compile to WASM.
          I won't go into detail on how to set up WASM and how to use it because there
          are way better tutorials from people who know Rust and/or WebAssembly.
          I will only talk shortly about the idea that I had regarding the
          the structure of the (in the end quite small) Rust implementation.
          After that I will bore you with some superficial theory about image processing.
        </p>
      </section>

      <div>
        <ol>
          <li id="fn01" role="doc-footnote">
            <a class="text-xl" role="doc-backlink" href="#fnref01">*</a>
            WASM is not *actually* magical. Nothing is. Except perhaps Unicorns,
            love and CPU branch prediction.
          </li>
        </ol>
      </div>

      <section>
        <h2 class="text-xl">The Idea</h2>
        <!-- prettier-ignore -->
        <p>
          We want to "lower" the image resolution by creating virtual round pixels (which actually consist of many normal pixels).
          The color of those virtual round pixels corresponds to the average color of a corresponding region in
          the original image.
          Theoretically the shape is not restricted to circles.
          In fact, there are
          <a
            class="link"
            href="https://www.imgonline.com.ua/eng/dot-pattern-effect.php"
            >online tools</a
          > to create this effect with different shapes, for example, hearts.
          With that, the program structure should be as follows: 
        </p>

        <br />

        <ol>
          <li>
            1. Create a grid corresponding to the virtual pixels to save the
            color
          </li>
          <li>
            2. Sample the original image evenly at each location in a way that
            the final image looks good
          </li>
          <li>3. Create a new image with the correct dimensions</li>
          <li>
            4. Draw the desired shape (here only circles) side by side into the
            new image
          </li>
        </ol>

        <br />

        <p>
          That's all! With that, you can reproduce the desired dotting effect.
          The following is some very short surface-level theory on data
          representation and image operations which I encountered while reading
          about image manipulation. If you are not interested in this, jump
          straight to the dottificator!
        </p>

        <br />
        <button
          class="h-20 w-60 element3d button3d"
          onclick={() => {
            document.getElementById("dottificator").scrollIntoView();
          }}
        >
          Go to dottificator!
        </button>
      </section>
      <section>
        <h2 class="text-xl">Some theory - Data Representation</h2>
        <!-- prettier-ignore -->
        <p>
          An image consists of an 2D-Array of pixels.
          Often when working with images you are using an abstraction that encapsulates an image into
          an object or a struct with different methods or functions to act on
          the image data.
          Because of this the underlying representation of the data doesn't matter (most of the time).
          If you are building a high performance, perhaps real-time, image processing application or library
          like <a class="link" href="https://opencv.org/">opencv</a> those things
          do matter, but just doing some transformations on an image to make it &#10024;pretty&#10024;
          and beat a time of eight and a half minutes you surely should not care
          about underlying implementations.<br>
          But, ignoring my own advice:<br>
          Storing and accessing data has a major impact on performance.
          Popular data-driven libraries such as <a class="link" href="https://numpy.org/doc/stable/reference/arrays.ndarray.html#internal-memory-layout-of-an-ndarray">numpy</a> often store data in a single contiguous block of memory, an array.
          Additonaly information about how to access the data is saved.
          In the case of numpy it's saved as so called strides.
          
          Let's say we have an 3x3 image saved in a numpy array of shape (3, 3).
          The actual data is still stored in a single contiguous block.
          The following demonstration shows exactly that.
        </p>
        <br />
        <div class="w-full flex items-center content-center">
          <div
            class="w-full flex flex-col md:flex-row items-center justify-center p-8 md:p-12"
          >
            <div
              class="w-full max-w-5xl flex flex-col md:flex-row items-stretch justify-center gap-6 md:gap-12"
            >
              <div class="grow bg-blue-500 p-6 md:p-8 rounded-lg shadow-lg">
                <h4 class="text-white text-xl md:text-2xl font-bold mb-4">
                  Abstraction
                </h4>
                <p class="text-white text-sm mb-4">
                  The logical view of the data (as a grid).
                </p>
                <div class="grid grid-cols-3 gap-1 w-fit mx-auto">
                  {#each numbers as number}
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      class="w-15 h-15 bg-blue-300 text-center p-4 text-black font-bold text-lg rounded-md transition-colors duration-200 hover:bg-yellow-300"
                      onmouseenter={() => (hoveredNumber = number)}
                      onmouseleave={() => (hoveredNumber = null)}
                    >
                      {number}
                    </div>
                  {/each}
                </div>
              </div>

              <div class="grow bg-red-500 p-6 md:p-8 rounded-lg shadow-lg">
                <h4 class="text-white text-xl md:text-2xl font-bold mb-4">
                  Memory
                </h4>
                <p class="text-white text-sm mb-4">
                  The physical view of the data (as a linear array).
                </p>
                <div
                  class="flex flex-wrap items-center justify-center p-4 bg-red-300 rounded-md"
                >
                  {#each numbers as number}
                    <div
                      class="w-15 h-15 text-center p-4 text-black font-bold text-lg rounded-md transition-colors duration-200 hover:bg-yellow-300"
                      class:bg-yellow-300={number === hoveredNumber}
                      class:rounded-md={number === hoveredNumber}
                    >
                      {number}{#if number < 9},&nbsp;{/if}
                    </div>
                  {/each}
                </div>
                <div class="items-center justify-center flex flex-col text-xl">
                  <div class="grid grid-cols-2 w-full">
                    <div class="p-2 text-right">Stride :</div>
                    <div class="p-2">[3, 1]</div>

                    <div class="p-2 text-right">Index :</div>
                    <div class="p-2">
                      {hoveredNumber ? `[${xIndex}, ${yIndex}]` : ""}
                    </div>

                    <div class="p-2 text-right">Offset :</div>
                    <div class="p-2">
                      {hoveredNumber
                        ? `${yIndex + 3 * xIndex} = (3 x ${xIndex}) + (1 x ${yIndex})`
                        : ""}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section>
        <h2 class="text-xl">Some theory - Image Operations</h2>
        <p>
          In computervision there are many different operations to apply to an
          image. Those operations can be categorized in three different
          categories which each have different effects on performance:
        </p>
        <br />
        <ul class="space-y-3">
          <li>
            <b>Point operations:</b> Those operations act on only a single pixel,
            for example inverting an image is considered an point operation because
            each Pixel is mapped to its inverse value.
          </li>
          <li>
            <b>Local operations:</b> Those act on a very local subset of pixels.
            For example convolution in a CNN is local because each pixel is mapped
            to a weighted average of pixels around itself.
          </li>
          <li>
            <b>Global operations:</b> Image wide transformations like scaling, rotation
            or translation are called global because each operation acts on all pixels
            of the image.
          </li>
        </ul>
        <br />

        <p>
          Thus the operations necessar for the dottification are local
          operations. For every virtual pixel a corresponding pixel neighborhood
          in the original image is checked.
        </p>
      </section>
    </div>

    <hr />

    <div
      class="flex flex-col md:flex-col justify-center items-center gap-4 mb-8 p-6 rounded-lg shadow-md"
    >
      <h1 id="dottificator" class="text-3xl font-bold text-center mb-8">
        The Dottificator
      </h1>

      <div
        class="flex flex-col md:flex-row justify-center items-center gap-4 mb-8 p-6 rounded-lg shadow-md"
      >
        <h3 class="text-xl">Image:</h3>
        <input
          type="file"
          onchange={handleFileUpload}
          accept="image/png, image/jpeg, image/gif"
          class="py-2 px-4 border border-gray-300 rounded-md bg-gray-50 text-gray-700"
        />

        <h3 class="text-xl">DotSize:</h3>
        <input
          type="range"
          id="numberInput"
          bind:value={dotSize}
          min="2"
          max="100"
          step="2"
          defaultValue="20"
        />
        <h3 class="text-xl">Dot Padding:</h3>
        <input
          type="range"
          id="numberInput"
          bind:value={dotPadding}
          min="0"
          max="10"
          step="1"
          defaultValue="1"
        />
      </div>
      <button
        onclick={processImageFromWASM}
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

<style>
  @import "$lib/components/ui95/assets/ui95.css";

  h2 {
    font-weight: bold;
  }
</style>
