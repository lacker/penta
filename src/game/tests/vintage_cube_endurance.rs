//! Endurance: a flash blocker that puts a graveyard back where it came from,
//! in random order below the existing library, without shuffling that library.

use super::*;

/// Endurance on the battlefield, with `graveyard` cards already in player
/// two's graveyard and `library` cards under their library.
fn staged(graveyard: &[CardDefinitionId], library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].library.clear();
    for (index, definition) in library.iter().enumerate() {
        let instance = card(
            80_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::Two,
        );
        game.players[PlayerId::Two.index()].library.push(instance);
    }
    for (index, definition) in graveyard.iter().enumerate() {
        let instance = card(
            80_100 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::Two,
        );
        game.players[PlayerId::Two.index()].graveyard.push(instance);
    }
    let endurance = game
        .put_onto_battlefield(PlayerId::One, cards::ENDURANCE)
        .expect("cataloged");
    (game, endurance)
}

/// Answers the enter-the-battlefield trigger by naming `target`, or by
/// naming nobody when it is `None`, and lets everything else take its
/// default. The trigger offers "you" and "your opponent" rather than a
/// seat, so the wanted option is read off the label.
fn answer_trigger(game: &mut Game, target: Option<PlayerId>) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = match (decision.kind, target) {
                (DecisionKind::TriggerPlacement, None) => Vec::new(),
                (DecisionKind::TriggerPlacement, Some(player)) => {
                    let wanted = if player == decision.player {
                        "you"
                    } else {
                        "your opponent"
                    };
                    decision
                        .options
                        .iter()
                        .filter(|option| option.label == wanted)
                        .map(|option| option.id)
                        .take(1)
                        .collect()
                }
                _ => decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .collect(),
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn library_definitions(game: &Game, player: PlayerId) -> Vec<CardDefinitionId> {
    game.players[player.index()]
        .library
        .iter()
        .map(|card| card.definition)
        .collect()
}

/// It has reach.
#[test]
fn it_has_reach() {
    let (game, endurance) = staged(&[], &[]);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == endurance)
        .expect("it is there");

    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Reach));
}

/// Flash, read the only way that matters: it is castable in the opponent's
/// turn, which is when a graveyard is worth answering.
#[test]
fn it_can_be_cast_on_their_turn() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    let endurance = card(82_000, cards::ENDURANCE, PlayerId::One);
    let endurance_id = endurance.id;
    game.players[PlayerId::One.index()].hand.push(endurance);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == endurance_id)
        ),
    );
}

/// The whole graveyard goes under the library, and the cards that were
/// already there keep their order above it.
#[test]
fn it_puts_a_graveyard_under_a_library() {
    let (mut game, _) = staged(
        &[
            cards::LIGHTNING_BOLT,
            cards::GRIZZLY_BEARS,
            cards::DARK_RITUAL,
        ],
        &[cards::SAVANNAH_LIONS, cards::GIANT_GROWTH],
    );

    answer_trigger(&mut game, Some(PlayerId::Two));

    assert!(
        game.players[PlayerId::Two.index()].graveyard.is_empty(),
        "all of it, not some of it",
    );
    let library = library_definitions(&game, PlayerId::Two);
    assert_eq!(library.len(), 5);
    assert_eq!(
        &library[3..],
        &[cards::SAVANNAH_LIONS, cards::GIANT_GROWTH],
        "the library it lands under is not disturbed",
    );
    let mut buried = library[..3].to_vec();
    buried.sort_unstable();
    let mut expected = vec![
        cards::LIGHTNING_BOLT,
        cards::GRIZZLY_BEARS,
        cards::DARK_RITUAL,
    ];
    expected.sort_unstable();
    assert_eq!(buried, expected, "the same three cards, in some order");
}

#[test]
fn local_graveyard_program_preserves_reference_results_and_zone_identities() {
    for graveyard in [
        Vec::new(),
        vec![
            cards::LIGHTNING_BOLT,
            cards::GRIZZLY_BEARS,
            cards::DARK_RITUAL,
        ],
    ] {
        for target in [None, Some(PlayerId::One), Some(PlayerId::Two)] {
            let (mut reference, _) = staged(&graveyard, &[cards::SAVANNAH_LIONS]);
            let old_ids = reference.players[PlayerId::Two.index()]
                .graveyard
                .iter()
                .map(|card| card.id)
                .collect::<Vec<_>>();
            let mut prepared = reference.clone();
            reference.set_prepared_engine_enabled(false);
            prepared.set_prepared_engine_enabled(true);

            answer_trigger(&mut reference, target);
            answer_trigger(&mut prepared, target);

            assert_eq!(prepared.players, reference.players);
            assert_eq!(prepared.battlefield, reference.battlefield);
            assert_eq!(prepared.events, reference.events);
            assert_eq!(prepared.pending_events, reference.pending_events);
            assert_eq!(prepared.pending_procedures, reference.pending_procedures);
            assert_eq!(
                prepared.card_left_graveyard_this_turn,
                reference.card_left_graveyard_this_turn
            );
            assert!(prepared.stack.is_empty() && reference.stack.is_empty());
            assert!(
                prepared.pending_decisions.is_empty() && reference.pending_decisions.is_empty()
            );
            if target == Some(PlayerId::Two) {
                assert_eq!(
                    reference.card_left_graveyard_this_turn[PlayerId::Two.index()],
                    !graveyard.is_empty()
                );
                assert!(
                    reference.players[PlayerId::Two.index()]
                        .library
                        .iter()
                        .all(|card| !old_ids.contains(&card.id)),
                    "moving to the library creates new zone identities"
                );
            }
        }
    }
}

