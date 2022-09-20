// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use egui::{style, Style};

use crate::theme;

use super::signal_select;
use std::rc::Rc;
use std::cell::RefCell;

pub struct VCDViewer {
    signal_select: signal_select::SignalSelect,
    init_theme: bool,
    theme_manager: theme::ThemeManager
}

impl VCDViewer {
    pub fn new(vcd: Rc<fastwave_backend::VCD>) -> Self {
        Self {
            signal_select: signal_select::SignalSelect::new(vcd.clone()),
            init_theme: false,
            theme_manager: theme::ThemeManager::new()
        }
    }
}

impl eframe::App for VCDViewer {
    fn clear_color(&self, visuals: &egui::Visuals) -> egui::Rgba {
        visuals.window_fill().into()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // at startup, we only need to set the initial style once
        if !self.init_theme {
            self.theme_manager.apply_theme(ctx);
            self.init_theme = true;
        }

        self.signal_select.draw(ctx, &self.theme_manager);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Center Panel");
            ui.label(r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#);

        });

        ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.));
        self.theme_manager.update(ctx);
    }
}
