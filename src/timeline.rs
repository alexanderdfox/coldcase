// src/timeline.rs
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::graph::KnowledgeGraph;
use crate::model::CaseDocument;

#[derive(Debug, Serialize)]
pub struct TimelineEvent {
	pub timestamp: Option<DateTime<Utc>>,
	pub description: String,
	pub source_doc: Uuid,
	pub entities: Vec<String>,
}

pub struct TimelineBuilder;

impl TimelineBuilder {
	pub fn build(docs: &[CaseDocument], _graph: &KnowledgeGraph) -> Vec<TimelineEvent> {
		let mut events = Vec::new();

		for doc in docs {
			// Extremely simple: treat each document as an event
			// In a real system you would parse dates from content + extract event phrases
			events.push(TimelineEvent {
				timestamp: doc.published_at.or(Some(doc.retrieved_at)),
				description: doc.title.clone(),
				source_doc: doc.id,
				entities: vec![], // fill from graph later
			});
		}

		events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
		events
	}
}