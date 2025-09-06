<script lang="ts">
    import "../app.css";
    import { base } from "$app/paths";
    import { MediaQuery } from "svelte/reactivity";
    import meta from "../lib/meta.json";
    import DesktopIcon from "$lib/components/ui95/desktop_icon.svelte";
    import OpenWindow from "$lib/components/ui95/openWindow.svelte";
    import Homebar from "$lib/components/ui95/homebar.svelte";
    import type { OpenWindow_t } from "$lib/components/ui95/openWindow.svelte";
    import type { ItemData } from "$lib/types";

    // #################### Desktop Icons ####################
    import item_badge from "$lib/experiments/spinnable_badge/item.js";
    import item_wasm from "$lib/experiments/dottify/item.js";
    import item_mobility from "$lib/experiments/mobility_models/item.js";
    import item_automaton from "$lib/experiments/cellautomaton/item.js";
    import item_diffImage from "$lib/experiments/diff_image/item.js";
    import item_dottify from "$lib/experiments/dottify/item.js";
    import Desktopenvironment from "$lib/components/ui95/desktopenvironment.svelte";

    // #################### Vars ####################
    let experiments: ItemData[] = [
        //item_badge,
        item_dottify,
        item_mobility,
        //item_automaton,
        //item_diffImage,
    ];

    let openWindows = new Map<number, OpenWindow_t>();
    let desktop: HTMLElement;
    let currentID = 0;
    const large = new MediaQuery("min-width: 800px");

    // #################### Functions ####################
    function getRandomInt(max: number) {
        return Math.floor(Math.random() * max);
    }

    function doubleclick(id: string) {
        const windowWith = 800;
        const windowHeight = 600;

        const newWindow: OpenWindow_t = {
            id: currentID++,
            height: windowHeight,
            width: windowWith,
            posX: getRandomInt(desktop.clientWidth - windowWith),
            posY: getRandomInt(desktop.clientHeight - windowHeight),
            displayData: experiments.filter((x) => x.id == id)[0],
        };
        openWindows.set(newWindow.id, newWindow);

        openWindows = openWindows;
    }

    function closeWindow(id: number) {
        if (openWindows.has(id)) {
            openWindows.delete(id);
        }
        openWindows = openWindows;
    }

    function updateWindowPosition(id: number, px: number, py: number) {
        const openWindow = openWindows.get(id);

        if (openWindow) {
            openWindow.posX = px;
            openWindow.posY = py;
        }
    }

    function updateWindowSize(id: number, width: number, height: number) {
        const openWindow = openWindows.get(id);

        if (openWindow) {
            openWindow.width = width;
            openWindow.height = height;
        }
    }
</script>

{#if large.current}
    <div class="w-screen h-screen">
        <Desktopenvironment></Desktopenvironment>
    </div>
{:else}
    <p>This page is only available on desktop</p>
{/if}

<style>
    @import "../lib/fonts/font.css";

    * {
        font-family: "Windows95", sans-serif;
        font-weight: 400;
    }
</style>
