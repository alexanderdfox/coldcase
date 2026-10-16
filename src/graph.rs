// src/graph.rs
use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::model::{CaseDocument, Entity, EntityType, Relation};

#[derive(Debug)]
pub struct KnowledgeGraph {
	graph: DiGraph<GraphNode, f32>,
	entity_index: HashMap<String, NodeIndex>, // lowercase name -> node
	relations: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphNode {
	Entity(Entity),
	Document { id: Uuid, title: String },
}

impl KnowledgeGraph {
	pub fn new() -> Self {
		Self {
			graph: DiGraph::new(),
			entity_index: HashMap::new(),
			relations: Vec::new(),
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
				idx
			};

			// Link document <-> entity
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
		self.entity_index.len()
	}

	pub fn save(&self, path: &str) -> Result<()> {
		// Simple export of entities + relations
		let export = GraphExport {
			entities: self
				.entity_index
				.values()
				.filter_map(|idx| match &self.graph[*idx] {
					GraphNode::Entity(e) => Some(e.clone()),
					_ => None,
				})
				.collect(),
			relations: self.relations.clone(),
		};

		let file = std::fs::File::create(path)?;
		serde_json::to_writer_pretty(file, &export)?;
		Ok(())
	}

	pub fn find_connections(&self, name: &str) -> Vec<(String, f32)> {
		let key = name.to_lowercase();
		let Some(&start) = self.entity_index.get(&key) else {
			return vec![];
		};

		let mut results = Vec::new();
		for edge in self.graph.edges(start) {
			if let GraphNode::Entity(e) = &self.graph[edge.target()] {
				results.push((e.name.clone(), *edge.weight()));
			}
		}
		results
	}
}

#[derive(Serialize)]
struct GraphExport {
	entities: Vec<Entity>,
	relations: Vec<Relation>,
}