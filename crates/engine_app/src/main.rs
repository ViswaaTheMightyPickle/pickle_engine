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

            ui.label("Viewport");

            let cell_size = 8.0;
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());
            let rect = response.rect;

            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 32, 38));

            for cell in self.world.visible_cells() {
                let color = if cell.material == MaterialId::SAND {
                    egui::Color32::from_rgb(230, 190, 90)
                } else if cell.material == MaterialId::STONE {
                    egui::Color32::from_rgb(130, 135, 145)
                } else {
                    continue;
                };

                let x = rect.left() + cell.pos.x as f32 * cell_size;
                let y = rect.top() + cell.pos.y as f32 * cell_size;

                let cell_rect =
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(cell_size, cell_size));

                painter.rect_filled(cell_rect, 0.0, color);
            }
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
