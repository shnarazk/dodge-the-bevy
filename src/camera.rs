use {crate::CollisionEvent, bevy::prelude::*};

//
// Camera
//
#[derive(Component, Debug, Default)]
pub struct MainCamera {
    pub shaker: Option<u32>,
}

pub fn setup_cammera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera::default());
}

pub fn shake_camera(
    mut camera_query: Query<&mut MainCamera>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    if collision_event.iter().next().is_some() {
        if let Some(mut camera) = camera_query.iter_mut().next() {
            if camera.shaker.is_none() {
                camera.shaker = Some(20);
            }
        }
    }
}

pub fn animate_camera(mut query: Query<(&mut Transform, &mut MainCamera)>) {
    if let Some((mut trans, mut camera)) = query.iter_mut().next() {
        if let Some(n) = camera.shaker {
            trans.rotation = Quat::from_rotation_z(n as f32 * 0.05 * std::f32::consts::PI);
            camera.shaker = n.checked_sub(1);
        }
    }
}
