//! Dragon's Maze cards whose audit lines named machinery built since.
//!
//! Extort moved into the shared ability helpers so the Pontiff can grant it,
//! and Emmara's shield is the first prevention installed on a whole group
//! rather than on one permanent.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

/// The Krasis evolves off a bigger arrival and ignores a smaller one.
#[test]
fn the_krasis_evolves_off_a_bigger_creature() {
    let mut game = ready();
    let krasis = creature(10_000, cards::BATTERING_KRASIS, PlayerId::One);
    let krasis_id = krasis.card.id;
    game.battlefield.push(krasis);

    for (index, definition) in [cards::GRIZZLY_BEARS, cards::AIR_ELEMENTAL]
        .into_iter()
        .enumerate()
    {
        game.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent: creature(
                10_100 + u32::try_from(index).expect("a short list"),
                definition,
                PlayerId::One,
            ),
            from: ZoneKind::Hand,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
        drain_pending(&mut game);
    }

    // The 2/2 has greater toughness than a 2/1, and the 4/4 is greater on
    // both, so each arrival is worth one counter.
    assert_eq!(counters(&game, krasis_id), 2);
}

/// Counts how many extort payments one spell offers with the given board.
fn extort_offers(others: usize) -> usize {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::PONTIFF_OF_BLIGHT, PlayerId::One));
    for index in 0..others {
        game.battlefield.push(creature(
            10_100 + u32::try_from(index).expect("a short list"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }

    let spell = card(20_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two mana covers a bear");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");

    // Each instance is its own trigger, and each resolves into its own
    // payment offer. Decline them one at a time and count how many the one
    // spell owed; anything else waiting (the ordering decision) takes its
    // first option.
    let mut offers = 0;
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            if decision.prompt.starts_with("Extort") {
                offers += 1;
            }
            let take = decision.minimum.max(1).min(decision.maximum);
            let chosen = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: chosen,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    offers
}

/// "If a creature has multiple instances of extort, each triggers separately",
/// so one spell owes one offer per creature plus the Pontiff's own.
#[test]
fn the_pontiff_grants_one_extort_instance_each() {
    assert_eq!(extort_offers(0), 1, "the Pontiff's printed instance");
    assert_eq!(extort_offers(2), 3, "and one for each other creature");
}

/// Emmara covers tokens and nothing else, including herself.
#[test]
fn emmara_shields_creature_tokens_only() {
    let mut game = ready();
    let emmara = creature(10_000, cards::EMMARA_TANDRIS, PlayerId::One);
    let emmara_id = emmara.card.id;
    game.battlefield.push(emmara);
    let token = token_permanent(
        10_100,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
        PlayerId::One,
    );
    let token_id = token.card.id;
    game.battlefield.push(token);
    let bear = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    for id in [token_id, bear_id, emmara_id] {
        game.damage_target_from(None, Some(Target::Permanent(id)), 1);
    }
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == token_id),
        "a 1/1 token survived a point it would otherwise die to",
    );
    let marked = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .damage
    };
    assert_eq!(marked(bear_id), 1, "a creature card takes its damage");
    assert_eq!(marked(emmara_id), 1, "and Emmara is a card too");
}

/// All damage, not only combat damage -- the token above took a noncombat
/// point, and a token entering after Emmara is covered as well.
#[test]
fn emmara_covers_a_token_that_arrives_later() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::EMMARA_TANDRIS, PlayerId::One));

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: token_permanent(
            10_100,
            tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
            PlayerId::One,
        ),
        from: ZoneKind::Battlefield,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    let token_id = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
            )
        })
        .expect("it arrived")
        .card
        .id;
    game.damage_target_from_kind(None, Some(Target::Permanent(token_id)), 5, true);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == token_id),
        "the shield reaches a token Emmara did not see enter",
    );
}

/// Notion Thief leaves the turn-based draw alone, then redirects later draws
/// in the same draw step to its controller.
#[test]
fn notion_thief_redirects_every_draw_after_the_first_draw_step_card() {
    let mut game = ready();
    game.battlefield.clear();
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].hand.clear();
        game.players[player.index()].library.clear();
        for index in 0..4 {
            game.players[player.index()].library.push(card(
                30_000
                    + u32::try_from(player.index()).expect("the player index fits in u32") * 100
                    + index,
                cards::ISLAND,
                player,
            ));
        }
    }
    game.put_onto_battlefield(PlayerId::One, cards::NOTION_THIEF)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    game.draw_step_draw_taken = [false; 2];

    game.draw_cards(PlayerId::Two, 1);
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
    assert!(game.players[PlayerId::One.index()].hand.is_empty());

    game.draw_cards(PlayerId::Two, 1);
    assert_eq!(
        game.players[PlayerId::Two.index()].hand.len(),
        1,
        "the opponent keeps only their first draw-step card",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "Notion Thief's controller draws the next card instead",
    );
}

