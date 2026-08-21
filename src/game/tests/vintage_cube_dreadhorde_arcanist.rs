//! Dreadhorde Arcanist: a one-mana spell back every attack, and more than
//! one mana's worth once something makes it bigger.

use super::*;

static OTHER_SAME_KIND_GRANT: AbilityDef = AbilityDef::alternative_cast(
    ManaCost::new(7, 0),
    AlternativeCastKindDef::WithoutPayingManaCost,
    Some("A different same-kind test grant."),
    EffectDef::None,
);
static IDENTICAL_FREE_GRANT: AbilityDef = AbilityDef::alternative_cast(
    ManaCost::new(0, 0),
    AlternativeCastKindDef::WithoutPayingManaCost,
    Some("An identical one-shot free-cast grant."),
    EffectDef::None,
);

/// Player One with an Arcanist out since last turn and `graveyard` behind
/// it, ready to declare attackers.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for definition in graveyard {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].graveyard.push(card);
    }
    let arcanist = game
        .put_onto_battlefield(PlayerId::One, cards::DREADHORDE_ARCANIST)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, arcanist)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Attacks with the Arcanist and answers the trigger's target with `wanted`,
/// leaving the "cast or decline" offer standing.
fn attack(game: &mut Game, arcanist: GameObjectId, wanted: Option<CardDefinitionId>) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: arcanist,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    settle(game);
    let Some(wanted) = wanted else {
        return;
    };
    // The trigger picks its target first; the offer to cast comes after.
    let seat = deciding(game).expect("the trigger asks what it points at");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, found)| found.card_definition() == Some(wanted))
        })
        .unwrap_or_else(|| panic!("{wanted:?} is a legal target"))
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

/// The free cast on offer for the graveyard card of `definition`, if any.
fn free_cast(game: &Game, definition: CardDefinitionId) -> Option<Action> {
    let card = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == definition)?
        .id;
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } => {
                *id == card
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
}

/// A Bolt in the graveyard comes back for nothing, and the Arcanist's
/// controller pays no mana for it.
#[test]
fn it_casts_a_one_mana_spell_for_free() {
    let (mut game, arcanist) = staged(&[cards::LIGHTNING_BOLT]);

    attack(&mut game, arcanist, Some(cards::LIGHTNING_BOLT));
    let cast = free_cast(&game, cards::LIGHTNING_BOLT).expect("the offer stands");
    game.apply(PlayerId::One, cast).expect("it casts for free");
    settle(&mut game);

    assert_eq!(game.players[1].life, 17, "the Bolt resolved");
    assert_eq!(
        game.players[0].mana_pool.total(),
        0,
        "and nothing was spent on it",
    );
    assert!(
        game.temporary_ability_grants.is_empty(),
        "accepting the offer returns its temporary grant"
    );
}

/// A one-shot offer names its granted cost exactly. Even when the card's own
/// flashback cost is affordable, the standing Arcanist decision does not turn
/// into a menu of unrelated graveyard permissions.
#[test]
fn the_offer_exposes_only_the_granted_alternative_cost() {
    let (mut game, arcanist) = staged(&[cards::FAITHLESS_LOOTING]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);

    attack(&mut game, arcanist, Some(cards::FAITHLESS_LOOTING));
    let card = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::FAITHLESS_LOOTING)
        .expect("Faithless Looting remains in the graveyard")
        .id;
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card: found, .. } if *found == card))
        .collect::<Vec<_>>();
    assert!(!casts.is_empty(), "the free cast is offered");

    let definition = game
        .catalog
        .get(cards::FAITHLESS_LOOTING)
        .expect("Faithless Looting is cataloged");
    assert!(casts.iter().all(|action| {
        let Action::CastSpell { choices, .. } = action else {
            return false;
        };
        let option = definition
            .play_option(choices.play_option())
            .expect("the cast names a play option");
        game.selected_alternative_kind(definition, option, card, choices.costs())
            == Some(AlternativeCastKindDef::WithoutPayingManaCost)
    }));
}

#[test]
fn the_offer_selects_its_exact_grant_among_same_kind_definitions() {
    let (mut game, arcanist) = staged(&[cards::LIGHTNING_BOLT]);
    let card = game.players[0].graveyard[0].id;
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: card,
        ability: OTHER_SAME_KIND_GRANT,
    });

    attack(&mut game, arcanist, Some(cards::LIGHTNING_BOLT));

    assert!(
        free_cast(&game, cards::LIGHTNING_BOLT).is_some(),
        "the unaffordable same-kind grant cannot replace Arcanist's exact free-cast grant",
    );
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::MayCastGranted { ability, .. }
            if ability != OTHER_SAME_KIND_GRANT
    ));
}

