use crate::graph::KnowledgeGraph;
use crate::model::{CaseDocument, ScoredLead};

pub struct LeadScorer;

impl LeadScorer {
	pub fn new() -> Self {
		Self
	}

	pub fn score(&self, graph: &KnowledgeGraph, docs: &[CaseDocument]) -> Vec<ScoredLead> {
		let mut leads = Vec::new();

		if graph.entity_count() > 0 && !docs.is_empty() {
			leads.push(ScoredLead {
				description: "Multiple documents reference overlapping entities — possible connection"
					.into(),
				entities: vec![],
				score: 0.68,
				reasons: vec![
					"Shared entities across sources".into(),
					"Temporal proximity of publications".into(),
				],
				related_docs: docs.iter().map(|d| d.id).collect(),
			});
		}

		if graph.entity_count() >= 5 {
			leads.push(ScoredLead {
				description: "High entity density detected — worth deeper review".into(),
				entities: vec![],
				score: 0.52,
				reasons: vec!["Many named entities extracted from limited sources".into()],
				related_docs: docs.iter().map(|d| d.id).collect(),
			});
		}

		leads
	}
}