use bevy::prelude::*;

use crate::config;
use rand::Rng;

pub struct EnemiesPlugin;

struct SpawnEnemiesTimer(Timer);

#[derive(Component)]
pub struct Enemy {
    _alive: bool,
}

#[derive(Component)]
pub struct Advancing {
    movement_speed: f32,
}

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemies)
            .insert_resource(SpawnEnemiesTimer(Timer::from_seconds(0.5, true)))
            .add_system(advancing_enemies_system)
            .add_system(spawn_enemies_system)
            .add_system(despawn_enemies_system);
    }
}

fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_handle = asset_server.load("textures/enemy_A.png");
    let mut enemy_start_transform = Transform::from_xyz(0.0, config::MAP_BOUNDS.y / 2.0, 0.0);
    enemy_start_transform.rotate_z(f32::to_radians(180.0));
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_handle,
            transform: enemy_start_transform,
            ..default()
        })
        .insert(Advancing {
            movement_speed: config::ENEMY_MOVEMENT_SEED,
        })
        .insert(Enemy { _alive: true });
}

fn advancing_enemies_system(mut query: Query<(&Advancing, &mut Transform)>) {
    let advancing_direction = Vec3::Y;
    for (advacing, mut trans) in &mut query {
        let advacing_distance = advancing_direction * advacing.movement_speed * config::TIME_STEP;
        let advacing_delta = advancing_direction * advacing_distance;
        trans.translation -= advacing_delta;
    }
}

fn spawn_enemies_system(
    time: Res<Time>,
    mut timer: ResMut<SpawnEnemiesTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        //TODO(amatej): I think the texture should be a resource? - load it just once
        let enemy_handle = asset_server.load("textures/enemy_A.png");
        let random_pos =
            rand::thread_rng().gen_range((-config::MAP_BOUNDS.x / 2.0)..(config::MAP_BOUNDS.x / 2.0));
        let mut enemy_start_transform =
            Transform::from_xyz(random_pos, config::MAP_BOUNDS.y / 2.0, 0.0);
        enemy_start_transform.rotate_z(f32::to_radians(180.0));
        commands
            .spawn_bundle(SpriteBundle {
                texture: enemy_handle,
                transform: enemy_start_transform,
                ..default()
            })
            .insert(Advancing {
                movement_speed: config::ENEMY_MOVEMENT_SEED,
            })
            .insert(Enemy { _alive: true });
    }
}

// Despawns enemies that go outside of the screen
// TODO(amatej): check all sides not just -Y
fn despawn_enemies_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Advancing>>,
) {
    for (advancing_entity, trans) in &mut query {
        //TODO(amatej): mayber despawn UNDER the screen
        if trans.translation.y < -config::MAP_BOUNDS.y / 2.0 {
            commands.entity(advancing_entity).despawn();
        }
    }
}
