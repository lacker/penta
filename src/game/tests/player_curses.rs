//! Player-enchanting Auras and the Innistrad block Curses built on them.

use super::*;
use crate::ImplementationStatus;

fn attached_curse(id: u32, definition: CardDefinitionId, player: PlayerId) -> Permanent {
    let mut curse = creature(id, definition, PlayerId::One);
    curse.attached_player = Some(player);
    curse
}

fn take_turn(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn an_enchant_player_spell_targets_and_attaches_to_that_player() {
    let mut game = ready_game();
    let curse = card(10_000, cards::CURSE_OF_THE_BLOODY_TOME, PlayerId::One);
    let curse_id = curse.id;
    game.players[PlayerId::One.index()].hand.push(curse);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == curse_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the Curse can target the opponent");
    game.apply(PlayerId::One, action)
        .expect("the Curse is cast");
    drain_pending(&mut game);

    let curse = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::CURSE_OF_THE_BLOODY_TOME)
        .expect("the Curse resolved");
    assert_eq!(curse.attached_player, Some(PlayerId::Two));
    assert_eq!(curse.attached_to, None);
    assert_eq!(curse.chosen_player, None);
}

#[test]
fn bloody_tome_mills_only_on_the_enchanted_players_upkeep() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_THE_BLOODY_TOME,
        PlayerId::Two,
    ));
    let before = game.players[PlayerId::Two.index()].library.len();

    take_turn(&mut game, PlayerId::One);
    assert_eq!(game.players[PlayerId::Two.index()].library.len(), before);

    take_turn(&mut game, PlayerId::Two);
    assert_eq!(
        game.players[PlayerId::Two.index()].library.len(),
        before - 2
    );
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
}

#[test]
fn deaths_hold_affects_only_creatures_the_enchanted_player_controls() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_DEATH_S_HOLD,
        PlayerId::Two,
    ));
    game.battlefield
        .push(creature(10_001, cards::SEDGE_TROLL, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SEDGE_TROLL, PlayerId::Two));

    let stats = |game: &Game, id| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the creature is present");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(stats(&game, GameObjectId(10_001)), (Some(2), Some(2)));
    assert_eq!(stats(&game, GameObjectId(10_002)), (Some(1), Some(1)));
}

#[test]
fn nightly_hunt_grants_the_attack_requirement_only_to_the_enchanted_player() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_THE_NIGHTLY_HUNT,
        PlayerId::Two,
    ));
    game.battlefield
        .push(creature(10_001, cards::SEDGE_TROLL, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SEDGE_TROLL, PlayerId::Two));

    let has_requirement = |game: &Game, id| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the creature is present");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::AttacksEachCombatIfAble)
    };
    assert!(!has_requirement(&game, GameObjectId(10_001)));
    assert!(has_requirement(&game, GameObjectId(10_002)));
}

#[test]
fn pierced_heart_damages_the_enchanted_player_on_their_upkeep() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_THE_PIERCED_HEART,
        PlayerId::Two,
    ));
    let before = game.players[PlayerId::Two.index()].life;

    take_turn(&mut game, PlayerId::Two);

    assert_eq!(game.players[PlayerId::Two.index()].life, before - 1);
}

#[test]
fn oblivion_exiles_two_cards_from_the_enchanted_players_graveyard() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_OBLIVION,
        PlayerId::Two,
    ));
    for id in 10_001..10_004 {
        game.players[PlayerId::Two.index()].graveyard.push(card(
            id,
            cards::MOUNTAIN,
            PlayerId::Two,
        ));
    }

    take_turn(&mut game, PlayerId::Two);
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 1);
    assert_eq!(game.players[PlayerId::Two.index()].exile.len(), 2);
}

#[test]
fn stalked_prey_rewards_the_creature_that_hit_the_enchanted_player() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_STALKED_PREY,
        PlayerId::Two,
    ));
    let attacker = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    game.damage_target_from_kind(
        Some(attacker_id),
        Some(Target::Player(PlayerId::Two)),
        1,
        true,
    );
    drain_pending(&mut game);

    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("the attacker remains");
    assert_eq!(attacker.counters(CounterKind::PlusOnePlusOne), 1);
}

#[test]
fn exhaustion_allows_one_spell_and_prohibits_the_second() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_EXHAUSTION,
        PlayerId::Two,
    ));
    let first = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let first_id = first.id;
    let second = card(10_002, cards::LIGHTNING_BOLT, PlayerId::Two);
    let second_id = second.id;
    game.players[PlayerId::Two.index()]
        .hand
        .extend([first, second]);
    game.players[PlayerId::Two.index()].mana_pool.red = 2;
    game.priority = PlayerId::Two;

    let first_cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == first_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::One))
            }
            _ => false,
        })
        .expect("the first spell is legal");
    game.apply(PlayerId::Two, first_cast)
        .expect("the first is cast");
    drain_pending(&mut game);
    game.priority = PlayerId::Two;

    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == second_id))
    );
}

#[test]
fn thirst_counts_every_curse_attached_to_the_same_player() {
    let mut game = ready_game();
    game.battlefield.push(attached_curse(
        10_000,
        cards::CURSE_OF_THIRST,
        PlayerId::Two,
    ));
    game.battlefield.push(attached_curse(
        10_001,
        cards::CURSE_OF_THE_BLOODY_TOME,
        PlayerId::Two,
    ));
    let before = game.players[PlayerId::Two.index()].life;

    take_turn(&mut game, PlayerId::Two);

    assert_eq!(game.players[PlayerId::Two.index()].life, before - 2);
}

#[test]
fn curse_coverage_is_complete_or_explicitly_partial() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::CURSE_OF_THE_BLOODY_TOME,
        cards::CURSE_OF_DEATH_S_HOLD,
        cards::CURSE_OF_OBLIVION,
        cards::CURSE_OF_STALKED_PREY,
        cards::CURSE_OF_THE_NIGHTLY_HUNT,
        cards::CURSE_OF_THE_PIERCED_HEART,
        cards::CURSE_OF_EXHAUSTION,
        cards::CURSE_OF_THIRST,
    ] {
        assert_eq!(
            catalog
                .get(definition)
                .expect("the Curse is cataloged")
                .rules
                .implementation_status(),
            ImplementationStatus::Complete,
        );
    }
    for definition in [
        cards::CURSE_OF_ECHOES,
        cards::CURSE_OF_MISFORTUNES,
        cards::CURSE_OF_BLOODLETTING,
    ] {
        assert_eq!(
            catalog
                .get(definition)
                .expect("the Curse is cataloged")
                .rules
                .implementation_status(),
            ImplementationStatus::Partial,
        );
    }
}
