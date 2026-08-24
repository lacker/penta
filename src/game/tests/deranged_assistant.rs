//! Deranged Assistant under the post-Hobbit mana-ability rule.
//!
//! Moving a card from a library as either the cost or effect now keeps an
//! activated ability from being a mana ability. The mill is still a cost, so
//! it is paid before the ordinary activated ability goes on the stack.

use super::*;
use crate::ImplementationStatus;

fn activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source: actual,
                    ..
                } if *actual == source
            )
        })
}

#[test]
fn mill_cost_requires_a_card_and_is_paid_before_resolution() {
    let mut game = ready_game();
    let assistant = creature(10_000, cards::DERANGED_ASSISTANT, PlayerId::One);
    let assistant_id = assistant.card.id;
    game.battlefield.push(assistant);
    game.players[PlayerId::One.index()].library.clear();

    assert!(
        activation(&game, assistant_id).is_none(),
        "the mill cost cannot be paid from an empty library",
    );

    game.players[PlayerId::One.index()]
        .library
        .push(card(10_001, cards::FOREST, PlayerId::One));
    assert!(
        !game.can_pay_cost(PlayerId::One, mana_cost!("{1}"), 0),
        "the mana planner cannot activate this nonmana ability while paying a cost",
    );
    let action = activation(&game, assistant_id).expect("one library card pays the mill cost");
    game.apply(PlayerId::One, action)
        .expect("the ordinary activated ability activates");

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        0,
        "the mana waits for the ability to resolve",
    );
    assert_eq!(game.stack.len(), 1, "the ability uses the stack");
    assert!(game.players[PlayerId::One.index()].library.is_empty());
    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::FOREST],
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == assistant_id)
            .expect("the Assistant remains on the battlefield")
            .tapped,
    );

    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 1);
}

#[test]
fn definition_is_complete_and_not_a_mana_ability() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::DERANGED_ASSISTANT)
        .expect("Deranged Assistant is cataloged");
    assert_eq!(
        card.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
    assert!(matches!(
        card.rules.ability_clauses()[0].definition,
        DeclarativeAbilityDef::Activated(_),
    ));
}
