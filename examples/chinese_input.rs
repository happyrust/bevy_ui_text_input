//! Example demonstrating built-in Chinese IME input support

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
    
    // Title and instructions
    commands.spawn((
        Text::new("中文输入测试 - Chinese Input Test\n\nThe library now has built-in IME support!\nJust click on the input field and start typing Chinese."),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
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
    
    // Chinese text input with Songti font
    commands.spawn((
        TextInputNode {
            clear_on_submit: false,
            ..default()
        },
        TextInputBuffer::default(),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            width: Val::Px(600.0),
            height: Val::Px(80.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(15.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(120.0),
            left: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
        BorderColor(Color::srgb(0.3, 0.5, 1.0)),
        BorderRadius::all(Val::Px(5.0)),
        TextInputStyle {
            cursor_color: Color::srgb(1.0, 1.0, 0.0),
            selection_color: Color::srgba(0.3, 0.6, 1.0, 0.4),
            cursor_width: 3.0,
            ..default()
        },
    ));
    
    // Multi-line input for longer Chinese text
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
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(15.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(230.0),
            left: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor(Color::srgb(0.5, 0.5, 0.5)),
        BorderRadius::all(Val::Px(5.0)),
        TextInputStyle::default(),
    ));
    
    // Example text
    commands.spawn((
        Text::new("Example Chinese text:\n你好世界！\n欢迎使用Bevy文本输入插件。\n现在支持中文输入了！🎉"),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(450.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
    
    // Submit info
    commands.spawn((
        Text::new("Press Enter to submit (see console for output)"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
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
        info!("✅ Submitted text: '{}'", event.text);
        info!("   Entity: {:?}", event.entity);
        info!("   Character count: {}", event.text.chars().count());
        info!("   Byte size: {} bytes", event.text.len());
    }
}