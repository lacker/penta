use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use toml::Value;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PackageId {
    name: String,
    version: String,
    source: Option<String>,
}

#[derive(Clone, Debug)]
struct LockedPackage {
    id: PackageId,
    checksum: Option<String>,
    dependencies: Vec<String>,
}

fn canonical_manifest(root: &Path) -> PathBuf {
    let packaged = root.join("Cargo.toml.orig");
    if packaged.is_file() {
        packaged
    } else {
        root.join("Cargo.toml")
    }
}

fn tracked_files(root: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = vec![
        root.join("build.rs"),
        canonical_manifest(root),
        root.join("rust-toolchain.toml"),
    ];
    let mut directories = Vec::new();
    collect_files(&root.join("src"), &mut files, &mut directories);
    collect_files(&root.join("decks"), &mut files, &mut directories);
    files.sort_by_key(|path| relative_name(root, path));
    directories.sort();
    directories.dedup();
    (files, directories)
}

fn lockfile(root: &Path) -> PathBuf {
    // Cargo packages this lock beside the normalized manifest, so the source
    // checkout and published crate share one canonical core resolution.
    root.join("Cargo.lock")
}

fn string_field<'a>(table: &'a toml::Table, field: &str, context: &str) -> &'a str {
    table
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("{context} must have a string {field}"))
}

fn parse_lockfile(path: &Path) -> Vec<LockedPackage> {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let lock: Value = toml::from_str(&source)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
    lock.get("package")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{} must contain package entries", path.display()))
        .iter()
        .map(|package| {
            let table = package
                .as_table()
                .unwrap_or_else(|| panic!("{} package entry must be a table", path.display()));
            let name = string_field(table, "name", "lockfile package").to_owned();
            let version =
                string_field(table, "version", &format!("lockfile package {name}")).to_owned();
            let dependencies = table
                .get("dependencies")
                .map(|dependencies| {
                    dependencies
                        .as_array()
                        .unwrap_or_else(|| {
                            panic!("lockfile package {name} dependencies must be an array")
                        })
                        .iter()
                        .map(|dependency| {
                            dependency
                                .as_str()
                                .unwrap_or_else(|| {
                                    panic!("lockfile package {name} dependency must be a string")
                                })
                                .to_owned()
                        })
                        .collect()
                })
                .unwrap_or_default();
            LockedPackage {
                id: PackageId {
                    name,
                    version,
                    source: table
                        .get("source")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                },
                checksum: table
                    .get("checksum")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                dependencies,
            }
        })
        .collect()
}

fn dependency_names(manifest: &Path) -> BTreeSet<String> {
    fn collect(table: &toml::Table, names: &mut BTreeSet<String>) {
        let Some(dependencies) = table.get("dependencies") else {
            return;
        };
        for (alias, specification) in dependencies
            .as_table()
            .expect("manifest dependencies must be a table")
        {
            let package = specification
                .as_table()
                .and_then(|specification| specification.get("package"))
                .and_then(Value::as_str)
                .unwrap_or(alias);
            names.insert(package.to_owned());
        }
    }

    // Only normal dependencies contribute to engine behavior. Build
    // dependencies implement the fingerprint itself, while dev dependencies
    // do not ship in the production engine.
    let source = fs::read_to_string(manifest)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", manifest.display()));
    let document: Value = toml::from_str(&source)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", manifest.display()));
    let table = document
        .as_table()
        .unwrap_or_else(|| panic!("{} must be a table", manifest.display()));
    let mut names = BTreeSet::new();
    collect(table, &mut names);
    if let Some(targets) = table.get("target") {
        for target in targets
            .as_table()
            .expect("manifest target must be a table")
            .values()
        {
            collect(
                target
                    .as_table()
                    .expect("manifest target configuration must be a table"),
                &mut names,
            );
        }
    }
    names
}

fn dependency_id(specification: &str, packages: &[LockedPackage]) -> PackageId {
    let mut parts = specification.splitn(3, ' ');
    let name = parts.next().expect("dependency has a name");
    let version = parts.next();
    let source = parts.next().map(|source| {
        source
            .strip_prefix('(')
            .and_then(|source| source.strip_suffix(')'))
            .unwrap_or(source)
    });
    let candidates = packages
        .iter()
        .filter(|package| {
            package.id.name == name
                && version.is_none_or(|expected| package.id.version == expected)
                && source.is_none_or(|expected| package.id.source.as_deref() == Some(expected))
        })
        .collect::<Vec<_>>();
    match candidates.as_slice() {
        [package] => package.id.clone(),
        [] => panic!("lockfile dependency {specification:?} does not name a package"),
        _ => panic!(
            "lockfile dependency {specification:?} is ambiguous; Cargo must include its version"
        ),
    }
}

