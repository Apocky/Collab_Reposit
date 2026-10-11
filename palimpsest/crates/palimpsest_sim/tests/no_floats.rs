//! SPEC: canon §21 test 33, §1.2, §1.11, §22: a lint step that fails the
//! build if `f32` or `f64` appears in simulation code outside `#[cfg(test)]`
//! code and the documented conversion boundary module.
//!
//! clippy.toml already forbids the `f32`/`f64` types and `HashMap`/`HashSet`
//! in every crate, and the workspace lints deny float arithmetic and unsafe
//! code. This test is the canon's explicit lint step: it scans the non-test
//! source of every workspace crate (canon test 33 names `palimpsest_sim`;
//! the slice applies the same rule to `palimpsest_math` and
//! `palimpsest_data`) for the tokens `f32`, `f64`, float literals,
//! `HashMap`, `HashSet` and `unsafe`, after removing comments, string and
//! char literals and every item marked `#[cfg(test)]`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// SPEC: canon §1.2 "the conversion boundary module (doc 02)". The slice has
/// no rendering, audio or UI layer, so no module may use floats.
const FLOAT_BOUNDARY_MODULES: &[&str] = &[];

/// Crates whose `src/` is scanned, relative to the workspace `crates/` dir.
const CRATES: &[&str] = &["palimpsest_math", "palimpsest_data", "palimpsest_sim"];

