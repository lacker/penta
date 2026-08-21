//! Soulbond: a symmetric pairing between two unpaired creatures.
//!
//! CR 702.94 is two triggered abilities rather than one, and the pair is
//! state rather than a one-shot effect: it survives until one of the two
//! stops being a creature its controller controls, and then both are free
//! again.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn partner(game: &Game, id: GameObjectId) -> Option<GameObjectId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .and_then(|permanent| permanent.paired_with)
}

/// Answers each waiting decision by taking the last option, which for the
/// pairing offer is a partner rather than declining.
fn drain_pairing(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Puts a creature onto the battlefield through the entry path and answers
/// with the object id the engine assigned it -- which is not the one handed
/// to the helper.
fn arrive(
    game: &mut Game,
    id: u32,
    definition: crate::ids::CardDefinitionId,
    player: PlayerId,
) -> GameObjectId {
    let before = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(id, definition, player),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pairing(game);
    game.battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == definition && !before.contains(&permanent.card.id)
        })
        .expect("it arrived")
        .card
        .id
}

/// The Forcemage arriving beside a creature pairs with it, and both grow.
#[test]
fn the_forcemage_pairs_as_it_arrives() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let mage_id = arrive(&mut game, 10_100, cards::TRUSTED_FORCEMAGE, PlayerId::One);

    assert_eq!(partner(&game, mage_id), Some(bear_id), "paired both ways");
    assert_eq!(partner(&game, bear_id), Some(mage_id));
    assert_eq!(stats(&game, mage_id), (Some(3), Some(3)));
    assert_eq!(stats(&game, bear_id), (Some(3), Some(3)));
}

/// The other half: an unpaired Forcemage already out pairs with a creature
/// that arrives later, and a third arrival finds it taken.
#[test]
fn an_unpaired_forcemage_waits_for_the_next_creature() {
    let mut game = ready();
    let mage = creature(10_000, cards::TRUSTED_FORCEMAGE, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    assert_eq!(partner(&game, mage_id), None, "nothing to pair with yet");
    assert_eq!(stats(&game, mage_id), (Some(2), Some(2)));

    let first = arrive(&mut game, 10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    assert_eq!(partner(&game, mage_id), Some(first));

    let second = arrive(&mut game, 10_200, cards::GRIZZLY_BEARS, PlayerId::One);
    assert_eq!(
        partner(&game, mage_id),
        Some(first),
        "an already-paired creature offers nothing",
    );
    assert_eq!(partner(&game, second), None);
}

/// The pair is state, so losing the partner frees the survivor and takes the
/// bonus with it.
#[test]
fn the_pair_breaks_when_the_partner_leaves() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mage_id = arrive(&mut game, 10_100, cards::TRUSTED_FORCEMAGE, PlayerId::One);
    assert_eq!(stats(&game, mage_id), (Some(3), Some(3)));

    game.battlefield
        .retain(|permanent| permanent.card.id != bear_id);
    game.check_state_based_actions();

    assert_eq!(partner(&game, mage_id), None, "freed by the state check");
    assert_eq!(
        stats(&game, mage_id),
        (Some(2), Some(2)),
        "and the bonus goes with the pair",
    );
}

/// "You control both", so a creature the opponent controls is never offered.
#[test]
fn soulbond_never_reaches_across_the_table() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two));

    let mage_id = arrive(&mut game, 10_100, cards::TRUSTED_FORCEMAGE, PlayerId::One);
    assert_eq!(partner(&game, mage_id), None, "not yours to pair with");
    assert_eq!(stats(&game, mage_id), (Some(2), Some(2)));
}

/// The intervening-if, separately from the pairing rule itself: a paired
/// Forcemage does not even offer a choice when a third creature arrives.
#[test]
fn a_paired_forcemage_offers_nothing_on_a_later_arrival() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::TRUSTED_FORCEMAGE, PlayerId::One));
    arrive(&mut game, 10_100, cards::GRIZZLY_BEARS, PlayerId::One);

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_200, cards::GRIZZLY_BEARS, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        game.pending_decisions.is_empty(),
        "the trigger never went on the stack",
    );
}