fn simulation_dependency_closure(
    path: &Path,
    direct_dependencies: &BTreeSet<String>,
) -> Vec<LockedPackage> {
    let packages = parse_lockfile(path);
    let by_id = packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect::<BTreeMap<_, _>>();
    let roots = packages
        .iter()
        .filter(|package| package.id.name == "penta" && package.id.source.is_none())
        .collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        panic!(
            "{} must contain exactly one local penta package",
            path.display()
        );
    };
    let roots = root
        .dependencies
        .iter()
        .map(|dependency| dependency_id(dependency, &packages))
        .filter(|dependency| direct_dependencies.contains(&dependency.name))
        .collect::<Vec<_>>();
    let resolved_names = roots
        .iter()
        .map(|dependency| dependency.name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        &resolved_names,
        direct_dependencies,
        "{} does not resolve every normal penta dependency from its manifest",
        path.display()
    );
    let mut pending = roots;
    let mut included = BTreeSet::from([root.id.clone()]);
    while let Some(id) = pending.pop() {
        if !included.insert(id.clone()) {
            continue;
        }
        let package = by_id
            .get(&id)
            .unwrap_or_else(|| panic!("lockfile closure lost package {} {}", id.name, id.version));
        pending.extend(
            package
                .dependencies
                .iter()
                .map(|dependency| dependency_id(dependency, &packages)),
        );
    }
    included.into_iter().map(|id| by_id[&id].clone()).collect()
}

fn verify_standalone_python_resolution(
    root: &Path,
    direct_dependencies: &BTreeSet<String>,
    canonical: &[LockedPackage],
) {
    // The source checkout carries a second lock because penta-py is a separate
    // workspace. It must resolve the core crate to the same dependency closure
    // so every binding exposes one simulation identity. Published crates do not
    // include that non-published workspace, so there is nothing to compare.
    let python_lock = root.join("bindings/penta-py/Cargo.lock");
    if !python_lock.is_file() {
        return;
    }
    println!("cargo::rerun-if-changed={}", python_lock.display());
    let python = simulation_dependency_closure(&python_lock, direct_dependencies);
    let identities = |packages: &[LockedPackage]| {
        packages
            .iter()
            .map(|package| {
                (
                    package.id.clone(),
                    package.checksum.clone().unwrap_or_else(|| "local".into()),
                )
            })
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        identities(canonical),
        identities(&python),
        "bindings/penta-py/Cargo.lock resolves penta's dependency closure differently from Cargo.lock; regenerate both lockfiles to expose one simulation fingerprint"
    );
}

fn collect_files(directory: &Path, files: &mut Vec<PathBuf>, directories: &mut Vec<PathBuf>) {
    directories.push(directory.to_path_buf());
    let mut entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| entry.expect("source directory entry is readable").path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            if path.file_name().is_none_or(|name| name != "tests") {
                collect_files(&path, files, directories);
            }
        } else if path.is_file() && is_simulation_input(&path) {
            files.push(path);
        }
    }
}

fn is_simulation_input(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("rs" | "yaml")
    ) && name != "tests.rs"
        && !name.ends_with("_tests.rs")
        && !path
            .components()
            .any(|component| component.as_os_str() == "bin")
}

fn relative_name(root: &Path, path: &Path) -> String {
    let relative = path
        .strip_prefix(root)
        .expect("fingerprinted input is below the manifest directory")
        .to_str()
        .expect("fingerprinted paths are UTF-8");
    // Cargo rewrites the manifest when packaging and keeps the authored file
    // beside it. Hash that original under the same logical name as a checkout.
    if relative == "Cargo.toml.orig" {
        "Cargo.toml".to_owned()
    } else {
        relative.replace('\\', "/")
    }
}

fn normalized(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
            output.push(b'\n');
            index += 2;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    output
}

