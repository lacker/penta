//! Build-time YAML schema and deck registry generation.

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
    #[serde(default = "default_order")]
    order: u32,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    rust_aliases: Vec<String>,
    #[serde(default)]
    description: String,
    main: Mapping,
    sideboard: Mapping,
}

fn default_order() -> u32 {
    u32::MAX
}

fn format_variant(directory: &str) -> &'static str {
    match directory {
        "old_school_93_94" => "OldSchool9394",
        "premodern" => "Premodern",
        "isd_m14_standard" => "IsdM14Standard",
        "som_m13_standard" => "SomM13Standard",
        "vintage_cube" => "VintageCube",
        "pauper_cube" => "PauperCube",
        _ => panic!("unknown deck format directory: {directory}"),
    }
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
        format_variant(&module);
        let deck: DeckFile =
            serde_yaml_ng::from_str(yaml).unwrap_or_else(|error| panic!("{path}: {error}"));
        let id = deck.id.clone().unwrap_or_else(|| {
            Path::new(path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        });
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

fn registry(mut sources: Vec<Source>) -> String {
    sources.sort_by(|a, b| {
        (&a.module, a.deck.order, &a.path).cmp(&(&b.module, b.deck.order, &b.path))
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
        for name in std::iter::once(&deck.name).chain(&deck.aliases) {
            assert!(
                names.insert((module, name.to_ascii_lowercase())),
                "{path}: duplicate deck name or alias {name:?} in {module}"
            );
        }
        for symbol in std::iter::once(id).chain(&deck.rust_aliases) {
            assert!(
                symbols.insert((module, symbol)),
                "{path}: duplicate Rust deck identifier {symbol:?} in {module}"
            );
        }
        writeln!(output,
            "BuiltinDeck {{ format: crate::Format::{}, name: {:?}, aliases: &{:?}, source: {:?}, main: &{:?}, sideboard: &{:?} }},",
            format_variant(module), deck.name, deck.aliases, path,
            entries(&deck.main, path), entries(&deck.sideboard, path)).unwrap();
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
                format!("Returns the {} deck.", source.deck.name)
            } else {
                source.deck.description.clone()
            };
            writeln!(output,
                "#[doc = {doc:?}]\n#[must_use]\npub fn r#{}() -> crate::Deck {{ super::BUILTIN_DECKS[{index}].build() }}",
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
            YAML.replace("sideboard: {}\n", ""),
            format!("{YAML}id: invalid-id\n"),
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
            "name: Other\nid: example\n",
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
    fn registry_orders_by_metadata_then_path_independent_of_discovery_order() {
        let sources = || {
            vec![
                Source::parse("decks/premodern/b.yaml", &YAML.replace("Example", "B")),
                Source::parse("decks/premodern/a.yaml", &YAML.replace("Example", "A")),
                Source::parse(
                    "decks/premodern/z.yaml",
                    &format!("{}order: 1\n", YAML.replace("Example", "Z")),
                ),
            ]
        };
        let output = registry(sources());
        let mut reversed = sources();
        reversed.reverse();
        assert_eq!(output, registry(reversed));
        assert!(output.find("name: \"Z\"").unwrap() < output.find("name: \"A\"").unwrap());
        assert!(output.find("name: \"A\"").unwrap() < output.find("name: \"B\"").unwrap());
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
