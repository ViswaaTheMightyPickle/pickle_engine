use eframe::egui;
use engine_core::{CellPos, MaterialId, STEP_SECONDS, World};
use engine_physics::PhysicsWorld;

#[derive(Default)]
struct EngineApp {
    playing: bool,
    world: World,
    physics: PhysicsWorld,
    brush: Option<MaterialId>,
}

impl eframe::App for EngineApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hybrid Pixel Engine - team starter");
            ui.horizontal_wrapped(|ui| {
                if ui.button("Play / Pause").clicked() {
                    self.playing = !self.playing;
                }
                if ui.button("Reset").clicked() {
                    self.world = World::new();
                    self.physics = PhysicsWorld::new();
                    self.playing = false;
                }
                if ui.button("Select Sand").clicked() {
                    self.brush = Some(MaterialId::SAND);
                }
                if ui.button("Select Stone").clicked() {
                    self.brush = Some(MaterialId::STONE);
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
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            let rect = response.rect;
            // BRUSH SYSTEM
            if let Some(mouse_pos) = response.hover_pos() {
                // Check whether left mouse button is held
                let mouse_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));

                // Only paint if LMB is held AND a brush is selected
                if mouse_down {
                    if let Some(material) = self.brush {
                        // Convert global mouse position to viewport coordinates
                        let mouse_x = mouse_pos.x - rect.left();
                        let mouse_y = mouse_pos.y - rect.top();

                        // Convert pixel coordinates to grid coordinates
                        let cell_x = (mouse_x / cell_size).floor() as i32;
                        let cell_y = (mouse_y / cell_size).floor() as i32;

                        // Place selected material
                        self.world.set(
                            CellPos {
                                x: cell_x,
                                y: cell_y,
                            },
                            material,
                        );
                    }
                }
            }

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
