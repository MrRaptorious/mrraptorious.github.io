<script lang="ts">
    import "../app.css";
    import { base } from "$app/paths";
    import meta from "../lib/meta.json";
    import DesktopIcon from "$lib/components/ui95/desktop_icon.svelte";
    import OpenWindow from "$lib/components/ui95/openWindow.svelte";
    import type { OpenWindow_t } from "$lib/components/ui95/openWindow.svelte";
    import type { ItemData } from "$lib/types";

    // experiments
    import item_badge from "./experiments/spinnable_badge/item.js";
    import item_wasm from "./experiments/dottify/item.js";
    import item_mobility from "./experiments/mobility_models/item.js";
    import item_automaton from "./experiments/cellautomaton/item.js";
    import item_diffImage from "./experiments/diff_image/item.js";

    let experiments: ItemData[] = [
        item_badge,
        item_mobility,
        item_automaton,
        item_diffImage,
    ];
    let openWindows = new Map<number, OpenWindow_t>();

    let desktop: HTMLElement;
    let currentID = 0;

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

<div class="flex flex-col h-screen w-screen">
    <!-- desktop -->
    <div class="w-full h-full relative bg-blue-500" bind:this={desktop}>
        {#each openWindows.values() as openWindow}
            <OpenWindow
                id={openWindow.id}
                parent={desktop}
                width={openWindow.width}
                height={openWindow.height}
                px={openWindow.posX}
                py={openWindow.posY}
                displayData={openWindow.displayData}
                {closeWindow}
                {updateWindowPosition}
                {updateWindowSize}
            ></OpenWindow>
        {/each}

        <!-- icons  -->
        <div class="space-y-1">
            {#each experiments as experiment, index}
                <DesktopIcon
                    name={experiment.title}
                    image={experiment.title}
                    dblclick={() => doubleclick(experiment.id)}
                ></DesktopIcon>
            {/each}
        </div>
    </div>

    <footer class="bg-pink-500 text-text-500 mt-auto">
        <!-- start button -->
        <div class="w-50 bg-red-500">
            <p>Home button</p>
        </div>
        <!-- open windows -->
        <div></div>
    </footer>
</div>
