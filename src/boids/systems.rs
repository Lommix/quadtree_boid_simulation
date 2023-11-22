use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::bevy_egui::EguiContexts;

use super::{components::*, resources::QuadBench, BoidUniverse};
use crate::{
    boids::{BOID_SIZE, CURSOR_QUAD_SIZE},
    quadtree::{coord::Coord, region::Region, slot_map::SlotId},
};

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
                    entity,
                    position: transform.translation,
                    velocity: velocity.value,
                },
            ));
        });
    bench.avarage_build_time = now.elapsed().as_micros();
}

pub fn update_boids(
    mut query: Query<(Entity, &Transform, &mut Collider, &mut Velocity)>,
    universe: Res<BoidUniverse>,
    mut bench: ResMut<QuadBench>,
) {
    let mut query_time: u128 = 0;
    query
        .iter_mut()
        .for_each(|(_entity, transform, mut collider, mut velocity)| {
            let x = transform.translation.x as i32;
            let y = transform.translation.y as i32;
            let win = universe.graph.size();
            let now = instant::Instant::now();

            // -------------------- collision query --------------------
            let query_region = collider
                .into_region(transform.translation)
                .with_margin((universe.vision * 10.0) as i32);
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

            let mut direction = velocity.value.normalize();

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

            let mut new_velocity = direction.normalize() * velocity.value.length();

            // -------------------- World Border --------------------
            let margin: i32 = 20;
            if (x < win.min.x + margin && velocity.value.x < 0.0)
                || (x > win.max.x - margin && velocity.value.x > 0.0)
            {
                new_velocity.x *= -1.0;
            }
            if (y < win.min.y + margin && velocity.value.y < 0.0)
                || (y > win.max.y - margin && velocity.value.y > 0.0)
            {
                new_velocity.y *= -1.0;
            }

            // finally set the new velocity
            velocity.value = new_velocity;
        });

    bench.avarage_query_time = query_time / query.iter().len() as u128;
}

pub fn move_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    universe: Res<BoidUniverse>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each_mut(|(mut transform, velocity)| {
            let direction = velocity.value.normalize();
            let rotation = Quat::from_rotation_z(-direction.x.atan2(direction.y) + PI / 2.0);
            transform.rotation = rotation;
            transform.translation += velocity.value * time.delta_seconds() * universe.speed;
        });
}

pub fn count_boids(query: Query<&Boid>, mut universe: ResMut<BoidUniverse>) {
    universe.boid_count = query.iter().count() as u32;
}

pub fn handle_mouse(
    mut commands: Commands,
    mut cursor_quad: Query<&mut Transform, With<Cursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>,
    universe: ResMut<BoidUniverse>,
    _egui_context: EguiContexts,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.single();
    let cursor_pos_win = window.cursor_position();

    if cursor_pos_win.is_none() {
        return;
    }

    if universe.mouse_used_by_egui {
        return;
    }

    let (camera, camera_transform) = camera.single();
    let mut cursor_quad_transform = cursor_quad.single_mut();

    match camera.viewport_to_world_2d(camera_transform, cursor_pos_win.unwrap()) {
        Some(pos) => {
            cursor_quad_transform.translation = Vec3::new(pos.x, pos.y, 0.0);

            if buttons.just_pressed(MouseButton::Left) {
                self::spawn_boids(&mut commands, &mut meshes, &mut materials, pos);
            }

            if buttons.just_pressed(MouseButton::Right) {
                self::despawn_boids(&mut commands, pos, &universe);
            }
        }
        None => {}
    }
}

fn spawn_boids(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    for _ in 0..100 {
        let x = position.x + (rand::random::<f32>() - 0.5) * (CURSOR_QUAD_SIZE / 2.0);
        let y = position.y + (rand::random::<f32>() - 0.5) * (CURSOR_QUAD_SIZE / 2.0);

        let initial_speed = 200.0 + rand::random::<f32>() * 200.0;
        let velocity = Vec3::new(
            (rand::random::<f32>() - 0.5) * initial_speed,
            (rand::random::<f32>() - 0.5) * initial_speed,
            0.0,
        );

        commands
            .spawn(MaterialMesh2dBundle {
                // texture: assets.load("boid.png"),
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(
                        BOID_SIZE,
                        BOID_SIZE / 2.0,
                    ))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(2., 2., 0.))),
                // texture: assets.load("/files/assets/boid.png"),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                ..Default::default()
            })
            .insert(Boid)
            .insert(Velocity { value: velocity })
            .insert(Collider::new(BOID_SIZE));
    }
}

fn despawn_boids(commands: &mut Commands, position: Vec2, universe: &ResMut<BoidUniverse>) {
    let query_region = Region::new(
        Coord::from_f32(
            position.x - (CURSOR_QUAD_SIZE / 2.0),
            position.y - (CURSOR_QUAD_SIZE / 2.0),
        ),
        Coord::from_f32(
            position.x + (CURSOR_QUAD_SIZE / 2.0),
            position.y + (CURSOR_QUAD_SIZE / 2.0),
        ),
    );
    let exclude: Vec<SlotId> = vec![];

    let result = universe.graph.query(&query_region, &exclude);
    result.iter().for_each(|body| {
        commands.entity(body.entity).despawn_recursive();
    });
}
