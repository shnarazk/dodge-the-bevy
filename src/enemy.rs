use {
    crate::{
        character::{Character, SpawnTimer},
        AppState, Z_AXIS,
    },
    bevy::prelude::*,
    rand::prelude::random,
};

//
// Enemy
//
#[derive(Debug)]
pub enum EnemyKind {
    Fly,
    Swim,
    Walk,
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub kind: EnemyKind,
    pub collided: bool,
}

pub fn setup_enemy(
    state: ResMut<State<AppState>>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if *state.current() != AppState::Game {
        return;
    }
    let window = windows.get_primary().unwrap();
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let (kind, sprites) = match (random::<f32>() * 3.0) as usize {
        1 => (
            EnemyKind::Swim,
            [
                asset_server.get_handle("sprites/enemySwimming_1.png"),
                asset_server.get_handle("sprites/enemySwimming_2.png"),
            ],
        ),
        2 => (
            EnemyKind::Walk,
            [
                asset_server.get_handle("sprites/enemyWalking_1.png"),
                asset_server.get_handle("sprites/enemyWalking_2.png"),
            ],
        ),
        _ => (
            EnemyKind::Fly,
            [
                asset_server.get_handle("sprites/enemyFlyingAlt_1.png"),
                asset_server.get_handle("sprites/enemyFlyingAlt_2.png"),
            ],
        ),
    };
    for handle in sprites {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = match kind {
        EnemyKind::Fly => asset_server.get_handle("sprites/enemyFlyingAlt_1.png"),
        EnemyKind::Swim => asset_server.get_handle("sprites/enemySwimming_1.png"),
        EnemyKind::Walk => asset_server.get_handle("sprites/enemyWalking_1.png"),
    };
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    let mut px = 0.5 * random::<f32>() * window.width();
    let mut py = 0.5 * random::<f32>() * window.height();
    let mut dx;
    let mut dy;
    match (random::<f32>() * 4.0) as usize {
        1 => {
            px = window.width() * 0.5 - 40.0;
            dx = -1.0;
            dy = random::<f32>() - 0.5;
        }
        2 => {
            px = -(window.width() * 0.5 - 40.0);
            dx = 1.0;
            dy = random::<f32>() - 0.5;
        }
        3 => {
            py = window.height() * 0.5 - 40.0;
            dx = random::<f32>() - 0.5;
            dy = -1.0;
        }
        _ => {
            py = -(window.height() * 0.5 - 40.0);
            dx = random::<f32>() - 0.5;
            dy = 1.0;
        }
    }
    const SPEED: f32 = 7.5;
    let dist: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
    assert!(dist < 2.0);
    dx *= SPEED / dist;
    dy *= SPEED / dist;
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(px, py, Z_AXIS),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(SpawnTimer(Timer::from_seconds(0.15, TimerMode::Once)))
        .insert(Character::from(texture_atlas).with_direction(dx, dy))
        .insert(Enemy {
            kind,
            collided: false,
        });
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
pub fn animate_enemy(
    // mut commands: Commands,
    windows: Res<Windows>,
    time: Res<Time>,
    mut query: Query<(
        // Entity,
        &mut Character,
        &mut SpawnTimer,
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut Enemy,
    )>,
) {
    let window = windows.get_primary().unwrap();
    for (mut enemy, mut timer, mut trans, mut sprite, mut et) in query.iter_mut() {
        trans.translation.x += enemy.diff_x;
        trans.translation.y += enemy.diff_y;
        trans.rotation = Quat::from_rotation_z(enemy.diff_y.atan2(enemy.diff_x));
        enemy.trans_x = trans.translation.x;
        enemy.trans_y = trans.translation.y;
        enemy.diff_x *= 1.01;
        enemy.diff_y *= 1.01;
        if 0.5 * window.width() < enemy.trans_x.abs() && 0.5 * window.height() < enemy.trans_y.abs()
        {
            // commands.entity(ent).despawn();

            let mut px = 0.5 * random::<f32>() * window.width();
            let mut py = 0.5 * random::<f32>() * window.height();
            let mut dx;
            let mut dy;
            match (random::<f32>() * 4.0) as usize {
                1 => {
                    px = window.width() * 0.5 - 40.0;
                    dx = -1.0;
                    dy = random::<f32>() - 0.5;
                }
                2 => {
                    px = -(window.width() * 0.5 - 40.0);
                    dx = 1.0;
                    dy = random::<f32>() - 0.5;
                }
                3 => {
                    py = window.height() * 0.5 - 40.0;
                    dx = random::<f32>() - 0.5;
                    dy = -1.0;
                }
                _ => {
                    py = -(window.height() * 0.5 - 40.0);
                    dx = random::<f32>() - 0.5;
                    dy = 1.0;
                }
            }
            let speed: f32 = match et.kind {
                EnemyKind::Fly => 9.0,
                EnemyKind::Swim => 6.2,
                EnemyKind::Walk => 4.0,
            };
            let dist: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
            dx *= speed / dist;
            dy *= speed / dist;

            trans.translation.x = px;
            trans.translation.y = py;
            enemy.trans_x = px;
            enemy.trans_y = py;
            enemy.diff_x = dx;
            enemy.diff_y = dy;
            et.collided = false;
        }
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % enemy.texture_atlas.textures.len();
        }
    }
}
