//! Reading the toughness of what was sacrificed.
//!
//! The follow-up runs after the permanent is gone, so whichever characteristic
//! it wants is last-known either way -- neither is harder to reach than the
//! other, and the card simply has to say which. Power was the only one
//! authored until these asked for the other. Every one is tested against a
//! Wall of Stone, because 0/8 is the only kind of food that can tell the two
//! readings apart.

use super::*;
use crate::ImplementationStatus;

/// The named source under player one, plus a creature to feed it.
fn board(source: CardDefinitionId, food: CardDefinitionId) -> (Game, GameObjectId, i16) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::Upkeep;
    let permanent = creature(10_000, source, PlayerId::One);
    let source_id = permanent.card.id;
    game.battlefield.push(permanent);
    game.battlefield.push(creature(10_001, food, PlayerId::One));
    let life = game.players[PlayerId::One.index()].life;
    (game, source_id, life)
}

/// Activates the source's only ability and answers the sacrifice choice.
fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("the ability is activatable");
    game.apply(PlayerId::One, action)
        .expect("the ability is activated");
    drain_pending(game);
}

/// Sedge Troll is 2/2, so power and toughness agree and the test would pass
/// either way. Wall of Stone is 0/8, where they disagree by eight.
#[test]
fn diamond_valley_pays_the_toughness_not_the_power() {
    let (mut game, valley, life) = board(cards::DIAMOND_VALLEY, cards::WALL_OF_STONE);
    activate(&mut game, valley);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life + 8,
        "eight toughness, not zero power"
    );
}

#[test]
fn life_chisel_reads_it_the_same_way() {
    let (mut game, chisel, life) = board(cards::LIFE_CHISEL, cards::WALL_OF_STONE);
    activate(&mut game, chisel);

    assert_eq!(game.players[PlayerId::One.index()].life, life + 8);
}

/// The creature really is sacrificed, which is what makes the reading
/// last-known.
#[test]
fn the_creature_is_gone_by_the_time_it_is_read() {
    let (mut game, valley, _) = board(cards::DIAMOND_VALLEY, cards::WALL_OF_STONE);
    activate(&mut game, valley);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::WALL_OF_STONE),
        "the Wall was eaten"
    );
}

/// Answers the pending offer, taking the last option to pay and declining
/// with an empty choice.
fn answer(game: &mut Game, pay: bool) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if pay {
                decision
                    .options
                    .last()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
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

/// Devour Flesh pays the player who lost the creature, not the caster.
#[test]
fn devour_flesh_pays_the_player_who_lost_the_creature() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    let wall = creature(10_000, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    let spell = card(10_001, cards::DEVOUR_FLESH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices.targets().iter().any(|selection| {
                        selection.targets().contains(&Target::Player(PlayerId::Two))
                    })
            }
            _ => false,
        })
        .expect("the spell can be aimed at the opponent");
    game.apply(PlayerId::One, cast).expect("it is cast");
    answer(&mut game, true);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == wall_id),
        "the Wall was eaten",
    );
    assert_eq!(game.players[1].life, 28, "eight toughness, eight life");
    assert_eq!(game.players[0].life, 20, "and the caster gains nothing");
}

/// Feed the Pack at the beginning of player one's end step, with `food` to
/// offer it.
fn feed_the_pack_end_step(food: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.battlefield.clear();
    game.battlefield
        .push(creature(10_000, cards::FEED_THE_PACK, PlayerId::One));
    game.battlefield.push(creature(10_001, food, PlayerId::One));
    game.step = Step::End;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game
}

fn wolves(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2),
            )
        })
        .count()
}

/// Eight toughness is eight Wolves, which is the whole reason to build it.
#[test]
fn feed_the_pack_counts_toughness_not_power() {
    let mut game = feed_the_pack_end_step(cards::WALL_OF_STONE);
    answer(&mut game, true);

    assert_eq!(wolves(&game), 8, "a 0/8 makes eight, not zero");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::WALL_OF_STONE),
        "and the Wall went",
    );
}

/// Declining is allowed, and costs nothing.
#[test]
fn feed_the_pack_may_be_declined() {
    let mut game = feed_the_pack_end_step(cards::WALL_OF_STONE);
    answer(&mut game, false);

    assert_eq!(wolves(&game), 0, "no sacrifice, no Wolves");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::WALL_OF_STONE),
        "and the Wall stayed",
    );
}

/// The Wolves it made are tokens, so they can never feed it back.
#[test]
fn feed_the_pack_will_not_eat_its_own_wolves() {
    let mut game = feed_the_pack_end_step(cards::WALL_OF_STONE);
    answer(&mut game, true);
    assert_eq!(wolves(&game), 8, "a pack to choose from");

    game.step = Step::End;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    for _ in 0..8 {
        if !game.pending_decisions.is_empty()
            || (game.stack.is_empty() && game.pending_triggers.is_empty())
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        game.pending_decisions.is_empty(),
        "eight Wolves on the battlefield and nothing it will eat",
    );
    assert_eq!(wolves(&game), 8, "so the pack is unchanged");
}

#[test]
fn every_sacrificed_toughness_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::DIAMOND_VALLEY,
        cards::LIFE_CHISEL,
        cards::DEVOUR_FLESH,
        cards::FEED_THE_PACK,
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
