use egui::{RichText, TextEdit};
use rand::Rng;
use std::time::{Duration, Instant};


pub struct MemoApp {
    pub letter_pairs: String,
    /// When the timer was last started
    pub start_time: Option<Instant>,
    pub elapsed: Duration,
    pub started: bool,
    pub user_input: String,
    pub show_text: bool,
    pub show_answer: bool,
    pub num_pairs: usize,
}

impl Default for MemoApp {
    fn default() -> Self {
        Self {
            letter_pairs: String::new(),
            start_time: None,
            elapsed: Duration::ZERO,
            started: false,
            user_input: String::new(),
            show_text: false,
            show_answer: false,
            num_pairs: 12,
        }
    }
}

impl eframe::App for MemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // println!("{}", self.letter_pairs);
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) && !self.is_running() && !self.show_answer && !self.show_text {
                self.gen_pairs(self.num_pairs);
            }
            if i.key_pressed(egui::Key::Space) && !self.show_text && !self.show_answer {
                self.started = true;
                self.toggle_timer();
            } else if i.key_pressed(egui::Key::R) && !self.show_text {
                self.show_answer = false;
                self.show_text = false;
                self.started = false;
                self.reset_timer();
                self.user_input.clear();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_running() {
                self.pairs(ui);
            }

            if !self.is_running() && self.started && !self.show_answer {
                self.show_text = true;
            }
            self.ui_timer(ctx);
            if self.show_text && self.started {
                ui.label(egui::RichText::new("Type here:").monospace());
                ui.add(TextEdit::multiline(&mut self.user_input).font(egui::TextStyle::Monospace));
                let button = ui.button(egui::RichText::new("Submit").monospace());
                if button.clicked() {
                    self.show_answer = true;
                    self.show_text = false;
                }
            };
            if self.started && self.show_answer {
                self.pairs(ui);
                ui.label(egui::RichText::new(self.user_input.clone()).monospace());
            }

            self.help_label(ui);

            egui::Area::new("bottom_right_slider".into())
                .anchor(egui::Align2::RIGHT_BOTTOM, [-20.0, -30.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Size:").monospace());

                        ui.add(egui::Slider::new(&mut self.num_pairs, 1..=100).show_value(true));
                    });
                });
        });
        ctx.request_repaint();
    }
}

impl MemoApp {
    fn pairs(&mut self, ui: &mut egui::Ui) {
        // egui::Grid::new("grid")
        //     .num_columns(4)
        //     .min_col_width(0.0)
        //     .show(ui, |ui| {
        //         for (i, pair) in self.letter_pairs.iter().enumerate() {
        //             ui.label(egui::RichText::new(pair).monospace());
        //             if (i + 1) % 4 == 0 {
        //                 ui.end_row();
        //             }
        //         }
        //     });
        ui.label(egui::RichText::new(self.letter_pairs.clone()).monospace());
    }

    fn current_time(&self) -> Duration {
        if let Some(start) = self.start_time {
            self.elapsed + start.elapsed()
        } else {
            self.elapsed
        }
    }

    fn toggle_timer(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed += start.elapsed();
            self.start_time = None;
        } else {
            self.start_time = Some(Instant::now())
        }
    }

    fn reset_timer(&mut self) {
        self.elapsed = Duration::ZERO;
        self.start_time = None;
    }

    fn is_running(&self) -> bool {
        self.start_time.is_some()
    }

    fn ui_timer(&self, ctx: &egui::Context) {
        // ui.label(
        //     egui::RichText::new(format!("{:.1}s", self.current_time().as_secs_f64())).monospace(),
        // );
        egui::TopBottomPanel::bottom("bottom_left").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{:.1}s", self.current_time().as_secs_f64()))
                        .monospace(),
                );
            })
        });
    }

    fn help_label(&self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new("Press SPACEBAR to start/stop, press R to restart the timer")
                .monospace(),
        );
    }

    pub fn gen_pairs(&mut self, num: usize) {
        fn parse_pairs(input: Vec<String>) -> String {
            let mut res = String::new();
            for (i, str) in input.iter().enumerate() {
                res.push_str(&format!("{str} "));
                if (i + 1) % 4 == 0 {
                    res.push('\n');
                }
            }
            res
        }
        let scheme: Vec<char> = "ABDEFGHIJKLMNOPQRSTUVWX".chars().collect();

        let mut rng = rand::rng();
        self.letter_pairs = parse_pairs(
            (0..num)
                .map(|_| {
                    loop {
                        let a = scheme[rng.random_range(0..23)];
                        let b = scheme[rng.random_range(0..23)];
                        if a != b {
                            return format!("{a}{b}");
                        }
                    }
                })
                .collect(),
        );
    }
}
