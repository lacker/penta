//! Semantic spell additional costs and their composed alternatives.

use super::*;
use crate::card::{CostQuantityDef, SpellAbilityDef, SpellAdditionalCostDef};

fn spell_cost(definition: CardDefinitionId) -> SpellAdditionalCostDef {
    let catalog = poc::catalog().expect("catalog builds");
    catalog
        .get(definition)
        .expect("the card is cataloged")
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            DeclarativeAbilityDef::Spell(spell) => spell.additional_cost(),
            _ => None,
        })
        .expect("the spell declares its additional cost")
}

fn escalate_cost(definition: CardDefinitionId) -> SpellAdditionalCostDef {
    let catalog = poc::catalog().expect("catalog builds");
    catalog
        .get(definition)
        .expect("the card is cataloged")
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) => modal.escalate_cost,
            _ => None,
        })
        .expect("the modal spell declares its Escalate cost")
}

fn cast_actions(game: &Game, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

fn targeted_removal_game(
    definition: CardDefinitionId,
    id: u32,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let spell = card(id, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    let victim = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, spell_id, victim)
}

#[test]
fn card_definitions_name_the_game_actions_their_costs_use() {
    assert!(matches!(
        spell_cost(cards::ANNIHILATING_GLARE),
        SpellAdditionalCostDef::Choice([
            SpellAdditionalCostDef::PayMana(_),
            SpellAdditionalCostDef::Sacrifice { .. }
        ])
    ));
    assert!(matches!(
        spell_cost(cards::FINAL_PAYMENT),
        SpellAdditionalCostDef::Choice([
            SpellAdditionalCostDef::PayLife(CostQuantityDef::Fixed(5)),
            SpellAdditionalCostDef::Sacrifice { .. }
        ])
    ));
    assert!(matches!(
        spell_cost(cards::FEED_THE_CYCLE),
        SpellAdditionalCostDef::Choice([
            SpellAdditionalCostDef::Forage,
            SpellAdditionalCostDef::PayMana(_)
        ])
    ));
    assert!(matches!(
        spell_cost(cards::VICIOUS_RIVALRY),
        SpellAdditionalCostDef::PayLife(CostQuantityDef::ChosenX)
    ));
    assert_eq!(
        spell_cost(cards::TOXIC_DELUGE),
        SpellAdditionalCostDef::PayLife(CostQuantityDef::ChosenX)
    );
    assert!(matches!(
        escalate_cost(cards::COLLECTIVE_BRUTALITY),
        SpellAdditionalCostDef::Discard {
            quantity: CostQuantityDef::Fixed(1),
            ..
        }
    ));
    assert!(matches!(
        escalate_cost(cards::COLLECTIVE_EFFORT),
        SpellAdditionalCostDef::Tap {
            quantity: CostQuantityDef::Fixed(1),
            ..
        }
    ));
}

#[test]
fn collective_effort_taps_one_untapped_creature_for_an_extra_mode() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let effort = card(109_000, cards::COLLECTIVE_EFFORT, PlayerId::One);
    let effort_id = effort.id;
    game.players[0].hand.push(effort);
    let payer = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let already_tapped = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.tap_permanent(already_tapped)
        .expect("it can be tapped");
    game.put_onto_battlefield(PlayerId::Two, cards::RURIC_THAR_THE_UNBOWED)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;

    let destroy_large = ModeId::from_index(0).expect("first mode");
    let counters = ModeId::from_index(2).expect("third mode");
    let cast = cast_actions(&game, effort_id)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    choices,
                    sacrifices,
                    ..
                } if choices.modes() == [destroy_large, counters] && sacrifices == &[payer]
            )
        })
        .expect("the fresh untapped creature can pay for one extra mode");
    assert!(
        cast_actions(&game, effort_id).iter().all(|action| {
            !matches!(
                action,
                Action::CastSpell { sacrifices, .. } if sacrifices.contains(&already_tapped)
            )
        }),
        "an already tapped creature is not a payment candidate",
    );

    game.apply(PlayerId::One, cast).expect("the cast is legal");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == payer)
            .expect("the payer remains on the battlefield")
            .tapped,
        "paying the Escalate cost taps the selected creature",
    );
}

fn borrowed_grace_game(white_mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.players[0].hand.clear();
    let grace = card(109_100, cards::BORROWED_GRACE, PlayerId::One);
    let grace_id = grace.id;
    game.players[0].hand.push(grace);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, white_mana);
    (game, grace_id)
}

#[test]
fn mana_escalate_cost_is_charged_once_for_the_extra_mode() {
    let (base_game, grace) = borrowed_grace_game(3);
    assert!(cast_actions(&base_game, grace).iter().all(|action| {
        matches!(
            action,
            Action::CastSpell { choices, .. } if choices.modes().len() == 1
        )
    }));

    let (mut escalated_game, grace) = borrowed_grace_game(5);
    let cast = cast_actions(&escalated_game, grace)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { choices, .. } if choices.modes().len() == 2
            )
        })
        .expect("{2}{W} plus the {1}{W} per-extra-mode cost is payable");
    escalated_game
        .apply(PlayerId::One, cast)
        .expect("the escalated cast is legal");
    assert_eq!(escalated_game.players[0].mana_pool.white, 0);
}

