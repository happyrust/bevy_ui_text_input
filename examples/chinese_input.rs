//! 中文输入测试示例
//! Chinese Input Test Example

use bevy::{
    color::palettes::css::{DARK_BLUE, LIGHT_BLUE, NAVY, RED, WHITE, YELLOW},
    prelude::*,
};
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputPrompt, TextInputQueue,
    TextInputStyle, TextSubmitEvent,
};

#[derive(Resource)]
struct ChineseFonts {
    songti: Handle<Font>,
    heiti: Handle<Font>,
}

#[derive(Component)]
struct FontSwitchButton {
    font_type: FontType,
}

#[derive(Clone)]
enum FontType {
    Songti,
    Heiti,
}

#[derive(Component)]
struct OutputText;

#[derive(Component)]
struct TextInputEntity;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (font_button_system, submit_system, button_interaction))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 加载中文字体
    let fonts = ChineseFonts {
        songti: asset_server.load("fonts/Songti.ttc"),
        heiti: asset_server.load("fonts/STHeiti Medium.ttc"),
    };
    commands.insert_resource(fonts);

    // UI 相机
    commands.spawn(Camera2d);

    // 主容器
    let root = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(20.0),
            ..default()
        })
        .id();

    // 标题
    let title = commands
        .spawn((
            Text::new("中文输入测试 / Chinese Input Test"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 32.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ))
        .id();

    // 说明文字
    let instruction = commands
        .spawn((
            Text::new("请尝试输入中文字符 (Please try typing Chinese characters)\n使用IME输入法，如拼音输入法 (Use IME input method like Pinyin)"),
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
        ))
        .id();

    // 字体切换按钮容器
    let button_container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .id();

    // 宋体按钮
    let songti_button = commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            Button,
            BackgroundColor(DARK_BLUE.into()),
            BorderColor(WHITE.into()),
            FontSwitchButton { font_type: FontType::Songti },
        ))
        .with_child((
            Text::new("宋体 Songti"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 16.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ))
        .id();

    // 黑体按钮
    let heiti_button = commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            Button,
            BackgroundColor(DARK_BLUE.into()),
            BorderColor(WHITE.into()),
            FontSwitchButton { font_type: FontType::Heiti },
        ))
        .with_child((
            Text::new("黑体 Heiti"),
            TextFont {
                font: asset_server.load("fonts/STHeiti Medium.ttc"),
                font_size: 16.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ))
        .id();


    // 文本输入框
    let text_input = commands
        .spawn((
            TextInputNode {
                clear_on_submit: false,
                ..default()
            },
            TextInputPrompt {
                text: "请输入中文文本... (Please input Chinese text...)".to_string(),
                color: Some(Color::srgb(0.5, 0.5, 0.5)),
                ..default()
            },
            TextInputBuffer::default(),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 24.0,
                ..default()
            },
            TextColor(WHITE.into()),
            Node {
                width: Val::Px(600.0),
                height: Val::Px(200.0),
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
            TextInputEntity,
        ))
        .id();

    // 提交按钮
    let submit_button = commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Button,
            BackgroundColor(NAVY.into()),
            BorderColor(WHITE.into()),
        ))
        .with_child((
            Text::new("提交 Submit"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 18.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ))
        .observe(move |_: Trigger<Pointer<Click>>, mut query: Query<&mut TextInputQueue>| {
            if let Ok(mut queue) = query.get_mut(text_input) {
                queue.add(bevy_ui_text_input::actions::TextInputAction::Submit);
            }
        })
        .id();

    // 输出显示区域
    let output_area = commands
        .spawn((
            Node {
                width: Val::Px(600.0),
                min_height: Val::Px(100.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            BorderColor(WHITE.into()),
        ))
        .with_child((
            Text::new("输出将显示在这里... (Output will appear here...)"),
            TextFont {
                font: asset_server.load("fonts/Songti.ttc"),
                font_size: 20.0,
                ..default()
            },
            TextColor(LIGHT_BLUE.into()),
            OutputText,
        ))
        .id();

    // 组装界面
    commands.entity(button_container).add_children(&[songti_button, heiti_button]);
    commands.entity(root).add_children(&[
        title,
        instruction,
        button_container,
        text_input,
        submit_button,
        output_area,
    ]);
}

fn font_button_system(
    mut interaction_query: Query<(&Interaction, &FontSwitchButton), (Changed<Interaction>, With<Button>)>,
    mut text_input_query: Query<&mut TextFont, With<TextInputEntity>>,
    fonts: Res<ChineseFonts>,
) {
    for (interaction, font_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text_font) = text_input_query.get_single_mut() {
                match font_button.font_type {
                    FontType::Songti => text_font.font = fonts.songti.clone(),
                    FontType::Heiti => text_font.font = fonts.heiti.clone(),
                }
            }
        }
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                bg_color.0 = RED.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                bg_color.0 = Color::srgb(0.2, 0.2, 0.8);
                border_color.0 = YELLOW.into();
            }
            Interaction::None => {
                bg_color.0 = DARK_BLUE.into();
                border_color.0 = WHITE.into();
            }
        }
    }
}

fn submit_system(
    mut events: EventReader<TextSubmitEvent>,
    mut output_query: Query<&mut Text, With<OutputText>>,
) {
    for event in events.read() {
        for mut text in output_query.iter_mut() {
            text.0 = format!("提交的内容: {}\nSubmitted: {}", event.text, event.text);
        }
    }
}
