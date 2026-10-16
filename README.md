# OSINT Cold Case Analyzer

A desktop tool written in Rust for analyzing publicly available information (OSINT) related to cold cases and investigative research.

It lets you ingest news articles, public reports, and local text files, then extracts entities, builds simple connections, generates timelines, and surfaces scored leads.

---

## Features

- **Ingest** public web pages (URLs) and local text/markdown files
- **Entity extraction** (people, locations, vehicles, etc.)
- **Knowledge graph** of documents ↔ entities
- **Timeline** generation
- **Lead scoring** with simple heuristics
- **Desktop GUI** built with `egui` + `eframe`
- Results exported to JSON (`output/` folder)

> **Important**: This tool is for legitimate research and investigative support using *publicly available* information only. Respect robots.txt, rate limits, terms of service, and all applicable laws. Do not use it for harassment, doxxing, or any illegal purpose.

---

## Requirements

- Rust (1.70+)
- A desktop environment (Windows, macOS, or Linux)

---

## Getting Started

```bash
# Clone the repository
git clone <your-repo-url>
cd coldcase

# Run the application
cargo run
