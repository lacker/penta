use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

const DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";
const DECLARATION_SUFFIX: &str = ": CardRecord = CardRecord::new(";
const HEADER_PREFIX: &str = "// ";
const HEADER_SEPARATOR: &str = " — ";
const AUDIT_PREFIX: &str = "// Audit: ";
const ADDITIONAL_REGISTRY_PREFIX: &str =
    "pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceEntry {
    symbol: Option<String>,
    collector_number: String,
    audit: Option<SourceAudit>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AuditStatus {
    Partial,
    MetadataOnly,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SourceAudit {
    pub(super) name: String,
    pub(super) status: AuditStatus,
    pub(super) gap: String,
}

#[test]
fn printed_set_sources_follow_collector_number_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();

    let mut definition_count = 0;
    let mut additional_printing_count = 0;
    for path in files {
        let source = fs::read_to_string(&path).expect("a printed set source file is readable");
        let entries = source_entries(&source, set_code_for_file(&path), &path);
        definition_count += entries
            .iter()
            .filter(|entry| entry.symbol.is_some())
            .count();

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

        let registry = registry_symbols(&source, &path);
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

        let additional_printings = additional_printings(&source, &path);
        if !additional_printings.is_empty() {
            let expected_set_code = set_code_for_file(&path);
            for printing in &additional_printings {
                assert_eq!(
                    printing.0,
                    expected_set_code,
                    "{}: wrong set code on an ADDITIONAL_PRINTINGS entry",
                    path.display()
                );
            }
            for printings in additional_printings.windows(2) {
                assert_ne!(
                    natural_collector_cmp(printings[0].1, printings[1].1),
                    Ordering::Greater,
                    "{}: additional printing {} is after {}",
                    path.display(),
                    printings[0].1,
                    printings[1].1
                );
            }
        }
        additional_printing_count += additional_printings.len();
    }

    assert_eq!(
        definition_count, 590,
        "the organization guard must cover every printed card definition"
    );
    assert_eq!(
        additional_printing_count, 388,
        "the organization guard must cover every additional printing"
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

fn set_code_for_file(path: &Path) -> &'static str {
    match path.file_name().and_then(|name| name.to_str()) {
        Some("alpha.rs") => "LEA",
        Some("arabian_nights.rs") => "ARN",
        Some("beta.rs") => "LEB",
        Some("unlimited.rs") => "2ED",
        Some("collectors_edition.rs") => "CED",
        Some("international_collectors_edition.rs") => "CEI",
        Some("antiquities.rs") => "ATQ",
        Some("revised.rs") => "3ED",
        Some("fallen_empires.rs") => "FEM",
        Some("legends.rs") => "LEG",
        Some("promo_1994.rs") => "P94",
        Some("the_dark.rs") => "DRK",
        Some("ice_age.rs") => "ICE",
        Some("mirage.rs") => "MIR",
        Some("visions.rs") => "VIS",
        Some("tempest.rs") => "TMP",
        Some("stronghold.rs") => "STH",
        Some("portal_second_age.rs") => "P02",
        Some("urzas_saga.rs") => "USG",
        Some("mercadian_masques.rs") => "MMQ",
        Some("nemesis.rs") => "NEM",
        Some("invasion.rs") => "INV",
        Some("planeshift.rs") => "PLS",
        Some("apocalypse.rs") => "APC",
        Some("odyssey.rs") => "ODY",
        Some("judgment.rs") => "JUD",
        Some("onslaught.rs") => "ONS",
        Some("darksteel.rs") => "DST",
        Some("future_sight.rs") => "FUT",
        Some("planar_chaos.rs") => "PLC",
        Some("innistrad.rs") => "ISD",
        Some("avacyn_restored.rs") => "AVR",
        Some("dark_ascension.rs") => "DKA",
        Some("magic_2013.rs") => "M13",
        Some("return_to_ravnica.rs") => "RTR",
        Some("dragons_maze.rs") => "DGM",
        Some("gatecrash.rs") => "GTC",
        Some("magic_2014.rs") => "M14",
        Some("theros.rs") => "THS",
        Some("modern_horizons_2.rs") => "MH2",
        Some(name) => panic!(
            "{}: add {name} to the official set-code map",
            path.display()
        ),
        None => panic!("{}: set source has no UTF-8 file name", path.display()),
    }
}

pub(super) fn old_school_source_audits(root: &Path) -> Vec<SourceAudit> {
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();

    let mut audits = Vec::new();
    for path in files {
        let set_code = set_code_for_file(&path);
        if !matches!(
            set_code,
            "LEA" | "LEB" | "ARN" | "ATQ" | "LEG" | "DRK" | "FEM" | "P94"
        ) {
            continue;
        }
        let source = fs::read_to_string(&path).expect("a printed set source file is readable");
        audits.extend(
            source_entries(&source, set_code, &path)
                .into_iter()
                .filter_map(|entry| entry.audit),
        );
    }
    audits
}

fn source_entries(source: &str, expected_set_code: &str, path: &Path) -> Vec<SourceEntry> {
    let lines = source.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with(AUDIT_PREFIX) {
            assert!(
                parse_audit(line).is_some(),
                "{}:{}: expected exact `// Audit: blocked|partial|metadata-only — GAP` comment",
                path.display(),
                index + 1
            );
            assert!(
                index > 0 && parse_header(lines[index - 1]).is_some(),
                "{}:{}: an Audit comment must immediately follow a card header",
                path.display(),
                index + 1
            );
        }
        if let Some(symbol) = declaration_symbol(line) {
            let directly_headered = index > 0 && parse_header(lines[index - 1]).is_some();
            let audited_header = index > 1
                && parse_audit(lines[index - 1])
                    .is_some_and(|(status, _)| status != AuditStatus::Blocked)
                && parse_header(lines[index - 2]).is_some();
            assert!(
                directly_headered || audited_header,
                "{}:{}: expected a card header, optionally followed by a partial or metadata-only Audit comment, immediately before {symbol}",
                path.display(),
                index + 1
            );
        }
    }

    let mut entries = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(header) = parse_header(line) else {
            continue;
        };
        assert_eq!(
            header.0,
            expected_set_code,
            "{}:{}: wrong set code in card header",
            path.display(),
            index + 1
        );

        let (symbol, audit) = match lines.get(index + 1).copied() {
            Some(next) if declaration_symbol(next).is_some() => {
                let symbol = declaration_symbol(next).expect("the declaration was recognized");
                validate_declaration(&lines, index + 1, symbol, header.2, path);
                (Some(symbol.to_string()), None)
            }
            Some(next) if parse_audit(next).is_some() => {
                let (status, gap) = parse_audit(next).expect("the Audit comment was recognized");
                let declaration = lines
                    .get(index + 2)
                    .and_then(|line| declaration_symbol(line));
                match status {
                    AuditStatus::Blocked => assert!(
                        declaration.is_none(),
                        "{}:{}: a blocked Audit entry cannot have a CardRecord declaration",
                        path.display(),
                        index + 1
                    ),
                    AuditStatus::Partial | AuditStatus::MetadataOnly => assert!(
                        declaration.is_some(),
                        "{}:{}: a partial or metadata-only Audit entry must immediately precede a CardRecord declaration",
                        path.display(),
                        index + 1
                    ),
                }
                if let Some(symbol) = declaration {
                    validate_declaration(&lines, index + 2, symbol, header.2, path);
                }
                (
                    declaration.map(str::to_string),
                    Some(SourceAudit {
                        name: header.2.to_string(),
                        status,
                        gap: gap.to_string(),
                    }),
                )
            }
            _ => {
                panic!(
                    "{}:{}: a card header must immediately precede either a CardRecord declaration or an Audit comment",
                    path.display(),
                    index + 1
                )
            }
        };
        entries.push(SourceEntry {
            symbol,
            collector_number: header.1.to_string(),
            audit,
        });
    }
    entries
}

