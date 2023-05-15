use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_prototype_lyon::prelude::*;
// use bevy_prototype_lyon::shapes;

use crate::quadtree::coord::Coord;
use crate::quadtree::region::Region;
use crate::quadtree::tree::QuadTree;

use self::bench::*;
use self::components::*;
use self::init::*;
use self::resources::*;
use self::systems::*;

mod bench;
mod init;
mod resources;
mod systems;

pub const PHYISCS_TICK_RATE: f32 = 90.;
pub mod components;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(QuadBench::default());
        app.add_startup_system(init_boid_scene);
        app.add_systems((
            build_or_update_quadtree
                .run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            update_boids.run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            move_system.run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            color_system.run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            ui_controls,
            remove_render_rects,
            render_quadtree,
        ));
    }
}

fn ui_controls(mut context: EguiContexts, mut universe: ResMut<BoidUniverse>) {
    egui::Window::new("Boid Control").show(context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut universe.speration, 0.0..=1.0).text("speration"));
        ui.add(egui::Slider::new(&mut universe.cohesion, 0.0..=1.0).text("cohesion"));
        ui.add(egui::Slider::new(&mut universe.alignment, 0.0..=1.0).text("alignment"));
        ui.add(egui::Slider::new(&mut universe.vision, 1.0..=10.0).text("vision"));
        ui.add(egui::Slider::new(&mut universe.speed, 0.0..=10.0).text("speed"));
        ui.add(egui::Checkbox::new(
            &mut universe.show_graph,
            "Render Graph",
        ));
    });
}

fn remove_render_rects(mut commands: Commands, query: Query<Entity, With<QuadNodeRect>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn render_quadtree(mut commands: Commands, mut universe: ResMut<BoidUniverse>) {
    if !universe.show_graph {
        return;
    }

    let regions = universe.graph.get_regions();

    regions.iter().for_each(|region| {
        let (w, h) = region.size_f32();

        let rect = shapes::Rectangle {
            extents: Vec2::new(w, h),
            origin: shapes::RectangleOrigin::BottomLeft,
        };
        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&rect),
                    transform: Transform::from_xyz(region.min.x as f32, region.min.y as f32, 1.0),
                    ..default()
                },
                Stroke::new(Color::WHITE, 1.0),
            ))
            .insert(QuadNodeRect);
    })
}
