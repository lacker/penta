use super::*;

type DeckBuilder = fn() -> Deck;

#[test]
fn old_school_top_level_builders_remain_compatible() {
    let builders: &[(DeckBuilder, DeckBuilder)] = &[
        (goblins, old_school_93_94::goblins),
        (sligh, old_school_93_94::sligh),
        (artifacts, old_school_93_94::artifacts),
        (robots, old_school_93_94::robots),
        (the_deck, old_school_93_94::the_deck),
        (mono_black, old_school_93_94::mono_black),
        (white_weenie, old_school_93_94::white_weenie),
        (erhnamgeddon, old_school_93_94::erhnamgeddon),
        (counterburn, old_school_93_94::counterburn),
        (lions_dib, old_school_93_94::lions_dib),
        (bwr_aggro, old_school_93_94::bwr_aggro),
        (gr_aggro, old_school_93_94::gr_aggro),
        (troll_disk, old_school_93_94::troll_disk),
        (jeskai_aggro, old_school_93_94::jeskai_aggro),
        (lions_dib_bolt, old_school_93_94::lions_dib_bolt),
        (mono_red_atog, old_school_93_94::mono_red_atog),
    ];

    for (top_level, namespaced) in builders {
        assert_eq!(top_level(), namespaced());
    }
}

#[test]
fn all_yaml_decks_resolve_and_validate_for_their_formats() {
    let catalog = card::catalog().unwrap();
    for source in BUILTIN_DECKS {
        let deck = source.resolve(&catalog);
        let Some(format) = source.format else {
            continue;
        };
        deck.clone()
            .validate_for_format(&catalog, format)
            .unwrap_or_else(|error| panic!("{}: {error}", source.source));
        assert_eq!(
            crate::protocol::deck_by_name_for_format(format, source.name),
            Some(deck.clone())
        );
        for alias in std::iter::once(&source.id).chain(source.aliases) {
            assert_eq!(
                crate::protocol::deck_by_name_for_format(
                    format,
                    &format!(" {} ", alias.to_ascii_uppercase())
                ),
                Some(deck.clone())
            );
        }
        if format == Format::Premodern {
            for id in &deck.main {
                let card = catalog.get(*id).unwrap();
                assert_eq!(
                    card.implementation_status(),
                    crate::ImplementationStatus::Complete,
                    "{}: {} is unsupported",
                    source.name,
                    card.name
                );
            }
        }
    }
}

#[test]
fn woe_hob_event_lists_preserve_published_sizes_and_stay_out_of_playable_menus() {
    let catalog = card::catalog().unwrap();
    let lists: Vec<_> = BUILTIN_DECKS
        .iter()
        .filter(|source| source.source.starts_with("decks/woe_hob_standard/"))
        .collect();
    assert_eq!(lists.len(), 16);
    for source in lists {
        assert_eq!(source.format, None);
        let deck = source.resolve(&catalog);
        let expected_main = match source.id {
            "izzet_spellementals_fazparte" | "izzet_spellementals_darth_vaner" => 61,
            _ => 60,
        };
        assert_eq!(deck.main.len(), expected_main, "{}", source.source);
        assert_eq!(deck.sideboard.len(), 15, "{}", source.source);
        for &format in Format::ALL {
            assert!(!crate::protocol::deck_names_for_format(format).contains(&source.name));
            for name in [source.name, source.id] {
                assert_eq!(crate::protocol::deck_by_name_for_format(format, name), None);
            }
        }
    }
}

#[test]
fn print_deck_report() {
    let catalog = card::catalog().unwrap();
    for source in BUILTIN_DECKS {
        println!("{} — {}", source.source, source.name);
        if source.format.is_none() {
            println!("  Format profile not registered.");
        }
        match source.resolve(&catalog).validate_supported_cards(&catalog) {
            Ok(()) => println!("  All cards supported."),
            Err(error @ crate::DeckError::UnsupportedCards(_)) => println!("  {error}"),
            Err(error) => panic!("{}: {error}", source.source),
        }
    }
}

