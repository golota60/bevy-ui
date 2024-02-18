use bevy::math::prelude::{Circle, RegularPolygon};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // Sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        sprite: Sprite {
            color: Color::rgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
            custom_size: Some(Vec2::splat(160.0)),
            ..default()
        },
        ..default()
    });

    // Circle mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(100.)).into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
    });

    // Hexagon mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(RegularPolygon::new(100., 6)).into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
        ..default()
    });
}
