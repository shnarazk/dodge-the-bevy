use {
    bevy::{asset::LoadState, core::FixedTimestep, input::system::exit_on_esc_system, prelude::*},
    dodge_the_bevy::{
        background::setup_background,
        camera::{animate_camera, setup_cammera, shake_camera, MainCamera},
        character::Character,
        collision::check_collision,
        enemy::{animate_enemy, setup_enemy},
        player::{animate_player, setup_player, Player},
        scorelabel::{update_score, ScorePlugin},
        CollisionEvent,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Ready,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dodge!".to_string(),
            width: 1200.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 1.0)))
        .init_resource::<GameResourceHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScorePlugin)
        .add_event::<CollisionEvent>()
        .add_state(AppState::Setup)
        // from 'state'
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup_background))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_cammera))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_player))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(play_bgm))
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(25.5))
                .with_system(play_bgm),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(0.55))
                .with_system(setup_enemy),
        )
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(shake_camera))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_camera))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_player))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_enemy))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(check_collision))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(track_mouse_movement))
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(update_score),
        )
        .add_system(exit_on_esc_system)
        .run()
}

//
// Configuration
//
// (from texture_atlas)
#[derive(Default)]
struct GameResourceHandles {
    sprites: Vec<HandleUntyped>,
    sounds: Vec<HandleUntyped>,
}

fn load_assets(mut handles: ResMut<GameResourceHandles>, asset_server: ResMut<AssetServer>) {
    handles.sprites = asset_server.load_folder("sprites").unwrap();
    handles.sounds = asset_server.load_folder("sounds").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<GameResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.sprites.iter().map(|handle| handle.id))
    {
        state.set(AppState::Ready).unwrap();
    }
}

// from Unofficial Bevy Cheat Book 'Convert cursor to world coodinates'
#[allow(clippy::type_complexity)]
fn track_mouse_movement(
    windows: ResMut<Windows>,
    mut queries: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Character, With<Player>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = position - size / 2.0;
        let camera_transform = queries.q0().single();
        let clicked = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mut q1 = queries.q1();
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

//
// BGM
//
#[allow(dead_code)]
fn play_bgm(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.get_handle("sounds/House In a Forest Loop.ogg");
    // let music = asset_server.get_handle("sounds/Windless Slopes.ogg");
    audio.play(music);
}
