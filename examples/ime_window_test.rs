//! IME window configuration test
//! Testing explicit window settings for IME

use bevy::{prelude::*, window::WindowPlugin, winit::WinitWindows};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "IME Test Window".to_string(),
                    // Try explicit IME properties if available
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (ime_system, explicit_ime_enable))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Text::new("IME Window Test - Try typing Chinese characters"),
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
    
    info!("🔍 IME Window Test started");
}

fn explicit_ime_enable(
    windows: Query<Entity, With<Window>>,
    winit_windows: Option<NonSend<WinitWindows>>,
    mut enabled: Local<bool>,
) {
    if *enabled {
        return;
    }
    
    let Some(winit_windows) = winit_windows else {
        return;
    };
    
    for window_entity in windows.iter() {
        if let Some(winit_window) = winit_windows.get_window(window_entity) {
            info!("🔧 Manually enabling IME on window");
            winit_window.set_ime_allowed(true);
            *enabled = true;
        }
    }
}

fn ime_system(
    mut ime_events: EventReader<bevy::window::Ime>,
) {
    for event in ime_events.read() {
        match event {
            bevy::window::Ime::Preedit { value, cursor, .. } => {
                info!("🔍 IME Preedit: '{}' (cursor: {:?})", value, cursor);
            }
            bevy::window::Ime::Commit { value, .. } => {
                info!("✅ IME Commit: '{}'", value);
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
