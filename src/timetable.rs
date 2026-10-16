use crate::model::{CaseDocument, TimelineEvent};
use crate::graph::KnowledgeGraph;

pub struct TimelineBuilder;

impl TimelineBuilder {
	pub fn build(docs: &[CaseDocument], _graph: &KnowledgeGraph) -> Vec<TimelineEvent> {
		let mut events: Vec<TimelineEvent> = docs
			.iter()
			.map(|doc| TimelineEvent {
				timestamp: doc.published_at.or(Some(doc.retrieved_at)),
				description: doc.title.clone(),
				source_doc: doc.id,
				entities: vec![],
			})
			.collect();

		events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
		events
	}
}