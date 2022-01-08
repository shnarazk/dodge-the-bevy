use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<WindowDescriptor>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        // mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(config.width, config.height)))).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..Default::default()
    });
}

