use eframe::egui;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use porazdelitve::*;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    n: u64,
    s: u64,
    r: u64,
    lambda: f64,
    a: f64,
    b: f64,
    mu: f64,
    sigma2: f64,

    info: String,

    cached_params: Option<CachedParams>,
    cached_plot_data: Option<PlotData>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct CachedParams {
    selected: DistChoice,
    p: f64,
    n: u64,
    s: u64,
    r: u64,
    lambda: f64,
    a: f64,
    b: f64,
    mu: f64,
    sigma2: f64,
}

#[derive(Debug)]
enum PlotData {
    Bars(Vec<Bar>),
    Line(Vec<[f64; 2]>),
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected: DistChoice::Bernoulli,
            p: 0.5,
            n: 10,
            s: 5,
            r: 5,
            lambda: 1.0,
            a: 0.0,
            b: 1.0,
            mu: 0.0,
            sigma2: 1.0,
            info: String::new(),
            cached_params: None,
            cached_plot_data: None,
        }
    }
}

impl MyApp {
    fn current_params(&self) -> CachedParams {
        CachedParams {
            selected: self.selected,
            p: self.p,
            n: self.n,
            s: self.s,
            r: self.r,
            lambda: self.lambda,
            a: self.a,
            b: self.b,
            mu: self.mu,
            sigma2: self.sigma2,
        }
    }

    fn get_plot_data(&mut self) -> &PlotData {
        let current_params = self.current_params();
        
        if self.cached_params.as_ref() != Some(&current_params) {
            self.cached_plot_data = Some(self.calculate_plot_data());
            self.cached_params = Some(current_params);
        }
        
        self.cached_plot_data.as_ref().unwrap()
    }

    fn calculate_plot_data(&self) -> PlotData {
        match self.selected {
            DistChoice::Bernoulli => {
                let bars = vec![
                    Bar::new(0.0, 1.0 - self.p).width(0.6),
                    Bar::new(1.0, self.p).width(0.6),
                ];
                PlotData::Bars(bars)
            },
            DistChoice::Binomska => {
                let d = Binomska::new(self.n, self.p);
                let mean = d.e();
                let std = d.var().sqrt();
                
                let x_min = (mean - 3.0 * std).max(0.0) as u64;
                let x_max = (mean + 3.0 * std).min(self.n as f64) as u64;

                let bars: Vec<Bar> = (x_min..=x_max)
                    .map(|k| Bar::new(k as f64, d.pmf(k)).width(0.6))
                    .collect();

                PlotData::Bars(bars)
            },
            DistChoice::Eksponentna => {
                let x_max = 4.0 / self.lambda;

                let points: Vec<[f64; 2]> = (0..=500)
                    .map(|i| {
                        let x = i as f64 / 500.0 * x_max;
                        let y = self.lambda * (-self.lambda * x).exp();
                        [x, y]
                    })
                    .collect();

                PlotData::Line(points)
            },
            DistChoice::EnakomernaInterval => {
                let height = 1.0 / (self.b - self.a);
                let margin = (self.b - self.a) * 0.1;
                
                let points = vec![
                    [self.a - margin, 0.0],
                    [self.a, 0.0],
                    [self.a, height],
                    [self.b, height],
                    [self.b, 0.0],
                    [self.b + margin, 0.0],
                ];
                
                PlotData::Line(points)
            },
            DistChoice::Geometrijska => {
                let d = Geometrijska::new(self.p);
                let x_max = ((-5.0 / d.q.ln()) - 1.0).ceil() as u64;

                let bars: Vec<Bar> = (0..=x_max)
                    .filter_map(|k| {
                        let pmf_val = d.pmf(k);
                        Some(Bar::new(k as f64, pmf_val).width(0.6))
                    })
                    .collect();

                PlotData::Bars(bars)
            },
            DistChoice::Hipergeometrijska => {
                let x_min = if self.n > self.r { self.n - self.r } else { 0 };
                let x_max = self.n.min(self.s);

                if x_min <= x_max {
                    let d = Hipergeometrijska::new(self.s, self.r, self.n);
                    let bars: Vec<Bar> = (x_min..=x_max)
                        .map(|k| Bar::new(k as f64, d.pmf(k)).width(0.6))
                        .collect();
                    PlotData::Bars(bars)
                } else {
                    PlotData::Bars(vec![])
                }
            },
            DistChoice::Normalna => {
                let sigma = self.sigma2.sqrt();
                let x_min = self.mu - 4.0 * sigma;
                let x_max = self.mu + 4.0 * sigma;

                let d = Normalna::new(self.mu, self.sigma2);
                let points: Vec<[f64; 2]> = (0..=500)
                    .map(|i| {
                        let x = x_min + (x_max - x_min) * i as f64 / 500.0;
                        let y = d.pdf(x);
                        [x, y]
                    })
                    .collect();

                PlotData::Line(points)
            },
            DistChoice::Poisson => {
                let x_max = (self.lambda + 4.0 * self.lambda.sqrt()).ceil() as u64;
                
                let d = Poisson::new(self.lambda);
                let bars: Vec<Bar> = (0..=x_max)
                    .filter_map(|k| {
                        let pmf_val = d.pmf(k);
                        Some(Bar::new(k as f64, pmf_val).width(0.6))
                    })
                    .collect();
                
                PlotData::Bars(bars)
            }
        }
    }

