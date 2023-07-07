#![allow(unused)]

mod sim;
mod consts;
mod util;
mod agent;
mod timer;
mod kinetic;
mod ui;
mod neuro;
mod progress_bar;
mod world;
mod source;
mod camera;
mod element;

use macroquad::prelude::*;
use crate::sim::*;
use crate::consts::*;
use crate::util::*;
pub use crate::source::*;

fn app_configuration() -> Conf {
    Conf{
        window_title: "LIVE 2.0".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 8,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(app_configuration)]
async fn main() {
    let cfg = SimConfig::default();
    let font = load_ttf_font("firacode.ttf").await.expect("can't load font resource!");
    let mut sim = Simulation::new(cfg, font.clone());
    sim.init();
    sim.autorun_new_sim();    
    
    loop {
        sim.input();
        sim.process_ui();
        if sim.is_running() {
            sim.update();
            sim.draw();
        }
        else {
            sim.signals_check();
        }
        sim.draw_ui();
        next_frame().await;
    }
}