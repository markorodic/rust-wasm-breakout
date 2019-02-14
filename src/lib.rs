extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

//////////////////////////////

#[wasm_bindgen]
struct GameState {

    pub game_area_width : i32,
    pub ball_position : Vec<i32>,
    pub ball_velocity : Vec<i32>,

    pub ball_diameter : i32,
    pub brick_size_x : i32,
    pub brick_size_y : i32,

    pub paddle_size : Vec<i32>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> Self {
        Self {
            game_area_width : 500,
            // ball: Ball::new(500),
            ball_position : vec![250, 250],
            ball_velocity : vec![1, 1],
            ball_diameter : 6,
            brick_size_x : 29,
            brick_size_y : 29,
            paddle_size : vec![60, 6],
        }
    }

    pub fn update(&mut self) {
        log!("updating");
    }
}

// struct Ball {
//     pub position : (i32, i32),
//     pub velocity : (i32, i32),
// }
//
// impl Ball {
//     fn new(size: i32) -> Self {
//         Ball {
//             position: (size / 2, size / 2),
//             velocity: (1,1),
//         }
//     }
//     fn advance_ball(&mut self) {
//         self.position.0 += self.velocity.0;
//         self.position.1 += self.velocity.1;
//     }
// }
