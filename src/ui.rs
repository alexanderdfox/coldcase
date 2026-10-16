use eframe::egui::{self, Color32, RichText, ScrollArea, TextEdit};
use std::thread;

use crate::extract;
use crate::graph::KnowledgeGraph;
use crate::ingest::Ingestor;
use crate::model::{CaseDocument, Entity, ScoredLead, TimelineEvent};
use crate::score::LeadScorer;
use crate::timeline::TimelineBuilder;

#[derive(Default)]
pub struct ColdCaseApp {
	// Input
	source_input: String,
	sources: Vec<String>,

	// State
	is_running: bool,
	status: String,
	error: Option<String>,

	// Results
	documents: Vec<CaseDocument>,
	entities: Vec<Entity>,
	leads: Vec<ScoredLead>,
	timelines: Vec<TimelineEvent>,
	entity_count: usize,

	// Shared result channel
	result_rx: Option<std::sync::mpsc::Receiver<AnalysisResult>>,
}

struct AnalysisResult {
	documents: Vec<CaseDocument>,
	entities: Vec<Entity>,
	leads: Vec<ScoredLead>,
	timelines: Vec<TimelineEvent>,
	entity_count: usize,
	error: Option<String>,
}

impl ColdCaseApp {
	pub fn new() -> Self {
		Self {
			status: "Ready. Add sources and click Analyze.".into(),
			..Default::default()
		}
	}

	fn add_source(&mut self) {
		let s = self.source_input.trim().to_string();
		if !s.is_empty() && !self.sources.contains(&s) {
			self.sources.push(s);
			self.source_input.clear();
		}
	}

	fn add_file(&mut self) {
		if let Some(path) = rfd::FileDialog::new()
			.add_filter("Text", &["txt", "md", "html", "json"])
			.pick_file()
		{
			let path_str = path.to_string_lossy().to_string();
			if !self.sources.contains(&path_str) {
				self.sources.push(path_str);
			}
		}
	}

	fn start_analysis(&mut self) {
		if self.sources.is_empty() {
			self.error = Some("Add at least one source.".into());
			return;
		}

		self.is_running = true;
		self.error = None;
		self.status = "Analyzing...".into();
		self.documents.clear();
		self.entities.clear();
		self.leads.clear();
		self.timelines.clear();

		let sources = self.sources.clone();
		let (tx, rx) = std::sync::mpsc::channel();
		self.result_rx = Some(rx);

		thread::spawn(move || {
			let result = run_analysis(sources);
			let _ = tx.send(result);
		});
	}

	fn poll_results(&mut self) {
		if let Some(rx) = &self.result_rx {
			if let Ok(result) = rx.try_recv() {
				self.is_running = false;
				self.result_rx = None;

				if let Some(err) = result.error {
					self.error = Some(err);
					self.status = "Failed.".into();
				} else {
					self.documents = result.documents;
					self.entities = result.entities;
					self.leads = result.leads;
					self.timelines = result.timelines;
					self.entity_count = result.entity_count;
					self.status = format!(
						"Done. {} docs, {} entities, {} leads.",
						self.documents.len(),
						self.entity_count,
						self.leads.len()
					);
				}
			}
		}
	}
}

fn run_analysis(sources: Vec<String>) -> AnalysisResult {
	let mut documents = Vec::new();
	let mut graph = KnowledgeGraph::new();
	let ingestor = Ingestor::new();

	for source in &sources {
		match ingestor.ingest(source) {
			Ok(doc) => {
				let entities = extract::extract_entities(&doc);
				graph.add_document(&doc, &entities);
				documents.push(doc);
			}
			Err(e) => {
				return AnalysisResult {
					documents: vec![],
					entities: vec![],
					leads: vec![],
					timelines: vec![],
					entity_count: 0,
					error: Some(format!("Failed to ingest {}: {}", source, e)),
				};
			}
		}
	}

	let entities = graph.entities();
	let timelines = TimelineBuilder::build(&documents, &graph);
	let leads = LeadScorer::new().score(&graph, &documents);
	let entity_count = graph.entity_count();

	// Save results
	let _ = std::fs::create_dir_all("output");
	let _ = graph.save("output/graph.json");
	if let Ok(file) = std::fs::File::create("output/leads.json") {
		let _ = serde_json::to_writer_pretty(file, &leads);
	}

	AnalysisResult {
		documents,
		entities,
		leads,
		timelines,
		entity_count,
		error: None,
	}
}

