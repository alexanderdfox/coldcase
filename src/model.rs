use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDocument {
	pub id: Uuid,
	pub title: String,
	pub source_url: Option<String>,
	pub content: String,
	pub published_at: Option<DateTime<Utc>>,
	pub retrieved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
	Person,
	Location,
	Organization,
	Vehicle,
	Weapon,
	Date,
	Event,
	Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
	pub id: Uuid,
	pub name: String,
	pub entity_type: EntityType,
	pub aliases: Vec<String>,
	pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
	pub from: Uuid,
	pub to: Uuid,
	pub relation_type: String,
	pub strength: f32,
	pub source_doc: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredLead {
	pub description: String,
	pub entities: Vec<Uuid>,
	pub score: f32,
	pub reasons: Vec<String>,
	pub related_docs: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
	pub timestamp: Option<DateTime<Utc>>,
	pub description: String,
	pub source_doc: Uuid,
	pub entities: Vec<String>,
}