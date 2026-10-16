use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use uuid::Uuid;

use crate::model::CaseDocument;

pub struct Ingestor {
	client: Client,
}

impl Ingestor {
	pub fn new() -> Self {
		Self {
			client: Client::builder()
				.user_agent("OSINT-ColdCase-Research/0.1 (educational)")
				.timeout(std::time::Duration::from_secs(25))
				.build()
				.expect("Failed to build HTTP client"),
		}
	}

	pub fn ingest(&self, source: &str) -> Result<CaseDocument> {
		if source.starts_with("http://") || source.starts_with("https://") {
			self.ingest_url(source)
		} else {
			self.ingest_file(source)
		}
	}

	fn ingest_url(&self, url: &str) -> Result<CaseDocument> {
		let resp = self.client.get(url).send()?;
		if !resp.status().is_success() {
			return Err(anyhow!("HTTP {}", resp.status()));
		}

		let html = resp.text()?;
		let document = Html::parse_document(&html);

		let title = document
			.select(&Selector::parse("title").unwrap())
			.next()
			.map(|t| t.text().collect::<String>())
			.unwrap_or_else(|| url.to_string());

		let body_selector = Selector::parse("article, main, .content, .post, body").unwrap();
		let content: String = document
			.select(&body_selector)
			.flat_map(|el| el.text())
			.collect::<Vec<_>>()
			.join(" ")
			.split_whitespace()
			.collect::<Vec<_>>()
			.join(" ");

		Ok(CaseDocument {
			id: Uuid::new_v4(),
			title: title.trim().to_string(),
			source_url: Some(url.to_string()),
			content,
			published_at: None,
			retrieved_at: Utc::now(),
		})
	}

	fn ingest_file(&self, path: &str) -> Result<CaseDocument> {
		let content = std::fs::read_to_string(path)?;
		Ok(CaseDocument {
			id: Uuid::new_v4(),
			title: path.to_string(),
			source_url: None,
			content,
			published_at: None,
			retrieved_at: Utc::now(),
		})
	}
}