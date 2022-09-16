use bevy::prelude::*;

use crate::config;

pub struct UiPlugin;

pub struct Scoreboard {
    pub score: usize,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 });
        app.add_startup_system(setup);
        app.add_system(update_scoreboard);
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

}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
