use std::f32::consts::PI;

use super::components::{Boid, Cursor};
use super::{BoidUniverse, CURSOR_QUAD_SIZE, BOID_SIZE};
use crate::boids::components::{Collider, Velocity};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn init_boid_scene(
    mut commands: Commands,
    window: Query<&Window>,
    mut Meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    let window = window.single();

    commands.insert_resource(BoidUniverse::new(
        Vec2::new(window.width() / -2.0, window.height() / -2.0),
        Vec2::new(window.width() / 2.0, window.height() / 2.0),
    ));

    let size = 5.0;

    // spawn cursor visiuals

    commands
        .spawn(
            (MaterialMesh2dBundle {
                mesh: Meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::splat(CURSOR_QUAD_SIZE))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: 0.4,
                })),
                ..default()
            }),
        )
        .insert(Cursor);

    // spawn default boids
    for _ in 0..3000 {
        let x = rand::random::<i32>() % (window.width() / 2.0) as i32;
        let y = rand::random::<i32>() % (window.height() / 2.0) as i32;
        let initial_speed = 200.0 + rand::random::<f32>() * 200.0;
        let velocity = Vec3::new(
            (rand::random::<f32>() - 0.5) * initial_speed,
            (rand::random::<f32>() - 0.5) * initial_speed,
            0.0,
        );

        commands
            .spawn(SpriteBundle {
                texture: assets.load("boid.png"),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..Default::default()
            })
            .insert(Boid)
            .insert(Velocity { value: velocity })
            .insert(Collider::new(BOID_SIZE));
    }
}
