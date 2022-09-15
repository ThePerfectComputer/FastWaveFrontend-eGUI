// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use egui::style;

use super::signal_select;
use std::rc::Rc;

pub struct VCDViewer {
    signal_select: signal_select::SignalSelect,
    init_style: bool
}

impl VCDViewer {
    pub fn new(vcd: Rc<fastwave_backend::VCD>) -> Self {
        Self {
            signal_select: signal_select::SignalSelect::new(vcd.clone()),
            init_style: false
        }
    }
    fn init_style_once(&mut self, ctx : &egui::Context) {
        // we only need to set the style once, at the very
        // beginning of running the GUI
        if !self.init_style {
            // derive base UI setting from the dark visual scheme
            ctx.set_visuals(egui::Visuals::dark());

            // clone style so that we can modify it
            let mut style = (*ctx.style()).clone();

            // we can indicate separation using two tone separation
            // instead of line-based separation.
            style.visuals.widgets.noninteractive.bg_stroke.width = 0.;

            // change the curvature and remove outline 
            // for selected box selection and selection like
            // UI elements such as selectable labels and buttons
            let nice_round = epaint::Rounding::same(6.8);
            style.visuals.widgets.hovered.rounding = nice_round;
            style.visuals.widgets.active.rounding = nice_round;
            style.visuals.widgets.inactive.rounding = nice_round;
            style.visuals.widgets.open.rounding = nice_round;

            style.visuals.widgets.hovered.bg_stroke.width = 0.;
            style.visuals.widgets.active.bg_stroke.width = 0.;
            style.visuals.widgets.inactive.bg_stroke.width = 0.;
            style.visuals.widgets.open.bg_stroke.width = 0.;

            // apply style changes
            ctx.set_style(style);

            // we don't need to modify style again after this for
            // the duration of this program
            self.init_style = true;
        }

    }
}

impl eframe::App for VCDViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.init_style_once(ctx);

        self.signal_select.draw(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Center Panel");
            ui.label(r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#);
        });
    }
}
