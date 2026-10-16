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
```

On first run it will download dependencies and open a native window.

---

## How to Use

1. In the left panel, paste a URL or click **Add file…** to select local `.txt` / `.md` files.
2. Click **Analyze**.
3. View results in the main panel:
   - Scored Leads
   - Extracted Entities
   - Timeline
   - Document summaries
4. Click **Open output folder** to see the generated JSON files (`graph.json`, `leads.json`).

---

## Project Structure

```
coldcase/
├── src/
│   ├── main.rs          # Entry point
│   ├── ui.rs            # egui GUI
│   ├── model.rs         # Data structures
│   ├── ingest.rs        # URL + file ingestion
│   ├── extract.rs       # Entity extraction
│   ├── graph.rs         # Knowledge graph
│   ├── timeline.rs      # Timeline builder
│   └── score.rs         # Lead scoring
├── output/              # Generated results (gitignored)
├── Cargo.toml
└── README.md
```

---

## Building a Release

```bash
cargo build --release
```

The binary will be at `target/release/coldcase`.

---

## Future Improvements

- Better Named Entity Recognition (NLP models)
- Semantic similarity / embeddings
- Graph visualization
- More source connectors (RSS, archive.org, etc.)
- Entity resolution & deduplication
- Session save/load

---

## License

MIT (or your preferred license)

---

## Disclaimer

This is an experimental research tool. Entity extraction is currently rule/regex-based and will produce false positives. Always verify findings manually. The authors assume no liability for misuse.
```
