use crate::random::*;
use serde::Serialize;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Serialize)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pause_time: u32,
}

pub struct SimData {
    rng: Rng,
    // simulation area
    width: i32,
    height: i32,
    // pause for touching the wall
    pause_init: u32,
    // speed
    speed_min: f32,
    speed_max: f32,
}

thread_local! {
    static RD_AGENTS: RefCell<Vec<Agent>> = RefCell::new(Vec::new());
    static RD_SIM_INFO: RefCell<SimData> = RefCell::new(SimData {rng:create_default_rng(), width: 0, height: 0, pause_init:0 , speed_max:0.0, speed_min:0.0});
}

fn create_default_rng() -> Rng {
    create_rng()
}

#[wasm_bindgen]
pub fn init_simulation(
    num_agents: u32,
    width: i32,
    height: i32,
    speed_min: f32,
    speed_max: f32,
    pause_init: u32,
) {
    let mut sim_info = SimData {
        width,
        height,
        rng: create_rng(),
        speed_min,
        speed_max,
        pause_init
    };

    let mut agents = Vec::with_capacity(num_agents as usize);

    for _ in 0..num_agents {
        let x = sim_info.rng.get() * width as f32;
        let y = sim_info.rng.get() * height as f32;
        

        let angle = sim_info.rng.get() * std::f32::consts::TAU;
        let speed = sim_info.rng.get_between(speed_min, speed_max);
        let vx = speed * angle.cos();
        let vy = speed * angle.sin();

        agents.push(Agent {
            x,
            y,
            vx,
            vy,
            pause_time: 0,
        });
    }

    sim_info.height = height;
    sim_info.width = width;
    sim_info.pause_init = pause_init;
    sim_info.speed_max = speed_max;
    sim_info.speed_min = speed_min;
    RD_AGENTS.with(|a| *a.borrow_mut() = agents);
    RD_SIM_INFO.with(|a| *a.borrow_mut() = sim_info);
}

#[wasm_bindgen]
pub fn update_simulation() -> JsValue {
    let mut result = Vec::new();
    RD_AGENTS.with(|agents| {
        RD_SIM_INFO.with(|sim_info| {
            let sim_info = sim_info.borrow();
            let mut agents_mut = agents.borrow_mut();
            for agent in agents_mut.iter_mut() {
                if agent.pause_time > 0 {
                    agent.pause_time -= 1;
                } else {
                    // Position aktualisieren
                    agent.x += agent.vx;
                    agent.y += agent.vy;

                    // An Wänden abprallen (links/rechts)
                    if agent.x < 0.0 {
                        agent.x = 0.0;
                        agent.vx = -agent.vx;
                        agent.pause_time = sim_info.pause_init;
                    }
                    if agent.x > sim_info.width as f32 {
                        agent.x = sim_info.width as f32;
                        agent.vx = -agent.vx;
                        agent.pause_time = sim_info.pause_init;
                    }
                    // An Wänden abprallen (oben/unten)
                    if agent.y < 0.0 {
                        agent.y = 0.0;
                        agent.vy = -agent.vy;
                        agent.pause_time = sim_info.pause_init;
                    }
                    if agent.y > sim_info.height as f32 {
                        agent.y = sim_info.height as f32;
                        agent.vy = -agent.vy;
                        agent.pause_time = sim_info.pause_init;
                    }
                }
            }
            result = agents_mut.clone();
        });
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}
