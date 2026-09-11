use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::SET_MODULES;
use crate::card::{CardCatalog, CardStructure};
use crate::{CardSet, Format};

mod inline_helpers;

const DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";
const PRINTING_DECLARATION_PREFIX: &str = "const ";
const PRINTING_DECLARATION_SUFFIX: &str = ": PrintingRecord =";
const HEADER_PREFIX: &str = "// ";
const HEADER_SEPARATOR: &str = " — ";
const REPRINT_SUFFIX: &str = " (reprint)";
const ALTERNATE_PRINTING_SUFFIX: &str = " (alternate printing)";
const AUDIT_PREFIX: &str = "// Audit: ";
const ADDITIONAL_REGISTRY_DECLARATION: &str =
    "pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceEntry {
    symbol: Option<String>,
    printing_symbol: Option<String>,
    collector_number: String,
    header_name: String,
    audit: Option<SourceAudit>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrintingKind {
    Reprint,
    Alternate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AuditStatus {
    Unsupported,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SourceAudit {
    pub(super) set: CardSet,
    pub(super) name: String,
    pub(super) status: AuditStatus,
    pub(super) gap: String,
}

#[test]
fn printed_set_sources_follow_collector_number_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();
    assert!(
        !files.is_empty(),
        "at least one printed set source must exist"
    );

    let mut source_sets = HashSet::new();
    let mut source_definitions = 0;
    let mut source_additional_printings = 0;
    for path in files {
        let (set, definitions, additional_printings) = validate_printed_set_source(&path);
        assert!(
            source_sets.insert(set),
            "{set:?} has more than one printed set source",
        );
        source_definitions += definitions;
        source_additional_printings += additional_printings;
    }

    let registered_printed_modules = SET_MODULES
        .iter()
        .filter(|module| module.set != CardSet::TOKEN)
        .collect::<Vec<_>>();
    assert_eq!(
        source_sets,
        registered_printed_modules
            .iter()
            .map(|module| module.set)
            .collect(),
        "printed source files and registered set modules must correspond",
    );
    assert_eq!(
        source_definitions,
        registered_printed_modules
            .iter()
            .map(|module| module.cards.len())
            .sum::<usize>(),
        "source declarations and registered definitions must correspond",
    );
    assert_eq!(
        source_additional_printings,
        registered_printed_modules
            .iter()
            .map(|module| module.additional_printings.len())
            .sum::<usize>(),
        "source and registered additional printings must correspond",
    );
}

fn validate_printed_set_source(path: &Path) -> (CardSet, usize, usize) {
    let set_source = set_source_for_file(path);
    let source = fs::read_to_string(path).expect("a printed set source file is readable");
    let entries = source_entries(&source, set_source, path);

    validate_double_faced_headers(&entries, set_source, path);

    for cards in entries.windows(2) {
        assert_eq!(
            natural_collector_cmp(&cards[0].collector_number, &cards[1].collector_number),
            Ordering::Less,
            "{}: collector number {} is not before {}",
            path.display(),
            cards[0].collector_number,
            cards[1].collector_number
        );
    }

    let registry = registry_symbols(&source, path);
    let declaration_symbols = entries
        .iter()
        .filter_map(|card| card.symbol.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        registry,
        declaration_symbols,
        "{}: CARDS must exactly mirror declaration order",
        path.display()
    );

    let definitions = entries
        .iter()
        .filter(|entry| entry.symbol.is_some())
        .count();
    let additional_printings = validate_additional_printings(&source, &entries, path);
    (set_source.set, definitions, additional_printings)
}

fn validate_double_faced_headers(entries: &[SourceEntry], set_source: SetSource, path: &Path) {
    let records = SET_MODULES
        .iter()
        .find(|module| module.set == set_source.set)
        .expect("a printed source has a registered set module")
        .cards;
    let declarations = entries
        .iter()
        .filter(|entry| entry.symbol.is_some())
        .collect::<Vec<_>>();
    assert_eq!(
        declarations.len(),
        records.len(),
        "{}: source declarations and registered definitions must correspond",
        path.display()
    );

    for (entry, record) in declarations.into_iter().zip(records) {
        let definition = record.definition(set_source.set);
        let CardStructure::DoubleFaced { front, back, .. } = &definition.structure else {
            continue;
        };
        let front_name = &definition
            .part(*front)
            .expect("a double-faced definition has its front part")
            .name;
        let back_name = &definition
            .part(*back)
            .expect("a double-faced definition has its back part")
            .name;
        let combined_name = format!("{front_name} // {back_name}");
        assert_eq!(
            entry.header_name,
            combined_name,
            "{}: double-faced card headers must list front and back face names",
            path.display()
        );
        assert_eq!(
            definition.name,
            combined_name,
            "{}: modeled double-faced CardRecord names must list front and back faces",
            path.display()
        );
    }
}

fn validate_additional_printings(source: &str, entries: &[SourceEntry], path: &Path) -> usize {
    let additional_printings = additional_printings(source, path);
    let upper_printings = entries
        .iter()
        .filter_map(|entry| entry.printing_symbol.as_deref())
        .collect::<Vec<_>>();
    if !upper_printings.is_empty() {
        assert_eq!(
            upper_printings,
            additional_printings,
            "{}: printing constants must exactly mirror ADDITIONAL_PRINTINGS",
            path.display()
        );
    }
    additional_printings.len()
}

/// A wrapped string literal continues with a single backslash. Two of them
/// is an escaped backslash instead, which leaves a stray `\\` and the
/// following indentation inside the rules text -- visible to players, and
/// invisible to every other check, since the card still compiles and
/// validates. Eleven soulbond cards shipped that way before this existed.
#[test]
fn wrapped_rules_text_uses_a_single_continuation_backslash() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut violations = Vec::new();
    for path in printed_set_files(&root.join("src/card/sets")) {
        let source = fs::read_to_string(&path).expect("a printed set source is readable");
        for (index, line) in source.lines().enumerate() {
            if line.ends_with("\\\\") && !line.ends_with("\\\\\\\\") {
                violations.push(format!("{}:{}", path.display(), index + 1));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "wrapped rules text must end in one backslash, not two:\n{}",
        violations.join("\n"),
    );
}

fn printed_set_files(sets: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for year in fs::read_dir(sets).expect("card set directory exists") {
        let path = year.expect("year directory entry is readable").path();
        if !path.is_dir()
            || !path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with('y'))
        {
            continue;
        }
        for entry in fs::read_dir(path).expect("year directory is readable") {
            let path = entry.expect("set file entry is readable").path();
            if path.extension().is_some_and(|extension| extension == "rs")
                && !path.ends_with("mod.rs")
            {
                files.push(path);
            }
        }
    }
    files
}

#[derive(Clone, Copy)]
struct SetSource {
    set: CardSet,
    code: &'static str,
}

fn set_source_for_file(path: &Path) -> SetSource {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let module = SET_MODULES
        .iter()
        .find(|module| root.join(module.source_path) == path)
        .unwrap_or_else(|| panic!("{}: set source has no catalog descriptor", path.display()));
    SetSource {
        set: module.set,
        code: module.set.code(),
    }
}

pub(super) fn all_source_audits(root: &Path) -> Vec<SourceAudit> {
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();
    files
        .into_iter()
        .flat_map(|path| {
            let set_source = set_source_for_file(&path);
            let source = fs::read_to_string(&path).expect("a printed set source file is readable");
            source_entries(&source, set_source, &path)
                .into_iter()
                .filter_map(|entry| entry.audit)
        })
        .collect()
}

pub(super) fn source_audits_for_format(
    root: &Path,
    catalog: &CardCatalog,
    format: Format,
) -> Vec<SourceAudit> {
    all_source_audits(root)
        .into_iter()
        .filter(|audit| {
            format.allows_set(audit.set)
                || catalog
                    .find_by_name(&audit.name)
                    .is_some_and(|id| catalog.is_allowed_in(id, format))
        })
        .collect()
}

fn source_entries(source: &str, set_source: SetSource, path: &Path) -> Vec<SourceEntry> {
    let lines = source.lines().collect::<Vec<_>>();
    validate_source_annotations(&lines, path);
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            parse_header(line)
                .map(|header| source_entry_for_header(&lines, index, header, set_source, path))
        })
        .collect()
}