#[test]
fn cedh_seed_decks_keep_commanders_separate() {
    let catalog = card::catalog().expect("built-in catalog");
    assert!(Format::Cedh.defers_deck_legality());
    let metadata = Format::Cedh
        .commander_definition()
        .expect("cEDH Commander policy metadata");
    assert_eq!(metadata.rules.starting_life, 40);
    assert!(metadata.banned_cards.contains(&"Mana Crypt"));
    assert_eq!(
        metadata.companion_only_banned_cards,
        &["Lutri, the Spellchaser"]
    );
    let decks = BUILTIN_DECKS
        .iter()
        .filter(|source| source.format == Some(Format::Cedh))
        .collect::<Vec<_>>();
    assert_eq!(decks.len(), 16, "cEDH keeps the top 16 seed lists");

    for source in decks {
        assert!(
            source.source.starts_with("decks/cedh/"),
            "{}",
            source.source
        );
        let deck = source.resolve(&catalog);
        assert!(
            (1..=2).contains(&deck.commanders.len()),
            "{} must retain one or two designated commanders",
            source.source
        );
        assert!(
            !deck.main.is_empty(),
            "{} must retain its mainboard",
            source.source
        );
        assert!(
            deck.sideboard.is_empty(),
            "{} has no gameplay sideboard in this imported corpus",
            source.source
        );
        deck.validate_for_format(&catalog, Format::Cedh)
            .unwrap_or_else(|error| panic!("{}: {error}", source.source));
    }
}

#[test]
fn duel_commander_seed_decks_resolve_the_published_top_eight() {
    let catalog = card::catalog().unwrap();
    let decks = BUILTIN_DECKS
        .iter()
        .filter(|source| source.format == Some(Format::DuelCommander))
        .collect::<Vec<_>>();
    assert_eq!(decks.len(), 8);
    for source in decks {
        let deck = source.resolve(&catalog);
        assert!((1..=2).contains(&deck.commanders.len()), "{}", source.name);
        assert_eq!(
            deck.commanders.len() + deck.main.len(),
            100,
            "{}",
            source.name
        );
        assert!(deck.sideboard.is_empty());
        deck.validate_for_format(&catalog, Format::DuelCommander)
            .unwrap();
    }
}

#[test]
fn eternal_event_lists_preserve_published_sizes_and_format_membership() {
    let catalog = card::catalog().unwrap();
    for (format, expected) in [(Format::Legacy, 16), (Format::Vintage, 8)] {
        let lists: Vec<_> = BUILTIN_DECKS
            .iter()
            .filter(|source| source.format == Some(format))
            .collect();
        assert_eq!(lists.len(), expected);
        for source in lists {
            let deck = source.resolve(&catalog);
            let main = if source.name.starts_with("BW Death & Taxes") {
                80
            } else if source.name == "Reanimator — The_shallow_grave" {
                61
            } else {
                60
            };
            assert_eq!(deck.main.len(), main, "{}", source.source);
            assert_eq!(deck.sideboard.len(), 15, "{}", source.source);
            let other = if format == Format::Legacy {
                Format::Vintage
            } else {
                Format::Legacy
            };
            assert!(crate::protocol::deck_by_name_for_format(other, source.name).is_none());
        }
    }
}

#[test]
fn eternal_restrictions_count_main_and_sideboard_together() {
    use crate::{DeckError, card::cards};
    let catalog = card::catalog().unwrap();
    let mut deck = Deck {
        commanders: vec![],
        main: vec![cards::ISLAND; 59],
        sideboard: vec![],
    };
    deck.main.push(cards::BLACK_LOTUS);
    assert!(
        deck.clone()
            .validate_for_format(&catalog, Format::Vintage)
            .is_ok()
    );
    assert!(matches!(
        deck.clone().validate_for_format(&catalog, Format::Legacy),
        Err(DeckError::BannedCard(_))
    ));
    deck.sideboard.push(cards::BLACK_LOTUS);
    assert!(matches!(
        deck.validate_for_format(&catalog, Format::Vintage),
        Err(DeckError::TooManyCopies {
            count: 2,
            limit: 1,
            ..
        })
    ));
}
