//! Cast facts that remain relevant after a permanent spell resolves.

use super::*;

#[test]
fn a_spell_copy_keeps_choices_and_payment_objects_but_not_actual_payment_facts() {
    let original = CastContext {
        source_zone: Some(CastSourceZone::Graveyard),
        alternative: Some(AlternativeCastKindDef::Escape),
        at_instant_speed: true,
        x: 3,
        repeatable_additional_costs: 2,
        additional_costs: vec![1, 0, 2],
        colors_of_mana_spent: ColorSet::empty()
            .with(ManaColor::Blue)
            .with(ManaColor::Black),
        phyrexian_symbols_paid_with_life: 1,
        exiled_payment_cards: vec![GameObjectId(41), GameObjectId(42)],
        via_flashback: true,
        via_suspend: true,
    };

    let copied = original.for_spell_copy();

    assert_eq!(copied.source_zone, None);
    assert_eq!(copied.alternative, original.alternative);
    assert_eq!(copied.x, 3);
    assert_eq!(copied.repeatable_additional_costs, 2);
    assert_eq!(copied.additional_costs, [1, 0, 2]);
    assert_eq!(copied.exiled_payment_cards, original.exiled_payment_cards);
    assert_eq!(copied.colors_spent_count(), 0);
    assert_eq!(copied.phyrexian_symbols_paid_with_life, 0);
    assert!(!copied.at_instant_speed);
    assert!(!copied.via_flashback);
    assert!(!copied.via_suspend);
}

fn staged_delve(
    spell: CardDefinitionId,
    graveyard: &[CardDefinitionId],
    colored_mana: ManaColor,
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let spell = game
        .build_zone(PlayerId::One, &[spell])
        .expect("the delve spell is cataloged")
        .into_iter()
        .next()
        .expect("one spell");
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for definition in graveyard {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("the payment card is cataloged")
            .into_iter()
            .next()
            .expect("one payment card");
        game.players[0].graveyard.push(card);
    }
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, colored_mana, 2);
    (game, spell_id)
}

fn cast_delve_spell(game: &mut Game, spell: GameObjectId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("colored mana and the graveyard pay the delve spell");
    game.apply(PlayerId::One, cast)
        .expect("the delve cast applies");
}

#[test]
fn murktide_counts_the_instant_and_sorcery_cards_used_for_delve() {
    let (mut game, murktide) = staged_delve(
        cards::MURKTIDE_REGENT,
        &[
            cards::LIGHTNING_BOLT,
            cards::DEMONIC_TUTOR,
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
        ],
        ManaColor::Blue,
    );

    cast_delve_spell(&mut game, murktide);
    let cast = game.stack.last().unwrap().cast.as_ref().unwrap();
    assert_eq!(cast.exiled_payment_cards.len(), 5);
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MURKTIDE_REGENT)
        .expect("Murktide resolved");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 2);

    let bolt = card(230_010, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].graveyard.push(bolt);
    let _ = game.exile_graveyard_cards(PlayerId::One, &[bolt_id]);
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MURKTIDE_REGENT)
        .expect("Murktide remains");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 3);
}

#[test]
fn soulflayer_reads_printed_keywords_from_its_delve_payment() {
    let (mut game, soulflayer) = staged_delve(
        cards::SOULFLAYER,
        &[
            cards::VOICELESS_SPIRIT,
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
        ],
        ManaColor::Black,
    );

    cast_delve_spell(&mut game, soulflayer);
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SOULFLAYER)
        .expect("Soulflayer resolved");
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::FirstStrike));
    assert!(!game.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance));
}

#[test]
fn ethereal_forager_can_return_the_instant_used_for_delve() {
    let (mut game, forager) = staged_delve(
        cards::ETHEREAL_FORAGER,
        &[
            cards::LIGHTNING_BOLT,
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
        ],
        ManaColor::Blue,
    );

    cast_delve_spell(&mut game, forager);
    drain_pending(&mut game);
    let forager = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.definition == cards::ETHEREAL_FORAGER)
        .expect("Forager resolved");
    forager.entered_controller_turn = 0;
    let forager = forager.card.id;
    game.turns_started = [2, 1];
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: forager,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("Forager attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("attack declaration finishes");
    while game.pending_decisions.is_empty() {
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the attack trigger advances to its optional choice");
    }
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .expect("the optional return is accepted");
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the linked instant returned to its owner's hand",
    );
}

#[test]
fn engineered_explosives_keeps_the_colors_spent_through_entry() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let explosives = game
        .build_zone(PlayerId::One, &[cards::ENGINEERED_EXPLOSIVES])
        .expect("Explosives is cataloged")
        .into_iter()
        .next()
        .unwrap();
    let explosives_id = explosives.id;
    game.players[0].hand.push(explosives);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == explosives_id && choices.x() == 3)
        })
        .expect("three colors can pay X=3");
    game.apply(PlayerId::One, cast).expect("Explosives is cast");
    assert_eq!(game.stack.last().unwrap().colors_spent_count(), 3);
    drain_pending(&mut game);

    let explosives = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ENGINEERED_EXPLOSIVES)
        .expect("Explosives resolved");
    assert_eq!(explosives.counters(CounterKind::named("charge")), 3);
    assert_eq!(
        explosives
            .cast
            .as_ref()
            .map_or(0, CastContext::colors_spent_count),
        3,
    );
    let explosives = explosives.card.id;
    let crucible = game
        .put_onto_battlefield(PlayerId::One, cards::CRUCIBLE_OF_WORLDS)
        .expect("Crucible is cataloged");
    let mountain = game
        .put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("Mountain is cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == explosives)
        })
        .expect("two mana and the sacrifice activate Explosives");
    game.apply(PlayerId::One, activate)
        .expect("Explosives activates");
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != crucible),
        "the nonland permanent with mana value three was destroyed",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == mountain),
        "the land was not destroyed",
    );
}