#[test]
fn annihilating_glare_offers_mana_or_a_semantic_sacrifice() {
    let (mut game, glare, victim) = targeted_removal_game(cards::ANNIHILATING_GLARE, 110_000);
    let tracker = game
        .put_onto_battlefield(PlayerId::One, cards::TIRELESS_TRACKER)
        .expect("cataloged");
    game.create_token(PlayerId::One, tokens::clue());
    drain_pending(&mut game);
    let fodder = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::clue()))
        .expect("the Clue exists")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    let actions = cast_actions(&game, glare);
    assert!(actions.iter().any(|action| {
        matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices.is_empty())
    }));
    let sacrifice = actions
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices == &[fodder])
        })
        .expect("sacrificing the artifact is offered beside paying four mana");

    game.apply(PlayerId::One, sacrifice)
        .expect("the cast is legal");
    drain_pending(&mut game);
    let tracker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tracker)
        .expect("the Tracker remains");
    assert_eq!(
        tracker.counters(CounterKind::PlusOnePlusOne),
        1,
        "sacrificing the Clue emits the named sacrifice event",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != victim)
    );

    let (mut mana_game, glare, _victim) = targeted_removal_game(cards::ANNIHILATING_GLARE, 110_100);
    mana_game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    mana_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let mana_cast = cast_actions(&mana_game, glare)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices.is_empty())
        })
        .expect("four mana pays with no permanent available");
    mana_game
        .apply(PlayerId::One, mana_cast)
        .expect("the mana branch validates and casts");
    assert_eq!(mana_game.players[0].mana_pool.black, 0);
    assert_eq!(mana_game.players[0].mana_pool.colorless, 0);
}

fn final_payment_game() -> (Game, GameObjectId, GameObjectId, GameObjectId) {
    let (mut game, payment, victim) = targeted_removal_game(cards::FINAL_PAYMENT, 111_000);
    let fodder = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[0].life = 20;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    (game, payment, victim, fodder)
}

#[test]
fn final_payment_pays_exactly_the_chosen_life_or_sacrifice_cost() {
    let (mut life_game, payment, _victim, fodder) = final_payment_game();
    let life_cast = cast_actions(&life_game, payment)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices.is_empty())
        })
        .expect("five life is one payment");
    life_game
        .apply(PlayerId::One, life_cast)
        .expect("the cast is legal");
    assert_eq!(life_game.players[0].life, 15);
    assert!(
        life_game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fodder),
        "the permanent was not sacrificed too",
    );

    let (mut sacrifice_game, payment, _victim, fodder) = final_payment_game();
    let sacrifice_cast = cast_actions(&sacrifice_game, payment)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices == &[fodder])
        })
        .expect("sacrificing the creature is the other payment");
    sacrifice_game
        .apply(PlayerId::One, sacrifice_cast)
        .expect("the cast is legal");
    assert_eq!(sacrifice_game.players[0].life, 20);
    assert!(
        sacrifice_game
            .battlefield
            .iter()
            .all(|permanent| permanent.card.id != fodder)
    );
}

fn feed_game(graveyard_cards: usize, make_food: bool, extra_black: bool) -> (Game, GameObjectId) {
    let (mut game, feed, _victim) = targeted_removal_game(cards::FEED_THE_CYCLE, 112_000);
    for index in 0..graveyard_cards {
        game.players[0].graveyard.push(card(
            112_100 + u32::try_from(index).expect("small fixture"),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    if make_food {
        game.create_token(PlayerId::One, tokens::food());
        drain_pending(&mut game);
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    if extra_black {
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    }
    (game, feed)
}

#[test]
fn feed_the_cycle_offers_each_way_to_forage_and_the_mana_alternative() {
    let (game, feed) = feed_game(3, true, true);
    let food = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::food()))
        .expect("the Food exists")
        .card
        .id;
    let ways = cast_actions(&game, feed)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { sacrifices, .. } => Some(sacrifices),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(ways.iter().any(Vec::is_empty), "paying black mana");
    assert!(
        ways.iter().any(|objects| objects == &[food]),
        "sacrifice Food"
    );
    assert!(
        ways.iter().any(|objects| objects.len() == 3),
        "exile three graveyard cards",
    );

    let (mut grave_game, feed) = feed_game(3, false, false);
    let exile_cast = cast_actions(&grave_game, feed)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices.len() == 3)
        })
        .expect("three graveyard cards pay the forage");
    grave_game
        .apply(PlayerId::One, exile_cast)
        .expect("the cast is legal");
    assert_eq!(grave_game.players[0].exile.len(), 3);

    let (mut food_game, feed) = feed_game(0, true, false);
    let food_cast = cast_actions(&food_game, feed)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices.len() == 1)
        })
        .expect("the Food pays the forage");
    food_game
        .apply(PlayerId::One, food_cast)
        .expect("the cast is legal");
    assert!(
        food_game
            .battlefield
            .iter()
            .all(|permanent| !is_token_with(permanent, tokens::food()))
    );
}

#[test]
fn vicious_rivalry_uses_the_life_payment_as_its_destroy_bound() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let rivalry = card(113_000, cards::VICIOUS_RIVALRY, PlayerId::One);
    let rivalry_id = rivalry.id;
    game.players[0].hand.push(rivalry);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let pearl = game
        .put_onto_battlefield(PlayerId::One, cards::MOX_PEARL)
        .expect("cataloged");
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[0].life = 20;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;

    let cast = cast_actions(&game, rivalry_id)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { choices, .. } if choices.x() == 2))
        .expect("two life chooses X=2");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, 18);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| { permanent.card.id != bears && permanent.card.id != pearl })
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "the five-mana creature is above X",
    );
}
