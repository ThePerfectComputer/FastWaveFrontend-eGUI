// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use std::cell::RefCell;
use std::rc::Rc;

use egui::style;

use crate::theme;

pub struct SignalSelect {
    vcd: Rc<fastwave_backend::VCD>,
    selected_module: fastwave_backend::ScopeIdx,
}

impl SignalSelect {
    pub fn new(vcd: Rc<fastwave_backend::VCD>) -> Self {
        SignalSelect {
            vcd: vcd,
            selected_module: fastwave_backend::ScopeIdx(0),
        }
    }
    pub fn draw(&mut self, ctx: &egui::Context, theme_manager: &theme::ThemeManager) {
        let max_width = 400.0;
        let window_round = epaint::Rounding::same(20.);

        let fill = if false {
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 100)
        } else {
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 25)
        };
        egui::SidePanel::left("signal select left panel")
            .default_width(300.)
            .width_range(100.0..=max_width)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                    |ui| {
                        ui.heading("Modules");
                        ui.add_space(3.0);
                    },
                );
                theme_manager.new_frame().show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            egui::ScrollArea::both().show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                self.draw_all_scopes(ui);
                            });
                        },
                    );
                });
            });
    }
    fn draw_all_scopes(&mut self, ui: &mut egui::Ui) {
        for root_scope_idx in self.vcd.root_scopes_by_idx() {
            self.draw_selectable_child_or_orphan_scope(root_scope_idx, ui);
        }
    }
    fn draw_selectable_child_or_orphan_scope(
        &mut self,
        scope_idx: fastwave_backend::ScopeIdx,
        ui: &mut egui::Ui,
    ) {
        let name = self.vcd.scope_name_by_idx(scope_idx);
        let fastwave_backend::ScopeIdx(idx) = scope_idx;
        if self.vcd.child_scopes_by_idx(scope_idx).is_empty() {
            if ui
                .add(egui::SelectableLabel::new(
                    self.selected_module == scope_idx,
                    name,
                ))
                .clicked()
            {
                self.selected_module = scope_idx;
            }
        } else {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                egui::Id::new(idx),
                false,
            )
            .show_header(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                    |ui| {
                        if ui
                            .add(egui::SelectableLabel::new(
                                self.selected_module == scope_idx,
                                name,
                            ))
                            .clicked()
                        {
                            self.selected_module = scope_idx;
                        }
                    },
                );
            })
            .body(|ui| self.draw_root_scope_view(scope_idx, ui));
        }
    }
    fn draw_root_scope_view(&mut self, root_idx: fastwave_backend::ScopeIdx, ui: &mut egui::Ui) {
        for child_scope_idx in self.vcd.child_scopes_by_idx(root_idx) {
            self.draw_selectable_child_or_orphan_scope(child_scope_idx, ui);
        }
    }
}
