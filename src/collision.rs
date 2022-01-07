use {
    crate::{enemy::Enemy, player::Player, CollisionEvent},
    bevy::{prelude::*, sprite::collide_aabb::collide},
};

//
// Collision detection
//
pub fn check_collision(
    mut player_query: Query<(&Transform, &mut Player)>,
    collider_query: Query<(&Transform, &Enemy)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut collision_channel: EventWriter<CollisionEvent>,
) {
    let (player_trans, mut player) = player_query.single_mut();
    // let player_size = player_trans.scale.truncate();
    for (enemy_trans, enemy) in collider_query.iter() {
        if enemy.collided {
            continue;
        }
        if let Some(_collision) = collide(
            player_trans.translation,
            Vec2::new(40.0, 40.0), // player_size,
            enemy_trans.translation,
            Vec2::new(40.0, 40.0), // enemy_trans.scale.truncate(),
        ) {
            collision_channel.send(CollisionEvent);
            player.score *= 0.5;
            if player.score < 1.0 {
                // should be game over by shifting to the next stage
            } else {
                audio.play(asset_server.get_handle("sounds/laserpew.ogg"));
            }
        }
    }
}
