// use wasm_bindgen::prelude::*;
// use serde::Serialize;
// use std::cell::RefCell;
// use crate::random::*;

// #[wasm_bindgen]
// #[derive(Clone, Serialize)]
// pub struct Agent {
//     pub x: f32,
//     pub y: f32,
//     pub vx: f32,
//     pub vy: f32,
// }

// pub struct SimData {
//  width: i32,
//  height :i32,
//  agent_count: usize,
//  fixed_distance: f32,
//  pause_time: i32,
//  rng: Rng,
// }

// thread_local! {
//     static RD_AGENTS: RefCell<Vec<Agent>> = RefCell::new(Vec::new());
//     static RD_SIM_INFO: RefCell<SimData> = RefCell::new(SimData { width: 0, height: 0 });
// }

// #[wasm_bindgen]
// pub fn init_simulation(num_agents: usize, width: i32, height: i32, speed: f32, pause_time: i32, fixed_distance: f32) {
//     let mut rng = create_rng();
//     let mut agents = Vec::with_capacity(num_agents);
// for _ in 0..num_agents {
//         let x = rng.get() * width as f32;
//         let y = rng.get() * height as f32;
//         let angle = rng.get() * std::f32::consts::TAU;
//         let v_sign = if rng.get_bool() { 1.0 } else { -1.0 };

//         let vx = speed * angle.cos() * v_sign;
//         let vy = speed * angle.sin() * v_sign;

//         agents.push(Agent { x, y, vx, vy });
//     }

//     RD_AGENTS.with(|a| *a.borrow_mut() = agents);
//     RD_SIM_INFO.with(|s| *s.borrow_mut() = SimData { agent_count: num_agents, width, height, rng,fixed_distance, pause_time });
// }

// #[wasm_bindgen]
// pub fn update_simulation() -> JsValue {
//     let mut result = Vec::new();
//     RD_AGENTS.with(|agents| {
//         RD_SIM_INFO.with(|sim_info| {
//             let sim_info = sim_info.borrow();
//             let mut agents_mut = agents.borrow_mut();
//             for agent in agents_mut.iter_mut() {
//                 // Position aktualisieren
//                 agent.x += agent.vx;
//                 agent.y += agent.vy;

//                 // An Wänden abprallen (links/rechts)
//                 if agent.x < 0.0 {
//                     agent.x = 0.0;
//                     agent.vx = -agent.vx;
//                 }
//                 if agent.x > sim_info.width as f32 {
//                     agent.x = sim_info.width as f32;
//                     agent.vx = -agent.vx;
//                 }
//                 // An Wänden abprallen (oben/unten)
//                 if agent.y < 0.0 {
//                     agent.y = 0.0;
//                     agent.vy = -agent.vy;
//                 }
//                 if agent.y > sim_info.height as f32 {
//                     agent.y = sim_info.height as f32;
//                     agent.vy = -agent.vy;
//                 }
//             }
//             result = agents_mut.clone();
//         });
//     });
//     serde_wasm_bindgen::to_value(&result).unwrap()
// }