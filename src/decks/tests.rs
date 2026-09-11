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
        deck.clone()
            .validate_for_format(&catalog, source.format)
            .unwrap_or_else(|error| panic!("{}: {error}", source.source));
        assert_eq!(
            crate::protocol::deck_by_name_for_format(source.format, source.name),
            Some(deck.clone())
        );
        for alias in std::iter::once(&source.id).chain(source.aliases) {
            assert_eq!(
                crate::protocol::deck_by_name_for_format(
                    source.format,
                    &format!(" {} ", alias.to_ascii_uppercase())
                ),
                Some(deck.clone())
            );
        }
        if source.format == Format::Premodern {
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
