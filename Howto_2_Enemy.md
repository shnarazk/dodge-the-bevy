# Enemy scene

## Refactoring 

Player and the Enemy share properties about moving.
Some move them to a sharable entity `Character`.


```rust
// charactor.rs
#[derive(Component, Debug)]
pub struct Character {
    pub texture_atlas: TextureAtlas,
    pub flip: bool,
    pub diff_x: f32,
    pub diff_y: f32,
    pub trans_x: f32,
    pub trans_y: f32,
}

impl Character {
    pub fn from(texture_atlas: TextureAtlas) -> Self {
        Self {
            texture_atlas,
            flip: false,
            diff_x: 0.0,
            diff_y: 0.0,
            trans_x: 0.0,
            trans_y: 0.0,
        }
    }
    pub fn with_direction(mut self, x: f32, y: f32) -> Self {
        self.diff_x = x;
        self.diff_y = y;
        self
    }
}
```

`Player` adapts to `Character`.

```rust
// player.rs
#[derive(Component, Default)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    ...
    commands
        .spawn_bundle(SpriteSheetBundle { ... })
        .insert(Character::from(texture_atlas))
        .insert(Player::default());
}
```

## Enemy setup

```rust
// enemy.rs

```

## Spawn mobs


## Main action


## Removing old creeps or reusing them after moving out of the screen
