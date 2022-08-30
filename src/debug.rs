use crate::config;
use bevy::prelude::*;

pub struct DebugPlugin;

#[derive(Component)]
pub struct DebugCollideDraw;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dragging_system);
        app.add_system(clear_debug_draws);
    }
}

#[derive(Component)]
pub struct Draggable;

pub fn spawn_square(commands: &mut Commands, p1: IVec2, p2: IVec2, color: Color) {
    commands
        .spawn()
        .insert(DebugCollideDraw)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: (p1 - p2).as_vec2().extend(1.0),
                translation: p2.as_vec2().lerp(p1.as_vec2(), 0.5).extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: color,
                ..default()
            },
            ..default()
        });
}

fn dragging_system(
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut Transform), With<Draggable>>,
) {
    let window = windows.get_primary().expect("No primary window");
    if mouse_button_input.just_pressed(MouseButton::Left) {
        //TODO(amatej): for now use just the closest
        let mut closest: Option<Entity> = None;
        if let Some(cur_pos) = window.cursor_position() {
            for (e, trans) in &query {
                if let Some(c) = closest {
                    if let Ok(c_t) = query.get_component::<Transform>(c) {
                        let dist = c_t.translation.truncate() - cur_pos;
                        let c_dist = trans.translation.truncate() - cur_pos;

                        if c_dist.length() < dist.length() {
                            closest = Some(e);
                        }
                    }
                } else {
                    closest = Some(e);
                }
            }
            println!("click at {:?}", window.cursor_position());
            if let Some(c) = closest {
                if let Ok(mut c_t) = query.get_component_mut::<Transform>(c) {
                    c_t.translation = (cur_pos - config::BOUNDS / 2.0).extend(0.0);
                }
            }
        }
    }
}

fn clear_debug_draws(mut commands: Commands, mut query: Query<Entity, With<DebugCollideDraw>>) {
    for e in &mut query {
        commands.entity(e).despawn();
    }
}