/// "Up to one target player" is satisfied by naming nobody.
#[test]
fn it_can_name_nobody() {
    let (mut game, _) = staged(&[cards::LIGHTNING_BOLT], &[cards::SAVANNAH_LIONS]);

    answer_trigger(&mut game, None);

    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "a graveyard nobody named is left alone",
    );
}

/// Evoked, it exiles a green card, buries the graveyard, and then goes to
/// the graveyard itself.
#[test]
fn evoking_it_still_buries_the_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.push(card(
        81_000,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));

    let endurance = card(81_001, cards::ENDURANCE, PlayerId::One);
    let endurance_id = endurance.id;
    game.players[PlayerId::One.index()].hand.push(endurance);
    // A green card in hand is the whole cost.
    game.players[PlayerId::One.index()]
        .hand
        .push(card(81_002, cards::GIANT_GROWTH, PlayerId::One));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == endurance_id && choices.costs().alternative().is_some())
        })
        .expect("evoke is offered with a green card in hand and no mana");
    game.apply(PlayerId::One, cast).expect("it is cast");

    answer_trigger(&mut game, Some(PlayerId::Two));

    assert!(
        game.players[PlayerId::Two.index()].graveyard.is_empty(),
        "the trigger happens even though the body does not stay",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::ENDURANCE),
        "evoke sacrifices it",
    );
}

/// "Up to one target player" is any player: naming yourself is the mode
/// nobody prints on the card, and it is how an Endurance answers something
/// eating your own graveyard.
#[test]
fn it_may_bury_your_own_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].graveyard.push(card(
        82_000,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));
    game.players[PlayerId::Two.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.push(card(
        82_001,
        cards::GRIZZLY_BEARS,
        PlayerId::Two,
    ));
    game.put_onto_battlefield(PlayerId::One, cards::ENDURANCE)
        .expect("cataloged");

    answer_trigger(&mut game, Some(PlayerId::One));

    assert!(
        game.players[PlayerId::One.index()].graveyard.is_empty(),
        "your own graveyard went under your own library",
    );
    assert_eq!(
        library_definitions(&game, PlayerId::One),
        vec![cards::LIGHTNING_BOLT],
        "which is where it went",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "and theirs was never named",
    );
}

/// Its ruling: "the mana value of a spell is determined by only its mana
/// cost, no matter what the total cost to cast that spell was." Evoked for
/// nothing, it is still a three-mana spell that a Spell Blast pays three
/// for.
#[test]
fn evoking_it_does_not_change_its_mana_value() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    let endurance = card(82_100, cards::ENDURANCE, PlayerId::One);
    let endurance_id = endurance.id;
    game.players[PlayerId::One.index()].hand.push(endurance);
    game.players[PlayerId::One.index()]
        .hand
        .push(card(82_101, cards::GIANT_GROWTH, PlayerId::One));
    let blast = card(82_102, cards::SPELL_BLAST, PlayerId::Two);
    let blast_id = blast.id;
    game.players[PlayerId::Two.index()].hand.push(blast);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 4);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == endurance_id && choices.costs().alternative().is_some())
        })
        .expect("evoke costs no mana at all");
    game.apply(PlayerId::One, cast).expect("it is cast");
    let on_stack = game.stack.last().expect("it is on the stack").id;
    game.priority = PlayerId::Two;

    let blasts = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == blast_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(on_stack)) =>
            {
                Some(choices.x())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        blasts,
        vec![3],
        "X must be three: what it cost to cast is not what it is worth",
    );
}

/// "If you pay the evoke cost, you can have the creature's own triggered
/// ability resolve before the evoke triggered ability. You can cast spells
/// after that ability resolves but before you have to sacrifice the
/// creature." Two triggers, ordered, and a real window between them: the
/// graveyard is already buried while a 3/4 with reach is still standing.
#[test]
fn the_evoke_sacrifice_can_be_left_until_after_the_trigger() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.push(card(
        81_400,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    let endurance = card(81_401, cards::ENDURANCE, PlayerId::One);
    let endurance_id = endurance.id;
    game.players[PlayerId::One.index()].hand.push(endurance);
    game.players[PlayerId::One.index()]
        .hand
        .push(card(81_402, cards::GIANT_GROWTH, PlayerId::One));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == endurance_id && choices.costs().alternative().is_some())
        })
        .expect("evoke is offered with a green card in hand and no mana");
    game.apply(PlayerId::One, cast).expect("it is cast");

    let on_battlefield = |game: &Game| {
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ENDURANCE)
    };

    // Answer everything the two triggers ask, stopping the moment the
    // graveyard has been buried: that is the window the ruling describes.
    for _ in 0..16 {
        if game.players[PlayerId::Two.index()].graveyard.is_empty() {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // The ordering decision wants every option; the trigger's own
            // question wants the opponent.
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .map_or_else(
                    || decision.options.iter().map(|option| option.id).collect(),
                    |option| vec![option.id],
                );
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.players[PlayerId::Two.index()].graveyard.is_empty(),
        "the graveyard went under the library",
    );
    assert!(
        on_battlefield(&game),
        "and the body is still here: the sacrifice is a separate trigger \
         that has not resolved yet",
    );

    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        !on_battlefield(&game),
        "once it does resolve, evoke takes the body after all",
    );
}
