<script lang="ts">
    import init, { rw_init_simulation, rw_update_simulation } from "$lib/wasm_processor/rust_wasm";
  import { onMount } from "svelte";

  // meta variables
  let canvas: HTMLCanvasElement | null = null;
  let animationFrame: number | null = null;
  let sim_running = false;
  let sim_started = false;
  let trace_mode = false;
  let old_agents: any[] = [];
  let render = draw;

  // sim variables
  let numAgents = 20;
  let speed_min = 5;
  let speed_max = 8;
  let range = 50;
  let movement_time = 50;

  // max speed never smaller than min speed
  $: {
    if (speed_min > speed_max) {
      speed_max = speed_min;
    }
  }

  // set renderer
  $: {
    // clear screen
    if (canvas){ 
      const ctx = canvas.getContext("2d");
      if (ctx){
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      }}
        console.log(trace_mode)
    if (!trace_mode) {
        console.log("TraceMode = Draw")
      render = draw;
    } else {
        console.log("TraceMode = Trace") 
      render = draw_traced;
    }
  }

  function resizeCanvas() {
    if (canvas) {
      // Setze die Canvas-Attribute auf die tatsächliche Pixelgröße
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width;
      canvas.height = rect.height;
    }
  }
    function draw_traced(agents: any[]) {

    if (!canvas) return;
    const ctx = canvas.getContext("2d");
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

  function draw(agents: any[]) {
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    drawAgentConnections(ctx, agents[0], agents);
    drawAgents(ctx, agents);
  }

  function drawAgents(ctx: CanvasRenderingContext2D, agents: any[]) {
    ctx.fillStyle = "#000000"; // teal-500 fallback

    for (const agent of agents) {
      ctx.beginPath();
      ctx.arc(agent.x, agent.y, 8, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  function drawAgentConnections(
    ctx: CanvasRenderingContext2D,
    focus: any,
    agents: any[],
  ) {
    // always draw radius around focused
    ctx.fillStyle = "#0000000F";
    ctx.strokeStyle = "#FFFFFF00";
    ctx.beginPath();
    ctx.arc(focus.x, focus.y, range, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();

    ctx.strokeStyle = "#FFFFFFFF";

    for (const agent of agents) {
      let x = focus.x - agent.x;
      let y = focus.y - agent.y;
      let d = Math.sqrt(x * x + y * y);

      // draw connections based on radius
      if (d < range && d != 0) {
        ctx.moveTo(focus.x, focus.y);
        ctx.lineTo(agent.x, agent.y);
      }
    }
    ctx.stroke();
  }

  async function toggleSimulation() {
    if (!sim_started) {
      if (!canvas) return;
      rw_init_simulation(
        numAgents,
        canvas.width,
        canvas.height,
        speed_min,
        speed_max,
        movement_time
      );

      sim_started = true;
    }

    sim_running = !sim_running;

    if (sim_running) {
      function frame() {
        const agents = rw_update_simulation(canvas!.width, canvas!.height, speed_min, speed_max, movement_time );

        render(agents);

        animationFrame = requestAnimationFrame(frame);
      }
      if (animationFrame) cancelAnimationFrame(animationFrame);
      frame();
    } else {
      if (animationFrame) cancelAnimationFrame(animationFrame);
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
        <div class="flex flex-row justify-between">
          <label for="rd_speed" class="text-subtext0 font-medium">Number of Agents</label>
          <label for="rd_speed" class="text-subtext0 font-medium">{numAgents}</label>
        </div>
        <input
          type="range"
          id="rd_num_agents"
          min="2"
          max="100"
          disabled={sim_running}
          bind:value={numAgents}
          on:change={() => {
            sim_started = false;
          }}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <div class="flex flex-row justify-between">
          <label for="rd_speed" class="text-subtext0 font-medium">Minimum Speed</label>
          <label for="rd_speed" class="text-subtext0 font-medium">{speed_min}</label>
        </div>
        <input
          type="range"
          id="rd_speed"
          min="1"
          max="10"
          step="0.1"
          bind:value={speed_min}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <div class="flex flex-row justify-between">
          <label for="rd_speed" class="text-subtext0 font-medium">Maximum Speed</label>
          <label for="rd_speed" class="text-subtext0 font-medium">{speed_max}</label>
        </div>
        <input
          type="range"
          id="rd_speed"
          min="1"
          max="10"
          step="0.1"
          bind:value={speed_max}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <div class="flex flex-row justify-between">
          <label for="rd_speed" class="text-subtext0 font-medium">Movement Time</label>
          <label for="rd_speed" class="text-subtext0 font-medium">{movement_time}</label>
        </div>
        <input
          type="range"
          id="rd_speed"
          min="1"
          max="100"
          bind:value={movement_time}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_speed" class="text-subtext0 font-medium">Range</label>
        <input
          type="range"
          id="rd_radius"
          min="1"
          max="1000"
          bind:value={range}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="rd_speed" class="text-subtext0 font-medium">Trace</label>
        <input
          type="checkbox"
          id="rd_trace"
          bind:checked={trace_mode}
          class="p-3 border border-overlay1 rounded-md focus:outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      <button
        class="mt-4 {sim_running
          ? 'bg-red-500'
          : 'bg-blue-500'} text-white font-bold py-3 px-6 rounded-lg shadow-lg transition duration-300 ease-in-out transform hover:scale-105"
        on:click={toggleSimulation}
      >
        {!sim_started ? "Start" : sim_running ? "Stop" : "Continue"}
      </button>
    </div>

    <div class="lg:w-2/3 p-6 rounded-lg shadow-md flex flex-col">
      <canvas
        bind:this={canvas}
        id="myCanvas"
        class="w-full h-[500px] border border-overlay1 rounded-md"
      ></canvas>
    </div>
  </div>

  <br />
</div>
