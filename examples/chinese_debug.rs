//! 中文输入调试测试示例
//! Chinese Input Debug Test Example

use bevy::ecs::message::MessageReader;
use bevy::{
    color::palettes::css::{DARK_BLUE, LIGHT_BLUE, NAVY, WHITE, YELLOW},
    input_focus::InputFocus,
    prelude::*,
    window::Ime,
    winit::WinitWindows,
};
use bevy_ui_text_input::{
    SubmitText, TextInputBuffer, TextInputNode, TextInputPlugin, TextInputPrompt, TextInputStyle,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                submit_system,
                debug_ime_system,
                debug_text_buffer,
                debug_focus_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI 相机
    commands.spawn(Camera2d);

    // 检查可用的系统字体
    info!("Setting up Chinese input test...");

    // 主容器
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(20.0)),
        row_gap: Val::Px(20.0),
        ..default()
    })
    .with_children(|parent| {
        // 标题
        parent.spawn((
            Text::new("中文输入调试测试 / Chinese Input Debug Test"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 28.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ));

        // 说明文字
        parent.spawn((
            Text::new("请尝试输入中文 - 调试信息将显示在控制台\n(Try typing Chinese - debug info will show in console)"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 16.0,
                ..default()
            },
            TextColor(LIGHT_BLUE.into()),
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ));

        // 文本输入框 - 使用系统字体
        parent.spawn((
            TextInputNode {
                clear_on_submit: false,
                ..default()
            },
            TextInputPrompt {
                text: "请输入中文...".to_string(),
                color: Some(Color::srgb(0.5, 0.5, 0.5)),
                ..default()
            },
            TextInputBuffer::default(),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"), // 使用宋体
                font_size: 24.0,
                ..default()
            },
            TextColor(WHITE.into()),
            Node {
                width: Val::Px(600.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(3.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
            BorderColor(YELLOW.into()),
            TextInputStyle {
                cursor_color: YELLOW.into(),
                selection_color: Color::srgb(0.3, 0.3, 0.7),
                ..default()
            },
            DebugTextInput, // 标记用于调试
        ));

        // 手动输入中文文本测试
        parent.spawn((
            Text::new("手动测试：你好世界 Hello World"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
        ));
    });
}

#[derive(Component)]
struct DebugTextInput;

// 调试IME事件
fn debug_ime_system(mut ime_events: EventReader<Ime>) {
    for event in ime_events.read() {
        match event {
            Ime::Preedit { value, cursor, .. } => {
                info!("🔍 IME Preedit: '{}' (cursor: {:?})", value, cursor);
            }
            Ime::Commit { value, .. } => {
                info!("✅ IME Commit: '{}'", value);
                for (i, ch) in value.chars().enumerate() {
                    info!("    Char {}: '{}' (U+{:04X})", i, ch, ch as u32);
                }
            }
            Ime::Enabled { .. } => {
                info!("🟢 IME Enabled");
            }
            Ime::Disabled { .. } => {
                info!("🔴 IME Disabled");
            }
        }
    }
}

// 调试文本缓冲区变化
fn debug_text_buffer(
    mut query: Query<&TextInputBuffer, (With<DebugTextInput>, Changed<TextInputBuffer>)>,
) {
    for buffer in query.iter_mut() {
        let text = buffer.get_text();
        info!("📝 TextBuffer changed: '{}'", text);
        for (i, ch) in text.chars().enumerate() {
            info!("    Buffer char {}: '{}' (U+{:04X})", i, ch, ch as u32);
        }
    }
}

// Debug focus changes
fn debug_focus_system(
    focus: Res<InputFocus>,
    windows: Query<Entity, With<bevy::window::Window>>,
    winit_windows: Option<NonSend<WinitWindows>>,
    mut manual_ime_enabled: Local<bool>,
) {
    if focus.is_changed() {
        info!("🎯 Focus changed: {:?}", focus.get());

        // Manually enable IME if focus changed to a text input
        if !*manual_ime_enabled {
            if let Some(winit_windows) = winit_windows {
                for window_entity in windows.iter() {
                    if let Some(winit_window) = winit_windows.get_window(window_entity) {
                        info!("🔧 Manually enabling IME due to focus change");
                        winit_window.set_ime_allowed(true);
                        *manual_ime_enabled = true;
                        break;
                    }
                }
            }
        }
    }
}

fn submit_system(mut events: MessageReader<SubmitText>) {
    for event in events.read() {
        info!("🚀 Text submitted: '{}'", event.text);
    }
}