/// Morgue Burst moves its creature-card target first, then still reads that
/// target's last-known power for the damage instruction.
#[test]
fn morgue_burst_reads_the_returned_cards_power() {
    let mut game = ready();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    let angel = card(31_000, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[PlayerId::One.index()].graveyard.push(angel);
    let burst = card(31_001, cards::MORGUE_BURST, PlayerId::One);
    let burst_id = burst.id;
    game.players[PlayerId::One.index()].hand.push(burst);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == burst_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Card(angel_id))
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the graveyard card and opponent are legal targets");
    game.apply(PlayerId::One, cast).expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "the creature card returned to hand",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        16,
        "the moved 4-power card still supplied the damage amount",
    );
}

/// Hired Torturer's random reveal resolves without asking the activating
/// player to choose a hidden card.
#[test]
fn hired_torturer_reveals_from_the_opponents_hand_at_random() {
    let mut game = ready();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand =
        vec![card(32_000, cards::LIGHTNING_BOLT, PlayerId::Two)];
    let torturer = creature(32_001, cards::HIRED_TORTURER, PlayerId::One);
    let torturer_id = torturer.card.id;
    game.battlefield.push(torturer);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.events.clear();

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == torturer_id)
        })
        .expect("the paid tap ability is available");
    game.apply(PlayerId::One, activate)
        .expect("the activation is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::Two.index()].life, 18);
    assert!(game.events.iter().any(|event| {
        matches!(
            event,
            GameEvent::CardRevealed {
                player: PlayerId::Two,
                definition: cards::LIGHTNING_BOLT,
                ..
            }
        )
    }));
}

fn sinister_possession_board() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    game.battlefield.clear();
    let host = creature(32_010, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut possession = creature(32_011, cards::SINISTER_POSSESSION, PlayerId::Two);
    possession.attached_to = Some(host_id);
    game.battlefield.push(possession);
    let other = creature(32_012, cards::GRIZZLY_BEARS, PlayerId::Two);
    let other_id = other.card.id;
    game.battlefield.push(other);
    game.check_state_based_actions();
    (game, host_id, other_id)
}

/// An enchanted attacker triggers when it attacks, but becoming blocked is
/// not a second spelling of the event.
#[test]
fn sinister_possession_triggers_once_when_its_host_attacks_and_is_blocked() {
    let (mut game, host, blocker) = sinister_possession_board();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: host,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the enchanted creature can attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the attack declaration completes");
    drain_pending(&mut game);

    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    game.apply(
        PlayerId::Two,
        Action::DeclareBlocker {
            blocker,
            attacker: host,
        },
    )
    .expect("the creature blocks the enchanted attacker");
    game.apply(PlayerId::Two, Action::FinishDeclaringBlockers)
        .expect("the block declaration completes");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        18,
        "the attack triggers once even though the creature also became blocked",
    );
}

/// The other event is the enchanted creature serving as the blocker, not an
/// arbitrary creature blocking it.
#[test]
fn sinister_possession_triggers_when_its_host_blocks() {
    let (mut game, host, attacker_id) = sinister_possession_board();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    {
        let attacker = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker_id)
            .expect("the attacker is present");
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    }
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == host)
        .expect("the enchanted creature is present")
        .blocking = vec![attacker_id];

    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 18);
}

/// Deadbridge Chant binds the randomly selected card once, then branches on
/// that same object's type instead of asking the player to make a selection.
#[test]
fn deadbridge_chant_puts_creatures_onto_the_battlefield_and_other_cards_in_hand() {
    for (selected, expected_zone) in [
        (cards::GRIZZLY_BEARS, ZoneKind::Battlefield),
        (cards::LIGHTNING_BOLT, ZoneKind::Hand),
    ] {
        let mut game = ready();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        game.players[PlayerId::One.index()].graveyard.clear();
        game.players[PlayerId::One.index()].library.clear();
        game.put_onto_battlefield(PlayerId::One, cards::DEADBRIDGE_CHANT)
            .expect("Deadbridge Chant is cataloged");
        drain_pending(&mut game);

        let selected_card = card(32_100, selected, PlayerId::One);
        game.players[PlayerId::One.index()]
            .graveyard
            .push(selected_card);
        game.step = Step::Upkeep;
        game.handle_upkeep_triggers();
        drain_pending(&mut game);

        let actual_zone = if game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == selected)
        {
            ZoneKind::Battlefield
        } else if game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == selected)
        {
            ZoneKind::Hand
        } else {
            panic!("the selected card did not reach its expected destination");
        };
        assert_eq!(actual_zone, expected_zone);
        assert!(game.pending_decisions.is_empty(), "the choice is random");
    }
}

