use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::*;

use crate::boids::components::{Collider, Velocity};

use super::{components::Boid, BoidUniverse};

pub fn init_boid_scene(
    mut commands: Commands,
    window: Query<&Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let window = window.single();

    commands.insert_resource(BoidUniverse::new(
        Vec2::new(window.width() / -2.0, window.height() / -2.0),
        Vec2::new(window.width() / 2.0, window.height() / 2.0),
    ));

    let size = 5.0;
    let rect = shapes::Rectangle {
        extents: Vec2::new(size, size),
        origin: shapes::RectangleOrigin::Center,
    };

    for _ in 0..500 {
        let x = rand::random::<i32>() % (window.width() / 2.0) as i32;
        let y = rand::random::<i32>() % (window.height() / 2.0) as i32;
        let initial_speed = 200.0 + rand::random::<f32>() * 100.0;
        let velocity = Vec3::new(
            (rand::random::<f32>() - 0.5) * initial_speed,
            (rand::random::<f32>() - 0.5) * initial_speed,
            0.0,
        );
        commands
            .spawn(
                (
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad::new(Vec2::new(size, size))))
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::Hsla {
                            hue: rand::random::<f32>() * 100.0,
                            saturation: 0.7,
                            lightness: 0.3,
                            alpha: 1.0,
                        })),
                        transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                        ..default()
                    }
                    // ShapeBundle {
                    //     path: GeometryBuilder::build_as(&rect),
                    //     transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    //     material: materials.add(ColorMaterial::from(Color::Hsla {
                    //         hue: 100.0,
                    //         saturation: 0.7,
                    //         lightness: 0.4,
                    //         alpha: 1.0,
                    //     })),
                    //     ..default()
                    // },
                    // Fill::color(Color::ORANGE_RED),
                    // Stroke::new(Color::ORANGE_RED, 2.0),
                ),
            )
            .insert(Boid)
            .insert(Velocity { value: velocity })
            .insert(Collider::new(20.0));
    }
}
