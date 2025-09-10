<script lang="ts">
    import "../app.css";
    import { MediaQuery } from "svelte/reactivity";
    import { page } from "$app/state";

    const keys: string[] = [...page.url.searchParams.keys()];
    console.log(keys);

    // #################### Desktop Icons ####################
    import Desktopenvironment from "$lib/components/ui95/desktopenvironment.svelte";

    const large = new MediaQuery("min-width: 800px");
</script>

{#if large.current}
    <div class="w-screen h-screen overflow-hidden fixed">
        <Desktopenvironment toOpen={keys}></Desktopenvironment>
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
