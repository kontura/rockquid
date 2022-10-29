use crate::config;
use crate::enemies::Enemy;
use bevy::prelude::*;
//use bevy_prototype_debug_lines::*;
use pathfinding::prelude::astar;
use rand::Rng;

pub struct MapPlugin;

#[derive(Default)]
pub struct Map {
    handles: Vec<HandleUntyped>,
    pub scroll_speed: f32,
}

//TODO(amatej): this should be u32 so its clear we have to convert back to map coors... that have
//zero,zero in the center not bottom left like Pos
//TODO(amatej): this represents position of a tile in the tiled world?
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
    fn _distance_of_y_only(&self, other: &Pos) -> u32 {
        self.y.abs_diff(other.y) as u32
    }

    pub fn from_world_vec3(input: &Vec3) -> Pos {
        let mut pos = (*input / Vec3::new(config::TILE_SIDE, config::TILE_SIDE, 1.0)).round();
        pos.x = pos.x + (config::TILES_PER_WIDTH / 2) as f32;
        pos.y = pos.y + (config::ROWS_PER_HEIGHT / 2) as f32;
        //TODO(amatej): Fix this ugliness
        Pos {
            x: pos.x as usize as i32,
            y: pos.y as usize as i32,
        }
    }

    pub fn to_world_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.x as f32 - config::TILES_PER_WIDTH as f32 / 2.0) * config::TILE_SIDE,
            (self.y as f32 - config::ROWS_PER_HEIGHT as f32 / 2.0) * config::TILE_SIDE,
            0.0,
        )
    }
}
impl std::ops::Add<&Pos> for &Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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
    //mut lines: ResMut<DebugLines>,
    row_query: Query<(Entity, &Row), With<ToBeProcessedRow>>,
    tile_query: Query<&Transform, With<Tile>>,
    mut query: Query<(&mut Transform, &mut Enemy), Without<Tile>>,
) {
    if row_query.is_empty() {
        return;
    }

    let (entity, row) = row_query.single();

    commands.entity(entity).remove::<ToBeProcessedRow>();

    // Spawn sides
    for side in vec![-config::MAP_BOUNDS.x / 2.0, config::MAP_BOUNDS.x / 2.0] {
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
        //println!("pos: {:?}", pos/32);
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

    // construct current map:
    // add +2 for debug
    let mut map_maze: [[bool; config::TILES_PER_WIDTH as usize + 2];
        config::ROWS_PER_HEIGHT as usize + 2] =
        [[true; config::TILES_PER_WIDTH as usize + 2]; config::ROWS_PER_HEIGHT as usize + 2];
    for tile_trans in &tile_query {
        let mp = Pos::from_world_vec3(&tile_trans.translation);
        map_maze[mp.y as usize][mp.x as usize] = false;
    }

    // Debug draw the map so we can see what we have
    //for xi in 0..config::TILES_PER_WIDTH + 1 {
    //    for yi in 0..config::ROWS_PER_HEIGHT + 1 {
    //        let p = Pos { 0: xi, 1: yi };
    //        let c: Color;
    //        if map_maze[yi as usize][xi as usize] {
    //            c = Color::GREEN
    //        } else {
    //            c = Color::RED
    //        }
    //        lines.line_colored(
    //            p.to_world_vec3() + Vec3::new(10.0, 10.0, 0.0),
    //            p.to_world_vec3() + Vec3::new(-10.0, -10.0, 0.0),
    //            1.0,
    //            c,
    //        );
    //        lines.line_colored(
    //            p.to_world_vec3() + Vec3::new(-10.0, 10.0, 0.0),
    //            p.to_world_vec3() + Vec3::new(10.0, -10.0, 0.0),
    //            1.0,
    //            c,
    //        );
    //    }
    //}

    for (trans, mut enemy) in &mut query {
        if !enemy.path.is_empty() {
            continue;
        }

        let successors = |input: Pos| -> Vec<(Pos, u32)> {
            let mut sucs: Vec<Pos> = vec![];
            for offset in vec![
                Pos { x: 1, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: -1 },
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: -1 },
                Pos { x: -1, y: 1 },
                Pos { x: -1, y: 0 },
                Pos { x: -1, y: -1 },
            ] {
                let testing_pos = &offset + &input;
                let new_pos = testing_pos;
                //println!("offset: {:?}", offset);
                //println!("input: {:?}", input);
                //println!("TILES_PER_WIDTH: {:?}", config::TILES_PER_WIDTH);

                //println!("testing_pos before: {:?}", testing_pos);
                //println!("testing_pos: {:?}", testing_pos);

                // Befora I push I should check if I am outside of map -> and if so not push!
                if testing_pos.x >= config::TILES_PER_WIDTH
                    || testing_pos.x < 0
                    || testing_pos.y >= config::ROWS_PER_HEIGHT
                    || testing_pos.y < 0
                {
                    continue;
                }
                if map_maze[testing_pos.y as usize][testing_pos.x as usize] {
                    sucs.push(new_pos);
                }
            }
            //println!("sucs: {:?}", sucs);
            sucs.into_iter().map(|p| (p, 1)).collect()
        };
        let my_pos = Pos::from_world_vec3(&trans.translation);
        //lines.line_colored(
        //    my_pos.to_world_vec3() + Vec3::new(10.0, 10.0, 0.0),
        //    my_pos.to_world_vec3() + Vec3::new(-10.0, -10.0, 0.0),
        //    1.0,
        //    Color::RED,
        //);
        //lines.line_colored(
        //    my_pos.to_world_vec3() + Vec3::new(-10.0, 10.0, 0.0),
        //    my_pos.to_world_vec3() + Vec3::new(10.0, -10.0, 0.0),
        //    1.0,
        //    Color::RED,
        //);

        //let sucs = successors(my_pos);
        //for s in sucs {
        //    let p = s.0;
        //    lines.line_colored(
        //        p.to_world_vec3() + Vec3::new(10.0, 10.0, 0.0),
        //        p.to_world_vec3() + Vec3::new(-10.0, -10.0, 0.0),
        //        1.0,
        //        Color::BLUE,
        //    );
        //    lines.line_colored(
        //        p.to_world_vec3() + Vec3::new(-10.0, 10.0, 0.0),
        //        p.to_world_vec3() + Vec3::new(10.0, -10.0, 0.0),
        //        1.0,
        //        Color::BLUE,
        //    );
        //}
        //println!("my_pos: {:?}", my_pos);
        let goal: Pos = Pos { x: my_pos.x, y: 0 };
        let result = astar(
            &my_pos,
            |p| successors(*p),
            |p| p.distance(&goal),
            |p| (*p) == goal,
        );
        //println!("result is: {:?}", result);
        if let Some(t) = result {
            enemy.path = t.0;
        }
    }
}

