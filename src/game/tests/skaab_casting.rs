//! The Innistrad Skaabs whose creature-card exile costs gate every cast.
//! Ruinator adds the distinct permission to pay that same printed cost while
//! the card itself is in its owner's graveyard.

use super::*;
use crate::card::CostQuantityDef;
use crate::card::{CardType, CostDef, DeclarativeAbilityDef, ObjectPredicateDef, ZoneKind};

fn staged_skaab(
    definition: CardDefinitionId,
    in_graveyard: bool,
    creature_cards: usize,
    noncreature_cards: usize,
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let skaab = card(10_000, definition, PlayerId::One);
    let skaab_id = skaab.id;
    if in_graveyard {
        game.players[0].graveyard.push(skaab);
    } else {
        game.players[0].hand.push(skaab);
    }
    for index in 0..creature_cards {
        game.players[0].graveyard.push(card(
            20_000 + u32::try_from(index).expect("few cards"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    for index in 0..noncreature_cards {
        game.players[0].graveyard.push(card(
            30_000 + u32::try_from(index).expect("few cards"),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    match definition {
        cards::SKAAB_GOLIATH => {
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);
        }
        cards::SKAAB_RUINATOR => {
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        }
        _ => unreachable!("the fixture stages only the two Skaabs"),
    }
    (game, skaab_id)
}

fn casts(game: &Game, skaab: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == skaab))
        .collect()
}

fn payments(actions: &[Action]) -> Vec<Vec<GameObjectId>> {
    actions
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { sacrifices, .. } => Some(sacrifices.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn goliath_requires_two_creature_cards_and_offers_each_pair() {
    let (short, goliath) = staged_skaab(cards::SKAAB_GOLIATH, false, 1, 2);
    assert!(
        casts(&short, goliath).is_empty(),
        "noncreature cards cannot complete the cost",
    );

    let (deep, goliath) = staged_skaab(cards::SKAAB_GOLIATH, false, 3, 0);
    let choices = payments(&casts(&deep, goliath));
    assert_eq!(choices.len(), 3, "three creature cards make three pairs");
    assert!(choices.iter().all(|choice| choice.len() == 2));
}

#[test]
fn goliath_exiles_its_payment_and_arrives_with_trample() {
    let (mut game, goliath) = staged_skaab(cards::SKAAB_GOLIATH, false, 2, 1);
    let action = casts(&game, goliath)
        .into_iter()
        .next()
        .expect("two creature cards pay the cost");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(game.players[0].exile.len(), 2, "both creatures were exiled");
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the noncreature card stayed behind",
    );
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SKAAB_GOLIATH)
        .expect("Goliath resolved");
    assert!(game.has_trample(permanent));
}

#[test]
fn ruinator_pays_three_creature_cards_from_hand() {
    let (mut game, ruinator) = staged_skaab(cards::SKAAB_RUINATOR, false, 3, 1);
    let action = casts(&game, ruinator)
        .into_iter()
        .next()
        .expect("the hand cast is offered");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(game.players[0].exile.len(), 3);
    assert_eq!(game.players[0].graveyard.len(), 1);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SKAAB_RUINATOR)
        .expect("Ruinator resolved");
    assert_eq!(game.power(permanent), Some(5));
    assert_eq!(game.toughness(permanent), Some(6));
    assert!(game.has_flying(permanent));
}

#[test]
fn ruinator_casts_itself_from_the_graveyard_but_cannot_exile_itself() {
    let (short, ruinator) = staged_skaab(cards::SKAAB_RUINATOR, true, 2, 1);
    assert!(
        casts(&short, ruinator).is_empty(),
        "the Ruinator itself is not one of the three creature-card payments",
    );

    let (mut game, ruinator) = staged_skaab(cards::SKAAB_RUINATOR, true, 3, 1);
    let action = casts(&game, ruinator)
        .into_iter()
        .next()
        .expect("its own graveyard permission offers the printed cast");
    let Action::CastSpell { sacrifices, .. } = &action else {
        unreachable!("the helper returned only casts");
    };
    assert_eq!(sacrifices.len(), 3);
    assert!(!sacrifices.contains(&ruinator));

    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);
    assert_eq!(game.players[0].exile.len(), 3);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SKAAB_RUINATOR),
        "Ruinator arrived from the graveyard",
    );
}

#[test]
fn both_skaabs_explicitly_exile_their_graveyard_costs() {
    let catalog = poc::catalog().expect("catalog builds");
    for (definition, count) in [(cards::SKAAB_GOLIATH, 2), (cards::SKAAB_RUINATOR, 3)] {
        let card = catalog.get(definition).expect("the card is cataloged");
        let cost = card
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.additional_cost(),
                _ => None,
            })
            .expect("the Skaab spell declares its creature-card cost");
        assert_eq!(
            cost,
            CostDef::exile(
                ObjectPredicateDef::HasType(CardType::Creature),
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(count),
            ),
            "{} should explicitly exile exactly {count} creature cards",
            card.name,
        );
    }
}
