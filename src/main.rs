// src/main.rs
use anyhow::Result;
use clap::Parser;
use tracing::info;

mod extract;
mod graph;
mod ingest;
mod model;
mod score;
mod timeline;

use crate::graph::KnowledgeGraph;
use crate::ingest::Ingestor;
use crate::model::CaseDocument;
use crate::score::LeadScorer;
use crate::timeline::TimelineBuilder;

#[derive(Parser, Debug)]
#[command(name = "osint-coldcase")]
#[command(about = "OSINT-focused cold case analysis tool")]
struct Args {
	/// URLs or local file paths to ingest
	#[arg(short, long, num_args = 1..)]
	sources: Vec<String>,

	/// Output directory for results
	#[arg(short, long, default_value = "output")]
	output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();
	let args = Args::parse();

	info!("Starting OSINT cold case analysis");

	let mut graph = KnowledgeGraph::new();
	let mut documents: Vec<CaseDocument> = Vec::new();

	// 1. Ingest
	let ingestor = Ingestor::new();
	for source in &args.sources {
		match ingestor.ingest(source).await {
			Ok(doc) => {
				info!("Ingested: {}", doc.title);
				documents.push(doc);
			}
			Err(e) => tracing::warn!("Failed to ingest {}: {}", source, e),
		}
	}

	// 2. Extract entities + build graph
	for doc in &documents {
		let entities = extract::extract_entities(doc);
		graph.add_document(doc, &entities);
	}

	// 3. Build timelines
	let timelines = TimelineBuilder::build(&documents, &graph);

	// 4. Score potential leads / connections
	let scorer = LeadScorer::new();
	let leads = scorer.score(&graph, &documents);

	// 5. Output
	std::fs::create_dir_all(&args.output)?;
	
	let graph_path = format!("{}/graph.json", args.output);
	graph.save(&graph_path)?;
	
	let leads_path = format!("{}/leads.json", args.output);
	serde_json::to_writer_pretty(
		std::fs::File::create(&leads_path)?,
		&leads,
	)?;

	let timeline_path = format!("{}/timelines.json", args.output);
	serde_json::to_writer_pretty(
		std::fs::File::create(&timeline_path)?,
		&timelines,
	)?;

	info!("Analysis complete. Results written to {}", args.output);
	info!("Documents processed: {}", documents.len());
	info!("Entities in graph: {}", graph.entity_count());
	info!("Potential leads scored: {}", leads.len());

	Ok(())
}