<script lang="ts">
    import init, { add_num } from '$lib/wasm_processor';

    let wasmLoaded = false;
    let result: number | null = null; 

    import { onMount } from 'svelte';
    onMount(async () => {
        try {
            await init(); 
            wasmLoaded = true; 
            console.log('WASM module loaded successfully!');
        } catch (e) {
            console.error('Failed to load WASM module:', e);
        }
    });

    function calculateSum() {
        if (wasmLoaded) {
            result = add_num(5, 7);
        } else {
            console.warn('WASM module not loaded yet.');
            result = null;
        }
    }
</script>

<h1>HELLO WASM</h1>

<button on:click={calculateSum} disabled={!wasmLoaded}>
    {wasmLoaded ? 'Addieren (5 + 7 via WASM)' : 'Lade WASM...'}
</button>

{#if result !== null}
    <p>Ergebnis der WASM-Addition: <strong>{result}</strong></p>
{/if}