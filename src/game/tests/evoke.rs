//! Evoke pays one particular alternative cost, even when other costs coexist.

use super::*;

fn cast_for(game: &mut Game, spell: GameObjectId, cost: Option<AlternativeCostId>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell && choices.costs().alternative() == cost
            )
        })
        .expect("the selected casting cost is payable");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
}

#[test]
fn evoke_does_not_sacrifice_for_another_printed_alternative_cost() {
    static ABILITIES: [AbilityDef; 5] = [
        AbilityDef::alternative_cast(
            mana_cost!("{1}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("Evoke {1}"),
            EffectDef::None,
        )
        .with_alternative_cost_binding(crate::Binding!("evoke")),
        abilities::flying(),
        AbilityDef::alternative_cast(
            mana_cost!("{2}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may pay {2} rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_cost_binding(crate::Binding!("other_alternative_cost")),
        abilities::evoke_sacrifice(),
        AbilityDef::triggered_if(
            "When this creature enters, if its other alternative cost was paid, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAlternativeCost(crate::Binding!(
                "other_alternative_cost"
            )),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];
    static REORDERED: [AbilityDef; 5] = [
        ABILITIES[2],
        ABILITIES[1],
        ABILITIES[0],
        ABILITIES[3],
        ABILITIES[4],
    ];
    for (reordered, cost, sacrificed) in [
        (false, AlternativeCostId(2), false),
        (false, AlternativeCostId(0), true),
        (true, AlternativeCostId(0), false),
        (true, AlternativeCostId(2), true),
    ] {
        let mut game = ready_game();
        let mut definition = game.catalog.get(cards::MULLDRIFTER).unwrap().clone();
        definition.rules = CardRules::new_creature(mana_cost!("{5}"), &["Elemental"], 2, 2)
            .with_abilities(if reordered { &REORDERED } else { &ABILITIES });
        synchronize_single_part_definition(&mut definition);
        game.catalog = CardCatalog::new(game.catalog.definitions().into_iter().map(|card| {
            if card.id == definition.id {
                definition.clone()
            } else {
                card.clone()
            }
        }))
        .expect("the two independent alternatives are valid");
        let spell = card(231_000, cards::MULLDRIFTER, PlayerId::One);
        let spell_id = spell.id;
        game.players[0].hand.push(spell);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let life = game.players[0].life;
        cast_for(&mut game, spell_id, Some(cost));
        drain_pending(&mut game);
        assert_eq!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::MULLDRIFTER),
            sacrificed,
            "only the evoke cost should cause a sacrifice: {cost:?}",
        );
        assert_eq!(game.players[0].life, life + i16::from(!sacrificed));
    }
}

fn staged_mulldrifter() -> (Game, GameObjectId, AlternativeCostId) {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::OMNISCIENCE)
        .expect("Omniscience is cataloged");
    let spell = card(231_100, cards::MULLDRIFTER, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let printed =
        game.catalog.get(cards::MULLDRIFTER).unwrap().play_options[0].alternative_costs[0].id;
    (game, spell_id, printed)
}

fn free_cost(game: &Game, spell: GameObjectId, printed: AlternativeCostId) -> AlternativeCostId {
    game.legal_actions(PlayerId::One)
        .iter()
        .find_map(|action| {
            let Action::CastSpell { card, choices, .. } = action else {
                return None;
            };
            let alternative = choices.costs().alternative()?;
            (*card == spell && alternative != printed).then_some(alternative)
        })
        .expect("Omniscience offers a distinct alternative")
}

fn assert_mulldrifter_result(game: &Game, evoked: bool, draws: usize, bodies: usize) {
    assert!(game.stack.is_empty());
    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.players[0].hand.len(),
        draws,
        "its draw trigger still resolves"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| game
                .permanent_types(permanent)
                .is_some_and(crate::card::CardTypeSet::is_creature))
            .count(),
        bodies
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|card| { card.definition == cards::MULLDRIFTER })
            .count(),
        usize::from(evoked)
    );
}

