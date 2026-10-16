mod extract;
mod graph;
mod ingest;
mod model;
mod score;
mod timeline;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
	tracing_subscriber::fmt::init();

	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([1100.0, 750.0])
			.with_title("OSINT Cold Case Analyzer"),
		..Default::default()
	};

	eframe::run_native(
		"OSINT Cold Case Analyzer",
		options,
		Box::new(|_cc| Ok(Box::new(ui::ColdCaseApp::new()))),
	)
}