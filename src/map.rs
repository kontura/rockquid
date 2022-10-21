use crate::config;
use bevy::prelude::*;
use rand::Rng;

pub struct MapPlugin;

#[derive(Default)]
pub struct Map {
    handles: Vec<HandleUntyped>,
    pub scroll_speed: f32,
}

#[derive(Component)]
struct Row {
    y_pos: f32,
}

#[derive(Component)]
struct ToBeProcessedRow;

#[derive(Component)]
pub struct Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PluginState {
    Loading,
    Loaded,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map {
            handles: Vec::new(),
            scroll_speed: config::SCROLL_SPEED,
        });
        app.add_state(PluginState::Loading);
        app.add_system_set(SystemSet::on_enter(PluginState::Loading).with_system(load_resources));
        app.add_system_set(SystemSet::on_update(PluginState::Loading).with_system(check_resources));
        app.add_system_set(SystemSet::on_enter(PluginState::Loaded).with_system(setup));
        app.add_system(scroll_map_system);
        app.add_system(generate_map_system);
    }
}

fn generate_map_system(
    mut commands: Commands,
    map: Res<Map>,
    row_query: Query<(Entity, &Row), With<ToBeProcessedRow>>,
) {
    if row_query.is_empty() {
        return;
    }

    let (entity, row) = row_query.single();

    commands.entity(entity).remove::<ToBeProcessedRow>();

    // Spawn sides
    for side in vec![
        -config::MAP_BOUNDS.x / 2.0,
        -config::MAP_BOUNDS.x / 2.0 + 4.0,
        config::MAP_BOUNDS.x / 2.0,
        config::MAP_BOUNDS.x / 2.0 - 4.0,
    ] {
        let random_tile_index = rand::thread_rng().gen_range(0..(map.handles.len()));
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(side, row.y_pos, 0.0)),
                texture: map.handles[random_tile_index].typed_weak(),
                ..Default::default()
            })
            .insert(Tile);
    }

    // Spawn middle per chance
    let from = -(config::MAP_BOUNDS.x / 2.0) as i32;
    let to = (config::MAP_BOUNDS.x / 2.0) as i32;
    let mut random_change_offset = 0;
    //println!("from {:?} to: {:?}", from, to);
    for pos in (from..to).step_by(config::TILE_SIDE as usize) {
        let random_chance = rand::thread_rng().gen_range(0..100);
        //println!("random chance: {:?}", random_chance);
        if random_chance > 98 - random_change_offset {
            if random_change_offset == 0 {
                random_change_offset = 80;
            } else {
                random_change_offset = random_change_offset - 20;
            }

            let random_tile_index = rand::thread_rng().gen_range(0..(map.handles.len()));
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(pos as f32, row.y_pos, 0.0)),
                    texture: map.handles[random_tile_index].typed_weak(),
                    ..Default::default()
                })
                .insert(Tile);
        } else {
            random_change_offset = 0;
        }
    }
}

fn scroll_map_system(
    map: Res<Map>,
    mut commands: Commands,
    mut tile_query: Query<(Entity, &mut Transform), With<Tile>>,
    mut row_query: Query<(Entity, &mut Row)>,
) {
    let scroll_direction = Vec3::Y;
    let scroll_distance = scroll_direction * map.scroll_speed * config::TIME_STEP;

    for (entity, mut row) in &mut row_query {
        row.y_pos -= scroll_distance.y;
        if row.y_pos < -config::MAP_BOUNDS.y / 2.0 - 2.0 * config::TILE_SIDE {
            row.y_pos += config::MAP_BOUNDS.y + 3.0 * config::TILE_SIDE;
            commands.entity(entity).insert(ToBeProcessedRow);
        }
    }

    for (tile, mut tile_trans) in &mut tile_query {
        tile_trans.translation -= scroll_distance;
        if tile_trans.translation.y < -config::MAP_BOUNDS.y / 2.0 - 2.0 * config::TILE_SIDE {
            commands.entity(tile).despawn();
        }
    }
}

fn check_resources(
    map: ResMut<Map>,
    mut state: ResMut<State<PluginState>>,
    asset_server: Res<AssetServer>,
) {
    if let bevy::asset::LoadState::Loaded =
        asset_server.get_group_load_state(map.handles.iter().map(|handle| handle.id))
    {
        state.set(PluginState::Loaded).unwrap();
    }
}

fn load_resources(mut map: ResMut<Map>, asset_server: Res<AssetServer>) {
    map.handles = asset_server.load_folder("textures/tiles").unwrap();
}

fn setup(mut commands: Commands, map: Res<Map>) {
    let tiles_per_map = (config::MAP_BOUNDS / config::TILE_SIDE).as_ivec2();

    // spawn side map bounds
    for side in vec![
        -config::MAP_BOUNDS.x / 2.0,
        -config::MAP_BOUNDS.x / 2.0 + 4.0,
        config::MAP_BOUNDS.x / 2.0,
        config::MAP_BOUNDS.x / 2.0 - 4.0,
    ] {
        for pos in -tiles_per_map.y / 2..tiles_per_map.y / 2 {
            let random_tile_index = rand::thread_rng().gen_range(0..(map.handles.len()));
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(side, pos as f32 * 4.0, 0.0)),
                    texture: map.handles[random_tile_index].typed_weak(),
                    ..Default::default()
                })
                .insert(Tile);
        }
    }

    // spawn rows
    for row_index in -3..tiles_per_map.y {
        commands.spawn().insert(Row {
            y_pos: config::MAP_BOUNDS.y / 2.0 - row_index as f32 * config::TILE_SIDE,
        });
    }
}
