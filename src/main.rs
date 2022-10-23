use bevy::{prelude::*, render::settings::WgpuSettings};

mod camera;
mod collision;
mod config;
mod debug;
mod enemies;
mod player;
mod ui;
use bevy_prototype_debug_lines::*;

mod map;

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rockquid".to_string(),
            width: config::WINDOW_BOUNDS[0],
            height: config::WINDOW_BOUNDS[1],
            ..default()
        })
        .insert_resource(WgpuSettings {
            // Use opengl to run on the pinephone
            backends: Some(bevy::render::settings::Backends::GL),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemies::EnemiesPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(map::MapPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(DebugLinesPlugin::default())
        .run();
}
