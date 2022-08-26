use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::config;
use crate::enemies;
use crate::collision;

pub struct PlayerPlugin;

struct ShootingTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .insert_resource(ShootingTimer(Timer::from_seconds(config::SHOT_SPEED, false)))
            .add_system(player_movement_system)
            .add_system(player_shooting_system)
            .add_system(despawn_shots_system)
            .add_system(collide_with_enemies_system)
            .add_system(collide_shots_with_enemies_system)
            .add_system(advancing_shots_system);
    }
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
}

#[derive(Component)]
struct Shot {
    movement_speed: f32,
}

impl Player {
    fn new(movement_speed: f32) -> Player {
        Player {
            movement_speed: movement_speed,
        }
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/ship_C.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player::new(config::PLAYER_SPEED));
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut horiz_movement_factor = 0.0;
    let mut vert_movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        horiz_movement_factor -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        horiz_movement_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        vert_movement_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        vert_movement_factor -= 1.0;
    }
    let movement_factor = Vec3::new(horiz_movement_factor, vert_movement_factor, 0.0);

    let movement_directions = transform.rotation * (Vec3::Y + Vec3::X);
    let movement_distance = movement_factor * ship.movement_speed * config::TIME_STEP;
    let translation_delta = movement_directions * movement_distance;
    transform.translation += translation_delta;

    let extents = Vec3::from((config::BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

fn player_shooting_system(
    mut commands: Commands,
    mut timer: ResMut<ShootingTimer>,
    time: Res<Time>,
    mut query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let transform = query.single_mut();

    if timer.0.tick(time.delta()).elapsed_secs() == config::SHOT_SPEED {
        //TODO(amatej): I think the texture should be a resource? - load it just once
        let shot_handle = asset_server.load("textures/shot.png");
        commands
            .spawn()
            .insert(Shot {
                movement_speed: config::SHOT_MOVEMENT_SEED,
            })
            .insert_bundle(SpriteBundle {
                texture: shot_handle,
                transform: Transform {
                    translation: transform.translation,
                    ..default()
                },
                ..default()
            });
        timer.0.reset();
    }
}

fn advancing_shots_system(mut query: Query<(&Shot, &mut Transform)>) {
    let advancing_direction = Vec3::Y;
    for (shot, mut trans) in &mut query {
        let advacing_distance = advancing_direction * shot.movement_speed * config::TIME_STEP;
        let advacing_delta = advancing_direction * advacing_distance;
        trans.translation += advacing_delta;
    }
}

fn collide_shots_with_enemies_system(
    mut commands: Commands,
    imgs: Res<Assets<Image>>,
    mut shots_query: Query<(Entity, &Transform, &Handle<Image>), With<Shot>>,
    mut enemy_query: Query<(Entity, &Transform, &Handle<Image>), With<enemies::Advancing>>,
) {
    for (enemy, enemy_trans, enemy_img_handle) in &mut enemy_query {
        if let Some(enemy_img) = imgs.get(enemy_img_handle) {
            for (shot, shot_trans, shot_img_handle) in &mut shots_query {
                if let Some(shot_img) = imgs.get(shot_img_handle) {
                    let collision = collision::collide(
                        enemy_trans,
                        enemy_img,
                        shot_trans,
                        shot_img,
                    );
                    if collision {
                        commands.entity(enemy).despawn();
                        commands.entity(shot).despawn();
                    }
                }
            }
        }
    }
}

// Despawns shots that go outside of the screen
// TODO(amatej): check all sides not just Y
fn despawn_shots_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Shot>>,
) {
    for (shot_entity, trans) in &mut query {
        //TODO(amatej): mayber despawn UNDER the screen
        if trans.translation.y < -config::BOUNDS.y / 2.0
            || trans.translation.y > config::BOUNDS.y / 2.0
        {
            commands.entity(shot_entity).despawn();
        }
    }
}

fn collide_with_enemies_system(
    mut commands: Commands,
    imgs: Res<Assets<Image>>,
    mut player_query: Query<(&Transform, &Handle<Image>), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &Handle<Image>), With<enemies::Advancing>>,
) {
    let (ship_transform, ship_img_handle) = player_query.single_mut();
    if let Some(ship_img) = imgs.get(ship_img_handle) {
        for (enemy, enemy_trans, enemy_img_handle) in &mut enemy_query {
            if let Some(enemy_img) = imgs.get(enemy_img_handle) {
                let collision = collision::collide(
                    ship_transform,
                    ship_img,
                    enemy_trans,
                    enemy_img,
                );
                if collision {
                    commands.entity(enemy).despawn();
                }
            }
        }
    }
}
