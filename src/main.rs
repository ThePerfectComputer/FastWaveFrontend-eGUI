// Copyright (C) 2022 Yehowshua Immanuel
// No part of the program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.
use eframe::egui;
use egui::{CollapsingHeader, Ui};
use fastwave_backend::*;
use std::fs::File;
use std::rc::Rc;


fn main() -> std::io::Result<()> {
    use std::time::Instant;

    // set default window size
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::emath::Vec2 {
        x: 300f32,
        y: 500f32,
    });

    eframe::run_native(
        "FastWave",
        options,
        Box::new(|_cc| {
            // get file
            let file_path = "./FastWaveBackend/tests/vcd-files/icarus/CPU.vcd";
            let file = File::open(file_path).unwrap();

            // parse VCD and time how long it takes
            let now = Instant::now();
            let vcd = Rc::new(parse_vcd(file).unwrap());
            let elapsed = now.elapsed();

            println!("Parsed VCD file {} : {:.2?}", file_path, elapsed);
            Box::new(ScopeView::new(vcd.clone()))
        }),
    );

    Ok(())
}

struct ScopeView {
    vcd: Rc<VCD>,
}

impl ScopeView {
    fn new(vcd: Rc<VCD>) -> Self {
        Self { vcd: vcd }
    }
    fn draw_all_scopes(&self, ui: &mut egui::Ui) {
        for root_scope_idx in self.vcd.root_scopes_by_idx() {
            self.draw_root_scope_view(root_scope_idx, ui);
        }
    }
    fn draw_root_scope_view(&self, root_idx: ScopeIdx, ui: &mut egui::Ui) {
        if self.vcd.child_scopes_by_idx(root_idx).is_empty() {
            ui.label("Placeholder");
        } else {
            for child_scope_idx in self.vcd.child_scopes_by_idx(root_idx) {
                let name = self.vcd.scope_name_by_idx(child_scope_idx);
                let ScopeIdx(idx) = child_scope_idx;
                egui::CollapsingHeader::new(name)
                    .id_source(idx)
                    .show(ui, |ui| {
                        self.draw_root_scope_view(child_scope_idx, ui);
                    });
            }
        }
    }
}

impl eframe::App for ScopeView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ctx.set_visuals(egui::Visuals::light());
                ui.heading("Scopes");
                self.draw_all_scopes(ui);
            });
        });
    }
}
