<script lang="ts">
  import { onMount } from "svelte";
  import init, {
    init_simulation,
    update_simulation,
  } from "../../../lib/wasm_processor/rust_wasm";

  let rd_canvas: HTMLCanvasElement | null = null;
  let rd_numAgents = 20;
  let rd_min_speed = 5;
  let rd_max_speed = 8;
  let rd_pause_time = 50;
  let rd_radius = 50;
  let rd_animationFrame: number | null = null;
  let rd_running = false;
  let rd_started = false;

  let rd_trace = false;
  let old_agents: any[] = [];

  let draw = rd_draw;

  // max speed never smaller than min speed
  $: {
    if (rd_min_speed > rd_max_speed) {
      rd_max_speed = rd_min_speed;
    }
  }

  // set renderer
  $: {
    // clear screen
    if (rd_canvas){ 
    const ctx = rd_canvas.getContext("2d");
    if (ctx){
    ctx.clearRect(0, 0, rd_canvas.width, rd_canvas.height);
    }}
    if (!rd_trace) {
      draw = rd_draw;
    } else {
      draw = rd_draw_traced;
    }
  }

  function resizeCanvas() {
    if (rd_canvas) {
      // Setze die Canvas-Attribute auf die tatsächliche Pixelgröße
      const rect = rd_canvas.getBoundingClientRect();
      rd_canvas.width = rect.width;
      rd_canvas.height = rect.height;
    }
  }

  function rd_draw_traced(agents: any[]) {
    if (!rd_canvas) return;
    const ctx = rd_canvas.getContext("2d");
    if (!ctx) return;

    ctx.strokeStyle = "#14b8a6";
    ctx.lineWidth = 2;

    console.log(agents.length);
    console.log(old_agents.length);

    for (let i = 0; i < agents.length; i++) {
      const oldA = old_agents[i];
      const newA = agents[i];
      if (!oldA || !newA) continue;
      ctx.beginPath();
      ctx.moveTo(oldA.x, oldA.y);
      ctx.lineTo(newA.x, newA.y);
      ctx.stroke();
    }

    old_agents = agents;
  }

  function rd_draw(agents: any[]) {
    if (!rd_canvas) return;
    const ctx = rd_canvas.getContext("2d");
    if (!ctx) return;
    ctx.clearRect(0, 0, rd_canvas.width, rd_canvas.height);
    rd_drawAgentConnections(ctx, agents[0], agents);
    rd_drawAgents(ctx, agents);
  }

  function rd_drawAgents(
    ctx: CanvasRenderingContext2D,
    agents: any[],
    size = 8,
  ) {
    ctx.fillStyle = "#000000"; // teal-500 fallback

    for (const agent of agents) {
      ctx.beginPath();
      ctx.arc(agent.x, agent.y, size, 0, Math.PI * 2);
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
        rd_min_speed,
        rd_max_speed,
        rd_pause_time,
      );

      rd_started = true;
    }

    rd_running = !rd_running;

    if (rd_running) {
      function frame() {
        const agents = update_simulation(
          rd_min_speed,
          rd_max_speed,
          rd_pause_time,
        );

        draw(agents);

        rd_animationFrame = requestAnimationFrame(frame);
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
  <div class="flex flex-col lg:flex-row flex-grow gap-6">
    <div class="lg:w-1/3 p-6 rounded-lg shadow-md flex flex-col space-y-4">
      <div class="flex flex-col space-y-2">
        <div class="flex flex-row justify-between space-y-2">
          <label for="rd_num_agents" class="text-subtext0 font-medium"
            >Num Agents</label
          >
          <p>{rd_numAgents}</p>
        </div>
        <input
          type="range"
          id="rd_num_agents"
          min="2"
          max="100"
          bind:value={rd_numAgents}
          disabled={rd_running}
          on:change={() => {
            rd_started = false;
          }}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_min_speed" class="text-subtext0 font-medium"
          >Min Speed</label
        >
        <input
          type="range"
          id="rd_min_speed"
          min="1"
          max="25"
          step="0.1"
          bind:value={rd_min_speed}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_max_speed" class="text-subtext0 font-medium"
          >Max Speed</label
        >
        <input
          type="range"
          id="rd_max_speed"
          min="1"
          max="25"
          step="0.1"
          bind:value={rd_max_speed}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <div class="flex flex-row justify-between space-y-2">
          <label for="rd_max_speed" class="text-subtext0 font-medium"
            >Pause time</label
          >
          <p>{rd_pause_time} t</p>
        </div>
        <input
          type="range"
          id="rd_max_speed"
          min="0"
          max="100"
          bind:value={rd_pause_time}
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

      <div class="flex flex-col space-y-2">
        <label for="rd_speed" class="text-subtext0 font-medium">Trace</label>
        <input
          type="checkbox"
          id="rd_trace"
          bind:checked={rd_trace}
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
