<script lang="ts">
    import "../app.css";

    import { MediaQuery } from "svelte/reactivity";

    // #################### Desktop Icons ####################
    import Desktopenvironment from "$lib/components/ui95/desktopenvironment.svelte";

    const large = new MediaQuery("min-width: 800px");
</script>

{#if large.current}
    <div class="w-screen h-screen overflow-hidden fixed">
        <Desktopenvironment></Desktopenvironment>
    </div>
{:else}
    <p>
        Sorry, this page is only available on desktop because you need a mouse!
    </p>
{/if}

<style>
    @import "../lib/fonts/font.css";

    * {
        font-family: "Windows95", sans-serif;
        font-weight: 400;
    }
</style>
