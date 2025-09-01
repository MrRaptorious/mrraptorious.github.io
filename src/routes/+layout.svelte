<script>
  import "../app.css";
  import { base } from '$app/paths';
  import meta from "../lib/meta.json" 
  
</script>


<div class="flex flex-col min-h-screen">

  <nav class="bg-crust-500 text-text-500 p-4 flex justify-between">
    <div class="flex flex-col items-center">
      <a href="{base}/">
        <img class="rounded-full object-scale-down w-15" src="{base}/profile.png" alt="profile">
      </a>
    </div>
      <ul class="flex space-x-4 justify-end">

      <a href="https://github.com/MrRaptorious">
        <img class="object-scale-down w-15" src="{base}/github-mark-white.png" alt="profile">
      </a>
      </ul>
    </nav>

  <main class="flex-1 p-4 bg-base-500 text-text-500">
      <slot />
  </main>

  <footer class="bg-mantle-500 text-text-500 p-4 text-center mt-auto">
      <p>Version {meta.version}</p>
  </footer>
</div>