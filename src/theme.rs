use std::sync::{Arc, Mutex};
use std::thread;

const FRAME_COLOR_DARK: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 0, 0, 100);
const FRAME_COLOR_LIGHT: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 0, 0, 25);
const SMALL_ROUND: epaint::Rounding = epaint::Rounding {
    nw: 6.8,
    ne: 6.8,
    sw: 6.8,
    se: 6.8,
};
const LARGE_ROUND: epaint::Rounding = epaint::Rounding {
    nw: 20.,
    ne: 20.,
    sw: 20.,
    se: 20.,
};


pub struct ThemeManager {
    /// we may need to update changes to the theme if the
    /// system switches into dark mode for example or when
    /// we first launch the program - to override egui style
    /// defaults
    system_dark_theme: Arc<Mutex<bool>>,
    viewer_dark_theme_is_active: bool,
    first_frame: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        let system_dark_active_init = match dark_light::detect() {
            dark_light::Mode::Dark => true,
            dark_light::Mode::Light => false,
        };
        let mut result = ThemeManager {
            system_dark_theme: Arc::new(Mutex::new(true)),
            viewer_dark_theme_is_active: system_dark_active_init,
            first_frame: true,
        };
        let thread_arc = result.system_dark_theme.clone();

        // spawn a thread that periodically checks and records if dark mode
        // is active
        thread::spawn(move || {
            while true {
                if let Ok(mut system_dark_theme) = thread_arc.lock() {
                    *system_dark_theme = match dark_light::detect() {
                        dark_light::Mode::Dark => {
                            // println!("got system theme state dark");
                            true
                        },
                        dark_light::Mode::Light => {
                            // println!("got system theme state light");
                            false
                        },
                    };
                }
                thread::sleep(std::time::Duration::from_millis(750));
            }
        });
        result
    }
    pub fn apply_theme(&self, ctx: &egui::Context) {
        // derive base UI setting from the dark visual scheme
        if self.viewer_dark_theme_is_active {
            ctx.set_visuals(egui::Visuals::dark());

        }
        else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // clone style so that we can modify it
        let mut style = (*ctx.style()).clone();

        // change the curvature and remove outline
        // for selected box selection and selection like
        // UI elements such as selectable labels and buttons
        style.visuals.widgets.noninteractive.rounding = SMALL_ROUND;
        style.visuals.widgets.hovered.rounding = SMALL_ROUND;
        style.visuals.widgets.active.rounding = SMALL_ROUND;
        style.visuals.widgets.inactive.rounding = SMALL_ROUND;
        style.visuals.widgets.open.rounding = SMALL_ROUND;

        // we can indicate separation using two tone separation
        // instead of line-based separation.
        style.visuals.widgets.noninteractive.bg_stroke.width = 0.;
        style.visuals.widgets.hovered.bg_stroke.width = 0.;
        style.visuals.widgets.active.bg_stroke.width = 0.;
        style.visuals.widgets.inactive.bg_stroke.width = 0.;
        style.visuals.widgets.open.bg_stroke.width = 0.;

        // apply style changes
        ctx.set_style(style);
    }
    pub fn update(&mut self, ctx: &egui::Context) {
        // dereferencing Arcs are up to 8x more expensive, and since many
        // functions must know the current theme in order to draw properly,
        // it's preferable for such functions to access a cheap copy of
        // ``system_dark_theme`` in ``viewer_dark_theme_is_active``
        let viewer_dark_theme_was_active = self.viewer_dark_theme_is_active.clone();
        let thread_arc = self.system_dark_theme.clone();

        // We don't block on acquiring a mutex lock to the current state of
        // the system dark/light theme, if such a lock is not available,
        // which means that the theme of the application may be briefly
        // out of snc with the system theme.
        if let Ok(system_dark_theme) = thread_arc.try_lock() {
            self.viewer_dark_theme_is_active = *system_dark_theme;
        };

        // we only need to update the viewer style if there was a change in
        // in system theme
        if self.viewer_dark_theme_is_active != viewer_dark_theme_was_active {
            self.apply_theme(ctx);
        }
    }
    pub fn new_frame(&self) -> egui::Frame {
        egui::Frame::none()
            .fill(if self.viewer_dark_theme_is_active {
                FRAME_COLOR_DARK
            } else {
                FRAME_COLOR_LIGHT
            })
            .rounding(LARGE_ROUND)
            .inner_margin(9.)
    }
}