fn validate_source_annotations(lines: &[&str], path: &Path) {
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with(AUDIT_PREFIX) {
            assert!(
                parse_audit(line).is_some(),
                "{}:{}: expected exact `// Audit: blocked|unsupported — GAP` comment",
                path.display(),
                index + 1
            );
            assert!(
                index > 0
                    && parse_header(lines[index - 1])
                        .is_some_and(|header| header.printing_kind.is_none()),
                "{}:{}: an Audit comment must immediately follow a card header",
                path.display(),
                index + 1
            );
        }
        if let Some(symbol) = declaration_symbol(line) {
            let header = lines[..index]
                .iter()
                .rposition(|candidate| parse_header(candidate).is_some())
                .and_then(|header_index| parse_header(lines[header_index]));
            assert!(
                header.is_some_and(|header| header.printing_kind.is_none()),
                "{}:{}: expected {symbol} inside a canonical card header block",
                path.display(),
                index + 1
            );
        }
        if let Some(symbol) = printing_declaration_symbol(line) {
            let header = lines[..index]
                .iter()
                .rposition(|candidate| parse_header(candidate).is_some())
                .and_then(|header_index| parse_header(lines[header_index]));
            assert!(
                header.is_some_and(|header| header.printing_kind.is_some()),
                "{}:{}: expected {symbol} inside a reprint or alternate-printing header block",
                path.display(),
                index + 1
            );
        }
    }
}

