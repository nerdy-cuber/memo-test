mod ui;
use ui::MemoApp;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let mut app = MemoApp::default();
    app.gen_pairs();
    // app.letter_pairs = gen_pairs();
    eframe::run_native(
        "deez nuts",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
