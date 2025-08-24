//! Debug IME commit not showing issue
use bevy::prelude::*;
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputQueue,
    TextInputStyle, actions::TextInputAction, actions::TextInputEdit,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            debug_ime_events,
            debug_input_focus,
            debug_queue_processing,
            debug_buffer_contents,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    // Text input with marker component
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
        DebugMarker,
    ));
    
    commands.spawn((
        Text::new("Click the input field and type Chinese characters"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(100.0),
            ..default()
        },
    ));
}

#[derive(Component)]
struct DebugMarker;

fn debug_ime_events(
    mut ime_events: EventReader<bevy::window::Ime>,
) {
    for event in ime_events.read() {
        match event {
            bevy::window::Ime::Preedit { value, cursor, .. } => {
                if !value.is_empty() {
                    info!("🔍 IME Preedit: '{}' (cursor: {:?})", value, cursor);
                }
            }
            bevy::window::Ime::Commit { value, .. } => {
                info!("✅ IME Commit received: '{}'", value);
                info!("  -> Characters: {:?}", value.chars().collect::<Vec<_>>());
                info!("  -> Bytes: {}", value.len());
            }
            bevy::window::Ime::Enabled { .. } => {
                info!("🟢 IME Enabled");
            }
            bevy::window::Ime::Disabled { .. } => {
                info!("🔴 IME Disabled");
            }
        }
    }
}

fn debug_input_focus(
    input_focus: Res<bevy::input_focus::InputFocus>,
    query: Query<Entity, With<DebugMarker>>,
) {
    if input_focus.is_changed() {
        if let Some(focused) = input_focus.get() {
            info!("📍 Input focus: {:?}", focused);
            for entity in query.iter() {
                if entity == focused {
                    info!("  -> Our text input is focused!");
                }
            }
        } else {
            info!("📍 No input focused");
        }
    }
}

fn debug_queue_processing(
    query: Query<(&TextInputQueue, Entity), (With<DebugMarker>, Changed<TextInputQueue>)>,
) {
    for (queue, entity) in query.iter() {
        if !queue.actions.is_empty() {
            info!("📝 Queue for {:?} has {} actions pending", entity, queue.actions.len());
            for (i, action) in queue.actions.iter().enumerate() {
                info!("    Action {}: {:?}", i, action);
            }
        }
    }
}

fn debug_buffer_contents(
    query: Query<&TextInputBuffer, (With<DebugMarker>, Changed<TextInputBuffer>)>,
) {
    for buffer in query.iter() {
        let text = buffer.get_text();
        if !text.is_empty() {
            info!("📄 Buffer updated: '{}'", text);
            info!("  -> Length: {} chars, {} bytes", text.chars().count(), text.len());
        }
    }
}