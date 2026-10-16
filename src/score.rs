// src/score.rs
use crate::graph::KnowledgeGraph;
use crate::model::{CaseDocument, ScoredLead};

pub struct LeadScorer;

impl LeadScorer {
	pub fn new() -> Self {
		Self
	}

	pub fn score(&self, graph: &KnowledgeGraph, docs: &[CaseDocument]) -> Vec<ScoredLead> {
		let mut leads = Vec::new();

		// Example heuristic scoring
		// Real system would use:
		// - frequency of co-occurrence
		// - rare entity combinations
		// - temporal proximity
		// - geographic clustering
		// - contradiction detection

		if graph.entity_count() > 0 && !docs.is_empty() {
			leads.push(ScoredLead {
				description: "Multiple documents reference overlapping entities — possible connection".into(),
				entities: vec![],
				score: 0.65,
				reasons: vec![
					"Shared entities across sources".into(),
					"Temporal proximity of publications".into(),
				],
				related_docs: docs.iter().map(|d| d.id).collect(),
			});
		}

		leads
	}
}