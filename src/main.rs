use bevy::{
    core_pipeline::bloom::{BloomPrefilterSettings, BloomSettings},
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use boids::BoidPlugin;
use wasm_bindgen::prelude::*;

pub mod boids;
pub mod quadtree;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run("#boids", 1280, 720);
}

#[wasm_bindgen]
pub fn run(canvas_id: &str, width: u32, height: u32) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(canvas_id.to_string()),
                resolution: WindowResolution::new(width as f32, height as f32),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(BoidPlugin)
        .add_startup_system(camera_init)
        .run();
}

fn camera_init(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.4,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.5,
                threshold_softness: 1.0,
            },
            ..default()
        },
    ));
}
