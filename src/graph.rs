use anyhow::Result;
use petgraph::graph::DiGraph;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::model::{CaseDocument, Entity, Relation};

#[derive(Debug)]
pub struct KnowledgeGraph {
	graph: DiGraph<GraphNode, f32>,
	entity_index: HashMap<String, petgraph::graph::NodeIndex>,
	relations: Vec<Relation>,
	entity_count: usize,
}

#[derive(Debug, Clone)]
enum GraphNode {
	Entity(Entity),
	Document { id: Uuid, title: String },
}

impl KnowledgeGraph {
	pub fn new() -> Self {
		Self {
			graph: DiGraph::new(),
			entity_index: HashMap::new(),
			relations: Vec::new(),
			entity_count: 0,
		}
	}

	pub fn add_document(&mut self, doc: &CaseDocument, entities: &[Entity]) {
		let doc_node = self.graph.add_node(GraphNode::Document {
			id: doc.id,
			title: doc.title.clone(),
		});

		for entity in entities {
			let key = entity.name.to_lowercase();
			let ent_node = if let Some(&idx) = self.entity_index.get(&key) {
				idx
			} else {
				let idx = self.graph.add_node(GraphNode::Entity(entity.clone()));
				self.entity_index.insert(key, idx);
				self.entity_count += 1;
				idx
			};

			self.graph.add_edge(doc_node, ent_node, 1.0);
			self.graph.add_edge(ent_node, doc_node, 1.0);

			self.relations.push(Relation {
				from: doc.id,
				to: entity.id,
				relation_type: "mentions".into(),
				strength: entity.confidence,
				source_doc: doc.id,
			});
		}
	}

	pub fn entity_count(&self) -> usize {
		self.entity_count
	}

	pub fn entities(&self) -> Vec<Entity> {
		self.entity_index
			.values()
			.filter_map(|idx| match &self.graph[*idx] {
				GraphNode::Entity(e) => Some(e.clone()),
				_ => None,
			})
			.collect()
	}

	pub fn save(&self, path: &str) -> Result<()> {
		let export = GraphExport {
			entities: self.entities(),
			relations: self.relations.clone(),
		};
		let file = std::fs::File::create(path)?;
		serde_json::to_writer_pretty(file, &export)?;
		Ok(())
	}
}

#[derive(Serialize)]
struct GraphExport {
	entities: Vec<Entity>,
	relations: Vec<Relation>,
}