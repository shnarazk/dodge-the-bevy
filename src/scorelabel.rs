use {crate::player::Player, bevy::prelude::*};
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_simple);
    }
}

#[derive(Component)]
pub struct ScoreLabel;

fn setup_simple(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Xolonium-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Xolonium-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreLabel);
}

pub fn simple_text_update(time: Res<Time>, mut query: Query<&mut Text, With<ScoreLabel>>) {
    let seconds = time.seconds_since_startup() as f32;
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", seconds as u32);
    }
}

pub fn update_score(
    mut player_query: Query<&mut Player>,
    mut score_query: Query<&mut Text, With<ScoreLabel>>,
) {
    let mut player = player_query.single_mut();
    player.score += 1.0;
    player.max_score = player.max_score.max(player.score);
    let mut score = score_query.single_mut();
    score.sections[1].value = format!("{}", player.score as u32);
}