fn hash_text(hash: &mut Sha256, label: &str, value: &str) {
    for bytes in [label.as_bytes(), value.as_bytes()] {
        hash.update(
            u64::try_from(bytes.len())
                .expect("fingerprint field length fits")
                .to_le_bytes(),
        );
        hash.update(bytes);
    }
}

fn sha256_hex(hash: Sha256) -> String {
    // Written out a byte at a time because sha2 0.11 returns a plain array,
    // which has no LowerHex implementation of its own.
    let digest = hash.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut hex, "{byte:02x}").expect("writing to a String cannot fail");
    }
    hex
}

const CARD_DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";
const CARD_ID_DOMAIN: &[u8] = b"penta/card-printing-id/v1\0";
const LEGACY_CARD_ID_FINGERPRINT: &str =
    "48b65efd1c927143dfc013c1ce5878e0f61959d0659f9c32f109c1075529f955";
const MAX_CARD_DEFINITION_ID: u64 = (1_u64 << 52) - 1;

fn derived_card_definition_id(scryfall_id: &str, nonce: u32) -> u64 {
    let mut hash = Sha256::new();
    hash.update(CARD_ID_DOMAIN);
    hash.update(scryfall_id.as_bytes());
    hash.update(nonce.to_be_bytes());
    let digest = hash.finalize();
    let prefix = u64::from_be_bytes(
        digest[..8]
            .try_into()
            .expect("SHA-256 digest always has an eight-byte prefix"),
    );
    let id = prefix >> 12;
    assert!(id > 0, "derived card definition ID must be nonzero");
    assert!(id <= MAX_CARD_DEFINITION_ID);
    id
}

fn quoted_argument(value: &str, prefix: &str, suffix: &str) -> Option<String> {
    value
        .strip_prefix(prefix)?
        .strip_suffix(suffix)
        .map(str::to_owned)
}

fn authored_card_id(value: &str, path: &Path, line: usize) -> (u64, bool) {
    let value = value.trim();
    if let Ok(id) = value
        .strip_suffix(',')
        .unwrap_or(value)
        .replace('_', "")
        .parse::<u64>()
    {
        assert!(
            id > 0,
            "{}:{line}: legacy ID must be nonzero",
            path.display()
        );
        assert!(
            id <= MAX_CARD_DEFINITION_ID,
            "{}:{line}: legacy ID must be JavaScript-safe",
            path.display()
        );
        return (id, true);
    }
    if let Some(scryfall_id) = quoted_argument(value, "PrintingAnchor::scryfall(\"", "\"),") {
        return (derived_card_definition_id(&scryfall_id, 0), false);
    }
    if let Some(arguments) = value
        .strip_prefix("PrintingAnchor::scryfall_with_nonce(\"")
        .and_then(|value| value.strip_suffix("),"))
    {
        let (scryfall_id, nonce) = arguments.rsplit_once("\", ").unwrap_or_else(|| {
            panic!("{}:{line}: malformed printing anchor nonce", path.display())
        });
        let nonce = nonce.parse::<u32>().unwrap_or_else(|error| {
            panic!(
                "{}:{line}: invalid printing anchor nonce: {error}",
                path.display()
            )
        });
        return (derived_card_definition_id(scryfall_id, nonce), false);
    }
    panic!(
        "{}:{line}: CardRecord identity must be a legacy integer or PrintingAnchor",
        path.display()
    );
}

fn collect_card_ids(
    directory: &Path,
    cards: &mut BTreeMap<String, u64>,
    legacy_cards: &mut BTreeMap<String, u64>,
) {
    let mut entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("card source directory entry is readable")
                .path()
        })
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "tests") {
                continue;
            }
            collect_card_ids(&path, cards, legacy_cards);
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let lines = source.lines().collect::<Vec<_>>();
        for (index, declaration) in lines.iter().enumerate() {
            let Some(declaration) = declaration.strip_prefix(CARD_DECLARATION_PREFIX) else {
                continue;
            };
            let Some((symbol, _)) = declaration.split_once(": CardRecord") else {
                continue;
            };
            let initializer_index = (index..lines.len().min(index + 3))
                .find(|candidate| lines[*candidate].trim().ends_with('('))
                .unwrap_or_else(|| {
                    panic!(
                        "{}:{}: CardRecord declaration is missing its constructor",
                        path.display(),
                        index + 1
                    )
                });
            let initializer = lines[initializer_index].trim();
            assert!(
                initializer.ends_with('('),
                "{}:{}: CardRecord initializer must put its identity on the next line",
                path.display(),
                initializer_index + 1
            );
            let identity = lines.get(initializer_index + 1).unwrap_or_else(|| {
                panic!(
                    "{}:{}: CardRecord declaration is missing its identity",
                    path.display(),
                    initializer_index + 1
                )
            });
            let (id, legacy) = authored_card_id(identity, &path, initializer_index + 2);
            assert!(
                cards.insert(symbol.to_owned(), id).is_none(),
                "duplicate CardRecord symbol {symbol}"
            );
            if legacy {
                legacy_cards.insert(symbol.to_owned(), id);
            }
        }
    }
}

