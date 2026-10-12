//! Content loader. SPEC: doc 02 §3.2, §3.4, §11.1, §11.3.
//!
//! Doc 02 §3.2 names this module: "`loader` (feature `loader`; file loading,
//! the §11.1 validation stages, hot reload)". It is a placeholder until doc 21
//! WP-006 adds `load_content(roots: &[ContentRoot], unlocks: &[Id32]) ->
//! Result<ContentDb, LoadReport>` (doc 02 §3.4), which reads
//! `content/<owner_doc>/<namespace>/<name>.json`, runs doc 02 §11.1's three
//! validation stages (the token pre-pass that already lives in
//! [`crate::content_check`], JSON Schema 2020-12 validation, the semantic
//! checks), builds the [`crate::registry::IdRegistry`] and the `ContentDb`,
//! and (behind `flag.dev.hot_reload`, doc 02 §11.3) reloads changed files.
//! No item is defined here yet: doc 02 §18 rule 4 reports a formula without a
//! worked example as a spec gap and never invents one.
