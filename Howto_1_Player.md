# Organizing the project

1. Create a crata with `cargo new`.

```
cargo new
```

2. Add bevy into `[dependency]`

3. Set up source tree.

```
assets/
src/
 - lib.rs
 - main.rs
```

4. Define global data.

- `AppState`

```rust
// lib.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Load,    // to load all textures, bgm, shaders
    Setup,   // to configure background, cameras, ssign loaded textures ot srites
    Game,    // to run game
    Restart, // to display RESTART and EXIT buttons and high score
}
```

5. Load assets at the first stage.

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

## Player definition

Now let's define `Player`, the main sprite. Since it's movable. We need a storage to hold its direction, speed and its textures. 

```rust
// player.rs

#[derive(Component, Default)]
pub struct Player;
    pub texture_atlas: TextureAtlas,
    pub flip: bool,
    pub diff_x: f32,
    pub diff_y: f32,
}

impl Character {
    pub fn from(texture_atlas: TextureAtlas) -> Self {
        Self {
            texture_atlas,
            flip: false,
            diff_x: 0.0,
            diff_y: 0.0,
        }
    }
}

```

This storage can be attached to the `bevy::prelude::SpriteBundle` with `bevey_ecs::system::EntityCommands::insert`.

```rust
// player.rs

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let sprite_handles = [
        asset_server.get_handle("sprites/bevy_logo_dark_1.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_2.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_3.png"),
    ];
    for handle in sprite_handles.iter() {
        if let Some(image) = textures.get(handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_index = texture_atlas.get_texture_index(&sprite_handles[0]).unwrap();
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
        .insert(Player::from(texture_atlas))
    ;
}
```

This `setup_player` is a system to be attached to the `App` with `bevy::app::App::add_system`.
Since, however, we use `AppState`, we should use `add_system_set` instead of it.

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
        run();
}
```

### Show it up

We need a camera to see the player.

```rust
// camera.rs

#[derive(Component, Debug, Default)]
pub struct MainCamera {
    pub shaker: Option<u32>,
}

pub fn setup_cammera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera::default());
    commands.spawn_bundle(UiCameraBundle::default());
}
``

```rust
// main.rs

        // Add the following to the chain.
        .add_system_set(
            SystemSet::on_enter(AppState::Setup)
                .with_system(setup_camera)
        )
```

## Player animation

The `Player` holds all sprites used in character animation now.
So we make a system that change displaying sprite periodically.
It uses a `Timer` attached to the player.


```rust
// player.rs

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    ...
        .insert(Timer::from_seconds(0.15, true))
        ;
}

pub fn animate_player(
    config: Res<WindowDescriptor>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Character,
            &mut Timer,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (mut player, mut timer, mut trans, mut sprite) in query.iter_mut() {
        trans.translation.x += player.diff_x);
        trans.translation.y += player.diff_y;

        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % player.texture_atlas.textures.len();
            sprite.flip_x = player.flip;
        }
    }
}
```

Add this system to the App.

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
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(animate_player))
        .run()
        ;
}
```

## Moving the player

Then make this player controllable with mouse position.

- The game window tracks the mouse position (bevy::window::Window::cursor_position).
- We need to map cursor position to the coordinates for sprites.
- We use a constant speed to move to any direction by normalizing 'delta'.

```rust
// player.rs
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
        .init_resource::<GameResourceHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Load)
        .add_system_set(SystemSet::on_enter(AppState::Load).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_assets))
        .add_system_set(
            SystemSet::on_enter(AppState::Setup)
                .with_system(setup_player)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(animate_player)
                .with_system(track_mouse_movement)
        )
        .add_system(exit_on_esc_system)
        .run()
        ;
}
```
