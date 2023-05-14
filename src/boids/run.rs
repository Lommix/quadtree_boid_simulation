use bevy::prelude::*;
use instant::*;

use crate::quadtree::region::Region;

use super::{bench::QuadBench, components::*, BoidUniverse};

pub fn build_or_update_quadtree(
    mut query: Query<(Entity, &Transform, &mut Collider, &Velocity), With<Boid>>,
    mut universe: ResMut<BoidUniverse>,
    mut bench: ResMut<QuadBench>,
) {

    let now = instant::Instant::now();

    universe.graph.clear();
    query
        .iter_mut()
        .for_each(|(entity, transform, mut collider, velocity)| {
            collider.id = Some(universe.graph.insert(
                collider.into_region(transform.translation),
                Body {
                    position: transform.translation,
                    velocity: velocity.value,
                },
            ));
        });
    bench.avarage_build_time = now.elapsed().as_nanos();
}

pub fn update_boids(
    mut query: Query<(Entity, &Transform, &mut Collider, &mut Velocity)>,
    universe: Res<BoidUniverse>,
    mut bench: ResMut<QuadBench>,
) {
    let mut query_time: u128 = 0;

    query
        .iter_mut()
        .for_each(|(entity, transform, mut collider, mut velocity)| {
            let x = transform.translation.x as i32;
            let y = transform.translation.y as i32;
            let win = universe.graph.size();

            let mut velo = velocity.value;

            let now = instant::Instant::now();
            // -------------------- collision query --------------------
            let query_region = collider.into_region(transform.translation).with_margin(5);
            let exclude = match &collider.id {
                Some(id) => vec![id.clone()],
                None => vec![],
            };

            let collisions = universe.graph.query(&query_region, &exclude);
            collider.nearby = collisions.len();

            query_time += now.elapsed().as_nanos();

            let (mass_center, aligment, separtion) = collisions.iter().fold(
                (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
                |(mcen, alg, sep), body| {
                    (
                        mcen + body.position.normalize(),
                        alg + body.velocity.normalize(),
                        sep + (transform.translation - body.position).normalize(),
                    )
                },
            );

            let mut direction = velo.normalize();

            // -------------------- Cohesion --------------------
            if mass_center.length() > 0.0 {
                direction += (mass_center.normalize() - transform.translation.normalize())
                    .normalize()
                    * universe.cohesion;
            }

            // -------------------- Alignment --------------------
            if aligment.length() > 0.0 {
                direction += aligment.normalize() * universe.alignment;
            }

            // -------------------- Separation --------------------
            if separtion.length() > 0.0 {
                direction += separtion.normalize() * universe.speration;
            }

            velo = direction.normalize() * velo.length();

            // -------------------- World Border --------------------
            let margin: i32 = 20;
            if (x < win.min.x + margin && velocity.value.x < 0.0)
                || (x > win.max.x - margin && velocity.value.x > 0.0)
            {
                velo.x *= -1.0;
            }
            if (y < win.min.y + margin && velocity.value.y < 0.0)
                || (y > win.max.y - margin && velocity.value.y > 0.0)
            {
                velo.y *= -1.0;
            }
            velocity.value = velo;
        });
    bench.avarage_query_time = query_time / query.iter().len() as u128;
}

pub fn move_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    universe: Res<BoidUniverse>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.value * time.delta_seconds() * universe.speed;
        let rotation = Quat::from_rotation_z(velocity.value.angle_between(Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }));
        transform.rotation = rotation;
    });
}

pub fn color_system(
    query: Query<(&Handle<ColorMaterial>, &Collider)>,
    time: Res<Time>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    query.iter().for_each(|(color_handle, collider)| {
        let color = colors.get_mut(color_handle).unwrap();
        let color_hsla = color.color.as_hsla();

        if let Color::Hsla {
            hue,
            saturation,
            lightness: _,
            alpha,
        } = color_hsla
        {
            color.color = Color::Hsla {
                hue,
                saturation,
                lightness: 0.3 + collider.nearby as f32 * 0.1,
                alpha,
            };
        };
    });
}