impl eframe::App for ColdCaseApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.poll_results();

		egui::TopBottomPanel::top("top").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.heading("OSINT Cold Case Analyzer");
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					if ui.button("Open output folder").clicked() {
						let _ = open::that("output");
					}
				});
			});
		});

		egui::SidePanel::left("sources_panel")
			.resizable(true)
			.default_width(320.0)
			.show(ctx, |ui| {
				ui.heading("Sources");
				ui.separator();

				ui.horizontal(|ui| {
					ui.add(
						TextEdit::singleline(&mut self.source_input)
							.hint_text("URL or path...")
							.desired_width(200.0),
					);
					if ui.button("Add").clicked() {
						self.add_source();
					}
				});

				if ui.button("Add file…").clicked() {
					self.add_file();
				}

				ui.add_space(8.0);
				ui.label(format!("{} sources", self.sources.len()));

				ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
					let mut to_remove = None;
					for (i, src) in self.sources.iter().enumerate() {
						ui.horizontal(|ui| {
							ui.label(src);
							if ui.small_button("×").clicked() {
								to_remove = Some(i);
							}
						});
					}
					if let Some(i) = to_remove {
						self.sources.remove(i);
					}
				});

				ui.add_space(12.0);

				let analyze_enabled = !self.is_running && !self.sources.is_empty();
				if ui
					.add_enabled(analyze_enabled, egui::Button::new("Analyze"))
					.clicked()
				{
					self.start_analysis();
				}

				if self.is_running {
					ui.spinner();
					ui.label("Working…");
				}

				ui.add_space(8.0);
				ui.label(RichText::new(&self.status).small());

				if let Some(err) = &self.error {
					ui.colored_label(Color32::RED, err);
				}
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				// Leads
				ui.heading("Scored Leads");
				if self.leads.is_empty() {
					ui.label("No leads yet.");
				} else {
					for lead in &self.leads {
						ui.group(|ui| {
							ui.label(RichText::new(format!("Score: {:.2}", lead.score)).strong());
							ui.label(&lead.description);
							for reason in &lead.reasons {
								ui.label(format!("• {}", reason));
							}
						});
						ui.add_space(6.0);
					}
				}

				ui.add_space(16.0);
				ui.separator();

				// Entities
				ui.heading(format!("Entities ({})", self.entities.len()));
				if self.entities.is_empty() {
					ui.label("No entities extracted.");
				} else {
					egui::Grid::new("entities_grid")
						.num_columns(3)
						.striped(true)
						.show(ui, |ui| {
							ui.label(RichText::new("Name").strong());
							ui.label(RichText::new("Type").strong());
							ui.label(RichText::new("Confidence").strong());
							ui.end_row();

							for e in &self.entities {
								ui.label(&e.name);
								ui.label(format!("{:?}", e.entity_type));
								ui.label(format!("{:.2}", e.confidence));
								ui.end_row();
							}
						});
				}

				ui.add_space(16.0);
				ui.separator();

				// Timeline
				ui.heading("Timeline");
				if self.timelines.is_empty() {
					ui.label("No timeline events.");
				} else {
					for ev in &self.timelines {
						let ts = ev
							.timestamp
							.map(|t| t.format("%Y-%m-%d %H:%M").to_string())
							.unwrap_or_else(|| "unknown".into());
						ui.label(format!("{}  —  {}", ts, ev.description));
					}
				}

				ui.add_space(16.0);
				ui.separator();

				// Documents
				ui.heading(format!("Documents ({})", self.documents.len()));
				for doc in &self.documents {
					ui.group(|ui| {
						ui.label(RichText::new(&doc.title).strong());
						if let Some(url) = &doc.source_url {
							if ui.link(url).clicked() {
								let _ = open::that(url);
							}
						}
						ui.label(
							RichText::new(
								doc.content.chars().take(280).collect::<String>() + "…",
							)
							.small()
							.weak(),
						);
					});
				}
			});
		});
	}
}