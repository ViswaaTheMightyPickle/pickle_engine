use eframe::egui;
use engine_core::{CellPos, MaterialId, STEP_SECONDS, World};
use engine_physics::PhysicsWorld;

#[derive(Default)]
struct EngineApp {
    playing: bool,
    world: World,
    physics: PhysicsWorld,
}

impl eframe::App for EngineApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hybrid Pixel Engine - team starter");
            ui.horizontal(|ui| {
                if ui.button("Play / Pause").clicked() {
                    self.playing = !self.playing;
                }
                if ui.button("Reset").clicked() {
                    self.world = World::new();
                    self.physics = PhysicsWorld::new();
                    self.playing = false;
                }
                if ui.button("Place one sand cell").clicked() {
                    self.world.set(CellPos { x: 31, y: 1 }, MaterialId::SAND);
                }
            });
            ui.label(format!("Playing: {}", self.playing));
            ui.label(format!(
                "Visible cells: {}",
                self.world.visible_cells().len()
            ));
            ui.label("TODO (Gienna): viewport, PNG import, asset placement, eraser.");
            ui.label("NOTE: physics and powder stepping are not implemented yet.");
        });

        if self.playing {
            self.world.step();
            self.physics.step(STEP_SECONDS);
            ui.ctx().request_repaint();
        }
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Hybrid Pixel Engine",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(EngineApp::default()))),
    )
}
