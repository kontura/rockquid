use bevy::prelude::*;

use crate::config;
use crate::player;

#[derive(Component)]
pub struct Heart;

pub struct UiPlugin;

pub struct Scoreboard {
    pub score: usize,
}

pub struct RedrawHealth {
    pub redraw: bool,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 });
        app.insert_resource(RedrawHealth { redraw: false });
        app.add_startup_system(setup);
        app.add_system(update_scoreboard);
        app.add_system(update_health);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Scoreboard
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: config::SCOREBOARD_FONT_SIZE,
                    color: config::SCOREBOARD_TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: config::SCOREBOARD_FONT_SIZE,
                color: config::SCOREBOARD_SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: config::SCOREBOARD_TEXT_PADDING,
                left: config::SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );

    draw_health(&mut commands, config::PLAYER_HEALTH, &asset_server);

}
fn draw_health(commands: &mut Commands, health: i32, asset_server: &Res<AssetServer>) {
    for i in 0..health {
        let left_padding: Val = config::SCOREBOARD_TEXT_PADDING + (i * 15) as f32;
        // Health
        commands.spawn_bundle( ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: config::HEALTH_TEXT_PADDING_TOP,
                    left: left_padding,
                    ..default()
                },
                ..default()
            },
            image: asset_server.load("textures/shot.png").into(),
            ..default()
        })
        .insert(Heart);
    }
}

fn update_health(mut commands: Commands,
                 redraw: Res<RedrawHealth>,
                 mut players: Query<&player::Player>,
                 mut hearth_query: Query<Entity, With<Heart>>,
                 asset_server: Res<AssetServer>) {
    if redraw.redraw {
        for heart in &mut hearth_query {
            commands.entity(heart).despawn();
        }

        for player in &mut players {
            draw_health(&mut commands, player.health, &asset_server);
        }
    }

}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
