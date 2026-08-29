//! An X-counted target slot on a spell whose X is paid twice.
//!
//! {X}{X}{U} means each creature reached costs two mana rather than one, so
//! what the mana buys and what the targeting offers have to agree: six mana
//! is islandwalk for two creatures, not five.

use super::*;

/// Part Water in hand with `blue` mana available and `creatures` creatures
/// on the battlefield to point at.
fn holding(creatures: u32, blue: u16) -> (Game, CardInstanceId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    let mut ids = Vec::new();
    for index in 0..creatures {
        let creature = creature(10_000 + index, cards::SEDGE_TROLL, PlayerId::One);
        ids.push(creature.card.id);
        game.battlefield.push(creature);
    }

    let spell = card(20_000, cards::PART_WATER, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = blue;
    game.priority = PlayerId::One;
    (game, spell_id, ids)
}

/// Every cast on offer, as (X, target count) pairs.
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

fn has_islandwalk(game: &Game, id: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    game.permanent_has_executable_keyword(
        permanent,
        KeywordAbility::Landwalk(BasicLandType::Island),
    )
}

#[test]
fn every_offer_targets_exactly_as_many_creatures_as_the_x_paid() {
    let (game, spell, _creatures) = holding(3, 7);
    let shapes = offered_shapes(&game, spell);
    assert!(!shapes.is_empty(), "the spell is castable");
    for (x, count) in shapes {
        assert_eq!(
            usize::from(u8::try_from(x).expect("small X")),
            count,
            "X={x} took {count}",
        );
    }
}

/// The doubled X is the point: with {U} spent on the coloured pip, six blue
/// left buys X=3, not X=6.
#[test]
fn each_point_of_x_costs_two_mana() {
    let (game, spell, _creatures) = holding(6, 7);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 3, "seven mana is {{U}} plus three doublings");
}

/// The control: five mana is {U} plus two doublings, so X stops at two even
/// with more creatures to point at.
#[test]
fn one_mana_short_buys_one_fewer_creature() {
    let (game, spell, _creatures) = holding(6, 5);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 2);
}

/// And the board is still a ceiling of its own.
#[test]
fn an_x_larger_than_the_board_is_not_offered() {
    let (game, spell, _creatures) = holding(2, 9);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 2, "two creatures, whatever the mana");
}

#[test]
fn it_grants_islandwalk_to_exactly_the_creatures_chosen() {
    let (mut game, spell, creatures) = holding(3, 5);
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
        .expect("five blue covers {X=2}{X=2}{U}");
    drain_pending(&mut game);

    assert!(has_islandwalk(&game, creatures[0]));
    assert!(has_islandwalk(&game, creatures[1]));
    assert!(
        !has_islandwalk(&game, creatures[2]),
        "the untargeted creature gained nothing",
    );
}
