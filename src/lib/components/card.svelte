<script>
  let {image, title, subtitle, tags, link} = $props();
</script>


<div class="max-w-sm rounded overflow-hidden shadow-lg, bg-surface0-500 w-100 ">
  <img class="w-full h-60 object-cover" src="{image}" alt="Experiment {title}">
  <div class="px-6 py-4">
    <div class="font-bold text-xl mb-2">{title}</div>
    <p class="text-subtext2-500 line-clamp-2"> {subtitle}
    </p>
  </div>
  <div class="px-6 pt-4 pb-2">
    {#each tags as tag}
    <span class="inline-block bg-base-500 rounded-full px-3 py-1 text-sm font-semibold text-text mr-2 mb-2">{tag}</span>
    {/each}
  </div>
</div>

<style>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  /* NEU: Umbruch bei langen Wörtern */
  word-break: break-word; /* Oder word-wrap: break-word; (älterer Standard) */
}
</style>