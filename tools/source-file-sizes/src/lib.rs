//! Repository-wide Rust source-file sizes and printed-card source layout.
//!
//! This crate is a check, not a library: nothing links it, and it depends on
//! nothing, so `cargo test -p source-file-sizes` is a filesystem walk that
//! finishes in about a second from cold. The whole body is test-only, which
//! is why the crate compiles to an empty lib outside `cargo test`.
#![cfg(test)]

use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

mod card_layout;

const MAX_RUST_SOURCE_LINES: usize = 1_000;
const SOURCE_DIRECTORY_NAMES: [&str; 4] = ["src", "tests", "examples", "benches"];
const EXCLUDED_DIRECTORY_NAMES: [&str; 4] = [".git", "target", "node_modules", ".pnpm-store"];
const EXCLUDED_REPOSITORY_DIRECTORY_PATHS: [&str; 12] = [
    "web/.next",
    "web/.pnp",
    "web/.vercel",
    "web/.vinext",
    "web/.wrangler",
    "web/.yarn",
    "web/app/wasm",
    "web/coverage",
    "web/dist",
    "web/out",
    "web/outputs",
    "web/work",
];

/// This crate lives at `tools/source-file-sizes`, so the repository root is
/// two levels up. Checked rather than assumed: a silently wrong root would
/// scan nothing and report success.
fn repository_root() -> PathBuf {
    let manifest_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_directory
        .ancestors()
        .nth(2)
        .expect("tools/source-file-sizes must sit two levels below the repository root")
        .to_path_buf();
    assert!(
        root.join("Cargo.toml").is_file() && root.join("src/card/sets").is_dir(),
        "resolved repository root does not look like this repository: {}",
        root.display()
    );
    root
}

#[test]
fn rust_source_files_stay_within_size_limit() {
    let repository_root = repository_root();
    let cargo_roots = discover_cargo_roots(&repository_root)
        .unwrap_or_else(|error| panic!("failed to discover Cargo roots: {error}"));
    assert!(
        cargo_roots.contains(&repository_root),
        "the repository workspace root must be covered"
    );
    assert!(
        cargo_roots.contains(&repository_root.join("bindings/penta-py")),
        "the standalone Python binding must be covered"
    );
    let source_files = rust_source_files(&cargo_roots)
        .unwrap_or_else(|error| panic!("failed to discover Rust source files: {error}"));

    let mut diagnostics = Vec::new();
    for source_file in source_files {
        let repository_relative = source_file
            .strip_prefix(&repository_root)
            .unwrap_or_else(|_| {
                panic!(
                    "discovered source outside repository: {}",
                    source_file.display()
                )
            });
        let contents = fs::read(&source_file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_file.display()));
        let line_count = physical_line_count(&contents);
        if line_count > MAX_RUST_SOURCE_LINES && !is_card_set_file(repository_relative) {
            diagnostics.push(format!(
                "{}: {line_count} lines (limit: {MAX_RUST_SOURCE_LINES})",
                repository_relative.display()
            ));
        }
    }

    diagnostics.sort();
    assert!(
        diagnostics.is_empty(),
        "oversized Rust source files; split each by concept. The only exemption is a direct \
         src/card/sets/y####/*.rs file, and there is no allowlist:\n  {}",
        diagnostics.join("\n  ")
    );
}

fn discover_cargo_roots(repository_root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut cargo_roots = Vec::new();
    visit_directories(repository_root, repository_root, &mut |directory| {
        if directory.join("Cargo.toml").is_file() {
            cargo_roots.push(directory.to_path_buf());
        }
        Ok(())
    })?;
    cargo_roots.sort();
    Ok(cargo_roots)
}

fn rust_source_files(cargo_roots: &[PathBuf]) -> io::Result<BTreeSet<PathBuf>> {
    let mut source_files = BTreeSet::new();
    for cargo_root in cargo_roots {
        for directory_name in SOURCE_DIRECTORY_NAMES {
            let source_directory = cargo_root.join(directory_name);
            if source_directory.is_dir() {
                visit_files(&source_directory, &mut |path| {
                    if path.extension() == Some(OsStr::new("rs")) {
                        source_files.insert(path.to_path_buf());
                    }
                    Ok(())
                })?;
            }
        }

        let build_script = cargo_root.join("build.rs");
        if build_script.is_file() {
            source_files.insert(build_script);
        }
    }
    Ok(source_files)
}

fn visit_directories(
    repository_root: &Path,
    directory: &Path,
    visitor: &mut impl FnMut(&Path) -> io::Result<()>,
) -> io::Result<()> {
    visitor(directory)?;
    for entry in sorted_directory_entries(directory)? {
        let file_type = entry.file_type()?;
        if file_type.is_dir() && !is_excluded_discovery_directory(repository_root, &entry.path()) {
            visit_directories(repository_root, &entry.path(), visitor)?;
        }
    }
    Ok(())
}

fn visit_files(
    directory: &Path,
    visitor: &mut impl FnMut(&Path) -> io::Result<()>,
) -> io::Result<()> {
    for entry in sorted_directory_entries(directory)? {
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if !is_excluded_directory(&entry.file_name()) {
                visit_files(&entry.path(), visitor)?;
            }
        } else if file_type.is_file() {
            visitor(&entry.path())?;
        }
    }
    Ok(())
}