fn source_entry_for_header(
    lines: &[&str],
    index: usize,
    header: ParsedHeader<'_>,
    set_source: SetSource,
    path: &Path,
) -> SourceEntry {
    assert_eq!(
        header.set_code,
        set_source.code,
        "{}:{}: wrong set code in card header",
        path.display(),
        index + 1
    );

    if let Some(printing_kind) = header.printing_kind {
        let block_end = lines[index + 1..]
            .iter()
            .position(|line| parse_header(line).is_some())
            .map_or(lines.len(), |offset| index + 1 + offset);
        let printing_declarations = lines[index + 1..block_end]
            .iter()
            .filter_map(|line| printing_declaration_symbol(line))
            .collect::<Vec<_>>();
        assert_eq!(
            printing_declarations.len(),
            1,
            "{}:{}: a printing header must contain exactly one PrintingRecord constant",
            path.display(),
            index + 1
        );
        assert_eq!(
            lines
                .get(index + 1)
                .and_then(|line| printing_declaration_symbol(line)),
            printing_declarations.first().copied(),
            "{}:{}: the PrintingRecord constant must immediately follow its header",
            path.display(),
            index + 1
        );
        let block = lines[index + 1..block_end].join("\n");
        let expected_constructor = match printing_kind {
            PrintingKind::Reprint => "PrintingRecord::reprint(",
            PrintingKind::Alternate => "PrintingRecord::alternate(",
        };
        let other_constructor = match printing_kind {
            PrintingKind::Reprint => "PrintingRecord::alternate(",
            PrintingKind::Alternate => "PrintingRecord::reprint(",
        };
        assert!(
            block.contains(expected_constructor) && !block.contains(other_constructor),
            "{}:{}: printing constant uses the wrong constructor",
            path.display(),
            index + 1
        );
        return SourceEntry {
            symbol: None,
            printing_symbol: printing_declarations
                .first()
                .map(|symbol| (*symbol).to_string()),
            collector_number: header.collector_number.to_string(),
            header_name: header.name.to_string(),
            audit: None,
        };
    }

    let audit = lines.get(index + 1).and_then(|line| parse_audit(line));
    let block_end = lines[index + 1..]
        .iter()
        .position(|line| parse_header(line).is_some())
        .map_or(lines.len(), |offset| index + 1 + offset);
    let declarations = lines[index + 1..block_end]
        .iter()
        .enumerate()
        .filter_map(|(offset, line)| {
            declaration_symbol(line).map(|symbol| (index + 1 + offset, symbol))
        })
        .collect::<Vec<_>>();

    let should_have_declaration = !matches!(audit, Some((AuditStatus::Blocked, _)));
    assert_eq!(
        declarations.len(),
        usize::from(should_have_declaration),
        "{}:{}: a canonical card block must contain exactly one CardRecord declaration unless it is blocked",
        path.display(),
        index + 1
    );
    let declaration = declarations.first().map(|(line, symbol)| {
        let name = validate_declaration(lines, *line, symbol, header.name, path);
        ((*symbol).to_string(), name)
    });
    let symbol = declaration.as_ref().map(|(symbol, _)| symbol.clone());
    let audit = audit.map(|(status, gap)| SourceAudit {
        set: set_source.set,
        name: declaration
            .map_or(header.name, |(_, name)| name)
            .to_string(),
        status,
        gap: gap.to_string(),
    });
    SourceEntry {
        symbol,
        printing_symbol: None,
        collector_number: header.collector_number.to_string(),
        header_name: header.name.to_string(),
        audit,
    }
}