/// Every keyword grant reaches both halves of the pair and lapses together.
#[test]
fn each_keyword_grant_reaches_both_creatures() {
    for (definition, keyword) in [
        (cards::SILVERBLADE_PALADIN, KeywordAbility::DoubleStrike),
        (cards::SPECTRAL_GATEGUARDS, KeywordAbility::Vigilance),
        (cards::ELGAUD_SHIELDMATE, KeywordAbility::Hexproof),
        (cards::WINGCRAFTER, KeywordAbility::Flying),
        (cards::HANWEIR_LANCER, KeywordAbility::FirstStrike),
        (cards::LIGHTNING_MAULER, KeywordAbility::Haste),
        (cards::GEIST_TRAPPERS, KeywordAbility::Reach),
        (cards::NIGHTSHADE_PEDDLER, KeywordAbility::Deathtouch),
        (cards::PATHBREAKER_WURM, KeywordAbility::Trample),
        (cards::NEARHEATH_PILGRIM, KeywordAbility::Lifelink),
    ] {
        let mut game = ready();
        let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
        let bear_id = bear.card.id;
        game.battlefield.push(bear);
        let bystander = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
        let bystander_id = bystander.card.id;
        game.battlefield.push(bystander);

        let source_id = arrive(&mut game, 10_100, definition, PlayerId::One);
        assert_eq!(partner(&game, source_id), Some(bear_id));

        let has = |game: &Game, id: GameObjectId| {
            game.permanent_has_executable_keyword(
                game.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .expect("still there"),
                keyword,
            )
        };
        assert!(has(&game, source_id), "{keyword:?} on the source");
        assert!(has(&game, bear_id), "{keyword:?} on the partner");
        assert!(!has(&game, bystander_id), "{keyword:?} reaches nobody else");

        game.battlefield
            .retain(|permanent| permanent.card.id != bear_id);
        game.check_state_based_actions();
        assert!(
            !has(&game, source_id),
            "{keyword:?} lapses when the pair breaks",
        );
    }
}

/// The two size grants differ only in the number, and both halves take it.
#[test]
fn each_size_grant_reaches_both_creatures() {
    for (definition, bonus, printed) in [
        (cards::DRUIDS_FAMILIAR, 2, (2, 2)),
        (cards::WOLFIR_SILVERHEART, 4, (4, 4)),
    ] {
        let mut game = ready();
        let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
        let bear_id = bear.card.id;
        game.battlefield.push(bear);

        let source_id = arrive(&mut game, 10_100, definition, PlayerId::One);
        assert_eq!(partner(&game, source_id), Some(bear_id));
        assert_eq!(
            stats(&game, bear_id),
            (Some(2 + bonus), Some(2 + bonus)),
            "the partner grew",
        );
        assert_eq!(
            stats(&game, source_id),
            (Some(printed.0 + bonus), Some(printed.1 + bonus)),
            "and so did the source",
        );

        game.battlefield
            .retain(|permanent| permanent.card.id != bear_id);
        game.check_state_based_actions();
        assert_eq!(
            stats(&game, source_id),
            (Some(printed.0), Some(printed.1)),
            "back to printed size once unpaired",
        );
    }
}

