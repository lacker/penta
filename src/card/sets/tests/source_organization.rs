use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

const DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";
const DECLARATION_SUFFIX: &str = ": CardRecord = CardRecord::new(";
const HEADER_PREFIX: &str = "// ";
const HEADER_SEPARATOR: &str = " — ";
const ADDITIONAL_REGISTRY_PREFIX: &str =
    "pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceCard {
    symbol: String,
    collector_number: String,
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
        let declarations = source
            .contains(DECLARATION_SUFFIX)
            .then(|| declarations(&source, set_code_for_file(&path), &path));
        let declarations = declarations.unwrap_or_default();
        definition_count += declarations.len();

        for cards in declarations.windows(2) {
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
        let declaration_symbols = declarations
            .iter()
            .map(|card| card.symbol.as_str())
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
        definition_count, 252,
        "the organization guard must cover every printed card definition"
    );
    assert_eq!(
        additional_printing_count, 380,
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
        Some("the_dark.rs") => "DRK",
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

fn declarations(source: &str, expected_set_code: &str, path: &Path) -> Vec<SourceCard> {
    let lines = source.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        if parse_header(line).is_some() {
            assert!(
                lines
                    .get(index + 1)
                    .is_some_and(|line| line.starts_with(DECLARATION_PREFIX)),
                "{}:{}: an organization header must immediately precede a CardRecord declaration",
                path.display(),
                index + 1
            );
        }
    }

    let mut cards = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(symbol) = line
            .strip_prefix(DECLARATION_PREFIX)
            .and_then(|line| line.strip_suffix(DECLARATION_SUFFIX))
        else {
            continue;
        };
        assert!(index > 0, "{}: declaration has no header", path.display());
        let header = parse_header(lines[index - 1]).unwrap_or_else(|| {
            panic!(
                "{}:{}: expected `// SET NUMBER — Name` immediately before {symbol}",
                path.display(),
                index + 1
            )
        });
        assert_eq!(
            header.0,
            expected_set_code,
            "{}:{}: wrong set code for {symbol}",
            path.display(),
            index
        );

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
            header.2,
            name,
            "{}:{}: header name must match CardRecord name",
            path.display(),
            index
        );
        cards.push(SourceCard {
            symbol: symbol.to_string(),
            collector_number: header.1.to_string(),
        });
    }
    cards
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

fn registry_symbols<'a>(source: &'a str, path: &Path) -> Vec<&'a str> {
    const REGISTRY_PREFIX: &str = "pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[";

    let start = source
        .find(REGISTRY_PREFIX)
        .unwrap_or_else(|| panic!("{}: CARDS registry is missing", path.display()));
    let body = &source[start + REGISTRY_PREFIX.len()..];
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
