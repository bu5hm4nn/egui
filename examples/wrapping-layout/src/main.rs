use eframe::{
    egui::{self, WidgetText},
    emath::Align,
    epaint::Stroke,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(380.0, 440.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Horizontal Wrapped Layouts",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.hyperlink_to("@npub1vdaeclr2mnntmyw...", "whocares");
                let text = " LotsOfTextPrecededByASpace5kgqfqqxwhkrkw60stn8aph4gm2h2053xvwvvlvjm3q9eqdpqxycrqvpqd3hhgar9wfujqarfvd4k2arncqzpgxqzz6sp5vfenc5l4uafsky0w069zs329edf608ggpjjveguwxfl3xlswg5vq9qyyssqj46d5x3gsnljffm79eqwszk4mk47lkxywdp8mxum7un3qm0ztwj9jf46cm4lw2un9hk4gttgtjdrk29h27xu4e3ume20sqsna8q7xwspqqkwq7";
                ui.label(text);
                ui.style_mut().visuals.widgets.noninteractive.fg_stroke = Stroke::new( 1.0, eframe::epaint::Color32::RED );
                ui.label("More text followed by two newlines\n\n");
                ui.style_mut().visuals.widgets.noninteractive.fg_stroke = Stroke::new( 1.0, eframe::epaint::Color32::GREEN );
                ui.label("more text, no newline");
                ui.reset_style();
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Hyperlink no newline:");
                let url = "https://i.nostrimg.com/c72f5e1a2e162fad2625e15651a654465c06016016f7743b496021cafa2a524e/file.jpeg";
                ui.hyperlink_to( url, url );
                ui.end_row();
                ui.label("Hyperlink break_anywhere=true");
                let mut job = WidgetText::from(url).into_text_job(ui.style(), egui::FontSelection::Default, Align::LEFT);
                job.job.wrap.break_anywhere = true;
                ui.hyperlink_to( job.job, url );
            });
       });
    }
}