/// The granted abilities go on each creature separately, so the pair has two
/// of them rather than sharing one.
#[test]
fn a_granted_activated_ability_lands_on_both_creatures() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One));
    let bystander = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    let mentor_id = arrive(&mut game, 10_100, cards::STERN_MENTOR, PlayerId::One);
    // The pairing offer chooses one of the two; the other is the control.
    let bear_id = partner(&game, mentor_id).expect("it paired with one of them");
    let bystander_id = if bear_id == bystander_id {
        GameObjectId(10_000)
    } else {
        bystander_id
    };
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.priority = PlayerId::One;

    let sources = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .filter_map(|action| match action {
                Action::ActivateAbility { source, .. } => Some(*source),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert!(sources(&game).contains(&mentor_id), "the Mentor has it");
    assert!(sources(&game).contains(&bear_id), "and so does its partner");
    assert!(
        !sources(&game).contains(&bystander_id),
        "and nobody else does",
    );

    game.battlefield
        .retain(|permanent| permanent.card.id != bear_id);
    game.check_state_based_actions();
    assert!(
        !sources(&game).contains(&mentor_id),
        "the ability goes when the pair breaks",
    );
}

/// The Escort's protection is the creature-type quality, so a Zombie's
/// damage stops landing on either half.
#[test]
fn the_escort_shields_both_creatures_from_zombies() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let zombie = token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::Two,
    );
    let zombie_id = zombie.card.id;
    game.battlefield.push(zombie);

    let escort_id = arrive(&mut game, 10_200, cards::DIREGRAF_ESCORT, PlayerId::One);
    assert_eq!(partner(&game, escort_id), Some(bear_id));

    for victim in [escort_id, bear_id] {
        game.damage_target_from(Some(zombie_id), Some(Target::Permanent(victim)), 1);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == victim)
                .expect("still there")
                .damage,
            0,
            "protection held",
        );
    }
}

/// The Lumberknot sits out until something pairs with it, and sits down
/// again when the pair breaks.
#[test]
fn the_lumberknot_needs_a_partner_to_do_anything() {
    let mut game = ready();
    let knot = creature(10_000, cards::FLOWERING_LUMBERKNOT, PlayerId::One);
    let knot_id = knot.card.id;
    game.battlefield.push(knot);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let can_attack = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == knot_id),
        )
    };
    game.step = Step::DeclareAttackers;
    assert!(!can_attack(&game), "unpaired, so it stays home");

    game.step = Step::PrecombatMain;
    let mage_id = arrive(&mut game, 10_100, cards::TRUSTED_FORCEMAGE, PlayerId::One);
    assert_eq!(
        partner(&game, knot_id),
        Some(mage_id),
        "the Forcemage took it"
    );
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::DeclareAttackers;
    game.priority = PlayerId::One;
    assert!(can_attack(&game), "paired, so it may attack");

    game.battlefield
        .retain(|permanent| permanent.card.id != mage_id);
    game.check_state_based_actions();
    assert!(
        !can_attack(&game),
        "and sits down again when the pair breaks"
    );
}

/// The same clause covers blocking.
#[test]
fn the_lumberknot_cannot_block_unpaired_either() {
    let mut game = ready();
    let knot = creature(10_000, cards::FLOWERING_LUMBERKNOT, PlayerId::One);
    let knot_id = knot.card.id;
    game.battlefield.push(knot);
    let mut attacker = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::One;

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::DeclareBlocker {
                blocker: knot_id,
                attacker: attacker_id,
            }),
        "unpaired, so it cannot block either",
    );
}

#[test]
fn every_soulbond_card_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::TRUSTED_FORCEMAGE,
        cards::SILVERBLADE_PALADIN,
        cards::SPECTRAL_GATEGUARDS,
        cards::ELGAUD_SHIELDMATE,
        cards::WINGCRAFTER,
        cards::HANWEIR_LANCER,
        cards::LIGHTNING_MAULER,
        cards::DRUIDS_FAMILIAR,
        cards::GEIST_TRAPPERS,
        cards::NIGHTSHADE_PEDDLER,
        cards::PATHBREAKER_WURM,
        cards::WOLFIR_SILVERHEART,
        cards::NEARHEATH_PILGRIM,
        cards::GALVANIC_ALCHEMIST,
        cards::STERN_MENTOR,
        cards::TANDEM_LOOKOUT,
        cards::STONEWRIGHT,
        cards::DIREGRAF_ESCORT,
        cards::FLOWERING_LUMBERKNOT,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
