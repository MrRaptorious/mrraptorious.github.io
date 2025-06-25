<script>
  import "../app.css";
</script>


<div class="flex flex-col min-h-screen">

  <nav class="bg-crust-500 text-text-500 p-4">
      <ul class="flex space-x-4 justify-end">
          <li><a href="/" class="hover:underline">Home</a></li>
          <!-- <li><a href="/experiments" class="hover:underline">Experiments</a></li> -->
          <li><a href="https://github.com/MrRaptorious" class="hover:underline">GitHub</a></li>
      </ul>
  </nav>

  <main class="flex-1 p-4 bg-base-500 text-text-500">
      <slot />
  </main>

  <footer class="bg-mantle-500 text-text-500 p-4 text-center mt-auto">
      <p>I dont have any copyright.. so yeah nothing to see here</p>
  </footer>
</div>