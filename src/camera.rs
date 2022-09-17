use bevy::prelude::*;

use crate::player;
use crate::config;

pub fn camera_follow_player(
    mut player_query: Query<&Transform, With<player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    let player_ship_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    let mut new_camera_pos = player_ship_transform.translation.truncate();
    if new_camera_pos.x > config::MAP_BOUNDS.x/2.0 - config::WINDOW_BOUNDS.x/2.0 {
        new_camera_pos.x = config::MAP_BOUNDS.x/2.0 - config::WINDOW_BOUNDS.x/2.0;
    }
    if new_camera_pos.x < -config::MAP_BOUNDS.x/2.0 + config::WINDOW_BOUNDS.x/2.0 {
        new_camera_pos.x = -config::MAP_BOUNDS.x/2.0 + config::WINDOW_BOUNDS.x/2.0;
    }

    if new_camera_pos.y > config::MAP_BOUNDS.y/2.0 - config::WINDOW_BOUNDS.y/2.0 {
        new_camera_pos.y = config::MAP_BOUNDS.y/2.0 - config::WINDOW_BOUNDS.y/2.0;
    }
    if new_camera_pos.y < -config::MAP_BOUNDS.y/2.0 + config::WINDOW_BOUNDS.y/2.0 {
        new_camera_pos.y = -config::MAP_BOUNDS.y/2.0 + config::WINDOW_BOUNDS.y/2.0;
    }

    camera_transform.translation.x = new_camera_pos.x;
    camera_transform.translation.y = new_camera_pos.y;
}
