#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
use self::core::*;
use eframe::egui;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(370.0, 160.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Http Debugger Pro 9.x Keygen",
        options,
        Box::new(|_cc| Box::new(HttpDebuggerKeygen::default())),
    );
    Ok(())
}

struct HttpDebuggerKeygen {
    version: u32,
    license_key: String,
}

impl Default for HttpDebuggerKeygen {
    fn default() -> Self {
        Self {
            version: get_httpdebugger_version().unwrap(),
            license_key: String::default(),
        }
    }
}

impl eframe::App for HttpDebuggerKeygen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Installed Version");
                ui.monospace(self.version.to_string());
            });
            ui.vertical_centered_justified(|ui| {
                ui.label("License key");
                ui.monospace(self.license_key.clone());
            });
            if ui.button("Crack").clicked() {
                let registry_key_name = create_registry_key_name(self.version).unwrap();
                let license_key = create_license_key();
                self.license_key = license_key.clone();
                write_to_registry(&registry_key_name, &license_key).unwrap();
            }
        });
    }
}
