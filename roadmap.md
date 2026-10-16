# OSINT Cold Case Analyzer – Roadmap

This document outlines the planned evolution of the project, from the current experimental prototype to a more capable and reliable investigative research tool.

---

## Current Status (v0.1)

- [x] Basic URL + local file ingestion
- [x] Regex-based entity extraction (Person, Location, Vehicle)
- [x] Simple knowledge graph (documents ↔ entities)
- [x] Basic timeline generation
- [x] Heuristic lead scoring
- [x] Desktop GUI with egui
- [x] JSON export of results
- [x] Project structure + README + .gitignore

**Limitations of current version:**
- Entity extraction is noisy and incomplete
- No real semantic understanding
- No cross-document similarity
- No persistent sessions
- Graph is not visualized
- Limited source types

---

## Phase 1 – Foundation & Reliability (v0.2 – v0.3)
**Goal:** Make the tool stable and more useful for real research sessions.

### Core Improvements
- [ ] Better error handling and user feedback in the GUI
- [ ] Progress indicators for long-running ingestion
- [ ] Cancel analysis button
- [ ] Save / Load analysis sessions
- [ ] Configurable output directory
- [ ] Dark / light theme toggle

### Data Quality
- [ ] Improved content extraction (readability algorithm or better selectors)
- [ ] Basic date parsing from content and meta tags
- [ ] Deduplicate near-identical documents
- [ ] Source metadata (domain, retrieval time, status)

### Export
- [ ] Export entities and leads as CSV
- [ ] Export timeline as Markdown
- [ ] Option to include raw text snippets with each entity

---

## Phase 2 – Better Intelligence (v0.4 – v0.6)
**Goal:** Move from keyword/regex extraction to meaningful analysis.

### Entity Extraction
- [ ] Integrate a proper Named Entity Recognition (NER) model
  - Option A: Call a local or remote spaCy / Hugging Face model
  - Option B: Use a pure-Rust NER solution if available
- [ ] Entity resolution / merging (e.g. “John Smith” ≈ “J. Smith”)
- [ ] Confidence scoring improvements
- [ ] Support for more entity types (Organization, Date, Event, Weapon, Vehicle details)

### Semantic Layer
- [ ] Document embeddings (sentence-transformers or similar via `candle` / ONNX)
- [ ] Semantic search across ingested documents
- [ ] Document similarity / clustering
- [ ] “Find related documents” feature

### Graph Enhancements
- [ ] Stronger relation types (co-occurrence, temporal, geographic)
- [ ] Simple path finding between entities
- [ ] Basic graph statistics (most connected entities, clusters)

---

## Phase 3 – Usability & Visualization (v0.7 – v0.9)
**Goal:** Make the tool pleasant and insightful to use for longer investigations.

### Visualization
- [ ] Interactive knowledge graph view (nodes + edges)
- [ ] Timeline visualization (vertical or horizontal)
- [ ] Entity frequency charts
- [ ] Heatmap of entity co-occurrence

### Workflow Features
- [ ] Notes / annotations on documents and entities
- [ ] Flag / star important leads
- [ ] Tagging system
- [ ] Filter and search within results
- [ ] Multi-select sources for re-analysis

### Source Expansion
- [ ] RSS / Atom feed support
- [ ] Local folder batch import
- [ ] Archive.org / Wayback Machine helper
- [ ] Simple public records search helpers (highly constrained)

---

## Phase 4 – Advanced Capabilities (v1.0+)
**Goal:** Turn it into a serious OSINT research assistant while remaining ethical and focused.

### Advanced Analysis
- [ ] Cross-case comparison mode
- [ ] Contradiction / inconsistency detection (basic)
- [ ] Geographic mapping of locations
- [ ] Temporal pattern detection
- [ ] Lead prioritization with explainable scores

### Integrations (optional / modular)
- [ ] Optional local LLM for summarization and Q&A over the case corpus
- [ ] Export to common formats used by investigators (with clear provenance)
- [ ] Plugin system for custom extractors

### Quality & Safety
- [ ] Stronger provenance tracking (every claim linked to source + snippet)
- [ ] Rate limiting and robots.txt respect by default
- [ ] Clear ethical guidelines and warnings in the UI
- [ ] Audit log of actions taken in a session

---

## Long-Term Vision

- Become a reliable **desktop OSINT workbench** focused on cold-case style research.
- Emphasize **transparency** and **provenance** over black-box answers.
- Stay lightweight and local-first where possible.
- Remain usable by individual researchers and small teams without requiring heavy infrastructure.

---

## Priority Order (Recommended)

1. **Session save/load + better error handling** (immediate usability)
2. **Improved content extraction + date parsing**
3. **Proper NER + entity resolution**
4. **Document embeddings + semantic search**
5. **Graph visualization**
6. **Timeline visualization + annotations**
7. **Advanced lead scoring and cross-document analysis**

---

## Contribution Ideas

- Improve the regex / NER rules
- Add new source connectors
- Design better UI layouts
- Write tests for extraction and scoring logic
- Create example analysis datasets (public domain material only)

---

*Last updated: October 2026*
```
