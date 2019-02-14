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
pub struct SharedState {
    pub game_area_width : i32,
    pub ball_position_x : i32,
    pub ball_position_y : i32,
    pub ball_velocity_x : i32,
    pub ball_velocity_y : i32,

    pub ball_diameter : i32,
    pub brick_size_x : i32,
    pub brick_size_y : i32,

    pub paddle_size_x : i32,
    pub paddle_size_y : i32,
    bricks : Vec<u8>,
    pub columns : i32,
    pub rows : i32,
    pub brick_size : i32,
}

#[wasm_bindgen]
impl SharedState {
    pub fn new() -> Self {
        let bricks = (0..14 * 14)
            .map(|i| {
                if i % 2 == 0 {
                    0
                }
                else {
                    1
                }
            }).collect();

        Self {
            game_area_width : 500,
            columns : 14,
            rows : 14,
            brick_size : 29,
            // ball: Ball::new(500),
            ball_position_x : 250,
            ball_position_y : 250,
            ball_velocity_x : 1,
            ball_velocity_y : 1,

            ball_diameter : 6,
            brick_size_x : 29,
            brick_size_y : 29,
            paddle_size_x : 60,
            paddle_size_y : 6,
            bricks,
        }
    }

    pub fn update(&mut self) {
        // log!("updating {}", self.ball_position_x);
        self.ball_position_x += self.ball_velocity_x;
        self.ball_position_y += self.ball_velocity_y;
    }
    pub fn bricks(&self) -> *const u8 {
        self.bricks.as_ptr()
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
