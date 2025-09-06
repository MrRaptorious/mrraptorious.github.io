<script lang="ts">
  interface ScreenSaverProps {
    deactivate: () => void;
  }

  let { deactivate } = $props();
</script>

<div class="fullscreen" onclick={() => deactivate()}></div>

<style>
  .fullscreen {
    background-color: black;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    /* Stellt sicher, dass es über allen anderen Elementen liegt */
    z-index: 99999;
  }
</style>
