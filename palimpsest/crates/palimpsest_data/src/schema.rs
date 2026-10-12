//! JSON Schema export. SPEC: doc 02 §3.2, §3.3, §11.1.
//!
//! Doc 02 §3.2 names this module: "`schema` (feature `schema`; JSON Schema
//! export)". It is a placeholder until doc 21 WP-005 adds the export: record
//! types derive `JsonSchema` (schemars, which doc 02 §3.3 allows only through
//! this feature) and `pal schema export` writes `schemas/*.schema.json` with
//! `"$schema": "https://json-schema.org/draft/2020-12/schema"` (doc 02
//! §11.1); CI fails if the committed schemas differ from the generated ones.
//! No item is defined here yet (doc 02 §18 rule 4).
