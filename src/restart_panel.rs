#![allow(unused)]

use bevy::render::view::visibility;
use {
    crate::{player::Player, AppState, GameOverEvent, RestartEvent},
    bevy::{app::AppExit, prelude::*},
};

const NORMAL_BUTTON: Color = Color::rgb(0.05, 0.05, 0.05);
const HOVERED_BUTTON: Color = Color::rgb(0.05, 0.25, 0.95);
const PRESSED_BUTTON: Color = Color::rgb(1.00, 0.25, 0.25);

#[derive(Component, Debug, Default)]
pub struct GameButtonsPlugin;

#[derive(Component, Debug, Default)]
pub struct GameButton {
    exit: bool,
}

#[derive(Component, Debug, Default)]
pub struct HighScoreLabel;

impl Plugin for GameButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_restart_panel)
            .add_system(restart_panel_system);
    }
}

#[allow(clippy::type_complexity)]
pub fn show_restart_panel(
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut query: ParamSet<(
        Query<&mut Style, With<GameButton>>,
        Query<(&mut Style, &mut Text), With<HighScoreLabel>>,
    )>,
) {
    for mut style in query.p0().iter_mut() {
        style.display = Display::Flex;
    }
    for (mut style, mut text) in query.p1().iter_mut() {
        style.display = Display::Flex;
        text.sections[0].value = format!(
            "Your high score is {:0>4.0}",
            player.iter().next().map_or(0.0, |p| p.max_score)
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn hide_restart_panel(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<&mut Style, With<GameButton>>,
        Query<&mut Style, With<HighScoreLabel>>,
    )>,
) {
    for mut style in query.p0().iter_mut() {
        style.display = Display::None;
    }
    for mut style in query.p1().iter_mut() {
        style.display = Display::None;
    }
}

#[allow(clippy::type_complexity)]
pub fn restart_panel_system(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &GameButton,
            &mut Visibility,
        ),
        (Changed<Interaction>, With<GameButton>),
    >,
) {
    for (interaction, mut color, children, button, mut visibility) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if button.exit {
                    app_exit_events.send(AppExit);
                } else {
                    restart_events.send(RestartEvent);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_restart_panel(
    mut commands: Commands,
    mut player: Query<&mut Player>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Xolonium-Regular.ttf");
    let font_size = 40.0;
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(30.0),
                    top: Val::Percent(20.0),
                    ..Default::default()
                },
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            text: Text::from_section(
                format!(
                    "Your high score is {:0>4}",
                    player.iter().next().map_or(0.0, |p| p.max_score)
                ),
                TextStyle {
                    font: font.clone(),
                    font_size,
                    color: Color::rgb(1.0, 0.3, 0.3),
                },
            ),
            ..Default::default()
        })
        .insert(HighScoreLabel);
    commands
        .spawn(ButtonBundle {
            style: Style {
                display: Display::None,
                // position_type: PositionType::Absolute,
                size: Size::new(Val::Px(250.0), Val::Px(80.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Restart",
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: Color::rgb(0.6, 0.9, 0.8),
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameButton { exit: false });
    commands
        .spawn(ButtonBundle {
            style: Style {
                display: Display::None,
                // position_type: PositionType::Absolute,
                size: Size::new(Val::Px(250.0), Val::Px(80.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Exit",
                    TextStyle {
                        font,
                        font_size,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                ),
                ..Default::default()
            });
        })
        .insert(GameButton { exit: true });
}
