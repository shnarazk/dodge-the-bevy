use bevy::prelude::*;

//
// Character, autonomous moving objects
//
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

// (from 'sprite_sheet')
#[allow(clippy::type_complexity, dead_code)]
pub fn animate_character(
    time: Res<Time>,
    mut query: Query<(
        &mut Character,
        &mut Timer,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut character, mut timer, mut trans, mut sprite) in query.iter_mut() {
        trans.translation.x += character.diff_x;
        trans.translation.y += character.diff_y;
        // memoize the location after moving
        character.trans_x = trans.translation.x;
        character.trans_y = trans.translation.y;
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % character.texture_atlas.textures.len();
            sprite.flip_x = character.flip;
        }
    }
}
