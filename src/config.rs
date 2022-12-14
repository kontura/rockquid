use bevy::prelude::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const MAP_BOUNDS: Vec2 = Vec2::new(1024.0, 1024.0);
pub const WINDOW_BOUNDS: Vec2 = Vec2::new(640.0, 1024.0);
pub const ENEMY_MOVEMENT_SEED: f32 = 100.0;
pub const SHOT_MOVEMENT_SEED: f32 = 800.0;
pub const SHOT_SPEED: f32 = 0.8;

pub const SCROLL_SPEED: f32 = 150.0;
pub const TILE_SIDE: f32 = 32.0;
pub const TILES_PER_WIDTH: i32 = (MAP_BOUNDS.x / TILE_SIDE) as i32;
pub const ROWS_PER_HEIGHT: i32 = (MAP_BOUNDS.y / TILE_SIDE) as i32;

// Player
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_HEALTH: i32 = 3;

// Scoreboard
pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const SCOREBOARD_SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

// Health
pub const HEALTH_TEXT_PADDING_TOP: Val = Val::Px(45.0);
