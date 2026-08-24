//! Bloodline Keeper's live Vampire count, transform, anthem, and shared token ability.

use super::*;
use crate::ImplementationStatus;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let keeper = game
        .put_onto_battlefield(PlayerId::One, cards::BLOODLINE_KEEPER)
        .expect("Bloodline Keeper is cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] += 1;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    (game, keeper)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn ability_text(game: &Game, source: GameObjectId, origin: AbilityOrigin) -> &'static str {
    let mut text = "";
    let _ = game.visit_effective_abilities(permanent(game, source), |effective| {
        if effective.origin == origin {
            text = effective.ability.text;
            return std::ops::ControlFlow::Break(());
        }
        std::ops::ControlFlow::Continue(())
    });
    text
}

fn activation(game: &Game, source: GameObjectId, prefix: &str) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                ability,
                ..
            } => *actual == source && ability_text(game, source, *ability).starts_with(prefix),
            _ => false,
        })
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("passing priority is legal");
    }
    game.check_state_based_actions();
}

fn vampire_token(id: u32, controller: PlayerId) -> Permanent {
    token_permanent(
        id,
        tokens::creature(&["Vampire"], &[ManaColor::Black], 2, 2),
        controller,
    )
}

#[test]
fn five_controlled_vampires_gate_the_transform_and_the_back_face_pumps_only_others() {
    let (mut game, keeper) = staged();
    for id in 10_100..10_103 {
        game.battlefield.push(vampire_token(id, PlayerId::One));
    }
    game.battlefield.push(vampire_token(10_200, PlayerId::Two));

    assert!(
        activation(&game, keeper, "{B}: Transform").is_none(),
        "three of yours, one of theirs, and the Keeper itself are only four controlled Vampires",
    );

    let fourth_other = vampire_token(10_103, PlayerId::One);
    let fourth_other_id = fourth_other.card.id;
    game.battlefield.push(fourth_other);
    let action = activation(&game, keeper, "{B}: Transform")
        .expect("the Keeper plus four other controlled Vampires satisfy the restriction");
    game.apply(PlayerId::One, action)
        .expect("the transform activation is legal");
    settle(&mut game);

    assert_eq!(permanent(&game, keeper).presented, CardPartId(1));
    assert_eq!(
        (
            game.power(permanent(&game, keeper)),
            game.toughness(permanent(&game, keeper))
        ),
        (Some(5), Some(5)),
        "Lord of Lineage does not pump itself",
    );
    assert_eq!(
        (
            game.power(permanent(&game, fourth_other_id)),
            game.toughness(permanent(&game, fourth_other_id)),
        ),
        (Some(4), Some(4)),
        "another controlled Vampire gets +2/+2",
    );
    assert_eq!(
        (
            game.power(permanent(&game, GameObjectId(10_200))),
            game.toughness(permanent(&game, GameObjectId(10_200))),
        ),
        (Some(2), Some(2)),
        "an opponent's Vampire is not affected",
    );
}

#[test]
fn the_shared_tap_ability_creates_a_black_two_two_flying_vampire() {
    let (mut game, keeper) = staged();
    let action = activation(&game, keeper, "{T}: Create")
        .expect("a Keeper that has been controlled since the turn began may tap");
    game.apply(PlayerId::One, action)
        .expect("the token activation is legal");
    settle(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the activation creates a token");
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(2), Some(2))
    );
    assert!(game.effective_subtypes(token).contains(&"Vampire"));
    assert_eq!(
        game.effective_colors(token, &game.effective_rules(token).unwrap()),
        [false, false, true, false, false],
    );
    assert!(game.permanent_has_executable_keyword(token, KeywordAbility::Flying));
}

#[test]
fn both_faces_are_flying_and_the_definition_is_complete() {
    let (mut game, keeper) = staged();
    assert!(
        game.permanent_has_executable_keyword(permanent(&game, keeper), KeywordAbility::Flying)
    );
    game.transform_permanent(keeper);
    assert!(
        game.permanent_has_executable_keyword(permanent(&game, keeper), KeywordAbility::Flying)
    );
    assert_eq!(
        game.catalog
            .get(cards::BLOODLINE_KEEPER)
            .expect("Bloodline Keeper is cataloged")
            .rules
            .implementation_status(),
        ImplementationStatus::Complete,
    );
}
