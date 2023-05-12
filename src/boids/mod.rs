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

use self::components::Body;
use self::init::*;
use self::run::*;

mod init;
mod run;

pub const PHYISCS_TICK_RATE: f32 = 1. / 60.;
pub mod components;

// quad render tag
#[derive(Component)]
pub struct QuadNodeRect;

// ---------------------------  Boid Universe ---------------------------
#[derive(Resource)]
pub struct BoidUniverse {
    pub graph: QuadTree<Body>,
    pub speration: f32,
    pub cohesion: f32,
    pub alignment: f32,
    pub speed: f32,
    pub show_graph: bool,
}

impl BoidUniverse {
    fn new(min: Vec2, max: Vec2) -> Self {
        print!("f : {:?} {:?}\n", min, max);
        let _min = Coord::from_f32(min.x, min.y);
        let _max = Coord::from_f32(max.x, max.y);
        print!("u : {:?} {:?}\n", _min, _max);
        Self {
            graph: QuadTree::new(Region::new(_min, _max)),
            speration: 0.0,
            cohesion: 0.0,
            speed: 1.0,
            alignment: 0.0,
            show_graph: true,
        }
    }
}

// ---------------------------  Boid Plugin ---------------------------
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_boid_scene);
        app.add_systems((
            build_or_update_quadtree.run_if(on_timer(Duration::from_secs_f32(PHYISCS_TICK_RATE))),
            // render_quadtree.run_if(on_timer(Duration::from_secs_f32(PHYISCS_TICK_RATE))),
            update_boids.run_if(on_timer(Duration::from_secs_f32(PHYISCS_TICK_RATE))),
            move_system.run_if(on_timer(Duration::from_secs_f32(PHYISCS_TICK_RATE))),
            color_system.run_if(on_timer(Duration::from_secs_f32(PHYISCS_TICK_RATE))),
            ui_controls,
        ));
    }
}

fn ui_controls(mut context: EguiContexts, mut universe: ResMut<BoidUniverse>) {
    egui::Window::new("Boid Control").show(context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut universe.speration, 0.0..=1.0).text("speration"));
        ui.add(egui::Slider::new(&mut universe.cohesion, 0.0..=1.0).text("cohesion"));
        ui.add(egui::Slider::new(&mut universe.alignment, 0.0..=1.0).text("alignment"));
        ui.add(egui::Slider::new(&mut universe.speed, 0.0..=10.0).text("speed"));
        ui.add(egui::Checkbox::new(&mut universe.show_graph, "Render Graph"));
    });
}

fn render_quadtree(
    mut commands: Commands,
    query: Query<Entity, With<QuadNodeRect>>,
    universe: Res<BoidUniverse>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });

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
                Stroke::new(Color::WHITE, 2.0),
            ))
            .insert(QuadNodeRect);
    })
}
