# RAG implementation plan

Status: proposed; not yet implemented. Retrieval runs locally; DeepSeek receives selected evidence needed for a request.

## Sources and lineage

Distinguish imported references, AI/learner-authored learning documents, curated claims, and conversations. All can be searchable, but conversation is not automatically evidence, and derivative summaries are not independent corroboration.

Retain source lineage. Set explicit source scope per learning document; cross-document search must be visibly configured.

## Ingestion

Initial formats: Markdown, UTF-8 text, text-based PDF. Provide an evidence viewer for citations without making a PDF reader the primary interface.

1. Import the original and compute its hash.
2. Capture title, author, language, edition, translator and locations.
3. Extract structure and text while preserving original bytes.
4. Identify headings, paragraphs, definitions, theorem/proof boundaries and examples where feasible.
5. Create stable passage IDs with source-version, parent and neighbor links.
6. Flag uncertain reading order, equations, footnotes and tables.
7. Build lexical and optional local embedding indexes.

Chunk by structure with bounded size and context overlap. A proof fragment should expand to its theorem and assumptions; a literary passage should expand to its surrounding scene.

Distinguish PDF file page index from printed page label. Scanned OCR is deferred: report unsupported or poor extraction clearly.

Ingestion/reindexing must be resumable. New source versions cannot silently inherit old anchors or assessments.

## Hybrid retrieval

Combine:
- Exact/full-text search for names, quotations, terminology, labels and notation.
- Multilingual semantic search for related ideas across wording and languages.
- Typed graph traversal for definitions, dependencies, evidence, corrections and alternative interpretations.

Start with a lexical baseline, with SQLite full-text search as a proposed implementation. Add local embeddings/vector retrieval after evaluating hardware, language quality, licensing and runtime compatibility. Store model/index versions; reindex explicitly when they change.

Fuse candidates, rank relevance and deduplicate shared origins. Reliability labels must not automatically exclude historically or pedagogically valuable errors. Optional reranking should earn its complexity through measured improvement.

## Request pipeline

1. Identify goal, active document, selected passage and Study/Critical review state.
2. Search enabled sources.
3. Combine/rank lexical and semantic candidates.
4. Expand essential parent/neighbor context and graph links.
5. Run targeted follow-up retrieval for missing definitions or evidence.
6. Apply budgets for rounds, candidates, graph depth, context tokens and output.
7. Assemble evidence with stable passage IDs and precise metadata.
8. Send bounded context to DeepSeek and validate structured output.
9. Resolve returned citation IDs against the retrieved bundle; reject unsupported citations.
10. Record retrieval/model provenance for later inspection.

“Deep” retrieval means bounded, targeted follow-up searches, not unlimited autonomous research. Show searched scope and incomplete coverage. Graph extraction is not required before a document can be studied.

## Mode behavior

| Mode | Retrieval emphasis |
| --- | --- |
| Study | Current material, definitions, examples, prerequisites and useful explanations |
| Critical review | Above plus stored objections, corrections, conflicting evidence and assessments |
| Hypothetical scenario | Explicit accepted premises and admitted rules, evaluated separately from prose |

Study does not demand verification or upgrade trust records. Explicit correction requests still require honest handling. Vector similarity and model confidence are not logical inference rules.

## Multilingual and subject handling

Presentation remains English. Preserve originals and label AI translations separately from published translations. Evaluate the languages actually used rather than assume uniform support.

Mathematics: preserve notation, quantified domains, assumptions and proof boundaries. Search is not proof checking; similar formulas are not necessarily equivalent.

Philosophy/biblical studies: preserve speaker, genre, edition, translation, historical context and interpretation. Separate explicit statements from reconstructions.

Literature/languages: retain exact passages and usage context; avoid turning a plausible reading or translation into a uniquely established conclusion.

## Evidence inspection

Citations open original passages, surrounding context, source/version and locators. Optional English translations remain linked to the original.

Show extraction limitations, retrieval relevance and derivative relationships. Critical review additionally surfaces assessments and known challenges.

If evidence is missing, say so. General model knowledge may be clearly identified, but must not acquire invented page references or source quotations.

## Local execution

Generate embeddings and store indexes locally. Do not upload entire collections merely to make them searchable. Selected source and conversation context will leave the computer for DeepSeek generation.

Embedding-model selection depends on actual CPU/RAM/GPU, languages, latency and license needs. Do not assume a GPU or equate the embedding model with the conversational model.

## Evaluation

Build a small collection with known answer passages and deliberately unanswerable questions. Include:
- Exact quotations and paraphrases.
- Cross-language queries.
- Theorems requiring nearby assumptions.
- Conflicting sources and a known correction.
- Different senses of one term.
- Several summaries derived from one source.
- Answers outside enabled source scope.
- Broken extraction, stale anchors and changed editions.

Measure candidate recall, ranking quality, citation correctness, context completeness, unsupported-answer handling, latency, memory and provider token use. Set numerical targets after measuring a baseline.

Release behavior:
- Relevant original passages are found and citations resolve.
- Unsupported citation IDs are rejected.
- Scope boundaries are respected.
- Corrections are discoverable during review.
- Summaries do not self-corroborate.
- Missing support is disclosed.
- Index failures are recoverable.

## Delivery order

1. Local ingestion and evidence viewer.
2. Full-text retrieval with citation validation.
3. Local multilingual embeddings and hybrid ranking.
4. Bounded context expansion and follow-up retrieval.
5. Graph-assisted retrieval and correction/dependency propagation.
6. OCR and specialized extraction when justified.
