use bevy::prelude::*;

use crate::config;
use crate::map;
use bevy_prototype_debug_lines::*;
use rand::Rng;

pub struct EnemiesPlugin;

struct SpawnEnemiesTimer(Timer);

#[derive(Component)]
pub struct Enemy {
    //TODO(amatej): not sure if _alive is wanted -> I delete it when killed..
    _alive: bool,
    pub path: Vec<map::Pos>,
    pub scroll_offset: Vec3,
}

#[derive(Component)]
pub struct Advancing {
    movement_speed: f32,
}

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnEnemiesTimer(Timer::from_seconds(0.5, true)))
            .add_system(advancing_enemies_system)
            .add_system(spawn_enemies_system)
            .add_system(despawn_enemies_system);
    }
}

fn advancing_enemies_system(
    map: Res<map::Map>,
    mut lines: ResMut<DebugLines>,
    mut query: Query<(&Advancing, &mut Transform, &mut Enemy)>,
) {
    for (advacing, mut trans, mut enemy) in &mut query {
        let advancing_direction: Vec3;
        if let Some(t) = enemy.path.first() {
            let target = t.to_world_vec3() - enemy.scroll_offset;

            advancing_direction = (target - trans.translation).normalize();

            // Debug draw path
            let mut previous_i = trans.translation;
            for i in &enemy.path {
                let t = i.to_world_vec3() - enemy.scroll_offset;
                lines.line(previous_i, t, 0.0);
                previous_i = t;
            }

            if (target - trans.translation.round())
                .abs()
                .cmple(Vec3::new(5.0, 5.0, 5.0))
                .all()
            {
                enemy.path.remove(0);
            }
            let advacing_distance: f32 =
                (advacing.movement_speed + map.scroll_speed) * config::TIME_STEP;
            let advacing_delta = advancing_direction * advacing_distance;
            trans.translation += advacing_delta;
        } else {
            // Stand on the current spot -> there is no way to pass
            // This could be important especially for bigger sizes of ships (enemies)
        }
    }
}

fn spawn_enemies_system(
    time: Res<Time>,
    mut timer: ResMut<SpawnEnemiesTimer>,
    mut commands: Commands,
    mut tile_query: Query<&Transform, With<map::Tile>>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        //TODO(amatej): I think the texture should be a resource? - load it just once
        let enemy_handle = asset_server.load("textures/enemy_A.png");
        let random_pos = rand::thread_rng().gen_range(
            ((-(config::TILES_PER_WIDTH - 3) as f32 / 2.0) as i32)
                ..(((config::TILES_PER_WIDTH - 3) as f32 / 2.0) as i32),
        );
        let random_pos_world = random_pos as f32 * config::TILE_SIDE;

        let mut random_pos_clear = true;
        // check if picked random_pos is free and don't spawn enemy if it isn't
        for tile_trans in &mut tile_query {
            if tile_trans.translation.y >= config::MAP_BOUNDS.y / 2.0 - 2.0 * config::TILE_SIDE {
                if tile_trans.translation.x >= random_pos_world - 2.0 * config::TILE_SIDE
                    && tile_trans.translation.x <= random_pos_world + 2.0 * config::TILE_SIDE
                {
                    random_pos_clear = false;
                    break;
                }
            }
        }

        if random_pos_clear {
            let random_speed_offset =
                rand::thread_rng().gen_range(0.0..config::ENEMY_MOVEMENT_SEED);
            let mut enemy_start_transform =
                Transform::from_xyz(random_pos_world, config::MAP_BOUNDS.y / 2.0, 0.0);
            enemy_start_transform.rotate_z(f32::to_radians(180.0));
            commands
                .spawn_bundle(SpriteBundle {
                    texture: enemy_handle,
                    transform: enemy_start_transform,
                    ..default()
                })
                .insert(Advancing {
                    movement_speed: random_speed_offset,
                })
                .insert(Enemy {
                    _alive: true,
                    scroll_offset: Vec3::ZERO,
                    path: vec![
                        //map::Pos{0:0, 1:0},
                        //Vec2::new(random_pos_world, (config::MAP_BOUNDS.y / 2.0) - 430.0),
                        //Vec2::new(random_pos_world-40.0, (config::MAP_BOUNDS.y / 2.0) - 240.0),
                        //Vec2::new(random_pos_world+40.0, (config::MAP_BOUNDS.y / 2.0) - 130.0),
                    ],
                });
        }
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
