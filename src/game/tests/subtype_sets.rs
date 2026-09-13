use super::*;
use crate::card::{Subtype, SubtypeSet};

#[test]
fn subtype_sets_drop_departing_families_in_both_engines() {
    for prepared in [false, true] {
        for (definition, remaining_type, expected) in [
            (
                cards::DRYAD_ARBOR,
                CardType::Creature,
                SubtypeSet::from_names(&["Dryad"]),
            ),
            (cards::RANCOR, CardType::Artifact, SubtypeSet::EMPTY),
            (cards::CANDY_TRAIL, CardType::Enchantment, SubtypeSet::EMPTY),
            (
                cards::NICOL_BOLAS_PLANESWALKER,
                CardType::Enchantment,
                SubtypeSet::EMPTY,
            ),
        ] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let target = GameObjectId(900_100);
            game.battlefield
                .push(creature(target.0, definition, PlayerId::One));
            assert!(!game.effective_subtypes(&game.battlefield[0]).is_empty());
            let object = spell_with_targets(
                900_101,
                cards::LIGHTNING_BOLT,
                PlayerId::One,
                vec![Target::Permanent(target)],
                0,
            );
            game.resolve_effect_def(
                ScopedEffect::primary(EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_card_types(CardTypeSet::single(remaining_type)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                }),
                &object,
                TriggerContext::empty(),
            );
            assert_eq!(game.effective_subtypes(&game.battlefield[0]), expected);
        }
    }
}

#[test]
fn subtype_sets_use_oracle_names_for_the_corrected_cards() {
    let catalog = crate::card::catalog().unwrap();
    for (name, expected) in [
        (
            "Cephalid Looter",
            SubtypeSet::from_names(&["Octopus", "Rogue"]),
        ),
        ("Cephalid Broker", SubtypeSet::from_names(&["Octopus"])),
        ("Cephalid Retainer", SubtypeSet::from_names(&["Octopus"])),
        (
            "Aboshan, Cephalid Emperor",
            SubtypeSet::from_names(&["Octopus", "Noble"]),
        ),
        ("The Wandering Emperor", SubtypeSet::EMPTY),
    ] {
        let definition = catalog
            .definitions()
            .into_iter()
            .find(|definition| definition.name == name)
            .unwrap();
        assert_eq!(definition.rules.subtype_set(), expected, "{name}");
    }
    let immunity = catalog
        .definitions()
        .into_iter()
        .find(|definition| definition.name == "Eldritch Immunity")
        .unwrap();
    assert!(immunity.rules.has_type(CardType::Kindred));
    assert!(immunity.rules.subtype_set().contains(Subtype::Eldrazi));
    assert!(Subtype::from_name("Cephalid").is_none());
}
