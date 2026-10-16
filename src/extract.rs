use regex::Regex;
use uuid::Uuid;

use crate::model::{CaseDocument, Entity, EntityType};

pub fn extract_entities(doc: &CaseDocument) -> Vec<Entity> {
	let mut entities = Vec::new();

	// Naive person patterns
	let person_re = Regex::new(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)+)\b").unwrap();
	for cap in person_re.captures_iter(&doc.content) {
		let name = cap[1].to_string();
		if name.split_whitespace().count() >= 2 && name.len() < 60 {
			entities.push(Entity {
				id: Uuid::new_v4(),
				name,
				entity_type: EntityType::Person,
				aliases: vec![],
				confidence: 0.55,
			});
		}
	}

	// Locations
	let location_re =
		Regex::new(r"\b(?:in|at|near|around|from)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\b").unwrap();
	for cap in location_re.captures_iter(&doc.content) {
		entities.push(Entity {
			id: Uuid::new_v4(),
			name: cap[1].to_string(),
			entity_type: EntityType::Location,
			aliases: vec![],
			confidence: 0.45,
		});
	}

	// Vehicles
	let vehicle_re = Regex::new(
		r"(?i)\b((?:dark|light|black|white|blue|red|green|gray|grey)\s+)?(?:sedan|truck|suv|van|pickup|motorcycle|bike|car)\b",
	)
	.unwrap();
	for cap in vehicle_re.captures_iter(&doc.content) {
		entities.push(Entity {
			id: Uuid::new_v4(),
			name: cap[0].to_string(),
			entity_type: EntityType::Vehicle,
			aliases: vec![],
			confidence: 0.6,
		});
	}

	// Deduplicate
	entities.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
	entities.dedup_by(|a, b| a.name.to_lowercase() == b.name.to_lowercase());

	entities
}