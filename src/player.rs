use {
    crate::{character::Character, Z_AXIS},
    bevy::prelude::*,
};

//
// Player
//
#[derive(Component, Debug, Default)]
pub struct Player {
    pub score: f32,
    pub max_score: f32,
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
        .insert(Character::from(texture_atlas))
        .insert(Player::default());
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
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
        trans.translation.x =
            (trans.translation.x + player.diff_x).clamp(-0.45 * config.width, 0.45 * config.width);
        trans.translation.y = (trans.translation.y + player.diff_y)
            .clamp(-0.45 * config.height, 0.45 * config.height);
        player.trans_x = trans.translation.x;
        player.trans_y = trans.translation.y;
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % player.texture_atlas.textures.len();
            sprite.flip_x = player.flip;
        }
    }
}
