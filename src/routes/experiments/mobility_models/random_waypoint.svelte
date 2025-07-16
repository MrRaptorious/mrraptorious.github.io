<script lang="ts">
  import { onMount } from "svelte";
  import init, {
    init_simulation,
    update_simulation,
  } from "../../../lib/wasm_processor";

  let rd_canvas: HTMLCanvasElement | null = null;
  let rd_numAgents = 20;
  let rd_speed = 50;
  let rd_radius = 50;
  let rd_animationFrame: number | null = null;
  let rd_running = false;
  let rd_started = false;

  function resizeCanvas() {
    if (rd_canvas) {
      // Setze die Canvas-Attribute auf die tatsächliche Pixelgröße
      const rect = rd_canvas.getBoundingClientRect();
      rd_canvas.width = rect.width;
      rd_canvas.height = rect.height;
    }
  }

  function rd_draw(agents: any[]) {
    if (!rd_canvas) return;
    const ctx = rd_canvas.getContext("2d");
    if (!ctx) return;
    ctx.clearRect(0, 0, rd_canvas.width, rd_canvas.height);

    rd_drawAgentConnections(ctx, agents[0], agents);
    rd_drawAgents(ctx, agents);
  }

  function rd_drawAgents(ctx: CanvasRenderingContext2D, agents: any[]) {
    ctx.fillStyle = "#000000"; // teal-500 fallback

    for (const agent of agents) {
      ctx.beginPath();
      ctx.arc(agent.x, agent.y, 8, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  function rd_drawAgentConnections(
    ctx: CanvasRenderingContext2D,
    focus: any,
    agents: any[],
  ) {
    // always draw radius around focused
    ctx.fillStyle = "#0000000F";
    ctx.strokeStyle = "#FFFFFF00";
    ctx.beginPath();
    ctx.arc(focus.x, focus.y, rd_radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();

    ctx.strokeStyle = "#FFFFFFFF";

    for (const agent of agents) {
      let x = focus.x - agent.x;
      let y = focus.y - agent.y;
      let d = Math.sqrt(x * x + y * y);

      // draw connections based on radius
      if (d < rd_radius && d != 0) {
        ctx.moveTo(focus.x, focus.y);
        ctx.lineTo(agent.x, agent.y);
      }
    }
    ctx.stroke();
  }

  async function rd_toggleSimulation() {
    if (!rd_started) {
      if (!rd_canvas) return;
      resizeCanvas();
      init_simulation(
        rd_numAgents,
        rd_canvas.width,
        rd_canvas.height,
        rd_speed / 10.0,
      );

      rd_started = true;
    }

    rd_running = !rd_running;

    if (rd_running) {
      function frame() {
        const agents = update_simulation();

        rd_draw(agents);

        rd_animationFrame = requestAnimationFrame(frame);
        console.log("test");
      }
      if (rd_animationFrame) cancelAnimationFrame(rd_animationFrame);
      frame();
    } else {
      if (rd_animationFrame) cancelAnimationFrame(rd_animationFrame);
    }
  }

  onMount(async () => {
    await init();
    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);
  });
</script>

<div class="flex flex-col lg:flex-col flex-grow gap-6">
  <h2 class="text-3xl font-semibold text-subtext1 mb-4">Random Waypoint</h2>
  <div class="flex flex-col lg:flex-row flex-grow gap-6">
    <div class="lg:w-1/3 p-6 rounded-lg shadow-md flex flex-col space-y-4">
      <div class="flex flex-col space-y-2">
        <label for="rd_num_agents" class="text-subtext0 font-medium"
          >Num Agents</label
        >
        <input
          type="range"
          id="rd_num_agents"
          min="2"
          max="100"
          bind:value={rd_numAgents}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_speed" class="text-subtext0 font-medium">Speed</label>
        <input
          type="range"
          id="rd_speed"
          min="0"
          max="100"
          bind:value={rd_speed}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_speed" class="text-subtext0 font-medium">Range</label>
        <input
          type="range"
          id="rd_radius"
          min="0"
          max="1000"
          bind:value={rd_radius}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <button
        class="mt-4 {rd_running
          ? 'bg-red-500'
          : 'bg-blue-500'} text-white font-bold py-3 px-6 rounded-lg shadow-lg transition duration-300 ease-in-out transform hover:scale-105"
        on:click={rd_toggleSimulation}
      >
        {!rd_started ? "Start" : rd_running ? "Stop" : "Continue"}
      </button>
    </div>

    <div class="lg:w-2/3 p-6 rounded-lg shadow-md flex flex-col">
      <canvas
        bind:this={rd_canvas}
        id="myCanvas"
        class="w-full h-[500px] border border-overlay1 rounded-md"
      ></canvas>
    </div>
  </div>

  <br />
</div>
