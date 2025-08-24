//! Test example for built-in IME support

use bevy::prelude::*;
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputStyle, TextSubmitEvent,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_submit)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    // Instructions
    commands.spawn((
        Text::new("Test IME Support - Try typing Chinese/Japanese/Korean text"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
    
    // First text input - with custom font for CJK support
    commands.spawn((
        TextInputNode {
            clear_on_submit: false,
            ..default()
        },
        TextInputBuffer::default(),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            width: Val::Px(600.0),
            height: Val::Px(60.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
        BorderColor(Color::WHITE),
        TextInputStyle {
            cursor_color: Color::srgb(1.0, 1.0, 0.0),
            selection_color: Color::srgba(0.3, 0.6, 1.0, 0.4),
            ..default()
        },
    ));
    
    // Second text input - default font
    commands.spawn((
        TextInputNode {
            clear_on_submit: false,
            ..default()
        },
        TextInputBuffer::default(),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            width: Val::Px(600.0),
            height: Val::Px(60.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(160.0),
            left: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.3, 0.1)),
        BorderColor(Color::WHITE),
        TextInputStyle {
            cursor_color: Color::srgb(0.0, 1.0, 1.0),
            selection_color: Color::srgba(1.0, 0.3, 0.6, 0.4),
            ..default()
        },
    ));
    
    // Multi-line text input
    commands.spawn((
        TextInputNode {
            clear_on_submit: false,
            ..default()
        },
        TextInputBuffer::default(),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            width: Val::Px(600.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(240.0),
            left: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.1, 0.1)),
        BorderColor(Color::WHITE),
        TextInputStyle::default(),
    ));
    
    // Submit info
    commands.spawn((
        Text::new("Press Enter to submit (text will be logged to console)"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

fn handle_submit(mut events: EventReader<TextSubmitEvent>) {
    for event in events.read() {
        info!("Text submitted from {:?}: '{}'", event.entity, event.text);
        info!("  Characters: {:?}", event.text.chars().collect::<Vec<_>>());
        info!("  Bytes: {} bytes", event.text.len());
    }
}