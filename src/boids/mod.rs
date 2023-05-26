use std::time::Duration;

use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_prototype_debug_lines::*;

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
pub const BOID_SPAWN_RATE: f32 = 100.0;
pub const CURSOR_QUAD_SIZE: f32 = 100.0;
pub const BOID_SIZE: f32 = 5.0;

pub mod components;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(QuadBench::default());
        app.add_startup_system(init_boid_scene);
        app.add_system(count_boids);
        app.add_system(handle_mouse);
        app.add_systems((
            build_or_update_quadtree
                .run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            update_boids.run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            move_system.run_if(on_timer(Duration::from_secs_f32(1. / PHYISCS_TICK_RATE))),
            ui_controls,
            render_quadtree,
        ));
    }
}

#[derive(Component, Debug)]
pub struct EguiWin {
    width: f32,
    height: f32,
    position: Vec3,
}

fn ui_controls(
    mut commands: Commands,
    mut eguiWin: Query<Entity, With<EguiWin>>,
    mut context: EguiContexts,
    mut universe: ResMut<BoidUniverse>,
    diagnostics: Res<Diagnostics>,
    bench: Res<QuadBench>,
) {
    egui::Window::new("Boid Control")
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(0.0, 0.0))
        .show(context.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut universe.speration, 0.0..=1.0).text("speration"));
            ui.add(egui::Slider::new(&mut universe.cohesion, 0.0..=1.0).text("cohesion"));
            ui.add(egui::Slider::new(&mut universe.alignment, 0.0..=1.0).text("alignment"));
            ui.add(egui::Slider::new(&mut universe.speed, 0.0..=10.0).text("speed"));
            ui.add(egui::Checkbox::new(
                &mut universe.show_graph,
                "Render Graph",
            ));
            ui.label(format!("Boid Count: {}", universe.boid_count));
            diagnostics
                .iter()
                .for_each(|diagnostic| match diagnostic.value() {
                    Some(value) => {
                        ui.label(format!("{} : {:.2}", diagnostic.name, value));
                    }
                    None => {}
                });
            ui.label(format!("avg. query time: {} ns", bench.avarage_query_time));
            ui.label(format!("avg. build time: {} us", bench.avarage_build_time));
        });
    universe.mouse_used_by_egui = context.ctx_mut().wants_pointer_input();
}

fn render_quadtree(
    mut commands: Commands,
    mut universe: ResMut<BoidUniverse>,
    mut lines: ResMut<DebugLines>,
) {
    if !universe.show_graph {
        return;
    }

    let regions = universe.graph.get_regions();

    regions.iter().for_each(|region| {
        let (min_x, min_y, max_x, max_y) = region.into_f32();

        let bottom_left = Vec3::new(min_x, min_y, 0.0);
        let bottom_right = Vec3::new(max_x, min_y, 0.0);
        let top_right = Vec3::new(max_x, max_y, 0.0);
        let top_left = Vec3::new(min_x, max_y, 0.0);

        lines.line(bottom_left, bottom_right, 0.0);
        lines.line(bottom_right, top_right, 0.0);
        lines.line(top_right, top_left, 0.0);
        lines.line(top_left, bottom_left, 0.0);
    })
}