#[test]
fn declining_removes_only_the_exact_grant_behind_the_offer() {
    let (mut game, _) = staged(&[cards::LIGHTNING_BOLT]);
    let card = game.players[0].graveyard[0].id;
    let existing = TemporaryAbilityGrant {
        object: card,
        ability: IDENTICAL_FREE_GRANT,
    };
    game.temporary_ability_grants.push(existing);
    game.offer_granted_cast(PlayerId::One, card, &IDENTICAL_FREE_GRANT);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the offer stands");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .expect("declining the exact offer is legal");

    assert_eq!(game.temporary_ability_grants, vec![existing]);
}

#[test]
fn a_rebuilt_granted_cast_offer_cannot_be_made_cancellable() {
    let (mut game, arcanist) = staged(&[cards::LIGHTNING_BOLT]);
    attack(&mut game, arcanist, Some(cards::LIGHTNING_BOLT));
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 94_101)
        .expect("the authored granted-cast offer reconstructs");

    let mut cancellable = wire;
    cancellable["decision"]["cancellable"] = serde_json::json!(true);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &cancellable,
        &hidden,
        94_102,
    )
    .expect_err("cancellation cannot bypass exact grant cleanup");
    assert!(
        error.contains("cancellability disagrees"),
        "unexpected error: {error}"
    );
}

/// "If that spell would be put into your graveyard, exile it instead."
#[test]
fn what_it_casts_is_exiled_rather_than_buried() {
    let (mut game, arcanist) = staged(&[cards::LIGHTNING_BOLT]);

    attack(&mut game, arcanist, Some(cards::LIGHTNING_BOLT));
    let cast = free_cast(&game, cards::LIGHTNING_BOLT).expect("the offer stands");
    game.apply(PlayerId::One, cast).expect("it casts");
    settle(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt is in exile",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and never went back to the graveyard",
    );
}

/// "Mana value less than or equal to this creature's power": a 1/3 cannot
/// reach a two-mana spell.
#[test]
fn a_spell_above_its_power_is_not_a_legal_target() {
    let (mut game, arcanist) = staged(&[cards::EXHUME]);

    attack(&mut game, arcanist, None);

    assert!(
        deciding(&game).is_none(),
        "nothing in the graveyard is small enough to point at",
    );
}

/// The bound is read live: a counter makes a two-mana spell reachable.
#[test]
fn counters_raise_what_it_can_reach() {
    let (mut game, arcanist) = staged(&[cards::EXHUME]);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == arcanist)
    {
        permanent.set_counters(CounterKind::PlusOnePlusOne, 1);
    }

    attack(&mut game, arcanist, Some(cards::EXHUME));

    let card = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::EXHUME)
        .expect("it is still there")
        .id;
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card)),
        "a 2/4 reaches a two-mana sorcery, and the offer to cast it stands",
    );
}

/// "Instant or sorcery card": a creature in the graveyard is not one.
#[test]
fn a_creature_card_is_not_a_legal_target() {
    let (mut game, arcanist) = staged(&[cards::SAVANNAH_LIONS]);

    attack(&mut game, arcanist, None);

    assert!(
        deciding(&game).is_none(),
        "a Savannah Lions is neither an instant nor a sorcery",
    );
}

/// "You may": declining leaves the card where it was, and takes the
/// permission back with it.
#[test]
fn declining_leaves_the_card_and_takes_the_permission_back() {
    let (mut game, arcanist) = staged(&[cards::LIGHTNING_BOLT]);
    attack(&mut game, arcanist, Some(cards::LIGHTNING_BOLT));
    let seat = deciding(&game).expect("the offer stands");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision.options.first().expect("Decline is offered").id;

    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("declining is legal");
    settle(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt stayed in the graveyard",
    );
    assert!(
        free_cast(&game, cards::LIGHTNING_BOLT).is_none(),
        "and is not castable any more: the permission was this trigger's alone",
    );
}

/// Your own graveyard, not theirs.
#[test]
fn it_does_not_reach_the_other_graveyard() {
    let (mut game, arcanist) = staged(&[]);
    let theirs = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].graveyard.push(theirs);

    attack(&mut game, arcanist, None);

    assert!(deciding(&game).is_none(), "the clause says your graveyard");
}
