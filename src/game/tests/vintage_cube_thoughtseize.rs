//! Thoughtseize: one mana, any card in their hand, two of your life.

use super::*;

/// Player One holding a Thoughtseize, Player Two holding `hand`.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1].hand.clear();
    for definition in hand {
        let card = game
            .build_zone(PlayerId::Two, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[1].hand.push(card);
    }
    let seize = game
        .build_zone(PlayerId::One, &[cards::THOUGHTSEIZE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = seize.id;
    game.players[0].hand.push(seize);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.priority = PlayerId::One;
    (game, id)
}

/// Casts it at Player Two, taking the card named by `wanted` when it is
/// offered.
fn seize(game: &mut Game, spell: GameObjectId, wanted: Option<CardDefinitionId>) {
    cast_at_two(game, spell);
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = wanted
                .and_then(|wanted| {
                    decision.options.iter().find(|option| {
                        option.card.is_some_and(|(_, characteristics)| {
                            characteristics.card_definition() == Some(wanted)
                        })
                    })
                })
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the choice accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Casts the Thoughtseize at Player Two and passes until something stops.
fn cast_at_two(game: &mut Game, spell: GameObjectId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("one black mana aimed across the table");
    game.apply(PlayerId::One, cast).expect("it is castable");
}

fn in_graveyard(game: &Game, definition: CardDefinitionId) -> bool {
    game.players[1]
        .graveyard
        .iter()
        .any(|card| card.definition == definition)
}

/// The chooser takes what they want and the owner discards it.
#[test]
fn it_takes_the_card_you_choose() {
    let staged_hand = [cards::GRIZZLY_BEARS, cards::LIGHTNING_BOLT];
    let (mut game, spell) = staged(&staged_hand);

    seize(&mut game, spell, Some(cards::LIGHTNING_BOLT));

    assert!(in_graveyard(&game, cards::LIGHTNING_BOLT), "the one chosen");
    assert!(
        !in_graveyard(&game, cards::GRIZZLY_BEARS),
        "and only the one chosen",
    );
    assert_eq!(game.players[1].hand.len(), 1);
}

/// A land in their hand is not on offer.
#[test]
fn a_land_is_not_a_legal_choice() {
    // Two nonland cards, so the choice is a real one and the offer is a
    // list to look at rather than a foregone conclusion.
    let (mut game, spell) = staged(&[cards::MOUNTAIN, cards::GRIZZLY_BEARS, cards::LIGHTNING_BOLT]);

    cast_at_two(&mut game, spell);
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let offered = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the choice is waiting");

    // Checked directly rather than by taking the first option: a land that
    // was on offer but sorted second would look the same afterwards.
    assert!(
        offered
            .options
            .iter()
            .all(|option| option
                .card
                .is_none_or(|(_, characteristics)| characteristics.card_definition()
                    != Some(cards::MOUNTAIN))),
        "a land is not a legal choice",
    );
    assert_eq!(offered.options.len(), 2, "both nonland cards and no more");
}

/// The life is not conditional: a hand of nothing but lands still costs two.
#[test]
fn the_life_is_paid_even_with_nothing_to_take() {
    let (mut game, spell) = staged(&[cards::MOUNTAIN, cards::MOUNTAIN]);

    seize(&mut game, spell, None);

    assert_eq!(game.players[0].life, 18, "two life either way");
    assert_eq!(game.players[1].hand.len(), 2, "and nothing was taken");
}

/// An empty hand is the same story.
#[test]
fn an_empty_hand_still_costs_two() {
    let (mut game, spell) = staged(&[]);

    seize(&mut game, spell, None);

    assert_eq!(game.players[0].life, 18);
}

/// Taking a card and paying the life happen together.
#[test]
fn it_costs_two_when_it_hits() {
    let (mut game, spell) = staged(&[cards::LIGHTNING_BOLT]);

    seize(&mut game, spell, Some(cards::LIGHTNING_BOLT));

    assert!(in_graveyard(&game, cards::LIGHTNING_BOLT));
    assert_eq!(game.players[0].life, 18);
}
