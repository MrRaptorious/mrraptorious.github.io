<script>
    import { base } from "$app/paths";
    import item_badge from "./experiments/spinnable_badge/item.js"
    import item_wasm from "./experiments/wasm_test/item.js"
    import Card from "$lib/components/card.svelte";
    let cardInfos = [
        item_badge,
        item_wasm,
    ];
    
    cardInfos.forEach(e => {
       console.log(e.link);
    });
</script>

<div class="flex flex-col m-5 space-y-10">
    <h1 class="text-5xl">Welcome to my page!</h1>
    <p>
        This is my website where I show my fun creations I made for the sake of
        learning and being interested in those technologies (and mostly just for
        fun!)
    </p>

    <div class="flex">
        {#each cardInfos as cardInfo}
            <div
                class="flex m-3 hover:scale-110 ease-in-out duration-300"
                on:click={() => {
                    location.href = cardInfo.link;
                }}
                role="button"
                tabindex="0"
                on:keypress={() => {}}
            >
                <Card
                    image={cardInfo.image}
                    title={cardInfo.title}
                    subtitle={cardInfo.subtitle}
                    tags={cardInfo.tags}
                    link={cardInfo.link}
                />
            </div>
        {/each}
    </div>
</div>
