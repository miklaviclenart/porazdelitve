use eframe::egui;
use porazdelitve::*;

#[derive(Debug, PartialEq)]
enum DistChoice {
    Bernoulli,
    Binomska,
    Eksponentna,
    EnakomernaInterval,
    Geometrijska,
    Hipergeometrijska,
    Normalna,
    Poisson,
}

struct MyApp {
    selected: DistChoice,

    p: f64,
    n: u32,
    lambda: f64,
    a: f64,
    b: f64,
    mu: f64,
    sigma2: f64,

    info: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected: DistChoice::Bernoulli,
            p: 0.5,
            n: 10,
            lambda: 1.0,
            a: 0.0,
            b: 1.0,
            mu: 0.0,
            sigma2: 1.0,
            info: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Raziskovalec porazdelitev");

            egui::ComboBox::from_label("Izberi porazdelitev")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, DistChoice::Bernoulli, "Bernoullijeva");
                    ui.selectable_value(&mut self.selected, DistChoice::Binomska, "Binomska");
                    ui.selectable_value(&mut self.selected, DistChoice::Eksponentna, "Eksponentna");
                    ui.selectable_value(&mut self.selected, DistChoice::EnakomernaInterval, "Enakomerna na intervalu");
                    ui.selectable_value(&mut self.selected, DistChoice::Geometrijska, "Geometrijska");
                    ui.selectable_value(&mut self.selected, DistChoice::Hipergeometrijska, "Hipergeometrijska");
                    ui.selectable_value(&mut self.selected, DistChoice::Normalna, "Normalna");
                    ui.selectable_value(&mut self.selected, DistChoice::Poisson, "Poissonova");
                });

            ui.separator();

            match self.selected {
                DistChoice::Bernoulli => {
                    ui.add(egui::Slider::new(&mut self.p, 0.0..=1.0).text("p"));
                }
                DistChoice::Binomska => {
                    ui.add(egui::Slider::new(&mut self.n, 1..=100).text("n"));
                    ui.add(egui::Slider::new(&mut self.p, 0.0..=1.0).text("p"));
                }
                DistChoice::Eksponentna => {
                    ui.add(egui::Slider::new(&mut self.lambda, 0.1..=10.0).text("λ"));
                }
                DistChoice::EnakomernaInterval => {
                    ui.add(egui::Slider::new(&mut self.a, -100.0..=100.0).text("a"));
                    ui.add(egui::Slider::new(&mut self.b, -100.0..=100.0).text("b"));
                }
                DistChoice::Normalna => {
                    ui.add(egui::Slider::new(&mut self.mu, -10.0..=10.0).text("μ"));
                    ui.add(egui::Slider::new(&mut self.sigma2, 0.1..=5.0).text("σ²"));
                }
                DistChoice::Poisson => {
                    ui.add(egui::Slider::new(&mut self.lambda, 0.1..=20.0).text("λ"));
                }
                _ => {
                    ui.label("Napaka");
                }
            }

            if ui.button("Prikaži").clicked() {
                self.info = match self.selected {
                    DistChoice::Bernoulli => {
                        let d = Bernoulli::new(self.p);
                        format!(
                            "Bernoulli (p={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.p,
                            d.e(),
                            d.var()
                        )
                    }
                    DistChoice::Binomska => {
                        let d = Binomska::new(self.n.into(), self.p);
                        format!(
                            "Bin (n={}, p={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.n,
                            self.p,
                            d.e(),
                            d.var()
                        )
                    }
                    DistChoice::Eksponentna => {
                        let d = Eksponentna::new(self.lambda);
                        format!(
                            "Exp (λ={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.lambda,
                            d.e(),
                            d.var()
                        )
                    }
                    DistChoice::EnakomernaInterval => {
                        let d = EnakomernaInterval::new(self.a, self.b);
                        format!(
                            "U (a={:.2}, b={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.a,
                            self.b,
                            d.e(),
                            d.var()
                        )
                    }
                    DistChoice::Normalna => {
                        let d = Normalna::new(self.mu, self.sigma2);
                        format!(
                            "N (μ={:.2}, σ²={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.mu,
                            self.sigma2,
                            d.e(),
                            d.var()
                        )
                    }
                    DistChoice::Poisson => {
                        let d = Poisson::new(self.lambda);
                        format!(
                            "Po (λ={:.2})\nPričakovana vrednost: {:.2}\nVarianca: {:.2}",
                            self.lambda,
                            d.e(),
                            d.var()
                        )
                    }
                    _ => "Not implemented yet".to_string(),
                };
            }

            ui.separator();
            ui.label(&self.info);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VERJETNOST",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::<MyApp>::default())
        }),
    )
}