fn declaration_symbol(line: &str) -> Option<&str> {
    let declaration = line.strip_prefix(DECLARATION_PREFIX)?;
    declaration
        .split_once(": CardRecord")
        .map(|(symbol, _)| symbol)
}

fn printing_declaration_symbol(line: &str) -> Option<&str> {
    line.strip_prefix(PRINTING_DECLARATION_PREFIX)?
        .split_once(PRINTING_DECLARATION_SUFFIX)
        .map(|(symbol, _)| symbol)
}

fn validate_declaration<'a>(
    lines: &[&'a str],
    index: usize,
    _symbol: &str,
    header_name: &str,
    path: &Path,
) -> &'a str {
    let initializer_index = (index..lines.len().min(index + 3))
        .find(|candidate| lines[*candidate].trim().ends_with('('))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: CardRecord declaration is missing its constructor",
                path.display(),
                index + 1
            )
        });
    let name = lines
        .get(initializer_index + 1)
        .and_then(|line| line.trim().strip_prefix('"'))
        .and_then(|line| line.strip_suffix("\","))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: expected a one-line canonical card name",
                path.display(),
                initializer_index + 2
            )
        });
    let scryfall_id = lines
        .get(initializer_index + 2)
        .and_then(|line| line.trim().strip_prefix('"'))
        .and_then(|line| line.strip_suffix("\","))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: expected a one-line debut-art Scryfall UUID",
                path.display(),
                initializer_index + 3
            )
        });
    assert!(
        scryfall_id.len() == 36
            && scryfall_id
                .bytes()
                .enumerate()
                .all(|(index, byte)| match index {
                    8 | 13 | 18 | 23 => byte == b'-',
                    _ => byte.is_ascii_hexdigit(),
                }),
        "{}:{}: invalid debut-art Scryfall UUID",
        path.display(),
        initializer_index + 3
    );
    assert!(
        header_name == name
            || header_name
                .strip_prefix(name)
                .and_then(|suffix| suffix.strip_prefix(" // "))
                .is_some_and(|back_name| !back_name.is_empty()),
        "{}:{}: header name must match the CardRecord name, optionally followed by a double-faced back name",
        path.display(),
        index + 1,
    );
    name
}

#[derive(Clone, Copy)]
struct ParsedHeader<'a> {
    set_code: &'a str,
    collector_number: &'a str,
    name: &'a str,
    printing_kind: Option<PrintingKind>,
}

fn parse_header(line: &str) -> Option<ParsedHeader<'_>> {
    let body = line.strip_prefix(HEADER_PREFIX)?;
    let (identity, name) = body.split_once(HEADER_SEPARATOR)?;
    let (set_code, collector_number) = identity.split_once(' ')?;
    let (name, printing_kind) = if let Some(name) = name.strip_suffix(REPRINT_SUFFIX) {
        (name, Some(PrintingKind::Reprint))
    } else if let Some(name) = name.strip_suffix(ALTERNATE_PRINTING_SUFFIX) {
        (name, Some(PrintingKind::Alternate))
    } else {
        (name, None)
    };
    if set_code.is_empty()
        || !set_code
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        || collector_number.is_empty()
        || collector_number.chars().any(char::is_whitespace)
        || name.is_empty()
    {
        return None;
    }
    Some(ParsedHeader {
        set_code,
        collector_number,
        name,
        printing_kind,
    })
}

fn parse_audit(line: &str) -> Option<(AuditStatus, &str)> {
    let body = line.strip_prefix(AUDIT_PREFIX)?;
    let (status, gap) = body.split_once(HEADER_SEPARATOR)?;
    if gap.is_empty() {
        return None;
    }
    let status = match status {
        "blocked" => AuditStatus::Blocked,
        "unsupported" => AuditStatus::Unsupported,
        _ => return None,
    };
    Some((status, gap))
}

