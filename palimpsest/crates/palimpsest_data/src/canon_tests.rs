//! Canon §21 acceptance tests in this crate's scope (29 and 34), plus a check
//! that every content-ID token in the canon itself is `id32`-unique (canon §22:
//! "∀ content id: regex ∧ fnv1a32 unique"). SPEC: canon §21.

use crate::content_check::{
    CheckError, HexU64Fields, Violation, ViolationKind, check_content_file, parse_and_check,
};
use crate::ids::{ContentId, Id32, IdError, validate_content_id};
use crate::registry::{ID32_COLLISION, IdRegistry, RegistryError};
use crate::test_support::first_brute_force_collision;
use crate::text_key::{TextKey, validate_text_key};
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// A miniature loader pipeline over JSON texts: number and hex checks, the
/// envelope and ID grammar, then the `id32` registry (doc 02 §11.1 steps 1
/// and 3; schema validation, step 2, is not in this crate yet).
fn load(files: &[&str]) -> Result<IdRegistry, Vec<String>> {
    let hex = HexU64Fields::none();
    let mut ids = Vec::new();
    let mut errors = Vec::new();
    for text in files {
        let value = match parse_and_check(text, &hex) {
            Ok(v) => v,
            Err(CheckError::Violations(vs)) => {
                errors.extend(vs.iter().map(Violation::to_string));
                continue;
            }
            Err(e) => {
                errors.push(e.to_string());
                continue;
            }
        };
        match check_content_file(&value, &hex) {
            Ok(found) => ids.extend(found),
            Err(vs) => errors.extend(vs.iter().map(Violation::to_string)),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    IdRegistry::build(ids, []).map_err(|es| es.iter().map(RegistryError::to_string).collect())
}

fn file_with_ids(ids: &[&str]) -> String {
    let records: Vec<String> = ids
        .iter()
        .map(|id| format!("{{\"id\":\"{id}\"}}"))
        .collect();
    format!(
        "{{\"schema\":\"test\",\"records\":[{}]}}",
        records.join(",")
    )
}

/// SPEC: canon §21 test 29: the loader rejects `Genre.Cyberpunk` and
/// `genre..x`.
#[test]
fn canon_t29_rejects_bad_id_grammar() {
    assert_eq!(
        validate_content_id("Genre.Cyberpunk"),
        Err(IdError::InvalidFirstByte { byte: b'G' })
    );
    assert_eq!(
        validate_content_id("genre..x"),
        Err(IdError::EmptySegment { segment: 1 })
    );
    assert!(ContentId::new("Genre.Cyberpunk").is_err());
    assert!(ContentId::new("genre..x").is_err());
    let errors = load(&[&file_with_ids(&[
        "genre.cyberpunk",
        "Genre.Cyberpunk",
        "genre..x",
    ])])
    .expect_err("rejected");
    assert_eq!(errors.len(), 2);
    assert!(
        errors[0].starts_with("\"/records/1/id\": "),
        "{}",
        errors[0]
    );
    assert!(
        errors[1].starts_with("\"/records/2/id\": "),
        "{}",
        errors[1]
    );
    // The valid ID alone loads.
    let reg = load(&[&file_with_ids(&["genre.cyberpunk"])]).expect("loads");
    assert_eq!(reg.id_of("genre.cyberpunk"), Some(Id32(0x99b5_4427)));
}

/// SPEC: canon §21 test 29: the loader rejects any two content IDs with equal
/// `fnv1a32`. The pair is found by brute force here, so the test does not
/// depend on a hard-coded collision.
#[test]
fn canon_t29_rejects_equal_fnv1a32() {
    let (a, b, h) = first_brute_force_collision();
    assert_ne!(a, b);
    assert_eq!(Id32::try_of(&a), Ok(Id32(h)));
    assert_eq!(Id32::try_of(&b), Ok(Id32(h)));
    // Same file, different files, either order: always rejected.
    for files in [
        vec![file_with_ids(&[&a, &b])],
        vec![file_with_ids(&[&b]), file_with_ids(&[&a])],
    ] {
        let texts: Vec<&str> = files.iter().map(String::as_str).collect();
        let errors = load(&texts).expect_err("collision");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].starts_with("id32_collision: "), "{}", errors[0]);
    }
    // A mod record colliding with loaded content gets error `id32_collision`.
    let mut reg = load(&[&file_with_ids(&[&a])]).expect("loads");
    let err = reg
        .insert_content(ContentId::new(&b).expect("valid"))
        .expect_err("mod collision");
    assert_eq!(err.code(), ID32_COLLISION);
    assert_eq!(err.code(), "id32_collision");
}

