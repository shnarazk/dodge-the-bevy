# Organizing the project

```
cargo new
```

```rust
// lib.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Load,
    Setup,
    Game,
    Restart,
}
```

```rust
// main.rs

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dodge!".to_string(),
            width: 1200.0,
            height: 800.0,
            ..Default::default()
        })
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_assets))
        run();
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
```

# Player

```rust
// player.rs

#[derive(Component, Default)]
pub struct Player;
    pub texture_atlas: TextureAtlas,
    pub flip: bool,
    pub diff_x: f32,
    pub diff_y: f32,
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in [
        asset_server.get_handle("sprites/bevy_logo_dark_1.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_2.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_3.png"),
    ] {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.load("sprites/bevy_logo_dark_1.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Z_AXIS),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Player::from(texture_atlas))
}

// from Unofficial Bevy Cheat Book 'Convert cursor to world coodinates'
#[allow(clippy::type_complexity)]
pub fn track_mouse_movement(
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
        if let Some(camera_transform) = queries.q0().iter().next() {
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
}
```

```rust
// main.rs

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dodge!".to_string(),
            width: 1200.0,
            height: 800.0,
            ..Default::default()
        })
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_assets))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(setup_player))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(check_mouse_movement))
        run();
}
```
