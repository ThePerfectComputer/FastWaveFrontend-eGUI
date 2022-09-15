// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use super::signal_select;
use std::rc::Rc;

pub struct VCDViewer {
    signal_select: signal_select::SignalSelect,
}

impl VCDViewer {
    pub fn new(vcd: Rc<fastwave_backend::VCD>) -> Self {
        Self {
            signal_select: signal_select::SignalSelect::new(vcd.clone()),
        }
    }
}

impl eframe::App for VCDViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.signal_select.draw(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Center Panel");
        });
    }
}
