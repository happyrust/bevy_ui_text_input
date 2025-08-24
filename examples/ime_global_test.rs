//! Test if IME events need global handling
use bevy::prelude::*;
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputQueue,
    TextInputStyle, actions::TextInputAction, actions::TextInputEdit,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, global_ime_handler)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        TextInputNode::default(),
        TextInputBuffer::default(),
        TextFont {
            font: asset_server.load("fonts/Songti.ttc"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            width: Val::Px(600.0),
            height: Val::Px(100.0),
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
        BorderColor(Color::WHITE),
        TextInputStyle::default(),
    ));
    
    info!("Test started - try typing Chinese");
}

// Global IME event handler as a system
fn global_ime_handler(
    mut ime_events: EventReader<bevy::window::Ime>,
    mut text_inputs: Query<&mut TextInputQueue, With<TextInputNode>>,
    input_focus: Res<bevy::input_focus::InputFocus>,
) {
    for event in ime_events.read() {
        match event {
            bevy::window::Ime::Commit { value, .. } => {
                info!("✅ Global handler: IME Commit '{}'", value);
                
                // Try to insert text manually
                if let Some(focused) = input_focus.get() {
                    if let Ok(mut queue) = text_inputs.get_mut(focused) {
                        info!("  Inserting into focused entity {:?}", focused);
                        for ch in value.chars() {
                            queue.add(TextInputAction::Edit(TextInputEdit::Insert(ch, false)));
                        }
                    }
                }
            }
            bevy::window::Ime::Preedit { value, .. } if !value.is_empty() => {
                info!("🔍 Global handler: Preedit '{}'", value);
            }
            _ => {}
        }
    }
}