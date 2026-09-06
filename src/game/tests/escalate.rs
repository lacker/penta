//! Escalate's shared modal shape and card-specific sequencing.

use super::*;
use crate::card::{CostDef, SpellAbilityDef};

fn modal(definition: CardDefinitionId) -> crate::card::ModalSpellDef {
    let catalog = poc::catalog().expect("catalog builds");
    catalog
        .get(definition)
        .expect("the card is cataloged")
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal))
                if modal.escalate_cost.is_some() =>
            {
                Some(modal)
            }
            _ => None,
        })
        .expect("the card has an Escalate modal spell")
}

#[test]
fn every_printed_escalate_card_uses_the_first_class_modal_shape() {
    let cards = [
        cards::BLESSED_ALLIANCE,
        cards::BORROWED_GRACE,
        cards::COLLECTIVE_EFFORT,
        cards::BORROWED_MALEVOLENCE,
        cards::COLLECTIVE_BRUTALITY,
        cards::BORROWED_HOSTILITY,
        cards::COLLECTIVE_DEFIANCE,
        cards::SAVAGE_ALLIANCE,
        cards::COLLECTIVE_RESISTANCE,
    ];
    let mut mana = 0;
    let mut discard = 0;
    let mut tap = 0;
    for definition in cards {
        let modal = modal(definition);
        assert_eq!(modal.minimum, 1);
        assert_eq!(usize::from(modal.maximum), modal.modes.len());
        assert!(!modal.may_repeat);
        match modal.escalate_cost.expect("checked above") {
            CostDef::Mana(_) => mana += 1,
            CostDef::Discard { .. } => discard += 1,
            CostDef::Tap { .. } => tap += 1,
            other => panic!("unexpected printed Escalate cost: {other:?}"),
        }
    }
    assert_eq!((mana, discard, tap), (7, 1, 1));
}

#[test]
fn collective_defiance_draws_exactly_the_number_discarded() {
    let mut game = ready_game();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let defiance = card(109_200, cards::COLLECTIVE_DEFIANCE, PlayerId::One);
    let defiance_id = defiance.id;
    game.players[0].hand.push(defiance);
    let discarded = [
        card(109_201, cards::MOUNTAIN, PlayerId::Two),
        card(109_202, cards::LIGHTNING_BOLT, PlayerId::Two),
        card(109_203, cards::GRIZZLY_BEARS, PlayerId::Two),
    ];
    let discarded_definitions = discarded
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    game.players[1].hand.extend(discarded);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);

    let hand_mode = ModeId::from_index(0).expect("first mode");
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == defiance_id
                        && choices.modes() == [hand_mode]
                        && choices.targets().iter().any(|slot| {
                            slot.targets() == [Target::Player(PlayerId::Two)]
                        })
            )
        })
        .expect("the hand-replacement mode can target the opponent");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        3,
        "three discarded, then three drawn"
    );
    assert!(discarded_definitions.iter().all(|definition| {
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == *definition)
    }));
}
