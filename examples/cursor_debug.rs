//! Debug cursor visibility

use bevy::{color::palettes::css::NAVY, prelude::*};
use bevy_ui_text_input::{TextInputNode, TextInputPlugin};
use bevy::input_focus::InputFocus;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (debug_focus, auto_focus_first_input))
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_child((
            TextInputNode::default(),
            Node {
                width: Val::Px(500.),
                height: Val::Px(250.),
                ..default()
            },
            BackgroundColor(NAVY.into()),
        ));
}

fn debug_focus(focus: Res<InputFocus>) {
    if focus.is_changed() {
        info!("Focus changed to: {:?}", focus.get());
    }
}

fn auto_focus_first_input(
    mut focus: ResMut<InputFocus>,
    text_inputs: Query<Entity, With<TextInputNode>>,
    mut focused: Local<bool>,
) {
    if !*focused && !text_inputs.is_empty() {
        if let Ok(entity) = text_inputs.single() {
            focus.set(entity);
            *focused = true;
            info!("Auto-focused text input: {:?}", entity);
        }
    }
}