fn declaration_symbol(line: &str) -> Option<&str> {
    line.strip_prefix(DECLARATION_PREFIX)
        .and_then(|line| line.strip_suffix(DECLARATION_SUFFIX))
}

fn validate_declaration(
    lines: &[&str],
    index: usize,
    symbol: &str,
    header_name: &str,
    path: &Path,
) {
    let id_line = lines
        .get(index + 1)
        .and_then(|line| line.trim().strip_prefix("cards::"))
        .and_then(|line| line.strip_suffix(','));
    assert_eq!(
        id_line,
        Some(symbol),
        "{}:{}: declaration symbol and card ID must match",
        path.display(),
        index + 1
    );
    let name = lines
        .get(index + 2)
        .and_then(|line| line.trim().strip_prefix('"'))
        .and_then(|line| line.strip_suffix("\","))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: expected a one-line canonical card name",
                path.display(),
                index + 3
            )
        });
    assert_eq!(
        header_name,
        name,
        "{}:{}: header name must match CardRecord name",
        path.display(),
        index + 1
    );
}

fn parse_header(line: &str) -> Option<(&str, &str, &str)> {
    let body = line.strip_prefix(HEADER_PREFIX)?;
    let (identity, name) = body.split_once(HEADER_SEPARATOR)?;
    let (set_code, collector_number) = identity.split_once(' ')?;
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
    Some((set_code, collector_number, name))
}