/// Replace comments, string literals and char literals with spaces, keeping
/// newlines so that line numbers survive.
fn blank_comments_and_literals(src: &str) -> Vec<char> {
    let c: Vec<char> = src.chars().collect();
    let mut out = c.clone();
    let n = c.len();
    let blank = |out: &mut Vec<char>, from: usize, to: usize| {
        for ch in out.iter_mut().take(to.min(n)).skip(from) {
            if *ch != '\n' {
                *ch = ' ';
            }
        }
    };
    let ident = |ch: char| ch.is_ascii_alphanumeric() || ch == '_';
    let mut i = 0;
    while i < n {
        // Line comment, including doc comments.
        if c[i] == '/' && i + 1 < n && c[i + 1] == '/' {
            let mut j = i;
            while j < n && c[j] != '\n' {
                j += 1;
            }
            blank(&mut out, i, j);
            i = j;
            continue;
        }
        // Block comment, nested.
        if c[i] == '/' && i + 1 < n && c[i + 1] == '*' {
            let mut depth = 0usize;
            let mut j = i;
            while j < n {
                if c[j] == '/' && j + 1 < n && c[j + 1] == '*' {
                    depth += 1;
                    j += 2;
                } else if c[j] == '*' && j + 1 < n && c[j + 1] == '/' {
                    depth -= 1;
                    j += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            blank(&mut out, i, j);
            i = j;
            continue;
        }
        // Raw strings: r"..", r#".."#, br#".."#.
        let raw_start = if c[i] == 'r' && (i == 0 || !ident(c[i - 1])) {
            Some(i + 1)
        } else if c[i] == 'b' && i + 1 < n && c[i + 1] == 'r' && (i == 0 || !ident(c[i - 1])) {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut j) = raw_start {
            let mut hashes = 0usize;
            while j < n && c[j] == '#' {
                hashes += 1;
                j += 1;
            }
            if j < n && c[j] == '"' {
                j += 1;
                'scan: while j < n {
                    if c[j] == '"' {
                        let mut k = 0;
                        while k < hashes && j + 1 + k < n && c[j + 1 + k] == '#' {
                            k += 1;
                        }
                        if k == hashes {
                            j += 1 + hashes;
                            break 'scan;
                        }
                    }
                    j += 1;
                }
                blank(&mut out, i, j);
                i = j;
                continue;
            }
        }
        // Ordinary and byte strings.
        if c[i] == '"' {
            let mut j = i + 1;
            while j < n && c[j] != '"' {
                if c[j] == '\\' {
                    j += 1;
                }
                j += 1;
            }
            blank(&mut out, i, j + 1);
            i = j + 1;
            continue;
        }
        // Char literals ('a', '\n', '\u{1F600}', '"'), not lifetimes ('a).
        if c[i] == '\'' {
            if i + 1 < n && c[i + 1] == '\\' {
                let mut j = i + 2;
                while j < n && c[j] != '\'' {
                    j += 1;
                }
                blank(&mut out, i, j + 1);
                i = j + 1;
                continue;
            }
            if i + 2 < n && c[i + 2] == '\'' {
                blank(&mut out, i, i + 3);
                i += 3;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Remove every item that follows a `#[cfg(test)]` attribute: up to the
/// matching `}` of its body, or up to `;` for `mod name;` and `use`
/// declarations. Returns the remaining text and the names of test-only
/// out-of-line modules (`#[cfg(test)] mod name;`).
fn strip_cfg_test(chars: &[char]) -> (Vec<char>, Vec<String>) {
    const ATTR: &str = "#[cfg(test)]";
    let attr: Vec<char> = ATTR.chars().collect();
    let mut out = chars.to_vec();
    let mut test_mods = Vec::new();
    let n = chars.len();
    let mut i = 0;
    while i + attr.len() <= n {
        if chars[i..i + attr.len()] != attr[..] {
            i += 1;
            continue;
        }
        let start = i;
        let mut j = i + attr.len();
        let mut depth = 0i32; // () and [] depth
        let end;
        loop {
            assert!(j < n, "unterminated #[cfg(test)] item");
            match chars[j] {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ';' if depth == 0 => {
                    let decl: String = chars[start + attr.len()..j].iter().collect();
                    let words: Vec<&str> = decl.split_whitespace().collect();
                    if let Some(pos) = words.iter().position(|w| *w == "mod")
                        && let Some(name) = words.get(pos + 1)
                    {
                        test_mods.push((*name).to_string());
                    }
                    end = j + 1;
                    break;
                }
                '{' if depth == 0 => {
                    let mut braces = 0i32;
                    let mut k = j;
                    loop {
                        assert!(k < n, "unbalanced braces after #[cfg(test)]");
                        match chars[k] {
                            '{' => braces += 1,
                            '}' => {
                                braces -= 1;
                                if braces == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        k += 1;
                    }
                    end = k + 1;
                    break;
                }
                _ => {}
            }
            j += 1;
        }
        for ch in out.iter_mut().take(end).skip(start) {
            if *ch != '\n' {
                *ch = ' ';
            }
        }
        i = end;
    }
    (out, test_mods)
}

/// One forbidden token found in non-test code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Violation {
    line: usize,
    token: String,
}

/// Scan cleaned text for `f32`/`f64` (as identifiers or literal suffixes),
/// float literals, `HashMap`, `HashSet` and `unsafe`.
fn scan_tokens(chars: &[char]) -> Vec<Violation> {
    let ident_start = |ch: char| ch.is_ascii_alphabetic() || ch == '_';
    let ident = |ch: char| ch.is_ascii_alphanumeric() || ch == '_';
    let n = chars.len();
    let mut line = 1usize;
    let mut out = Vec::new();
    let mut i = 0;
    while i < n {
        let ch = chars[i];
        if ch == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if ident_start(ch) {
            let s = i;
            while i < n && ident(chars[i]) {
                i += 1;
            }
            let tok: String = chars[s..i].iter().collect();
            if matches!(
                tok.as_str(),
                "f32" | "f64" | "HashMap" | "HashSet" | "unsafe"
            ) {
                out.push(Violation { line, token: tok });
            }
            continue;
        }
        if ch.is_ascii_digit() {
            let s = i;
            let after_dot = s > 0 && chars[s - 1] == '.'; // tuple index: t.0.1
            while i < n && ident(chars[i]) {
                i += 1;
            }
            let mut tok: String = chars[s..i].iter().collect();
            let radix_prefixed =
                tok.starts_with("0x") || tok.starts_with("0o") || tok.starts_with("0b");
            let mut float = false;
            if !after_dot && !radix_prefixed {
                // 1.5, 2. (but not 0..5 or 1.max(2))
                if i < n && chars[i] == '.' {
                    let next = chars.get(i + 1).copied().unwrap_or(' ');
                    if next.is_ascii_digit() {
                        float = true;
                        i += 1;
                        while i < n && ident(chars[i]) {
                            i += 1;
                        }
                        tok = chars[s..i].iter().collect();
                    } else if next != '.' && !ident_start(next) {
                        float = true;
                    }
                }
                // 1e5, 1E-3, 2.5e3
                let body: Vec<char> = tok.chars().filter(|&c| c != '_').collect();
                let mut k = 0;
                while k < body.len() && (body[k].is_ascii_digit() || body[k] == '.') {
                    k += 1;
                }
                if k < body.len() && (body[k] == 'e' || body[k] == 'E') {
                    let rest = &body[k + 1..];
                    let rest = if rest.first().is_some_and(|&c| c == '+' || c == '-') {
                        &rest[1..]
                    } else {
                        rest
                    };
                    if rest.first().is_some_and(char::is_ascii_digit) {
                        float = true;
                    }
                }
                // A sign inside an exponent splits the token: 1e-3.
                if !float
                    && (tok.ends_with('e') || tok.ends_with('E'))
                    && tok
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '_' || c == 'e' || c == 'E')
                {
                    let sign = chars.get(i).copied();
                    let digit = chars.get(i + 1).copied();
                    if matches!(sign, Some('+' | '-')) && digit.is_some_and(|d| d.is_ascii_digit())
                    {
                        float = true;
                    }
                }
            }
            if tok.ends_with("f32") || tok.ends_with("f64") || float {
                out.push(Violation { line, token: tok });
            }
            continue;
        }
        i += 1;
    }
    out
}

/// Full pipeline for one source text.
fn violations(src: &str) -> (Vec<Violation>, Vec<String>) {
    let blanked = blank_comments_and_literals(src);
    let (code, test_mods) = strip_cfg_test(&blanked);
    (scan_tokens(&code), test_mods)
}

fn workspace_crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/ directory")
        .to_path_buf()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()))
        .map(|e| e.expect("dir entry").path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// SPEC: canon §21 test 33. No `f32`/`f64` (and, per §1.11 and §22, no
/// `HashMap`/`HashSet`, and no `unsafe`) in non-test code of any workspace crate.
#[test]
fn canon_t33_no_floats() {
    let root = workspace_crates_dir();
    let mut report = Vec::new();
    let mut scanned = 0usize;
    for krate in CRATES {
        let src = root.join(krate).join("src");
        let mut files = Vec::new();
        rust_files(&src, &mut files);
        assert!(!files.is_empty(), "no sources under {}", src.display());
        // Out-of-line modules declared `#[cfg(test)] mod name;` are test code.
        let mut test_only: BTreeSet<PathBuf> = BTreeSet::new();
        let mut results = Vec::new();
        for f in &files {
            let text = fs::read_to_string(f).expect("read source");
            let (v, mods) = violations(&text);
            let dir = f.parent().expect("parent").to_path_buf();
            let stem = f.file_stem().expect("stem").to_string_lossy().to_string();
            for m in mods {
                test_only.insert(dir.join(format!("{m}.rs")));
                test_only.insert(dir.join(&stem).join(format!("{m}.rs")));
                test_only.insert(dir.join(&m).join("mod.rs"));
            }
            results.push((f.clone(), v));
        }
        for (f, v) in results {
            if test_only.contains(&f) {
                continue;
            }
            let rel = f
                .strip_prefix(&root)
                .expect("under crates/")
                .to_string_lossy()
                .to_string();
            if FLOAT_BOUNDARY_MODULES.contains(&rel.as_str()) {
                continue;
            }
            scanned += 1;
            for x in v {
                report.push(format!("crates/{rel}:{}: `{}`", x.line, x.token));
            }
        }
    }
    assert!(scanned >= 25, "scanned only {scanned} files");
    assert!(
        report.is_empty(),
        "forbidden tokens in non-test code:\n{}",
        report.join("\n")
    );
}

/// The scanner itself: it must flag real uses and ignore comments, strings,
/// char literals, ranges, tuple indices, hex literals and `#[cfg(test)]` items.
#[test]
fn scanner_self_test() {
    let tokens =
        |src: &str| -> Vec<String> { violations(src).0.into_iter().map(|v| v.token).collect() };
    // Flagged.
    assert_eq!(tokens("let x: f64 = y;"), ["f64"]);
    assert_eq!(tokens("fn g(v: f32) {}"), ["f32"]);
    assert_eq!(tokens("let x = 1.5;"), ["1.5"]);
    assert_eq!(tokens("let x = 2.;"), ["2"]);
    assert_eq!(tokens("let x = 1e5;"), ["1e5"]);
    assert_eq!(tokens("let x = 1e-3;"), ["1e"]);
    assert_eq!(tokens("let x = 3f64;"), ["3f64"]);
    assert_eq!(tokens("let x = 3_f32;"), ["3_f32"]);
    assert_eq!(tokens("use std::collections::HashMap;"), ["HashMap"]);
    assert_eq!(tokens("let s: HashSet<u8>;"), ["HashSet"]);
    assert_eq!(tokens("unsafe { }"), ["unsafe"]);
    assert_eq!(tokens("#[cfg(not(test))]\nfn f() -> f64 { 0 }"), ["f64"]);
    // Ignored.
    assert!(tokens("// f64 HashMap 1.5\n/* f32 /* nested f64 */ 2.5 */").is_empty());
    assert!(tokens("/// doc: f64\n//! inner: unsafe\nfn a() {}").is_empty());
    assert!(tokens(r#"let s = "f64 1.5 \" f32";"#).is_empty());
    assert!(tokens(r##"let s = r#"f64 "quoted" 2.5"#;"##).is_empty());
    assert!(tokens("let c = '\"'; let d = 'f'; fn l<'a>(x: &'a u8) {}").is_empty());
    assert!(tokens("for i in 0..5 {} let r = 1..=9; let t = p.0.1; let m = 1.max(2);").is_empty());
    assert!(tokens("let h = 0x1E5; let b = 0b1010; let u = 1usize; let e = 10_000i64;").is_empty());
    assert!(tokens("#![forbid(unsafe_code)]").is_empty());
    assert!(
        tokens("#[cfg(test)]\nmod tests {\n fn f() -> f64 { 1.5 }\n let m: HashMap<u8, u8>;\n}")
            .is_empty()
    );
    assert!(
        tokens("#[cfg(test)]\n#[allow(clippy::float_cmp)]\nfn helper(x: [f64; 2]) -> f64 { x[0] }")
            .is_empty()
    );
    // Line numbers survive comment and literal removal.
    let (v, _) = violations("/* a\nb */\n\"x\ny\"\nlet z: f32;");
    assert_eq!(
        v,
        [Violation {
            line: 5,
            token: "f32".to_string()
        }]
    );
    // Out-of-line test modules are recorded.
    let (_, mods) = violations("#[cfg(test)]\nmod canon_tests;\npub mod real;");
    assert_eq!(mods, ["canon_tests"]);
}