/// SPEC: canon §21 test 29: the text key
/// `loc.arch.melee.blade_long.skin.post_apocalypse.flavor` passes the text-key
/// grammar and gets no `id32`.
#[test]
fn canon_t29_text_key_has_no_id32() {
    const KEY: &str = "loc.arch.melee.blade_long.skin.post_apocalypse.flavor";
    assert_eq!(validate_text_key(KEY), Ok(()));
    let key = TextKey::new(KEY).expect("valid text key");
    assert_eq!(key.as_str(), KEY);
    // No id32: it is not a content ID, so neither Id32 nor the registry takes it.
    assert_eq!(
        Id32::try_of(KEY),
        Err(IdError::TooManySegments { count: 7 })
    );
    assert!(ContentId::new(KEY).is_err());
    // Even a short `loc.` key that fits the content-ID regex gets no id32.
    assert_eq!(
        Id32::try_of("loc.item.name"),
        Err(IdError::TextKeyNamespace)
    );
    let errors = load(&[&file_with_ids(&[KEY])]).expect_err("not a content id");
    assert!(
        errors[0].starts_with("\"/records/0/id\": "),
        "{}",
        errors[0]
    );
    let reg = load(&[&file_with_ids(&["arch.melee.blade_long"])]).expect("loads");
    assert_eq!(reg.id_of(KEY), None);
}

/// SPEC: canon §21 test 34: the content validator rejects any JSON number with
/// a fraction or exponent part, wherever it sits, and reports its path.
#[test]
fn canon_t34_rejects_fraction_or_exponent() {
    let none = HexU64Fields::none();
    for tok in ["1.5", "1e3", "-0.0", "1.0", "2E0", "1e-3", "0.0", "3.25e+2"] {
        let text = format!(
            "{{\"schema\":\"t\",\"records\":[{{\"id\":\"item.a\",\"stats\":{{\"mass\":[10,{tok}]}}}}]}}"
        );
        let err = parse_and_check(&text, &none).expect_err(tok);
        let CheckError::Violations(vs) = err else {
            panic!("{tok}: expected violations");
        };
        assert_eq!(
            vs,
            vec![Violation {
                path: "/records/0/stats/mass/1".to_string(),
                kind: ViolationKind::NonInteger,
            }],
            "{tok}"
        );
        assert!(load(&[&text]).is_err());
    }
    // Integers, including the largest safe one, pass.
    let clean =
        "{\"schema\":\"t\",\"records\":[{\"id\":\"item.a\",\"mass\":9007199254740991,\"d\":-4}]}";
    assert!(parse_and_check(clean, &none).is_ok());
    assert!(load(&[clean]).is_ok());
}

/// Every backtick span of `text` (inline code), skipping fenced blocks.
fn inline_code_spans(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| !l.trim_start().starts_with("```"))
        .flat_map(|l| l.split('`').skip(1).step_by(2))
        .collect()
}

fn id_tokens(span: &str) -> impl Iterator<Item = &str> {
    span.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
        .map(|t| t.trim_end_matches('.'))
        .filter(|t| t.contains('.'))
}

/// SPEC: canon §1.1, §19 and §22: every content-ID token written in the canon
/// (in a namespace §19 assigns) has a unique `fnv1a32`. Skipped when the spec
/// is not next to the workspace (a standalone export).
#[test]
fn canon_spec_ids_are_collision_free() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../docs/palimpsest/spec/00_CANON.md"
    );
    let Ok(canon) = std::fs::read_to_string(path) else {
        std::eprintln!("skipped: {path} not found");
        return;
    };
    let (Some(s19), Some(s20)) = (
        canon.find("## 19. Namespace ownership"),
        canon.find("## 20. Glossary"),
    ) else {
        panic!("canon §19 or §20 heading not found");
    };
    let namespaces: BTreeSet<&str> = inline_code_spans(&canon[s19..s20])
        .into_iter()
        .filter_map(|s| s.split('.').next())
        .filter(|ns| !ns.is_empty() && *ns != "loc")
        .collect();
    assert!(namespaces.contains("genre") && namespaces.contains("crate"));
    let ids: BTreeSet<ContentId> = inline_code_spans(&canon)
        .into_iter()
        .flat_map(id_tokens)
        .filter_map(|t| ContentId::new(t).ok())
        .filter(|id| namespaces.contains(id.namespace()))
        .collect();
    assert!(ids.len() > 300, "only {} ids found", ids.len());
    assert!(ids.contains(&ContentId::new("genre.cyberpunk").expect("valid")));
    if let Err(errors) = IdRegistry::build(ids, []) {
        panic!("canon ids collide: {errors:?}");
    }
    // The t29 counter-examples are in the canon text and are not IDs.
    assert!(canon.contains("`Genre.Cyberpunk`") && canon.contains("`genre..x`"));
}
