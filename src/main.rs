use bevy::{
    core_pipeline::bloom::BloomSettings, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use boids::BoidPlugin;
use wasm_bindgen::prelude::*;

pub mod boids;
pub mod quadtree;

fn main() {
    run("#boids", 1280, 720);
}

#[wasm_bindgen(start)]
fn init() {
    run("#boids", 1280, 720);
    // set default start, so main is not called by wasm init.
}

#[wasm_bindgen]
pub fn run(canvas_id: &str, width: u32, height: u32) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // canvas: Some(canvas_id.to_string()),
                resolution: WindowResolution::new(width as f32, height as f32),
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(BoidPlugin)
        .add_systems(Startup, camera_init)
        .run();
}

fn camera_init(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings {
            intensity: 0.4,
            high_pass_frequency: 0.8,
            low_frequency_boost: 0.8,
            ..default()
        });
}
