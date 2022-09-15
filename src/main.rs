// Copyright (C) 2022 Yehowshua Immanuel
// No part of this program may be redistributed, copied, acquired,
// or modified under any circumstance except with explicit permission
// from Yehowshua Immanuel.

use std::fs::File;
use std::rc::Rc;

mod signal_select;
mod vcd_viewer;

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
            let vcd = Rc::new(fastwave_backend::parse_vcd(file).unwrap());
            let elapsed = now.elapsed();

            println!("Parsed VCD file {} : {:.2?}", file_path, elapsed);
            Box::new(vcd_viewer::VCDViewer::new(vcd.clone()))
        }),
    );

    Ok(())
}
