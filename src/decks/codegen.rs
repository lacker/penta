//! Build-time YAML schema and deck registry generation.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_yaml_ng::Mapping;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeckFile {
    name: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    rust_aliases: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    commanders: Mapping,
    main: Mapping,
    #[serde(default)]
    sideboard: Mapping,
}

fn format_variant(directory: &str) -> Option<&'static str> {
    Some(match directory {
        "old_school_93_94" => "OldSchool9394",
        "premodern" => "Premodern",
        "isd_m14_standard" => "IsdM14Standard",
        "som_m13_standard" => "SomM13Standard",
        "vintage_cube" => "VintageCube",
        "pauper_cube" => "PauperCube",
        "cedh" => "Cedh",
        _ => return None,
    })
}

fn identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c == '_')
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !matches!(value, "_" | "self" | "super" | "crate")
}

fn entries<'a>(mapping: &'a Mapping, path: &str) -> Vec<(&'a str, usize)> {
    mapping
        .iter()
        .map(|(name, count)| {
            let name = name
                .as_str()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| panic!("{path}: card name must be a nonempty string"));
            let count = count
                .as_u64()
                .and_then(|count| usize::try_from(count).ok())
                .filter(|count| *count > 0)
                .unwrap_or_else(|| panic!("{path}: {name}: card count must be a positive integer"));
            (name, count)
        })
        .collect()
}

struct Source {
    path: String,
    module: String,
    id: String,
    deck: DeckFile,
}

