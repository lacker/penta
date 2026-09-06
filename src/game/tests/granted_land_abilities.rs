//! Auras that hand an activated ability to what they enchant. The catalog
//! can express the grant without anything ever reaching it, so what needs
//! covering is that the ability is genuinely offered on the host -- on a
//! land, which has no abilities of its own to piggyback on -- and that a
//! granted mana ability is reachable by the planner, which is the only
//! thing that ever activates one.

use super::*;

/// `aura` resolved onto a permanent player one controls: a Mountain for the
/// land Auras, a Grizzly Bears otherwise.
fn enchanted(aura: CardDefinitionId, host: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut permanent = creature(64_000, host, PlayerId::One);
    permanent.entered_controller_turn = 0;
    game.battlefield.push(permanent);

    let spell = card(64_001, aura, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the Aura is castable onto its host");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    let host_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(host))
        .expect("the host is still on the battlefield")
        .card
        .id;
    (game, host_id)
}

/// Whether player one may activate an ability whose source is `source`.
fn can_activate(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::ActivateAbility { source: s, .. } if s == source))
}

#[test]
fn a_plain_land_offers_nothing_to_activate() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut land = creature(64_100, cards::MOUNTAIN, PlayerId::One);
    land.entered_controller_turn = 0;
    let land_id = land.card.id;
    game.battlefield.push(land);
    assert!(
        !can_activate(&game, land_id),
        "a Mountain's mana ability is not an ordinary action"
    );
}

#[test]
fn the_tar_hands_the_land_a_drain() {
    let (mut game, land) = enchanted(cards::CAUSTIC_TAR, cards::MOUNTAIN);
    assert!(
        can_activate(&game, land),
        "the granted ability is offered on the land it enchants"
    );

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == land
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the opponent is a legal target for the drain");
    game.apply(PlayerId::One, activation)
        .expect("tapping the land pays for it");
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 17, "three life gone");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == land && permanent.tapped),
        "and the land is tapped for it"
    );
}

#[test]
fn the_harmony_turns_a_creature_into_a_mana_source() {
    let (mut game, _) = enchanted(cards::MULTANI_S_HARMONY, cards::GRIZZLY_BEARS);
    game.players[0].mana_pool = ManaPool::default();
    let bolt = card(64_200, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == bolt_id)),
        "the planner reaches the granted mana ability to pay {{R}}"
    );
}
