//! Harvest Pyre chooses X from a nonmana casting cost. The fixed `{1}{R}`
//! price must not collapse that choice to zero, and the moved cards must feed
//! the same X that the resolving damage reads.

use super::*;
use crate::card::{CostQuantityDef, SpellAdditionalCostDef};

fn pyre_game(fodder: usize) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let pyre = card(10_000, cards::HARVEST_PYRE, PlayerId::One);
    let pyre_id = pyre.id;
    game.players[PlayerId::One.index()].hand.push(pyre);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    for index in 0..fodder {
        game.players[PlayerId::One.index()].graveyard.push(card(
            20_000 + u32::try_from(index).expect("small fixture"),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }

    let victim = creature(30_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    (game, pyre_id, victim_id)
}

fn pyre_casts(game: &Game, pyre: GameObjectId) -> Vec<(u16, Vec<GameObjectId>)> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } if card == pyre => Some((choices.x(), sacrifices)),
            _ => None,
        })
        .collect()
}

#[test]
fn cost_is_semantically_exile_x_from_the_graveyard() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::HARVEST_PYRE)
        .expect("Harvest Pyre is cataloged");
    let cost = card
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            DeclarativeAbilityDef::Spell(spell) => spell.additional_cost(),
            _ => None,
        })
        .expect("Harvest Pyre declares its additional cost");

    assert_eq!(
        cost,
        SpellAdditionalCostDef::exile_with_quantity(
            ObjectPredicateDef::Any,
            ZoneKind::Graveyard,
            CostQuantityDef::ChosenX,
        ),
    );
}

#[test]
fn fixed_mana_cost_still_offers_every_payable_x() {
    let (game, pyre, _) = pyre_game(3);
    let casts = pyre_casts(&game, pyre);

    for x in 0..=3 {
        assert!(
            casts
                .iter()
                .any(|(chosen, paid)| { *chosen == x && paid.len() == usize::from(x) }),
            "X={x} is offered with exactly that many graveyard cards",
        );
    }
    assert!(
        casts
            .iter()
            .all(|(x, paid)| *x <= 3 && paid.len() == usize::from(*x)),
        "the graveyard bounds X and every action carries its full payment",
    );
}

#[test]
fn exiled_cards_and_damage_share_the_chosen_x() {
    let (mut game, pyre, victim) = pyre_game(3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } if *card == pyre && choices.x() == 3 && sacrifices.len() == 3
            )
        })
        .expect("the graveyard pays for X=3");
    game.apply(PlayerId::One, cast)
        .expect("Harvest Pyre is cast");
    pass_priority_pair(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 3);
    let victim = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == victim)
        .expect("the 4/4 survives three damage");
    assert_eq!(victim.damage, 3);
}