fn sorted_directory_entries(directory: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn is_excluded_directory(name: &OsStr) -> bool {
    EXCLUDED_DIRECTORY_NAMES
        .iter()
        .any(|excluded| name == OsStr::new(excluded))
}

fn is_excluded_discovery_directory(repository_root: &Path, directory: &Path) -> bool {
    if directory.file_name().is_some_and(is_excluded_directory) {
        return true;
    }

    // A nested checkout carries some other branch's files: the harness puts
    // linked worktrees under `.claude/worktrees`, and a stray clone can sit
    // anywhere. Their paths are not this repository's, so they miss the
    // card-set exemption and report another branch's set files as violations
    // here. A linked worktree's `.git` is a file rather than a directory,
    // which is why this asks whether the entry exists at all.
    if directory.join(".git").exists() {
        return true;
    }

    let Ok(repository_relative) = directory.strip_prefix(repository_root) else {
        return false;
    };
    EXCLUDED_REPOSITORY_DIRECTORY_PATHS
        .iter()
        .any(|excluded| repository_relative == Path::new(excluded))
}

fn is_card_set_file(repository_relative: &Path) -> bool {
    let mut components = repository_relative.components();
    let expected_prefix = ["src", "card", "sets"];
    for expected in expected_prefix {
        if components.next() != Some(Component::Normal(OsStr::new(expected))) {
            return false;
        }
    }

    let Some(Component::Normal(year_directory)) = components.next() else {
        return false;
    };
    let Some(year_directory) = year_directory.to_str() else {
        return false;
    };
    let year_bytes = year_directory.as_bytes();
    if year_bytes.len() != 5
        || year_bytes[0] != b'y'
        || !year_bytes[1..].iter().all(u8::is_ascii_digit)
    {
        return false;
    }

    let Some(Component::Normal(file_name)) = components.next() else {
        return false;
    };
    components.next().is_none() && Path::new(file_name).extension() == Some(OsStr::new("rs"))
}

fn physical_line_count(contents: &[u8]) -> usize {
    contents.split_inclusive(|byte| *byte == b'\n').count()
}

#[test]
fn card_set_file_exemption_is_exact() {
    assert!(is_card_set_file(Path::new("src/card/sets/y1993/alpha.rs")));
    assert!(is_card_set_file(Path::new(
        "src/card/sets/y9999/example.rs"
    )));

    for path in [
        "card/sets/y1993/alpha.rs",
        "src/card/sets/1993/alpha.rs",
        "src/card/sets/Y1993/alpha.rs",
        "src/card/sets/y123/alpha.rs",
        "src/card/sets/y12345/alpha.rs",
        "src/card/sets/y12a4/alpha.rs",
        "src/card/sets/y١٢٣٤/alpha.rs",
        "src/card/sets/y1993/alpha.RS",
        "src/card/sets/y1993/nested/alpha.rs",
        "wasm/src/card/sets/y1993/alpha.rs",
    ] {
        assert!(!is_card_set_file(Path::new(path)), "matched {path}");
    }
}

#[test]
fn physical_line_count_handles_final_newlines() {
    assert_eq!(physical_line_count(b""), 0);
    assert_eq!(physical_line_count(b"one"), 1);
    assert_eq!(physical_line_count(b"one\n"), 1);
    assert_eq!(physical_line_count(b"one\ntwo"), 2);
    assert_eq!(physical_line_count(b"one\ntwo\n"), 2);
    assert_eq!(physical_line_count(b"one\r\ntwo\r\n"), 2);
    assert_eq!(physical_line_count(b"\n\n"), 2);
}

#[test]
fn cargo_root_discovery_exclusions_are_repository_relative() {
    let repository_root = Path::new("repository");
    assert!(is_excluded_discovery_directory(
        repository_root,
        &repository_root.join("web/dist")
    ));
    assert!(is_excluded_discovery_directory(
        repository_root,
        &repository_root.join("web/app/wasm")
    ));
    assert!(is_excluded_discovery_directory(
        repository_root,
        &repository_root.join("crates/target")
    ));
    assert!(!is_excluded_discovery_directory(
        repository_root,
        &repository_root.join("crates/dist")
    ));
}

#[test]
fn a_nested_checkout_is_left_to_its_own_repository() {
    let root = std::env::temp_dir().join("penta-source-file-sizes-nested-checkout");
    let worktree = root.join("worktree");
    let ordinary = root.join("ordinary");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&worktree).expect("the temporary directories are creatable");
    fs::create_dir_all(&ordinary).expect("the temporary directories are creatable");
    // What `git worktree add` leaves behind: a file, not a directory.
    fs::write(worktree.join(".git"), "gitdir: elsewhere\n").expect("the marker is writable");

    assert!(is_excluded_discovery_directory(&root, &worktree));
    assert!(!is_excluded_discovery_directory(&root, &ordinary));

    let _ = fs::remove_dir_all(&root);
}
