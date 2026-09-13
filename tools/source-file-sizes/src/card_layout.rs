//! Close the gaps that rustfmt and Rust's generic identifier lints leave open.

use super::{fs, is_card_set_file, repository_root, rust_source_files};

// rustfmt targets 100, but occasionally leaves deeply nested expressions a
// little wider. Bound that overflow without accepting collapsed declarations.
const MAX_CARD_SOURCE_COLUMNS: usize = 120;
const CARD_DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";

#[test]
fn printed_card_sources_follow_layout_rules() {
    let root = repository_root();
    let files = rust_source_files(std::slice::from_ref(&root))
        .expect("repository Rust sources must be readable");
    let mut diagnostics = Vec::new();
    let mut set_files = 0;
    for path in files {
        let relative = path.strip_prefix(&root).expect("source is in repository");
        if !is_card_set_file(relative) {
            continue;
        }
        set_files += 1;
        let source = fs::read_to_string(&path).expect("printed card source must be readable");
        diagnostics.extend(
            layout_violations(&source)
                .into_iter()
                .map(|(line, message)| format!("{}:{line}: {message}", relative.display())),
        );
    }
    assert!(
        set_files > 0,
        "the check must discover printed card sources"
    );
    assert!(
        diagnostics.is_empty(),
        "printed-card layout violations; wrap long strings with escaped newlines, run rustfmt, \
         and keep collector numbers in headers rather than CardRecord symbols:\n{}",
        diagnostics.join("\n")
    );
}

fn layout_violations(source: &str) -> Vec<(usize, String)> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut violations = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let width = line.chars().count();
        if width > MAX_CARD_SOURCE_COLUMNS {
            violations.push((
                index + 1,
                format!("{width} columns (limit: {MAX_CARD_SOURCE_COLUMNS})"),
            ));
        }

        let Some(symbol) = line
            .strip_prefix(CARD_DECLARATION_PREFIX)
            .and_then(|declaration| declaration.split_once(": CardRecord"))
            .map(|(symbol, _)| symbol)
        else {
            continue;
        };
        // The authoring/build contract puts the card name first, on its own
        // line, after the constructor. rustfmt may wrap the declaration first.
        let Some(name) = lines[index + 1..]
            .iter()
            .take(3)
            .find_map(|line| line.trim().strip_prefix('"')?.strip_suffix("\","))
        else {
            violations.push((index + 1, format!("cannot read card name for {symbol}")));
            continue;
        };
        if has_collector_suffix(symbol, name) {
            violations.push((
                index + 1,
                format!("{symbol}: remove the collector-number suffix"),
            ));
        }
    }
    violations
}

fn has_collector_suffix(symbol: &str, name: &str) -> bool {
    let Some((_, suffix)) = symbol.rsplit_once('_') else {
        return false;
    };
    if !suffix.starts_with(|c: char| c.is_ascii_digit())
        || !suffix.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return false;
    }
    // Numbers actually printed in a name belong in its symbol too, including
    // Spider-Man 2099 and Black Waltz No. 3. A second appended number does not.
    let name_symbol = name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_uppercase)
        .collect::<Vec<_>>()
        .join("_");
    symbol != name_symbol
}

#[test]
fn width_check_includes_strings_comments_and_unformatted_call_chains() {
    for line in [
        format!("// {}", "x".repeat(118)),
        format!("\"{}\",", "x".repeat(118)),
        format!("call(\"{}\").with_ability(effect);", "x".repeat(120)),
    ] {
        let violations = layout_violations(&line);
        assert_eq!(violations.len(), 1, "missed: {line}");
        assert_eq!(violations[0].0, 1);
        assert!(violations[0].1.contains("columns"));
    }
    assert!(layout_violations(&"é".repeat(120)).is_empty());
    assert_eq!(layout_violations(&"é".repeat(121)).len(), 1);
    assert!(layout_violations(&format!("{}\r\n", "x".repeat(120))).is_empty());
}

#[test]
fn names_reject_collector_suffixes_but_preserve_printed_numbers() {
    for (symbol, name) in [
        ("SOUL_PARTITION_26", "Soul Partition"),
        ("B_F_M_BIG_FURRY_MONSTER_28A", "B.F.M. (Big Furry Monster)"),
        ("AGENT_7_7", "Agent 7"),
    ] {
        assert!(has_collector_suffix(symbol, name));
    }
    for (symbol, name) in [
        ("SOUL_PARTITION", "Soul Partition"),
        ("SPIDER_MAN_2099", "Spider-Man 2099"),
        ("BLACK_WALTZ_NO_3", "Black Waltz No. 3"),
        ("PAIN_101", "Pain 101"),
        (
            "MICHELANGELO_WEIRDNESS_TO_11",
            "Michelangelo, Weirdness to 11",
        ),
    ] {
        assert!(!has_collector_suffix(symbol, name));
    }
}

#[test]
fn declaration_scan_handles_wrapping_and_reports_unreadable_names() {
    let declaration = "pub(in crate::card::sets) static SOUL_PARTITION_26: CardRecord =";
    for initializer in [" CardRecord::new(\n", "\n    CardRecord::new(\n"] {
        let source = format!("{declaration}{initializer}    \"Soul Partition\",\n");
        let violations = layout_violations(&source);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].0, 1);
        assert!(violations[0].1.contains("collector-number suffix"));
    }
    let violations = layout_violations(declaration);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].1.contains("cannot read card name"));
}