/// Rot Farm Skeleton pays its mill cost while still in the graveyard, then
/// returns only when the activated ability resolves.
#[test]
fn rot_farm_skeleton_mills_as_a_graveyard_activation_cost() {
    let mut game = ready();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::One.index()].library = (0..5)
        .map(|index| card(33_000 + index, cards::ISLAND, PlayerId::One))
        .collect();
    let skeleton = card(33_100, cards::ROT_FARM_SKELETON, PlayerId::One);
    let skeleton_id = skeleton.id;
    game.players[PlayerId::One.index()].graveyard.push(skeleton);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == skeleton_id)
        })
        .expect("the graveyard activation is available at sorcery speed");
    game.apply(PlayerId::One, activate)
        .expect("the activation is legal");

    assert_eq!(game.players[PlayerId::One.index()].library.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 5);
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.id == skeleton_id)
    );

    drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 4);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ROT_FARM_SKELETON)
    );
}

/// Release makes every player fill the five printed roles independently. An
/// artifact creature can fill the first role, but cannot also satisfy the
/// later creature choice.
#[test]
fn release_sacrifices_one_distinct_permanent_of_each_type() {
    let mut game = ready();
    game.battlefield.clear();
    let roles = [
        cards::ORNITHOPTER,
        cards::GRIZZLY_BEARS,
        cards::BRED_FOR_THE_HUNT,
        cards::PLAINS,
        cards::JACE_MEMORY_ADEPT,
    ];
    for player in [PlayerId::One, PlayerId::Two] {
        for definition in roles {
            game.put_onto_battlefield(player, definition)
                .expect("the role permanent is cataloged");
            drain_pending(&mut game);
        }
    }

    let release = game
        .catalog
        .get(cards::CATCH_RELEASE)
        .expect("Catch // Release is cataloged")
        .parts[1]
        .rules
        .ability_clauses()[0]
        .declarative_effect()
        .expect("Release is declarative");
    assert!(
        matches!(release, EffectDef::ChooseForEachPlayer(_)),
        "the second split-card part is Release: {release:?}",
    );
    let source = spell(34_000, cards::CATCH_RELEASE, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(release),
        &source,
        TriggerContext::empty(),
    );
    assert!(
        game.pending_decisions.is_empty(),
        "unique roles resolve without a choice: {:?}",
        game.pending_decisions
            .iter()
            .map(|decision| &decision.observation)
            .collect::<Vec<_>>(),
    );

    let remaining = game
        .battlefield
        .iter()
        .map(|permanent| (permanent.controller, permanent.card.definition))
        .collect::<Vec<_>>();
    assert!(remaining.is_empty(), "unsacrificed roles: {remaining:?}");
    for player in [PlayerId::One, PlayerId::Two] {
        let graveyard = &game.players[player.index()].graveyard;
        assert_eq!(graveyard.len(), roles.len());
        assert!(
            roles
                .iter()
                .all(|definition| graveyard.iter().any(|card| card.definition == *definition))
        );
    }
}

/// Catch uses the first split-card form and performs all three instructions
/// on the same target.
#[test]
fn catch_steals_and_untaps_its_target() {
    let mut game = ready();
    game.battlefield.clear();
    let mut bear = creature(35_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    bear.tapped = true;
    game.battlefield.push(bear);
    let catch = card(35_001, cards::CATCH_RELEASE, PlayerId::One);
    let catch_id = catch.id;
    game.players[PlayerId::One.index()].hand.push(catch);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == catch_id
                    && choices.play_option() == PlayOptionId::DEFAULT
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bear_id))
            }
            _ => false,
        })
        .expect("Catch can target the opponent's permanent");
    game.apply(PlayerId::One, cast).expect("Catch is cast");
    drain_pending(&mut game);

    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bear_id)
        .expect("the target remains on the battlefield");
    assert_eq!(bear.controller, PlayerId::One);
    assert!(!bear.tapped);
}

/// Blast of Genius binds the card the player actually discarded, then reads
/// that card's mana value after it has moved to the graveyard.
#[test]
fn blast_of_genius_reads_the_discarded_cards_mana_value() {
    let mut game = ready();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    let angel = card(36_000, cards::SERRA_ANGEL, PlayerId::One);
    game.players[PlayerId::One.index()].library = vec![
        card(36_001, cards::ISLAND, PlayerId::One),
        card(36_002, cards::LIGHTNING_BOLT, PlayerId::One),
        angel,
    ];
    let effect = game
        .catalog
        .get(cards::BLAST_OF_GENIUS)
        .expect("Blast of Genius is cataloged")
        .rules
        .ability_clauses()[0]
        .declarative_effect()
        .expect("Blast of Genius is declarative");
    let source = spell_with_targets(
        36_100,
        cards::BLAST_OF_GENIUS,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );

    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &source,
        TriggerContext::empty(),
    );
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the discard choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == "Serra Angel")
        .expect("the drawn Angel can be discarded")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the discard choice is legal");

    assert_eq!(game.players[PlayerId::Two.index()].life, 15);
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL)
    );
}
