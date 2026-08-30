//! Through the Breach: a creature put onto the battlefield for one turn.

use super::*;

/// Answers every pending decision with the last option it offered, then
/// resolves whatever is left on the stack.
fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Casts the Breach from hand with the mana already available.
fn cast_breach(game: &mut Game, breach: CardInstanceId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == breach))
        .expect("five mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(game);
    drain_pending(game);
}

fn breach_with(hand: &[CardDefinitionId]) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for (offset, definition) in hand.iter().enumerate() {
        let id = 97_100 + u32::try_from(offset).expect("a short test hand");
        game.players[0]
            .hand
            .push(card(id, *definition, PlayerId::One));
    }
    let breach = card(97_000, cards::THROUGH_THE_BREACH, PlayerId::One);
    let breach_id = breach.id;
    game.players[0].hand.push(breach);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    (game, breach_id)
}

/// The whole point: a creature that never had to be cast, on the battlefield
/// and able to attack the turn it arrives.
#[test]
fn the_breach_puts_a_creature_down_hasty_without_casting_it() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);

    cast_breach(&mut game, breach_id);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("the creature is on the battlefield");
    assert!(
        game.permanent_has_executable_keyword(angel, KeywordAbility::Haste),
        "carrying haste, which is what makes the turn worth anything",
    );
    assert!(
        game.stack.is_empty(),
        "it was put onto the battlefield rather than cast",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .all(|card| card.definition != cards::SERRA_ANGEL),
        "and it left the hand",
    );
}

/// The post-move effect waits for a prospective entry to finish. Meddling
/// Mage pauses to choose a card name as it enters; haste must follow that
/// decision and attach to the permanent identity the entry finally minted.
#[test]
fn the_post_move_effect_waits_for_an_as_enters_choice() {
    let (mut game, breach_id) = breach_with(&[cards::MEDDLING_MAGE]);

    cast_breach(&mut game, breach_id);

    let mage = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MEDDLING_MAGE)
        .expect("the Mage finished entering");
    assert!(
        game.permanent_has_executable_keyword(mage, KeywordAbility::Haste),
        "the successor received haste after the as-enters decision",
    );
}

/// The rent comes due at the next end step, and the creature is sacrificed
/// by a clause it carries rather than by anything that still names it.
#[test]
fn the_creature_sacrifices_itself_at_the_next_end_step() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);
    cast_breach(&mut game, breach_id);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "it is there to begin with",
    );

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    settle(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::SERRA_ANGEL),
        "and gone by the end step",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "sacrificed rather than exiled",
    );
}

/// A hand with nothing to put down is not offered a choice at all, and the
/// spell still resolves.
#[test]
fn a_hand_without_a_creature_is_never_asked() {
    let (mut game, breach_id) = breach_with(&[cards::LIGHTNING_BOLT]);

    cast_breach(&mut game, breach_id);

    assert!(game.pending_decisions.is_empty(), "nothing to choose from");
    assert!(game.battlefield.is_empty(), "and nothing arrived");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::THROUGH_THE_BREACH),
        "the spell still resolved and was spent",
    );
}

/// The offer names creature cards, not the whole hand.
#[test]
fn only_creature_cards_are_offered() {
    let (mut game, breach_id) = breach_with(&[cards::LIGHTNING_BOLT, cards::SERRA_ANGEL]);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == breach_id))
        .expect("five mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);

    let decision = game.pending_decisions.first().expect("a choice is offered");
    let offered = decision
        .observation
        .options
        .iter()
        .map(|option| option.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(offered, vec!["Serra Angel"]);
}

/// Splice onto Arcane: a second Breach revealed from hand and paid for adds
/// its clause to the one being cast, so two creatures arrive.
#[test]
fn a_spliced_breach_puts_a_second_creature_down() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    let spliced = card(97_500, cards::THROUGH_THE_BREACH, PlayerId::One);
    let spliced_id = spliced.id;
    game.players[0].hand.push(spliced);
    // {4}{R} for the cast and {2}{R}{R} for the splice.
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == breach_id && choices.spliced() == [spliced_id]
            }
            _ => false,
        })
        .expect("the splice is on offer");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::SERRA_ANGEL
                || permanent.card.definition == cards::GRIZZLY_BEARS)
            .count(),
        2,
        "one creature from the spell and one from the clause spliced onto it",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.id == spliced_id),
        "and the spliced card stayed in hand",
    );
    assert_eq!(game.players[0].mana_pool.total(), 0, "both costs were paid");
}

/// Without the mana for the splice cost, only the plain cast is offered.
#[test]
fn the_splice_cost_is_owed() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);
    let spliced = card(97_600, cards::THROUGH_THE_BREACH, PlayerId::One);
    let spliced_id = spliced.id;
    game.players[0].hand.push(spliced);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == breach_id && choices.spliced() == [spliced_id])
        }),
        "five mana pays for the cast and not for the splice",
    );
    assert!(
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == breach_id && choices.spliced().is_empty())
        }),
        "but the plain cast is still there",
    );
}

/// A card with no splice clause is not a legal thing to splice.
#[test]
fn a_card_without_splice_cannot_be_spliced() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);
    let angel = game.players[0]
        .hand
        .iter()
        .find(|card| card.definition == cards::SERRA_ANGEL)
        .expect("the Angel is in hand")
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == breach_id && choices.spliced() == [angel])
        }),
        "splice is a clause the card has to print",
    );
}

/// "Putting the card onto the battlefield is optional. When the ability
/// resolves, you can choose not to."
#[test]
fn the_creature_may_be_left_in_hand() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == breach_id))
        .expect("five mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("a choice is offered");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("taking nothing is an answer");
    drain_pending(&mut game);

    assert!(game.battlefield.is_empty(), "nothing was put down");
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "and the Angel is still in hand",
    );
}

/// "A card with a splice ability can't be spliced onto itself": the Breach
/// being cast is on the stack rather than in hand, so with only that copy
/// there is nothing to reveal.
#[test]
fn the_breach_cannot_splice_itself() {
    let (mut game, breach_id) = breach_with(&[cards::SERRA_ANGEL]);
    // The splice cost as well as the printed one, so nothing but the rules
    // is standing in the way.
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let offers = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == breach_id))
        .count();
    assert_eq!(
        offers, 1,
        "one plain cast and no spliced one: it is its own only copy",
    );
}
