//! Nyxborn Hydra reads its own +1/+1 counters twice: as a creature they are
//! its body, and as a bestowed Aura they are what the enchanted creature
//! gets. Both halves come from the X the spell was cast for.

use super::*;

/// Casts the Hydra for `x`, optionally bestowing it onto `onto`.
fn cast(x: u16, onto: Option<GameObjectId>) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.add_unrestricted_mana(
        PlayerId::One,
        ManaColor::Green,
        if onto.is_some() { 2 } else { 1 },
    );
    for _ in 0..x {
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    }
    let hydra = card(25_000, cards::NYXBORN_HYDRA, PlayerId::One);
    let hydra_id = hydra.id;
    game.players[0].hand.push(hydra);

    let targets = onto.map(Target::Permanent).into_iter().collect::<Vec<_>>();
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == hydra_id
                    && choices.x() == x
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .copied()
                        .eq(targets.iter().copied())
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("the Hydra is castable for X={x} onto {onto:?}"));
    game.apply(PlayerId::One, cast).expect("the Hydra is cast");
    pass_priority_pair(&mut game);
    game.check_state_based_actions();
    (game, hydra_id)
}

/// Casting moves the card between zones, so the permanent carries a new
/// object id: everything after a cast has to find it by definition.
fn find(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
}

fn stats(game: &Game, definition: CardDefinitionId) -> Option<(i16, i16)> {
    let permanent = find(game, definition)?;
    Some((game.power(permanent)?, game.toughness(permanent)?))
}

#[test]
fn cast_as_a_creature_it_arrives_with_x_counters() {
    let (game, _) = cast(3, None);
    assert_eq!(
        stats(&game, cards::NYXBORN_HYDRA),
        Some((3, 3)),
        "a 0/0 with three +1/+1 counters"
    );
}

#[test]
fn bestowed_it_hands_its_counters_to_what_it_enchants() {
    let (mut game, _) = {
        let mut staged = ready_game();
        staged.battlefield.clear();
        let bear = creature(25_100, cards::GRIZZLY_BEARS, PlayerId::One);
        let bear_id = bear.card.id;
        staged.battlefield.push(bear);
        staged.players[0].hand.clear();
        staged.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
        for _ in 0..2 {
            staged.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        }
        let hydra = card(25_000, cards::NYXBORN_HYDRA, PlayerId::One);
        let hydra_id = hydra.id;
        staged.players[0].hand.push(hydra);
        let cast = staged
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == hydra_id
                        && choices.x() == 2
                        && choices
                            .targets()
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .copied()
                            .eq([Target::Permanent(bear_id)])
                }
                _ => false,
            })
            .expect("the Hydra can be bestowed onto the Bears for X=2");
        staged
            .apply(PlayerId::One, cast)
            .expect("the Hydra is cast");
        pass_priority_pair(&mut staged);
        staged.check_state_based_actions();
        (staged, hydra_id)
    };
    game.check_state_based_actions();

    assert_eq!(
        stats(&game, cards::GRIZZLY_BEARS),
        Some((4, 4)),
        "2/2 plus the Aura's two counters"
    );
    let aura = find(&game, cards::NYXBORN_HYDRA).expect("the bestowed Hydra is on the battlefield");
    assert_eq!(
        aura.attached_to,
        find(&game, cards::GRIZZLY_BEARS).map(|bear| bear.card.id),
        "bestowed, it is an Aura on the Bears rather than a creature of its own"
    );
}