fn generate_card_ids(root: &Path) {
    let mut cards = BTreeMap::new();
    let mut legacy_cards = BTreeMap::new();
    collect_card_ids(&root.join("src/card/sets"), &mut cards, &mut legacy_cards);
    let mut legacy_registry = String::new();
    for (symbol, id) in legacy_cards {
        writeln!(&mut legacy_registry, "{symbol} {id}")
            .expect("writing legacy IDs to a String cannot fail");
    }
    let mut legacy_hash = Sha256::new();
    legacy_hash.update(legacy_registry.as_bytes());
    assert_eq!(
        sha256_hex(legacy_hash),
        LEGACY_CARD_ID_FINGERPRINT,
        "legacy card definition IDs are immutable; new records must use PrintingAnchor",
    );
    let mut ids = BTreeMap::new();
    for (symbol, id) in &cards {
        if let Some(existing) = ids.insert(*id, symbol) {
            panic!("card definition ID {id} is shared by {existing} and {symbol}");
        }
    }
    let all_ids = cards.values().copied().collect::<Vec<_>>();
    let mut generated = String::from(
        "// @generated by build.rs from CardRecord declarations.\n\
         use crate::CardDefinitionId;\n\n",
    );
    for (symbol, id) in cards {
        writeln!(
            &mut generated,
            "pub const {symbol}: CardDefinitionId = CardDefinitionId::new({id});"
        )
        .expect("writing generated card IDs to a String cannot fail");
    }
    generated.push_str(
        "\n#[cfg(test)]\npub(crate) const ALL_CARD_DEFINITION_IDS: &[CardDefinitionId] = &[\n",
    );
    for id in all_ids {
        writeln!(&mut generated, "    CardDefinitionId::new({id}),")
            .expect("writing generated card IDs to a String cannot fail");
    }
    generated.push_str("];\n");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo output directory"))
        .join("card_definition_ids.rs");
    fs::write(&output, generated)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", output.display()));
}

fn main() {
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    generate_card_ids(&root);
    let (files, directories) = tracked_files(&root);
    let direct_dependencies = dependency_names(&canonical_manifest(&root));
    let lockfile = lockfile(&root);
    let dependencies = simulation_dependency_closure(&lockfile, &direct_dependencies);
    println!("cargo::rerun-if-changed={}", lockfile.display());
    verify_standalone_python_resolution(&root, &direct_dependencies, &dependencies);
    for directory in directories {
        println!("cargo::rerun-if-changed={}", directory.display());
    }
    let mut hash = Sha256::new();
    hash.update(b"penta-simulation-v2\0");
    for path in files {
        println!("cargo::rerun-if-changed={}", path.display());
        let relative = relative_name(&root, &path);
        let source = fs::read(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let bytes = normalized(&source);
        hash.update(
            u64::try_from(relative.len())
                .expect("relative path length fits")
                .to_le_bytes(),
        );
        hash.update(relative.as_bytes());
        hash.update(
            u64::try_from(bytes.len())
                .expect("file length fits")
                .to_le_bytes(),
        );
        hash.update(bytes);
    }
    for package in dependencies {
        hash_text(&mut hash, "dependency.name", &package.id.name);
        hash_text(&mut hash, "dependency.version", &package.id.version);
        hash_text(
            &mut hash,
            "dependency.source",
            package.id.source.as_deref().unwrap_or("local"),
        );
        hash_text(
            &mut hash,
            "dependency.checksum",
            package.checksum.as_deref().unwrap_or("local"),
        );
    }
    let hex = sha256_hex(hash);
    println!("cargo::rustc-env=PENTA_SIMULATION_FINGERPRINT=sha256-{hex}");
}
