use eframe::egui;

mod lib_knock100_get_point;
use crate::lib_knock100_get_point::get_point;

#[derive(Default)]
pub struct EguiSample {}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {Self::default()}
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let _ = &eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().vline(
                300.0,                                        // x
                std::ops::RangeInclusive::new(100.0, 300.0),  // y
                *&eframe::egui::Stroke {width:3.0, color:eframe::egui::Color32::RED},       // width, color
            );
            ui.painter().rect_filled(
                eframe::egui::Rect { min: eframe::egui::Pos2 {x:50.0, y:50.0}, 
                       max: eframe::egui::Pos2 {x:150.0,y:150.0},},   // location
                8.0,                                    // curve
                eframe::egui::Color32::from_rgb(199,21,133),          // color
            );
            ui.painter().circle_filled(
                eframe::egui::Pos2 {x:450.0, y:300.0},  // location
                50.0,                     // radius
                eframe::egui::Color32::GREEN,           // color
            );
        });
        println!("x: {}, y: {}", x, y);
    }
}

fn main() {
    let x = get_point("input center point x(50.0〜550.0)".to_string());
    let y = get_point("input center point y(50.0〜150.0)".to_string());
    println!("x: {}, y: {}", x, y);
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native("knock100_060", options, Box::new(|cc| Box::new(EguiSample::new(cc))));
}