fn scroll_map_system(
    map: Res<Map>,
    mut commands: Commands,
    mut tile_query: Query<(Entity, &mut Transform), With<Tile>>,
    mut row_query: Query<(Entity, &mut Row)>,
    mut enemy_query: Query<&mut Enemy, Without<Tile>>,
) {
    let scroll_direction = Vec3::Y;
    let scroll_distance = scroll_direction * map.scroll_speed * config::TIME_STEP;

    for (entity, mut row) in &mut row_query {
        row.y_pos -= scroll_distance.y;
        if row.y_pos < -config::MAP_BOUNDS.y / 2.0 {
            row.y_pos += config::MAP_BOUNDS.y;
            commands.entity(entity).insert(ToBeProcessedRow);
        }
    }

    for (tile, mut tile_trans) in &mut tile_query {
        tile_trans.translation -= scroll_distance;
        if tile_trans.translation.y < -config::MAP_BOUNDS.y / 2.0 - 2.0 * config::TILE_SIDE {
            commands.entity(tile).despawn();
        }
    }
    for mut enemy in &mut enemy_query {
        enemy.scroll_offset += scroll_distance;
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
    // spawn side map bounds
    for side in vec![-config::MAP_BOUNDS.x / 2.0, config::MAP_BOUNDS.x / 2.0] {
        for pos in (-config::ROWS_PER_HEIGHT / 2)..(config::ROWS_PER_HEIGHT / 2) {
            let random_tile_index = rand::thread_rng().gen_range(0..(map.handles.len()));
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        side,
                        pos as f32 * config::TILE_SIDE,
                        0.0,
                    )),
                    texture: map.handles[random_tile_index].typed_weak(),
                    ..Default::default()
                })
                .insert(Tile);
        }
    }

    // spawn rows
    for row_index in 0..config::ROWS_PER_HEIGHT {
        commands.spawn().insert(Row {
            y_pos: config::MAP_BOUNDS.y / 2.0 - row_index as f32 * config::TILE_SIDE,
        });
    }
}
