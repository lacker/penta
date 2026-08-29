//! "Tap X target creatures" on a spell rather than an activated ability.
//!
//! The X-linked target count already had to be right for Candelabra of
//! Tawnos; this is the same declaration reached through the casting path, so
//! what these check is that the spell offers exactly X targets and refuses an
//! X the board cannot fill.

use super::*;

/// Word of Binding in hand with `mana` black available, and `creatures`
/// untapped creatures on the other side.
fn holding(creatures: u32, mana: u16) -> (Game, CardInstanceId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    let mut ids = Vec::new();
    for index in 0..creatures {
        let creature = creature(10_000 + index, cards::SEDGE_TROLL, PlayerId::Two);
        ids.push(creature.card.id);
        game.battlefield.push(creature);
    }

    let spell = card(20_000, cards::WORD_OF_BINDING, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = mana;
    game.priority = PlayerId::One;
    (game, spell_id, ids)
}

/// Every cast of the spell on offer, as (X, target count) pairs.
fn offered_shapes(game: &Game, spell: CardInstanceId) -> Vec<(u16, usize)> {
    let mut shapes = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => Some((
                choices.x(),
                choices
                    .targets()
                    .iter()
                    .map(|slot| slot.targets().len())
                    .sum(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    shapes.sort_unstable();
    shapes.dedup();
    shapes
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

#[test]
fn every_offer_targets_exactly_as_many_creatures_as_the_x_paid() {
    let (game, spell, _creatures) = holding(3, 5);
    let shapes = offered_shapes(&game, spell);
    assert!(!shapes.is_empty(), "the spell is castable");
    for (x, count) in shapes {
        assert_eq!(
            usize::from(u8::try_from(x).expect("small X")),
            count,
            "X={x} took {count}"
        );
    }
}

/// Three creatures, so X=3 is the ceiling however much black mana is spare.
#[test]
fn an_x_larger_than_the_board_is_not_offered() {
    let (game, spell, _creatures) = holding(3, 8);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 3, "the creatures are the ceiling, not the mana");
}

/// The control: {X}{B}{B} with two black leaves nothing for X, so only the
/// empty cast is on offer.
#[test]
fn without_spare_mana_only_an_x_of_zero_is_offered() {
    let (game, spell, _creatures) = holding(3, 2);
    assert_eq!(offered_shapes(&game, spell), vec![(0, 0)]);
}

#[test]
fn it_taps_exactly_the_creatures_chosen() {
    let (mut game, spell, creatures) = holding(3, 4);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.x() == 2
                    && choices.targets().iter().any(|slot| slot.targets()
                        == [Target::Permanent(creatures[0]), Target::Permanent(creatures[1])]))
        })
        .expect("two of the three creatures is a legal choice");
    game.apply(PlayerId::One, action)
        .expect("four black covers {X=2}{B}{B}");
    drain_pending(&mut game);

    assert!(tapped(&game, creatures[0]));
    assert!(tapped(&game, creatures[1]));
    assert!(
        !tapped(&game, creatures[2]),
        "the untargeted creature stayed up",
    );
}
