//! 极简的IME测试
//! Minimal IME test to debug IME events at system level

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ime_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Text::new("Try typing Chinese characters - check console"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
    
    info!("🔍 IME Test started - type Chinese characters and check console");
}

fn ime_system(
    mut ime_events: EventReader<bevy::window::Ime>,
) {
    for event in ime_events.read() {
        match event {
            bevy::window::Ime::Preedit { value, cursor, .. } => {
                info!("🔍 Raw IME Preedit: '{}' (cursor: {:?})", value, cursor);
            }
            bevy::window::Ime::Commit { value, .. } => {
                info!("✅ Raw IME Commit: '{}'", value);
            }
            bevy::window::Ime::Enabled { .. } => {
                info!("🟢 Raw IME Enabled");
            }
            bevy::window::Ime::Disabled { .. } => {
                info!("🔴 Raw IME Disabled");
            }
        }
    }
}