fn parse_audit(line: &str) -> Option<(AuditStatus, &str)> {
    let body = line.strip_prefix(AUDIT_PREFIX)?;
    let (status, gap) = body.split_once(HEADER_SEPARATOR)?;
    if gap.is_empty() {
        return None;
    }
    let status = match status {
        "blocked" => AuditStatus::Blocked,
        "partial" => AuditStatus::Partial,
        "metadata-only" => AuditStatus::MetadataOnly,
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

fn additional_printings<'a>(source: &'a str, path: &Path) -> Vec<(&'a str, &'a str)> {
    let start = source.find(ADDITIONAL_REGISTRY_PREFIX).unwrap_or_else(|| {
        panic!(
            "{}: ADDITIONAL_PRINTINGS registry is missing",
            path.display()
        )
    });
    let body = &source[start + ADDITIONAL_REGISTRY_PREFIX.len()..];
    let body = body.split_once("];").map_or_else(
        || {
            panic!(
                "{}: ADDITIONAL_PRINTINGS registry is malformed",
                path.display()
            )
        },
        |(body, _)| body,
    );

    body.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let entry = line.trim();
            let (expression, comment) = entry.split_once("// ").unwrap_or_else(|| {
                panic!(
                    "{}: ADDITIONAL_PRINTINGS entry needs an EOL `// SET NUMBER` comment: {entry:?}",
                    path.display()
                )
            });
            assert!(
                !comment.contains("// "),
                "{}: malformed ADDITIONAL_PRINTINGS comment {comment:?}",
                path.display()
            );
            let expression = expression.trim_end().strip_suffix(',').unwrap_or_else(|| {
                panic!(
                    "{}: ADDITIONAL_PRINTINGS expression must end in a comma: {entry:?}",
                    path.display()
                )
            });
            assert!(
                (expression.starts_with("PrintingRecord::reprint(")
                    || expression.starts_with("PrintingRecord::alternate("))
                    && expression.ends_with(')'),
                "{}: malformed ADDITIONAL_PRINTINGS expression {expression:?}",
                path.display()
            );

            let (set_code, collector_number) = comment.split_once(' ').unwrap_or_else(|| {
                panic!(
                    "{}: expected exact `// SET NUMBER` comment, got {comment:?}",
                    path.display()
                )
            });
            assert!(
                !set_code.is_empty()
                    && set_code
                        .bytes()
                        .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
                    && !collector_number.is_empty()
                    && !collector_number.chars().any(char::is_whitespace),
                "{}: expected exact `// SET NUMBER` comment, got {comment:?}",
                path.display()
            );
            (set_code, collector_number)
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
