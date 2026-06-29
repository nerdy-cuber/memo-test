mod ui;
use ui::MemoApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let app = MemoApp::default();
    // app.num_pairs = contents.parse::<usize>().unwrap_or(24);
    // app.gen_pairs(app.num_pairs);
    // app.letter_pairs = gen_pairs();
    eframe::run_native("memo test", options, Box::new(|_cc| Ok(Box::new(app))))
}
