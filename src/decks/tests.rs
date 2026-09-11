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
fn all_yaml_decks_resolve_and_are_legal_in_their_formats() {
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
