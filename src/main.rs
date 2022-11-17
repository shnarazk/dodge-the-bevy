use {
    bevy::{asset::LoadState, prelude::*, time::FixedTimestep},
    dodge_the_bevy::{
        background::{setup_background, ColoredMesh2dPlugin},
        camera::{animate_camera, setup_camera, shake_camera, MainCamera},
        character::Character,
        collision::check_collision,
        enemy::{animate_enemy, setup_enemy, Enemy},
        player::{animate_player, setup_player, Player},
        restart_panel::{
            hide_restart_panel, restart_panel_system, setup_restart_panel, show_restart_panel,
        },
        score_label::{update_score, ScorePlugin},
        AppState, CollisionEvent, GameOverEvent, RestartEvent,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Dodge!".to_string(),
                width: 1200.0,
                height: 800.0,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 1.0)))
        .init_resource::<GameResourceHandles>()
        .add_plugin(ScorePlugin)
        .add_plugin(ColoredMesh2dPlugin)
        .add_event::<CollisionEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<RestartEvent>()
        .add_state(AppState::Load)
        // from 'state'
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_assets))
        .add_system_set(
            SystemSet::on_enter(AppState::Setup)
                .with_system(setup_background)
                .with_system(setup_camera)
                .with_system(setup_player)
                .with_system(setup_restart_panel)
                .with_system(game_start),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(hide_restart_panel)
                .with_system(play_bgm),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(shake_camera)
                .with_system(animate_camera)
                .with_system(animate_player)
                .with_system(animate_enemy)
                .with_system(check_collision)
                .with_system(track_mouse_movement)
                .with_system(game_over),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_run_criteria(FixedTimestep::step(25.5))
                .with_system(play_bgm),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(setup_enemy)
                .with_run_criteria(FixedTimestep::step(0.55)),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(update_score),
        )
        .add_system_set(SystemSet::on_enter(AppState::Restart).with_system(show_restart_panel))
        .add_system_set(
            SystemSet::on_update(AppState::Restart)
                .with_system(check_restart)
                .with_system(restart_panel_system),
        )
        .run()
}

//
// Configuration
//
// (from texture_atlas)
#[derive(Default, Resource)]
struct GameResourceHandles {
    sprites: Vec<HandleUntyped>,
    sounds: Vec<HandleUntyped>,
}

fn load_assets(mut handles: ResMut<GameResourceHandles>, asset_server: ResMut<AssetServer>) {
    handles.sprites = asset_server.load_folder("sprites").unwrap();
    handles.sounds = asset_server.load_folder("sounds").unwrap();
}

fn check_assets(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<GameResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.sprites.iter().map(|handle| handle.id))
    {
        state.set(AppState::Setup).unwrap();
    }
}

// from Unofficial Bevy Cheat Book 'Convert cursor to world coodinates'
#[allow(clippy::type_complexity)]
fn track_mouse_movement(
    windows: ResMut<Windows>,
    mut queries: ParamSet<(
        Query<&Transform, With<MainCamera>>,
        Query<&mut Character, With<Player>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = position - size / 2.0;
        if let Some(camera_transform) = queries.p0().iter().next() {
            let clicked = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            let mut q1 = queries.p1();
            let mut player = q1.single_mut();
            let dx = clicked.x - player.trans_x;
            let dy = clicked.y - player.trans_y;
            let dist2 = dx.powi(2) + dy.powi(2);
            if 100.0 < dist2 {
                let dist = dist2.sqrt();
                player.flip = dx < 0.0;
                player.diff_x = 10.0 * dx / dist;
                player.diff_y = 10.0 * dy / dist;
            } else {
                player.flip = false;
                player.diff_x = 0.0;
                player.diff_y = 0.0;
            }
        }
    }
}

//
// BGM
//
fn play_bgm(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.get_handle("sounds/House In a Forest Loop.ogg");
    audio.play(music);
}

fn game_start(mut state: ResMut<State<AppState>>) {
    state.set(AppState::Game).unwrap();
}

fn game_over(
    mut commands: Commands,
    mut enemies: Query<Entity, With<Enemy>>,
    mut game_end: EventReader<GameOverEvent>,
    mut state: ResMut<State<AppState>>,
) {
    if game_end.iter().next().is_some() {
        state.set(AppState::Restart).unwrap();
        for ent in enemies.iter_mut() {
            commands.entity(ent).despawn();
        }
    }
}

fn check_restart(
    mut restart_channel: EventReader<RestartEvent>,
    mut state: ResMut<State<AppState>>,
) {
    if restart_channel.iter().next().is_some() {
        state.set(AppState::Game).unwrap();
    }
}
