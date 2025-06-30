<script>
  let {image, title, subtitle, tags, link} = $props();
</script>


<div class="max-w-sm rounded overflow-hidden shadow-lg, bg-surface0-500 ">
    <img class="w-full object-cover" src="{image}" style="width: 300px; height: 200px;" alt="Experiment">
    <div class="px-6 py-4">
      <div class="font-bold text-xl mb-2">{title}</div>
      <p class="text-subtext2-500">{subtitle}</p>
    </div>
    <div class="px-6 pt-4 pb-2">
      {#each tags as tag}
      <span class="inline-block bg-base-500 rounded-full px-3 py-1 text-sm font-semibold text-text mr-2 mb-2">{tag}</span>
      {/each}
    </div>
  </div>