#[test]
fn evoke_and_omniscience_are_independent_casting_choices() {
    for (evoked, free, mana_left) in [(false, false, 0), (true, false, 2), (false, true, 5)] {
        let (mut game, spell, printed) = staged_mulldrifter();
        let granted = free_cost(&game, spell, printed);
        let offered = game
            .legal_actions(PlayerId::One)
            .iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if *card == spell => {
                    Some(choices.costs().clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            offered.len(),
            3,
            "normal, evoke, and free; no combined alternative"
        );
        assert!(offered.iter().all(|cost| cost.additional().is_empty()));
        let cost = if free {
            Some(granted)
        } else if evoked {
            Some(printed)
        } else {
            None
        };
        cast_for(&mut game, spell, cost);
        assert_eq!(game.players[0].mana_pool.total(), mana_left);
        let omniscience = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::OMNISCIENCE)
            .unwrap()
            .card
            .id;
        game.exile_permanent(omniscience);
        drain_pending(&mut game);
        assert_mulldrifter_result(&game, evoked, 2, usize::from(!evoked));
    }
}

fn round_trip(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 231_200)
        .expect("the chosen alternative cost reconstructs")
}

#[test]
fn evoke_choice_survives_spell_and_permanent_checkpoints() {
    for evoked in [false, true] {
        let (mut game, spell, printed) = staged_mulldrifter();
        let selected = if evoked {
            printed
        } else {
            free_cost(&game, spell, printed)
        };
        cast_for(&mut game, spell, Some(selected));
        let (wire, _) = checkpoint_fixture(&game, PlayerId::One);
        assert_eq!(
            wire.pointer("/checkpoint/stack/0/castAlternativeCostBinding")
                .and_then(serde_json::Value::as_str),
            evoked.then_some("evoke"),
        );
        game = round_trip(&game);
        pass_priority_pair(&mut game);
        assert!(
            game.battlefield
                .iter()
                .any(|permanent| { permanent.card.definition == cards::MULLDRIFTER }),
            "the creature has entered before its triggers resolve"
        );
        let (wire, _) = checkpoint_fixture(&game, PlayerId::One);
        assert_eq!(
            wire["checkpoint"]["battlefield"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|permanent| permanent["castAlternativeCostBinding"].as_str())
                .collect::<Vec<_>>(),
            if evoked { vec!["evoke"] } else { vec![] },
        );
        game = round_trip(&game);
        drain_pending(&mut game);
        assert_mulldrifter_result(&game, evoked, 2, usize::from(!evoked));
    }
}

#[test]
fn evoke_checkpoint_binding_falls_back_to_the_signature_and_rejects_unknown_names() {
    let (mut game, spell, printed) = staged_mulldrifter();
    cast_for(&mut game, spell, Some(printed));
    let (mut wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    wire["checkpoint"]["stack"][0]
        .as_object_mut()
        .unwrap()
        .remove("castAlternativeCostBinding");
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        231_200,
    )
    .expect("the signature identifies the cost when the additive field is absent");
    drain_pending(&mut rebuilt);
    assert_mulldrifter_result(&rebuilt, true, 2, 0);

    wire["checkpoint"]["stack"][0]["castAlternativeCostBinding"] =
        serde_json::json!("unknown_alternative_cost");
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        231_200,
    )
    .expect_err("unknown names must not silently discard the paid-cost choice");
    assert!(
        error.contains("unknown alternative-cost binding"),
        "{error}"
    );
}

#[test]
fn evoke_spell_copies_keep_the_selected_cost_through_reconstruction() {
    for evoked in [false, true] {
        let (mut game, spell, printed) = staged_mulldrifter();
        let selected = if evoked {
            printed
        } else {
            free_cost(&game, spell, printed)
        };
        cast_for(&mut game, spell, Some(selected));
        let original = game.stack.last().unwrap().clone();
        game.push_copy(original, PlayerId::One, Vec::new());
        game = round_trip(&game);
        drain_pending(&mut game);
        assert_mulldrifter_result(&game, evoked, 4, if evoked { 0 } else { 2 });
    }
}

#[test]
fn evoke_does_not_follow_a_creature_through_a_blink() {
    let (mut game, spell, printed) = staged_mulldrifter();
    cast_for(&mut game, spell, Some(printed));
    pass_priority_pair(&mut game);
    let source = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MULLDRIFTER)
        .unwrap()
        .card
        .id;
    let exiled = game
        .exile_permanent_returning_card(source)
        .expect("the creature was exiled");
    game.return_exiled_card(exiled, ZoneKind::Battlefield, None, None, false, None);
    game = round_trip(&game);
    drain_pending(&mut game);
    assert_mulldrifter_result(&game, false, 4, 1);
}
