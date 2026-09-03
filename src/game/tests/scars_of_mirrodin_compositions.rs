//! Scars of Mirrodin cards whose complete behavior composes several shared
//! runtime shapes. These tests cover the interaction boundaries that are not
//! already exercised by a single mechanic test.

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

fn spell_in_hand(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let spell = card(id, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    spell_id
}

fn has_permanent(game: &Game, definition: CardDefinitionId, controller: PlayerId) -> bool {
    game.battlefield.iter().any(|permanent| {
        permanent.card.definition == definition && permanent.controller == controller
    })
}

#[test]
fn razor_hippogriff_gains_life_from_the_card_it_actually_returns() {
    let mut game = ready();
    game.players[PlayerId::One.index()].life = 10;
    game.players[PlayerId::One.index()]
        .graveyard
        .push(card(9_900, cards::SOL_RING, PlayerId::One));
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(9_901, cards::RAZOR_HIPPOGRIFF, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "the targeted artifact moved to hand",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        11,
        "the moved card's mana value is read through the zone-change result",
    );
}

#[test]
fn shape_anew_uses_the_sacrificed_artifacts_controller_and_the_first_match() {
    let mut game = ready();
    let shape = spell_in_hand(&mut game, 10_000, cards::SHAPE_ANEW);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    let artifact = creature(10_100, cards::SOL_RING, PlayerId::Two);
    let artifact_id = artifact.card.id;
    game.battlefield.push(artifact);
    // Libraries are stored bottom-to-top: the Forest is revealed first and
    // Black Lotus is the first artifact below it.
    game.players[PlayerId::Two.index()].library = vec![
        card(20_001, cards::BLACK_LOTUS, PlayerId::Two),
        card(20_000, cards::FOREST, PlayerId::Two),
    ];

    let cast = cast_action(shape, vec![Target::Permanent(artifact_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).expect("Shape Anew is cast");
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "the targeted artifact was sacrificed",
    );
    assert!(
        has_permanent(&game, cards::BLACK_LOTUS, PlayerId::Two),
        "the first artifact entered under the target controller's control",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::FOREST],
        "the other revealed card was shuffled back into that library",
    );
}

#[test]
fn genesis_wave_moves_only_the_chosen_eligible_cards_to_the_battlefield() {
    let mut game = ready();
    let wave = spell_in_hand(&mut game, 40_000, cards::GENESIS_WAVE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    // Top-first: land, creature, instant.
    game.players[PlayerId::One.index()].library = vec![
        card(40_102, cards::LIGHTNING_BOLT, PlayerId::One),
        card(40_101, cards::GRIZZLY_BEARS, PlayerId::One),
        card(40_100, cards::FOREST, PlayerId::One),
    ];

    let cast = cast_action(wave, Vec::new(), Vec::new(), 3);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast)
        .expect("Genesis Wave is cast for X=3");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the eligible revealed permanents are offered");
    let choices = decision
        .options
        .iter()
        .filter(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                matches!(
                    characteristics.card_definition(),
                    Some(cards::FOREST | cards::GRIZZLY_BEARS)
                )
            })
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(choices.len(), 2, "both eligible permanents are offered");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: choices,
        },
    )
    .expect("both permanents may be chosen");
    drain_pending(&mut game);

    assert!(has_permanent(&game, cards::FOREST, PlayerId::One));
    assert!(has_permanent(&game, cards::GRIZZLY_BEARS, PlayerId::One,));
    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::LIGHTNING_BOLT)
            .count(),
        1,
        "the unchosen nonpermanent is the revealed remainder",
    );
}

#[test]
fn flesh_allergy_counts_its_sacrifice_and_the_creature_it_destroys() {
    let mut game = ready();
    let allergy = spell_in_hand(&mut game, 45_000, cards::FLESH_ALLERGY);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let payment = creature(45_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let payment_id = payment.card.id;
    let victim = creature(45_200, cards::SERRA_ANGEL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([payment, victim]);

    let cast = cast_action(
        allergy,
        vec![Target::Permanent(victim_id)],
        vec![payment_id],
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast)
        .expect("the creature pays the additional cost");
    assert!(
        !has_permanent(&game, cards::GRIZZLY_BEARS, PlayerId::One),
        "the additional cost is paid while casting",
    );
    drain_pending(&mut game);

    assert!(!has_permanent(&game, cards::SERRA_ANGEL, PlayerId::Two));
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        18,
        "both creatures died before the life-loss amount was read",
    );
}

#[test]
fn nim_deathmantle_returns_the_triggering_card_and_attaches_to_its_successor() {
    let mut game = ready();
    let deathmantle = creature(50_000, cards::NIM_DEATHMANTLE, PlayerId::One);
    let deathmantle_id = deathmantle.card.id;
    let dead = creature(50_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let dead_id = dead.card.id;
    game.battlefield.extend([deathmantle, dead]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    game.move_permanents_to_graveyard(&[dead_id]);
    let decision = (0..12)
        .find_map(|_| {
            if let Some(decision) = game.observe(PlayerId::One).decision {
                return Some(decision);
            }
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("the trigger advances toward its payment");
            None
        })
        .expect("Nim Deathmantle offers its optional payment");
    let pay = decision
        .options
        .iter()
        .find(|option| option.label.starts_with("Pay"))
        .expect("the payment option is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![pay],
        },
    )
    .expect("four mana pays for the return");
    drain_pending(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the triggering card returned");
    assert_ne!(returned.card.id, dead_id, "zone changes create a successor");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == deathmantle_id)
            .expect("the Equipment remains")
            .attached_to,
        Some(returned.card.id),
        "the Equipment follows the returned card's new identity",
    );
    assert_eq!(game.power(returned), Some(4), "the attached +2/+2 applies");
}