fn registry_symbols<'a>(source: &'a str, path: &Path) -> Vec<&'a str> {
    const REGISTRY_DECLARATION: &str = "pub(in crate::card::sets) static CARDS: &[&CardRecord] =";

    let start = source
        .find(REGISTRY_DECLARATION)
        .unwrap_or_else(|| panic!("{}: CARDS registry is missing", path.display()));
    let body = source[start + REGISTRY_DECLARATION.len()..]
        .trim_start()
        .strip_prefix("&[")
        .unwrap_or_else(|| panic!("{}: CARDS registry is malformed", path.display()));
    let body = body.split_once("];").map_or_else(
        || panic!("{}: CARDS registry is malformed", path.display()),
        |(body, _)| body,
    );
    body.split(',')
        .filter_map(|entry| {
            let entry = entry.trim();
            (!entry.is_empty()).then_some(entry)
        })
        .map(|entry| {
            entry
                .strip_prefix('&')
                .unwrap_or_else(|| panic!("{}: malformed CARDS entry {entry:?}", path.display()))
        })
        .collect()
}

fn additional_printings<'a>(source: &'a str, path: &Path) -> Vec<&'a str> {
    let start = source
        .find(ADDITIONAL_REGISTRY_DECLARATION)
        .unwrap_or_else(|| {
            panic!(
                "{}: ADDITIONAL_PRINTINGS registry is missing",
                path.display()
            )
        });
    let body = source[start + ADDITIONAL_REGISTRY_DECLARATION.len()..]
        .trim_start()
        .strip_prefix("&[")
        .unwrap_or_else(|| {
            panic!(
                "{}: ADDITIONAL_PRINTINGS registry is malformed",
                path.display()
            )
        });
    let body = body.split_once("];").map_or_else(
        || {
            panic!(
                "{}: ADDITIONAL_PRINTINGS registry is malformed",
                path.display()
            )
        },
        |(body, _)| body,
    );

    body.split(',')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let symbol = entry.trim();
            assert!(
                !symbol.contains("//"),
                "{}: ADDITIONAL_PRINTINGS entries are printing constant names, not expressions",
                path.display()
            );
            assert!(
                !symbol.is_empty()
                    && symbol.bytes().all(|byte| {
                        byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_'
                    }),
                "{}: malformed ADDITIONAL_PRINTINGS constant {symbol:?}",
                path.display()
            );
            symbol
        })
        .collect()
}

fn natural_collector_cmp(left: &str, right: &str) -> Ordering {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left.len() && right_index < right.len() {
        let left_is_digit = left[left_index].is_ascii_digit();
        let right_is_digit = right[right_index].is_ascii_digit();
        let left_end = run_end(left, left_index, left_is_digit);
        let right_end = run_end(right, right_index, right_is_digit);
        let left_run = &left[left_index..left_end];
        let right_run = &right[right_index..right_end];

        let order = if left_is_digit && right_is_digit {
            compare_digit_runs(left_run, right_run)
        } else {
            left_run.cmp(right_run)
        };
        if order != Ordering::Equal {
            return order;
        }
        left_index = left_end;
        right_index = right_end;
    }

    match (left_index == left.len(), right_index == right.len()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => unreachable!("the comparison loop stops only at the end of a value"),
    }
}

fn run_end(value: &[u8], start: usize, is_digit: bool) -> usize {
    value[start..]
        .iter()
        .position(|byte| byte.is_ascii_digit() != is_digit)
        .map_or(value.len(), |offset| start + offset)
}

fn compare_digit_runs(left: &[u8], right: &[u8]) -> Ordering {
    let left_significant = significant_digits(left);
    let right_significant = significant_digits(right);
    left_significant
        .len()
        .cmp(&right_significant.len())
        .then_with(|| left_significant.cmp(right_significant))
        .then_with(|| left.len().cmp(&right.len()))
        .then_with(|| left.cmp(right))
}

fn significant_digits(value: &[u8]) -> &[u8] {
    let first_nonzero = value
        .iter()
        .position(|byte| *byte != b'0')
        .unwrap_or(value.len() - 1);
    &value[first_nonzero..]
}
