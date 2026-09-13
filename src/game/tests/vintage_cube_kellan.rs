//! Kellan, Planar Trailblazer: a one-mana 2/1 that grows twice, and only in
//! the order the two clauses name.

use super::*;

/// Kellan on the battlefield under Player One, with `mana` red available.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for (index, definition) in [cards::MOUNTAIN, cards::FOREST].into_iter().enumerate() {
        let id = 281_000 + u32::try_from(index).expect("two cards");
        game.players[0]
            .library
            .push(card(id, definition, PlayerId::One));
    }
    let kellan = game
        .put_onto_battlefield(PlayerId::One, cards::KELLAN_PLANAR_TRAILBLAZER)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, mana);
    (game, kellan)
}

fn settle(game: &mut Game) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Activates Kellan's `index`th ability.
fn activate(game: &mut Game, kellan: GameObjectId, index: u8) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                ..
            } => *source == kellan && *ability == AbilityId(index),
            _ => false,
        })
        .unwrap_or_else(|| panic!("ability {index} is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

fn subtypes(game: &Game, kellan: GameObjectId) -> Vec<&'static str> {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kellan)
        .expect("Kellan is there");
    let mut types = game
        .effective_subtypes(permanent)
        .iter()
        .map(crate::card::Subtype::name)
        .collect::<Vec<_>>();
    types.sort_unstable();
    types
}

/// He starts as a 2/1 Scout with nothing else.
#[test]
fn he_starts_as_a_scout() {
    let (game, kellan) = staged(0);

    assert_eq!(subtypes(&game, kellan), vec!["Faerie", "Human", "Scout"]);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kellan)
        .expect("there");
    assert_eq!(game.power(permanent), Some(2));
    assert_eq!(game.toughness(permanent), Some(1));
}

/// The first activation makes him a Detective and hands him the trigger.
#[test]
fn the_first_activation_makes_a_detective() {
    let (mut game, kellan) = staged(2);

    activate(&mut game, kellan, 0);

    assert_eq!(
        subtypes(&game, kellan),
        vec!["Detective", "Faerie", "Human"],
        "the Scout is gone rather than joined",
    );
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kellan)
        .expect("there");
    assert_eq!(game.power(permanent), Some(2), "still a 2/1");
}

/// And the granted trigger works: connecting exiles a card he may play.
#[test]
fn the_detective_exiles_a_card_when_he_connects() {
    let (mut game, kellan) = staged(2);
    activate(&mut game, kellan, 0);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(kellan, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);

    assert_eq!(game.players[1].life, 18, "two damage got through");
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "and the top card is in exile",
    );
    assert_eq!(game.players[0].library.len(), 1);
}

/// The second activation does nothing while he is still a Scout.
#[test]
fn the_second_activation_waits_for_the_first() {
    let (mut game, kellan) = staged(3);

    activate(&mut game, kellan, 1);

    assert_eq!(
        subtypes(&game, kellan),
        vec!["Faerie", "Human", "Scout"],
        "a Scout is not a Detective",
    );
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kellan)
        .expect("there");
    assert_eq!(game.power(permanent), Some(2), "and he is still a 2/1");
}

/// In order, the second makes him a 3/2 Rogue with double strike.
#[test]
fn the_second_activation_makes_a_rogue() {
    let (mut game, kellan) = staged(5);
    activate(&mut game, kellan, 0);

    activate(&mut game, kellan, 1);

    assert_eq!(subtypes(&game, kellan), vec!["Faerie", "Human", "Rogue"]);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kellan)
        .expect("there");
    assert_eq!(game.power(permanent), Some(3));
    assert_eq!(game.toughness(permanent), Some(2));
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::DoubleStrike));
}

/// The Detective's trigger stays with him: a Rogue who was a Detective still
/// exiles a card when he connects.
#[test]
fn the_rogue_keeps_what_the_detective_gained() {
    let (mut game, kellan) = staged(5);
    activate(&mut game, kellan, 0);
    activate(&mut game, kellan, 1);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(kellan, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);

    assert!(
        !game.players[0].exile.is_empty(),
        "the granted trigger came with him",
    );
}
