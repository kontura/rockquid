
use bevy::prelude::*;
use crate::player;

pub fn camera_follow_player(
    mut player_query: Query<&Transform, With<player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    let player_ship_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_ship_transform.translation.x;
    camera_transform.translation.y = player_ship_transform.translation.y;
}