    fn update_info(&mut self) {
        self.info = match self.selected {
            DistChoice::Bernoulli => {
                let d = Bernoulli::new(self.p);
                format!(
                    "Bernoulli (p={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.p, d.e(), d.var()
                )
            }
            DistChoice::Binomska => {
                let d = Binomska::new(self.n, self.p);
                format!(
                    "Binomska (n={}, p={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.n, self.p, d.e(), d.var()
                )
            }
            DistChoice::Eksponentna => {
                let d = Eksponentna::new(self.lambda);
                format!(
                    "Eksponentna (λ={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.lambda, d.e(), d.var()
                )
            }
            DistChoice::EnakomernaInterval => {
                let d = EnakomernaInterval::new(self.a, self.b);
                format!(
                    "Enakomerna (a={:.3}, b={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.a, self.b, d.e(), d.var()
                )
            }
            DistChoice::Normalna => {
                let d = Normalna::new(self.mu, self.sigma2);
                format!(
                    "Normalna (μ={:.3}, σ²={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.mu, self.sigma2, d.e(), d.var()
                )
            }
            DistChoice::Poisson => {
                let d = Poisson::new(self.lambda);
                format!(
                    "Poissonova (λ={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.lambda, d.e(), d.var()
                )
            }
            DistChoice::Geometrijska => {
                let d = Geometrijska::new(self.p);
                format!(
                    "Geometrijska (p={:.3})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.p, d.e(), d.var()
                )
            }
            DistChoice::Hipergeometrijska => {
                let d = Hipergeometrijska::new(self.s, self.r, self.n);
                format!(
                    "Hipergeometrijska (s={}, r={}, n={})\nPričakovana vrednost: {:.3}\nVarianca: {:.3}",
                    self.s, self.r, self.n, d.e(), d.var()
                )
            }
        };
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

            let mut params_changed = false;

            match self.selected {
                DistChoice::Bernoulli | DistChoice::Geometrijska => {
                    let old_p = self.p;
                    ui.add(egui::Slider::new(&mut self.p, 0.01..=0.99).text("p"));
                    if (old_p - self.p).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::Binomska => {
                    let old_n = self.n;
                    let old_p = self.p;
                    ui.add(egui::Slider::new(&mut self.n, 1..=100).text("n"));
                    ui.add(egui::Slider::new(&mut self.p, 0.01..=0.99).text("p"));
                    if old_n != self.n || (old_p - self.p).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::Eksponentna => {
                    let old_lambda = self.lambda;
                    ui.add(egui::Slider::new(&mut self.lambda, 0.1..=10.0).text("λ"));
                    if (old_lambda - self.lambda).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::EnakomernaInterval => {
                    let old_a = self.a;
                    let old_b = self.b;
                    ui.add(egui::Slider::new(&mut self.a, -10.0..=10.0).text("a"));
                    ui.add(egui::Slider::new(&mut self.b, -10.0..=10.0).text("b"));

                    // Ensure a < b
                    if self.a >= self.b {
                        self.b = self.a + 0.1;
                    }
                    
                    if (old_a - self.a).abs() > f64::EPSILON || (old_b - self.b).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::Normalna => {
                    let old_mu = self.mu;
                    let old_sigma2 = self.sigma2;
                    ui.add(egui::Slider::new(&mut self.mu, -10.0..=10.0).text("μ"));
                    ui.add(egui::Slider::new(&mut self.sigma2, 0.1..=5.0).text("σ²"));
                    if (old_mu - self.mu).abs() > f64::EPSILON || (old_sigma2 - self.sigma2).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::Poisson => {
                    let old_lambda = self.lambda;
                    ui.add(egui::Slider::new(&mut self.lambda, 0.1..=20.0).text("λ"));
                    if (old_lambda - self.lambda).abs() > f64::EPSILON {
                        params_changed = true;
                    }
                }
                DistChoice::Hipergeometrijska => {
                    let old_s = self.s;
                    let old_r = self.r;
                    let old_n = self.n;
                    ui.add(egui::Slider::new(&mut self.s, 1..=100).text("s (uspešni elementi)"));
                    ui.add(egui::Slider::new(&mut self.r, 1..=100).text("r (skupaj elementov)"));
                    ui.add(egui::Slider::new(&mut self.n, 1..=100).text("n (vzorec)"));
                    
                    if self.s > self.r {
                        self.r = self.s;
                    }
                    if self.n > self.r {
                        self.n = self.r;
                    }
                    
                    if old_s != self.s || old_r != self.r || old_n != self.n {
                        params_changed = true;
                    }
                }
            }

            if ui.button("Prikaži").clicked() || params_changed {
                self.update_info();
                self.cached_params = None;
                self.cached_plot_data = None;
            }

            ui.separator();
            ui.label(&self.info);

            let plot = Plot::new("distribution-plot")
                                                    .view_aspect(2.0);

            plot.show(ui, |plot_ui| {
                match self.get_plot_data() {
                    PlotData::Bars(bars) => {
                        if !bars.is_empty() {
                            let chart = BarChart::new("Distribution".to_string(), bars.clone());
                            plot_ui.bar_chart(chart);
                        }
                    }
                    PlotData::Line(points) => {
                        let line = Line::new("Distribution".to_string(), PlotPoints::from(points.clone()));
                        plot_ui.line(line);
                    }
                }
            });
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