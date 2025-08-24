//! Sticky note example that mimics VeloNode styling

use bevy::{
    prelude::*,
    window::WindowResized,
};
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputPrompt, TextInputStyle,
    TextSubmitEvent,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_submit, animate_shadow, handle_resize))
        .run();
}

#[derive(Component)]
struct StickyNote;

#[derive(Component)]
struct NoteShadow;

#[derive(Component)]
struct NoteContainer;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Create a canvas-like background with grid
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("📝 Sticky Notes"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Container for sticky notes
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(30.0),
                        row_gap: Val::Px(30.0),
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        max_width: Val::Px(1200.0),
                        ..default()
                    },
                    NoteContainer,
                ))
                .with_children(|parent| {
                    // Define sticky notes data
                    let notes = [
                        (Color::srgb(1.0, 0.925, 0.675), "Type your ideas here...\n\nThis is a sticky note with VeloNode styling!", -0.02),
                        (Color::srgb(0.678, 0.847, 0.902), "Another note with different color...\n\n✨ Supports emojis!", 0.015),
                        (Color::srgb(0.596, 0.984, 0.596), "Tasks:\n• Build amazing apps\n• Learn Bevy\n• Have fun!", -0.01),
                        (Color::srgb(1.0, 0.753, 0.796), "Important reminder:\n\nDon't forget to save your work!", 0.025),
                    ];

                    // Create each sticky note
                    for (bg_color, _text, rotation) in notes.iter() {
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(250.0),
                                    height: Val::Px(250.0),
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                                Transform::from_rotation(Quat::from_rotation_z(*rotation)),
                                StickyNote,
                            ))
                            .with_children(|note_parent| {
                                // Shadow layer
                                note_parent.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        left: Val::Px(4.0),
                                        top: Val::Px(4.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
                                    BorderColor(Color::NONE),
                                    BorderRadius::all(Val::Px(2.0)),
                                    NoteShadow,
                                ));

                                // Main note container
                                note_parent
                                    .spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            padding: UiRect::all(Val::Px(15.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        BackgroundColor(*bg_color),
                                        BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.1)),
                                        BorderRadius::all(Val::Px(2.0)),
                                    ))
                                    .with_children(|content_parent| {
                                        // Text input area
                                        content_parent.spawn((
                                            TextInputNode::default(),
                                            TextInputBuffer::default(),
                                            TextInputStyle {
                                                cursor_color: Color::srgb(0.2, 0.2, 0.2),
                                                selection_color: Color::srgba(0.3, 0.5, 0.8, 0.3),
                                                selected_text_color: None,
                                                ..default()
                                            },
                                            TextInputPrompt::new("Write something..."),
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.1, 0.1, 0.1)),
                                            BackgroundColor(Color::NONE),
                                        ));
                                        
                                        // Small decorative element (like a pin)
                                        content_parent
                                            .spawn((
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Px(20.0),
                                                    height: Val::Px(20.0),
                                                    top: Val::Px(-10.0),
                                                    left: Val::Percent(50.0),
                                                    margin: UiRect::left(Val::Px(-10.0)),
                                                    border: UiRect::all(Val::Px(2.0)),
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                                                BorderColor(Color::srgb(0.6, 0.1, 0.1)),
                                                BorderRadius::all(Val::Percent(50.0)),
                                            ))
                                            .with_children(|pin_parent| {
                                                // Inner circle for the pin
                                                pin_parent.spawn((
                                                    Node {
                                                        position_type: PositionType::Absolute,
                                                        width: Val::Px(8.0),
                                                        height: Val::Px(8.0),
                                                        top: Val::Px(4.0),
                                                        left: Val::Px(4.0),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.9, 0.3, 0.3)),
                                                    BorderRadius::all(Val::Percent(50.0)),
                                                ));
                                            });
                                    });
                            });
                    }
                });
        });
}

fn handle_submit(mut events: EventReader<TextSubmitEvent>) {
    for event in events.read() {
        println!("Submitted text from entity {:?}: {}", event.entity, event.text);
    }
}

fn animate_shadow(time: Res<Time>, mut query: Query<&mut Node, With<NoteShadow>>) {
    let offset = (time.elapsed_secs() * 2.0).sin() * 2.0;
    for mut style in query.iter_mut() {
        style.left = Val::Px(4.0 + offset);
        style.top = Val::Px(4.0 + offset);
    }
}

fn handle_resize(
    mut resize_events: EventReader<WindowResized>,
    mut container_query: Query<&mut Node, With<NoteContainer>>,
) {
    for _event in resize_events.read() {
        for mut style in container_query.iter_mut() {
            // Adjust layout based on window size
            style.flex_wrap = FlexWrap::Wrap;
        }
    }
}