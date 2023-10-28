use eframe::egui;
use crate::egui::Color32;
use crate::egui::plot::Bar;
use crate::egui::plot::BarChart;
use egui::plot::Plot;
use crate::egui::plot::Legend;

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(800.0, 1200.00)),
		..Default::default()
	};
	eframe::run_native(
	"Compound Monthly, now with graph!",
	options,
	Box::new(|cc| Box::new(MyApp::new(cc))),
	)
}
#[derive(Default)]
struct MyApp {
    starting_balance: String,
    time: String,
    percentage: String,
    deposit: String,
    final_balance: String,
    profit_vec: Vec<Bar>,
    capital_vec: Vec<Bar>,
}


fn compound_over_years(time: u32, bal: f64, rate: f64, dep: f64, profit_vec: &mut Vec<Bar>, capital_vec: &mut Vec<Bar>) -> f64 {
    *profit_vec = Vec::new();
    *capital_vec = Vec::new();

    let mut value_x = 0.5;
    let mut res: f64 = bal;
    //let mut value_to_add = 0.0;
    let mut current_capital_item = bal;
    let mut current_profit_item = 0.0;

    for _ in 0..(time * 12) {
        let value_to_add = res * (rate / 100.0);
        current_profit_item += value_to_add;
        res = res + value_to_add;
        current_capital_item += dep;
        res += dep;
        profit_vec.push(Bar::new(value_x, current_profit_item));
        capital_vec.push(Bar::new(value_x, current_capital_item));
        value_x += 1.0;
    }
    res
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            starting_balance: "1000".to_string(),
            time: "5".to_string(),
            percentage: "7".to_string(),
            deposit: "300".to_string(),
            final_balance: "Run a calculation to proceed".to_string(),
            profit_vec: Vec::new(),
            capital_vec: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Graphic Compounding");

            ui.label("Starting Balance".to_string());
            let _starting_balance = ui.add(egui::TextEdit::singleline(&mut self.starting_balance));
            ui.label("Years".to_string());
            let _time = ui.add(egui::TextEdit::singleline(&mut self.time));
            ui.label("Percent Return Per Month".to_string());
            let _percentage = ui.add(egui::TextEdit::singleline(&mut self.percentage));
            ui.label("Monthly Contribution".to_string());
            let _deposit = ui.add(egui::TextEdit::singleline(&mut self.deposit));
			if ui.button("Calculate").clicked() {
                let time = self.time.parse::<u32>().unwrap();
                let starting_balance = self.starting_balance.parse::<f64>().unwrap();
                let percentage = self.percentage.parse::<f64>().unwrap();
                let deposit = self.deposit.parse::<f64>().unwrap();
                let temp_result = compound_over_years(
                    time,
                    starting_balance,
                    percentage,
                    deposit,
                    &mut self.profit_vec,
                    &mut self.capital_vec
                    ).to_string();
                self.final_balance = format!("${}", &temp_result);
			}
            ui.label("Final Balance:".to_string());
            ui.label(&self.final_balance);

            // Graph part start

            let profit = BarChart::new(self.profit_vec.clone())
            .width(0.7)
            .color(Color32::from_rgb(144, 238, 144))
            .name("Profit");

            let capital = BarChart::new(self.capital_vec.clone())
            .width(0.7)
            .name("Capital")
            .color(Color32::from_rgb(173, 216, 255))
            .stack_on(&[&profit]);

            ui.vertical(|ui| {

            Plot::new("Normal Distribution Demo")
                .legend(Legend::default())
                .clamp_grid(true)
                .show(ui, |plot_ui| {
                plot_ui.bar_chart(profit);
                plot_ui.bar_chart(capital);
            })
            .response

            });

            //Graph part end

        });
	}
}