impl Source {
    fn parse(path: &str, yaml: &str) -> Self {
        let path_parts: Vec<_> = path.split('/').collect();
        assert!(
            path_parts.len() == 3 && path_parts[0] == "decks",
            "expected decks/<format>/<deck>.yaml: {path}"
        );
        let module = path_parts[1].to_owned();
        assert!(
            identifier(&module),
            "{path}: invalid deck directory {module:?}"
        );
        let deck: DeckFile =
            serde_yaml_ng::from_str(yaml).unwrap_or_else(|error| panic!("{path}: {error}"));
        let id = Path::new(path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if let Some(explicit_id) = &deck.id {
            assert_eq!(explicit_id, &id, "{path}: id must match the filename stem");
        }
        for name in std::iter::once(&deck.name).chain(&deck.aliases) {
            assert!(
                !name.trim().is_empty() && name.trim() == name,
                "{path}: deck names and aliases must be nonempty and trimmed"
            );
        }
        for symbol in std::iter::once(&id).chain(&deck.rust_aliases) {
            assert!(
                identifier(symbol),
                "{path}: invalid Rust deck identifier {symbol:?}"
            );
        }
        // Check counts before emitting source, including sections with no cards.
        entries(&deck.commanders, path);
        entries(&deck.main, path);
        entries(&deck.sideboard, path);
        Self {
            path: path.to_owned(),
            module,
            id,
            deck,
        }
    }
}

// Compare digit runs numerically without parsing into a bounded integer.
fn alphanumeric_cmp(left: &str, right: &str) -> Ordering {
    let left = left.to_lowercase();
    let right = right.to_lowercase();
    let (mut left, mut right) = (left.as_str(), right.as_str());
    while let (Some(a), Some(b)) = (left.chars().next(), right.chars().next()) {
        let ordering = if a.is_ascii_digit() && b.is_ascii_digit() {
            let a_end = left
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(left.len());
            let b_end = right
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(right.len());
            let a = left[..a_end].trim_start_matches('0');
            let b = right[..b_end].trim_start_matches('0');
            let ordering = a.len().cmp(&b.len()).then_with(|| a.cmp(b));
            left = &left[a_end..];
            right = &right[b_end..];
            ordering
        } else {
            left = &left[a.len_utf8()..];
            right = &right[b.len_utf8()..];
            a.cmp(&b)
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

fn registry(mut sources: Vec<Source>) -> String {
    sources.sort_by(|a, b| {
        a.module
            .cmp(&b.module)
            .then_with(|| alphanumeric_cmp(&a.deck.name, &b.deck.name))
            .then_with(|| a.path.cmp(&b.path))
    });
    let mut names = BTreeSet::new();
    let mut symbols = BTreeSet::new();
    let mut modules: BTreeMap<&str, Vec<(usize, &Source)>> = BTreeMap::new();
    let mut output = String::from("// @generated from decks/**/*.yaml by build.rs.\n");
    output.push_str("pub(crate) const BUILTIN_DECKS: &[BuiltinDeck] = &[\n");
    for (index, source) in sources.iter().enumerate() {
        let Source {
            path,
            module,
            id,
            deck,
        } = source;
        let mut lookup_names =
            BTreeSet::from([id.to_ascii_lowercase(), deck.name.to_ascii_lowercase()]);
        for alias in &deck.aliases {
            assert!(
                lookup_names.insert(alias.to_ascii_lowercase()),
                "{path}: redundant lookup alias {alias:?}"
            );
        }
        for name in lookup_names {
            assert!(
                names.insert((module, name.clone())),
                "{path}: duplicate deck name, id, or alias {name:?} in {module}"
            );
        }
        for symbol in std::iter::once(id).chain(&deck.rust_aliases) {
            assert!(
                symbols.insert((module, symbol)),
                "{path}: duplicate Rust deck identifier {symbol:?} in {module}"
            );
        }
        let format = format_variant(module).map_or_else(
            || "None".to_owned(),
            |variant| format!("Some(crate::Format::{variant})"),
        );
        writeln!(output,
            "BuiltinDeck {{ format: {format}, id: {id:?}, name: {:?}, aliases: &{:?}, source: {:?}, commanders: &{:?}, main: &{:?}, sideboard: &{:?} }},",
            deck.name, deck.aliases, path,
            entries(&deck.commanders, path), entries(&deck.main, path), entries(&deck.sideboard, path)).unwrap();
        modules.entry(module).or_default().push((index, source));
    }
    output.push_str("];\n");
    for (module, decks) in modules {
        writeln!(
            output,
            "/// Built-in decks from `decks/{module}/`.\npub mod {module} {{"
        )
        .unwrap();
        for (index, source) in decks {
            let doc = if source.deck.description.is_empty() {
                format!("{} deck.", source.deck.name)
            } else {
                source.deck.description.clone()
            };
            writeln!(output,
                "#[doc = {doc:?}]\n#[allow(clippy::doc_markdown)]\n#[must_use]\npub fn r#{}() -> crate::Deck {{ super::BUILTIN_DECKS[{index}].build() }}",
                source.id).unwrap();
            for alias in &source.deck.rust_aliases {
                writeln!(output, "pub use r#{} as r#{alias};", source.id).unwrap();
            }
        }
        output.push_str("}\n");
    }
    output
}

pub(crate) fn generate(root: &Path, files: &[PathBuf]) -> String {
    let sources = files
        .iter()
        .filter(|path| path.starts_with(root.join("decks")))
        .filter(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("yaml" | "yml")
            )
        })
        .map(|path| {
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/");
            let yaml = fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            Source::parse(&relative, &yaml)
        })
        .collect();
    registry(sources)
}

#[cfg(test)]
mod tests {
    use super::{Source, entries, generate, registry};

    const YAML: &str = "name: Example\nmain:\n  Mountain: 2\n  'Urza''s Tower': 1\nsideboard: {}\n";

    #[test]
    fn yaml_preserves_card_order_and_decodes_quoted_names() {
        let source = Source::parse("decks/premodern/example.yaml", YAML);
        assert_eq!(
            entries(&source.deck.main, &source.path),
            [("Mountain", 2), ("Urza's Tower", 1)]
        );
        assert!(entries(&source.deck.sideboard, &source.path).is_empty());
        assert_eq!(source.id, "example");
        assert!(entries(&source.deck.commanders, &source.path).is_empty());
    }

    #[test]
    fn omitted_sideboard_generates_the_same_deck_as_an_empty_mapping() {
        let path = "decks/premodern/example.yaml";
        let omitted = Source::parse(path, &YAML.replace("sideboard: {}\n", ""));
        assert_eq!(
            registry(vec![omitted]),
            registry(vec![Source::parse(path, YAML)])
        );

        let populated = Source::parse(
            path,
            &YAML.replace("sideboard: {}", "sideboard:\n  Island: 2"),
        );
        assert_eq!(entries(&populated.deck.sideboard, path), [("Island", 2)]);
    }

    #[test]
    fn yaml_accepts_optional_commander_mapping() {
        let source = Source::parse(
            "decks/cedh/example.yaml",
            &YAML.replace(
                "main:\n",
                "commanders:\n  Rograkh, Son of Rohgahh: 1\nmain:\n",
            ),
        );
        assert_eq!(
            entries(&source.deck.commanders, &source.path),
            [("Rograkh, Son of Rohgahh", 1)]
        );
    }

    #[test]
    fn inventories_can_precede_format_registration_without_bypassing_yaml_checks() {
        let path = "decks/future_pool/example.yaml";
        let output = registry(vec![Source::parse(path, YAML)]);
        assert!(output.contains("format: None"));
        assert!(output.contains("pub mod future_pool"));
        assert!(
            std::panic::catch_unwind(|| {
                Source::parse(path, &YAML.replace("Mountain: 2", "Mountain: 0"))
            })
            .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| { Source::parse(path, &format!("{YAML}staged: true\n")) })
                .is_err()
        );
    }

    #[test]
    fn malformed_metadata_sections_and_counts_are_rejected() {
        for yaml in [
            YAML.replace("name: Example", "title: Example"),
            YAML.replace("name: Example", "name: ''"),
            YAML.replace("name: Example", "name: Example\nname: Duplicate"),
            YAML.replace("Mountain: 2", "Mountain: 0"),
            YAML.replace("Mountain: 2", "Mountain: -1"),
            YAML.replace("Mountain: 2", "Mountain: 1.5"),
            YAML.replace("Mountain: 2", "Mountain: 1\n  Mountain: 2"),
            YAML.replace("sideboard: {}", "sideboard: []"),
            YAML.replace("sideboard: {}", "sideboard:\n  Island: 0"),
            format!("{YAML}id: other\n"),
            format!("{YAML}order: 1\n"),
        ] {
            assert!(
                std::panic::catch_unwind(|| Source::parse("decks/premodern/example.yaml", &yaml))
                    .is_err(),
                "{yaml}"
            );
        }
    }

    #[test]
    fn registry_rejects_ambiguous_names_aliases_and_constructors() {
        for metadata in [
            "name: EXAMPLE\n",
            "name: Other\naliases: [example]\n",
            "name: Other\nrust_aliases: [example]\n",
            "name: Other\naliases: [other]\n",
        ] {
            let first = Source::parse("decks/premodern/example.yaml", YAML);
            let second = Source::parse(
                "decks/premodern/other.yaml",
                &YAML.replace("name: Example\n", metadata),
            );
            assert!(std::panic::catch_unwind(|| registry(vec![first, second])).is_err());
        }
    }

    #[test]
    fn registry_orders_alphanumerically_by_name_independent_of_discovery_order() {
        let sources = || {
            vec![
                Source::parse(
                    "decks/premodern/b.yaml",
                    &YAML.replace("Example", "Deck 10"),
                ),
                Source::parse("decks/premodern/a.yaml", &YAML.replace("Example", "deck 2")),
                Source::parse(
                    "decks/premodern/z.yaml",
                    &YAML.replace("Example", "Another deck"),
                ),
            ]
        };
        let output = registry(sources());
        let mut reversed = sources();
        reversed.reverse();
        assert_eq!(output, registry(reversed));
        assert!(
            output.find("name: \"Another deck\"").unwrap()
                < output.find("name: \"deck 2\"").unwrap()
        );
        assert!(
            output.find("name: \"deck 2\"").unwrap() < output.find("name: \"Deck 10\"").unwrap()
        );
    }

    #[test]
    fn generation_includes_every_yaml_file_and_ignores_documentation() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let files: Vec<_> = std::fs::read_dir(root.join("decks"))
            .unwrap()
            .flat_map(|entry| {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    std::fs::read_dir(path)
                        .unwrap()
                        .map(|entry| entry.unwrap().path())
                        .collect()
                } else {
                    vec![path]
                }
            })
            .collect();
        let yaml_files: Vec<_> = files
            .iter()
            .filter(|path| {
                matches!(
                    path.extension().and_then(|ext| ext.to_str()),
                    Some("yaml" | "yml")
                )
            })
            .collect();
        assert_eq!(yaml_files.len(), crate::decks::BUILTIN_DECKS.len());
        let output = generate(root, &files);
        for path in yaml_files {
            assert!(output.contains(path.strip_prefix(root).unwrap().to_str().unwrap()));
        }
        for source in crate::decks::BUILTIN_DECKS {
            assert!(output.contains(source.source));
        }
        assert_eq!(
            output.matches("BuiltinDeck { format:").count(),
            crate::decks::BUILTIN_DECKS.len()
        );
    }
}
