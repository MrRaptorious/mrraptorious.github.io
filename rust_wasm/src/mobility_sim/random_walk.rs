use crate::random::*;
use serde::Serialize;
use core::num;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

#[wasm_bindgen]
#[derive(Clone, Serialize)]
pub struct RwAgent {
    pub x: f32,
    pub y: f32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub speed: f32,
    pub movement_time: u32,
}

pub struct SimData {
    rng: Rng,
    num_agents: u32,
    // simulation area
    width: i32,
    height: i32,
    // speed
    speed_min: f32,
    speed_max: f32,
    movement_time:u32
}

thread_local! {
    static RD_AGENTS: RefCell<Vec<RwAgent>> = const { RefCell::new(Vec::new()) };
    static RD_SIM_INFO: RefCell<SimData> = RefCell::new(SimData {rng:create_default_rng(), num_agents:0, width: 0, height: 0, speed_max:0.0, speed_min:0.0,movement_time:0});
}

fn create_default_rng() -> Rng {
    create_rng()
}

#[wasm_bindgen]
pub fn rw_init_simulation(
    num_agents: u32,
    width: i32,
    height: i32,
    speed_min: f32,
    speed_max: f32,
    movement_time: u32,
) {
    let mut sim_info = SimData {
        width,
        height,
        rng: create_rng(),
        speed_min,
        speed_max,
        movement_time,
        num_agents,
    };

    let mut agents = Vec::with_capacity(num_agents as usize);

    for _ in 0..num_agents {
        let x = sim_info.rng.get() * width as f32;
        let y = sim_info.rng.get() * height as f32;
        

        let angle = sim_info.rng.get() * std::f32::consts::TAU;
        let speed = sim_info.rng.get_between(speed_min, speed_max);

        agents.push(RwAgent {
            x,
            y,
            angle_x: angle.cos(),
            angle_y: angle.sin(),
            speed,
            movement_time,
        });
    }

    sim_info.height = height;
    sim_info.width = width;
    sim_info.movement_time= movement_time;
    sim_info.speed_max = speed_max;
    sim_info.speed_min = speed_min;
    sim_info.num_agents = num_agents;
    RD_AGENTS.with(|a| *a.borrow_mut() = agents);
    RD_SIM_INFO.with(|a| *a.borrow_mut() = sim_info);
}

#[wasm_bindgen]
pub fn rw_update_simulation(width:i32, height:i32, speed_min: f32, speed_max: f32, movement_time: u32) -> JsValue {
    let mut result = Vec::new();
    RD_AGENTS.with(|agents| {
        RD_SIM_INFO.with(|sim_info| {
            let mut sim_info = sim_info.borrow_mut();
            let mut agents_mut = agents.borrow_mut();

            if sim_info.movement_time!= movement_time {sim_info.movement_time = movement_time;}
            if sim_info.speed_max != speed_max {sim_info.speed_max = speed_max;}
            if sim_info.speed_min != speed_min {sim_info.speed_min = speed_min;}
            if sim_info.height != height {sim_info.height = height;}
            if sim_info.width != width {sim_info.width = width;}

            for agent in agents_mut.iter_mut() {
              // update if new direction needed 
              if agent.movement_time == 0 {
                agent.movement_time = sim_info.movement_time;

                // new direction
                let angle = sim_info.rng.get() * std::f32::consts::TAU;
                agent.angle_x = angle.cos();   
                agent.angle_y = angle.sin();   

                // new speed
                agent.speed = sim_info.rng.get_between(speed_min, speed_max); 
                
              }

              agent.movement_time -= 1;
              
              let vx = agent.speed * agent.angle_x;
              let vy = agent.speed * agent.angle_y;

              // Position aktualisieren
                agent.x += vx;
                agent.y += vy;

                if agent.x < 0.0 {
                    agent.x = 0.0;
                    agent.angle_x = -agent.angle_x; 
                }
                if agent.x > sim_info.width as f32 {
                    agent.x = sim_info.width as f32;
                    agent.angle_x = -agent.angle_x; 
                }
                if agent.y < 0.0 {
                    agent.y = 0.0;
                    agent.angle_y = -agent.angle_y; 
                }
                if agent.y > sim_info.height as f32 {
                    agent.y = sim_info.height as f32;
                    agent.angle_y = -agent.angle_y; 
                }
            }
            result = agents_mut.clone();
        });
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}
