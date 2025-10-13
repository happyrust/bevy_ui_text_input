//! Debug IME commit issues
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, RenderCreation, WgpuSettings},
};
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputQueue, TextInputStyle,
    actions::TextInputAction, actions::TextInputEdit,
};
use std::panic::{self, AssertUnwindSafe};

fn main() {
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    if let Err(err) = panic::catch_unwind(AssertUnwindSafe(run_app)) {
        panic::set_hook(previous_hook);
        eprintln!(
            "Failed to start `ime_debug` example: {}",
            display_panic(err)
        );
        eprintln!(
            "Tip: run on a machine with GPU access or point WGPU to a software adapter via `WGPU_ADAPTER_NAME`."
        );
    } else {
        panic::set_hook(previous_hook);
    }
}

fn display_panic(err: Box<dyn std::any::Any + Send>) -> String {
    match err.downcast::<String>() {
        Ok(message) => *message,
        Err(err) => match err.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "unknown panic".to_string(),
        },
    }
}

fn run_app() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    force_fallback_adapter: true,
                    backends: Some(Backends::PRIMARY | Backends::GL),
                    ..default()
                }),
                ..default()
            }),
            TextInputPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (ime_debug_system, debug_queue))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Text input
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
        BorderColor::all(Color::WHITE),
        TextInputStyle::default(),
    ));

    // Instructions
    commands.spawn((
        Text::new("Type Chinese characters and check console for debug info"),
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

fn ime_debug_system(
    mut ime_events: MessageReader<bevy::window::Ime>,
    mut text_inputs: Query<&mut TextInputQueue>,
    input_focus: Res<bevy::input_focus::InputFocus>,
) {
    for event in ime_events.read() {
        match event {
            bevy::window::Ime::Preedit { value, cursor, .. } => {
                info!("🔍 IME Preedit: '{}' (cursor: {:?})", value, cursor);
            }
            bevy::window::Ime::Commit { value, .. } => {
                info!("✅ IME Commit: '{}'", value);
                info!("  Characters: {:?}", value.chars().collect::<Vec<_>>());

                // Manual insertion for debugging
                if let Some(entity) = input_focus.get() {
                    if let Ok(mut queue) = text_inputs.get_mut(entity) {
                        info!("  Adding to queue for entity {:?}", entity);
                        for ch in value.chars() {
                            info!("    Inserting char: '{}' (U+{:04X})", ch, ch as u32);
                            queue.add(TextInputAction::Edit(TextInputEdit::Insert(ch, false)));
                        }
                    } else {
                        error!("  Failed to get TextInputQueue for entity {:?}", entity);
                    }
                } else {
                    error!("  No focused entity!");
                }
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

fn debug_queue(query: Query<(Entity, &TextInputQueue, &TextInputBuffer), Changed<TextInputQueue>>) {
    for (entity, queue, buffer) in query.iter() {
        if !queue.actions.is_empty() {
            info!(
                "📝 Queue for {:?} has {} actions",
                entity,
                queue.actions.len()
            );
        }
        let text = buffer.get_text();
        if !text.is_empty() {
            info!("📄 Buffer text: '{}'", text);
        }
    }
}
