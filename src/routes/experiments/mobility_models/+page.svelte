<script lang="ts">
  import RandomDirection from "./random_direction.svelte";
    import RandomWalk from "./random_walk.svelte";

</script>

<div class="min-h-screen bg-base-500 p-4 flex flex-col space-y-10">
  <h1
    class="text-4xl font-bold text-center text-text mb-8 rounded-lg p-4 shadow-md"
  >
    MobilityModels
  </h1>

  <p>
    Mobility models are used in different scenarios. I learned about them in a course on mobile communication. They were used to model the behavior of mobile clients. With concrete movement, it's possible to simulate things like delay-tolerant routing protocols or handover management.
  <br/>
    This page shows two different, very simple mobility models to play with.
  <br>
  For more information look at the original <a  class="color-pink-500 dark:text-gray-400" href="https://www.researchgate.net/publication/2563707_A_Survey_of_Mobility_Models_for_Ad_Hoc_Network_Research">Paper</a>.
  </p>


  <br>

  <h2 class="text-3xl font-semibold text-subtext1 mb-4">Random Walk</h2>
  <p>
    The random walk is one of the simpler models to simulate mobility.
    Its goal is to simulate random movement of the simulated nodes.
  <br/>
  The idea is that each node chooses a random direction from <b>[0, 360]</b> degrees and a random speed from <b>[speed_min, speed_max]</b>. Then each node moves for a fixed distance or, in this case, time. If the node hits a wall, it simply reflects."
  </p>
  <br>
  <RandomWalk></RandomWalk>

  <h2 class="text-3xl font-semibold text-subtext1 mb-4">Random Direction</h2>
  <p>
  The random direction model is also a fairly simple mobility model.
  <br>
  The first step is to choose a direction to head in. Then the node will move until it hits a wall. After a small pause at the wall, a new direction is chosen. A node can choose any direction as long as it's away from the wall."
  </p>
  <RandomDirection></RandomDirection>

</div>
