//! Three more cards paid in a creature's toughness.
//!
//! Death's Caress reads its target after destroying it, so both the subtype
//! it checks and the toughness it pays are last-known. Disciple of
//! Griselbrand and Korozda Guildmage take the sacrifice as their own decision
//! rather than as a cost paid beforehand, because what was sacrificed has to
//! be readable by what follows.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    game
}

/// Answers a pending sacrifice offer by taking its last option.
fn answer(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Casts the named sorcery at its only legal target.
fn cast(game: &mut Game, spell: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the spell has a legal target");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(game);
}

/// Death's Caress with `victim` under the opponent and the spell in hand.
fn caress_board(victim: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, victim, PlayerId::Two));
    let spell = card(10_001, cards::DEATHS_CARESS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    (game, spell_id)
}

/// A Human dies and pays its toughness, read after it has already left.
#[test]
fn deaths_caress_pays_for_a_human() {
    let (mut game, spell) = caress_board(cards::ELITE_INQUISITOR);
    cast(&mut game, spell);

    assert!(game.battlefield.is_empty(), "the Inquisitor was destroyed");
    assert_eq!(game.players[0].life, 22, "a 2/2 Human pays two");
}

/// A creature that is not a Human pays nothing, however big it is.
#[test]
fn deaths_caress_pays_nothing_for_a_wall() {
    let (mut game, spell) = caress_board(cards::WALL_OF_STONE);
    cast(&mut game, spell);

    assert!(game.battlefield.is_empty(), "the Wall was destroyed anyway");
    assert_eq!(game.players[0].life, 20, "but eight toughness buys nothing");
}

/// The Disciple reads the toughness of what it ate.
#[test]
fn the_disciple_gains_the_sacrificed_toughness() {
    let mut game = ready();
    let disciple = creature(10_000, cards::DISCIPLE_OF_GRISELBRAND, PlayerId::One);
    let disciple_id = disciple.card.id;
    game.battlefield.push(disciple);
    game.battlefield
        .push(creature(10_001, cards::WALL_OF_STONE, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == disciple_id),
        )
        .expect("the ability is activatable");
    game.apply(PlayerId::One, action).expect("it is activated");
    answer(&mut game);

    assert_eq!(game.players[0].life, 28, "eight toughness, not zero power");
}

/// The Guildmage makes one Saproling per point of toughness, and will not
/// eat the Saprolings it made.
#[test]
fn the_guildmage_makes_one_saproling_per_toughness() {
    let mut game = ready();
    let mage = creature(10_000, cards::KOROZDA_GUILDMAGE, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    game.battlefield
        .push(creature(10_001, cards::WALL_OF_STONE, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let saprolings = |game: &Game| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                is_token_with(
                    permanent,
                    tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1),
                )
            })
            .count()
    };

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == mage_id && targets.is_empty()
            )
        })
        .expect("the sacrifice ability is activatable");
    game.apply(PlayerId::One, action).expect("it is activated");
    answer(&mut game);

    assert_eq!(saprolings(&game), 8, "a 0/8 is worth eight Saprolings");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::WALL_OF_STONE),
        "and the Wall went",
    );
}

/// Its own Saprolings are never on the menu: with three of them out beside
/// a Wall, the offer names the Wall and the Guildmage and nothing else.
#[test]
fn the_guildmage_will_not_eat_its_own_saprolings() {
    let mut game = ready();
    let mage = creature(10_000, cards::KOROZDA_GUILDMAGE, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    let wall = creature(10_001, cards::WALL_OF_STONE, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    for index in 0..3 {
        game.battlefield.push(token_permanent(
            10_100 + index,
            tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1),
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == mage_id && targets.is_empty()
            )
        })
        .expect("the sacrifice ability is activatable");
    game.apply(PlayerId::One, action).expect("it is activated");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the offer was made");
    let offered = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(card, _)| card))
        .collect::<Vec<_>>();
    assert_eq!(
        offered.len(),
        2,
        "the Wall and the Guildmage, never the three Saprolings",
    );
    assert!(offered.contains(&wall_id) && offered.contains(&mage_id));
}

#[test]
fn every_toughness_payout_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::DEATHS_CARESS,
        cards::DISCIPLE_OF_GRISELBRAND,
        cards::KOROZDA_GUILDMAGE,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
