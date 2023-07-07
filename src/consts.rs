#![allow(unused)]

pub const SCREEN_WIDTH: f32         = 1600.0;
pub const SCREEN_HEIGHT: f32        = 900.0;
pub const WORLD_W: f32              = 3200.0;
pub const WORLD_H: f32              = 1800.0;

pub const AGENTS_NUM: usize         = 0;
pub const AGENTS_NUM_MIN: usize     = 0;
pub const AGENT_SIZE_MIN: i32       = 4;
pub const AGENT_SIZE_MAX: i32       = 10;
pub const AGENT_SPEED: f32          = 40.0;
pub const AGENT_VISION_RANGE: f32   = 250.0;
pub const AGENT_ROTATION: f32       = 2.0;

pub const ASTER_SIZE_MIN: u32       = 8;
pub const ASTER_SIZE_MAX: u32       = 18;
pub const ASTER_NUM: usize          = 4098;
pub const ASTER_SPEED: f32          = 100.0; 

pub const FIX_DT: f32               = 1.0/30.0;
pub const ZOOM_RATE: f32            = 1.0/800.0;
pub const SCREEN_RATIO: f32         = SCREEN_WIDTH/SCREEN_HEIGHT;