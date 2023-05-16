use bevy::{diagnostic::Diagnostics, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

#[derive(Resource, Default)]
pub struct QuadBench {
    pub avarage_query_time: u128,
    pub avarage_build_time: u128,
}

pub fn update_benchmark(
    mut context: EguiContexts,
    bench: Res<QuadBench>,
    diagnostics: Res<Diagnostics>,
) {
    egui::Window::new("------ Benchmark ------").show(context.ctx_mut(), |ui| {
        ui.label(format!(
            "Average query time: {} ns",
            bench.avarage_query_time
        ));
        ui.label(format!(
            "Average build time: {} us",
            bench.avarage_build_time
        ));

        diagnostics
            .iter()
            .for_each(|diagnostic| match diagnostic.value() {
                Some(value) => {
                    ui.label(format!("{:?} : {:.2}", diagnostic.name, value));
                }
                None => {}
            })
    });
}
