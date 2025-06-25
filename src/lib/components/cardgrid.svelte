<script>
    import Card from "./card.svelte";
    let cardInfos = [
        {image:"/placeholder_image.svg", title:"WebGL", subtitle:"subtitle test", tags:["GenArt","webGL"], link:"experiments/webgl"},
        {image:"/placeholder_image.svg", title:"Meine Karte", subtitle:"subtitle test", tags:["GenArt","webGL"], link:""},
    ];

</script>

<div class="flex">
{#each cardInfos as cardInfo }
<div class="flex m-3 hover:scale-110 ease-in-out duration-300" on:click={() => {location.href=cardInfo.link}} role="button" tabindex=0 on:keypress={()=>{}}>
<Card image="{cardInfo.image}" title="{cardInfo.title}" subtitle="{cardInfo.subtitle}" tags="{cardInfo.tags}" link="{cardInfo.link}"/>
</div>    
{/each}
</div>
