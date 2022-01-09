#![allow(unused)]

use bevy::render::view::visibility;
use {
    crate::{player::Player, AppState, GameOverEvent, RestartEvent},
    bevy::{app::AppExit, prelude::*},
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug, Default)]
pub struct GameButtonsPlugin;

#[derive(Component, Debug, Default)]
pub struct GameButton {
    exit: bool,
}

#[derive(Component, Debug, Default)]
pub struct GameButtonLabel;

impl Plugin for GameButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_restart_panel)
            .add_system(game_button_system);
    }
}

#[allow(clippy::type_complexity)]
pub fn show_restart_panel(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<&mut Visibility, With<GameButton>>,
        QueryState<&mut Visibility, With<GameButtonLabel>>,
    )>,
) {
    for mut visibility in query.q0().iter_mut() {
        visibility.is_visible = true;
    }
    for mut visibility in query.q1().iter_mut() {
        visibility.is_visible = true;
    }
}

#[allow(clippy::type_complexity)]
pub fn hide_restart_panel(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<&mut Visibility, With<GameButton>>,
        QueryState<&mut Visibility, With<GameButtonLabel>>,
    )>,
) {
    for mut visibility in query.q0().iter_mut() {
        visibility.is_visible = false;
    }
    for mut visibility in query.q1().iter_mut() {
        visibility.is_visible = false;
    }
}

#[allow(clippy::type_complexity)]
pub fn game_button_system(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiColor,
            &Children,
            &GameButton,
            &mut Visibility,
        ),
        (Changed<Interaction>, With<GameButton>),
    >,
) {
    let mut changed = false;
    for (interaction, mut color, children, button, mut visibility) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                if button.exit {
                    app_exit_events.send(AppExit);
                } else {
                    restart_events.send(RestartEvent);
                }
                changed = true;
            }
            Interaction::Hovered => {
                // text.sections[0].value = "QUIT".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
    if changed {
        for (_, _, _, _, mut visibility) in interaction_query.iter_mut() {
            visibility.is_visible = false;
        }
    }
}

pub fn setup_restart_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Xolonium-Regular.ttf");
    let font_size = 40.0;
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                // position_type: PositionType::Absolute,
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Restart",
                        TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(GameButtonLabel);
        })
        .insert(GameButton { exit: false });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                // position_type: PositionType::Absolute,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Exit",
                        TextStyle {
                            font,
                            font_size,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(GameButtonLabel);
        })
        .insert(GameButton { exit: true });
}
