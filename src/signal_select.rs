// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use std::rc::Rc;

pub struct SignalSelect {
    vcd: Rc<fastwave_backend::VCD>,
    selected_signal: fastwave_backend::SignalIdx,
}

impl SignalSelect {
    pub fn new(vcd: Rc<fastwave_backend::VCD>) -> Self {
        SignalSelect {
            vcd: vcd,
            selected_signal: fastwave_backend::SignalIdx(0),
        }
    }
    pub fn draw(&self, ctx: &egui::Context) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ctx.set_visuals(egui::Visuals::light());
                ui.heading("Modules");
                self.draw_all_scopes(ui);
            });
        });
    }
    fn draw_all_scopes(&self, ui: &mut egui::Ui) {
        for root_scope_idx in self.vcd.root_scopes_by_idx() {
            let name = self.vcd.scope_name_by_idx(root_scope_idx);
            let fastwave_backend::ScopeIdx(idx) = root_scope_idx;
            if self.vcd.child_scopes_by_idx(root_scope_idx).is_empty() {
                ui.label(name);
            } else {
                egui::CollapsingHeader::new(name)
                    .id_source(idx)
                    .show(ui, |ui| {
                        self.draw_root_scope_view(root_scope_idx, ui);
                    });
            }
        }
    }
    fn draw_root_scope_view(&self, root_idx: fastwave_backend::ScopeIdx, ui: &mut egui::Ui) {
        for child_scope_idx in self.vcd.child_scopes_by_idx(root_idx) {
            let name = self.vcd.scope_name_by_idx(child_scope_idx);
            let fastwave_backend::ScopeIdx(idx) = child_scope_idx;
            if self.vcd.child_scopes_by_idx(child_scope_idx).is_empty() {
                ui.label(name);
            } else {
                egui::CollapsingHeader::new(name)
                    .id_source(idx)
                    .show(ui, |ui| {
                        self.draw_root_scope_view(child_scope_idx, ui);
                    });
            }
        }
    }
}
