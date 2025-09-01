<script lang="ts">
  import { onMount } from "svelte";

  let videoElement: HTMLVideoElement;
  let canvasElement: HTMLCanvasElement;
  let latestImage: string = '';
  let allowCamera = false;

  onMount(() => {
    // Kamerazugriff anfordern
    navigator.mediaDevices.getUserMedia({ video: true })
      .then(stream => {
        // Stream dem Video-Element zuweisen
        //allowCamera = true;
        videoElement.srcObject = stream;
      })
      .catch(err => {
        console.error("Fehler beim Zugriff auf die Kamera: ", err);
      });
  });

    function captureImage() {}

</script>


<main>
  <h2>Kamera</h2>
  <div class="flex flex-wrap">
    {#if allowCamera}
      <video bind:this={videoElement} autoplay muted></video>
    {:else}
    <div class="max-w-lg mx-auto p-8 bg-red-100 text-red-800 border border-red-200 rounded-lg text-center">
      <p>Bitte erlaube den Zugriff auf die Kamera, um diese Funktion zu nutzen. Lade die Seite neu, nachdem du die Berechtigung erteilt hast.</p>
    </div>
    {/if}
    <canvas bind:this={canvasElement} style="display: none;"></canvas>
    <h1>test</h1>
  </div>

  <div class="container flex flex-row w-full">
    <div>
      <h3>Camera Feed</h3>
    </div>

    <div>
      <h3>Motion Feed</h3>
    </div>
  </div>

  <br>
  <button on:click={captureImage}>Foto aufnehmen</button>

  <br>
  {#if latestImage}
    <h2>Aktuellstes Foto</h2>
    <img src={latestImage} alt="Captured from camera" />
  {/if}

</main>