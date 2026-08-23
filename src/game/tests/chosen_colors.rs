//! Naming a colour as the effect resolves.
//!
//! The colour cannot be fixed in the declaration -- what to name depends on
//! what is on the stack -- so the effect settles who receives it, asks, and
//! applies the answer. Everything downstream is ordinary: the protection is
//! the same keyword a printed "protection from red" grants, and the repaint
//! is the same colour-setting leaf an animation uses.

use super::*;
use crate::ImplementationStatus;

/// The five colours in the order the decision offers them.
const OFFERED: [ManaColor; 5] = [
    ManaColor::White,
    ManaColor::Blue,
    ManaColor::Black,
    ManaColor::Red,
    ManaColor::Green,
];

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

/// Passes priority until a decision appears, then takes the colour at
/// `index` in the offered order.
fn name_color(game: &mut Game, index: usize) {
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
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
        .expect("a colour was asked for");
    assert_eq!(decision.prompt, "Choose a color");
    assert_eq!(decision.options.len(), 5, "five colours, never colourless");
    let option = decision.options[index].id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the decision accepts what it offered");
    drain_pending(game);
}

fn protected_from(game: &Game, id: GameObjectId, color: ManaColor) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .is_some_and(|permanent| {
            game.permanent_has_executable_keyword(permanent, protection_keyword(color))
        })
}

/// The Aristocrat eats a creature and names a colour; only that colour is
/// protected against.
#[test]
fn the_aristocrat_gains_protection_from_the_named_colour() {
    let mut game = ready();
    let mage = creature(10_000, cards::CARTEL_ARISTOCRAT, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    game.battlefield
        .push(creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == mage_id),
        )
        .expect("there is another creature to eat");
    game.apply(PlayerId::One, action).expect("it is activated");
    // Index 3 is red.
    name_color(&mut game, 3);

    assert!(protected_from(&game, mage_id, ManaColor::Red), "red named");
    for color in OFFERED {
        if color != ManaColor::Red {
            assert!(
                !protected_from(&game, mage_id, color),
                "and only red: {color:?} was not named",
            );
        }
    }
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS),
        "the bear paid for it",
    );
}

/// Brave the Elements reaches every white creature you control and nothing
/// else -- not your green one, and not the opponent's white one.
#[test]
fn brave_the_elements_covers_only_your_white_creatures() {
    let mut game = ready();
    let mine = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let green = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let green_id = green.card.id;
    game.battlefield.push(green);
    let theirs = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let spell = card(10_003, cards::BRAVE_THE_ELEMENTS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("it is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    // Index 2 is black.
    name_color(&mut game, 2);

    assert!(
        protected_from(&game, mine_id, ManaColor::Black),
        "your white creature is covered",
    );
    assert!(
        !protected_from(&game, green_id, ManaColor::Black),
        "your green one is not white",
    );
    assert!(
        !protected_from(&game, theirs_id, ManaColor::Black),
        "and the opponent's white one is not yours",
    );
}

/// The Protector's entry trigger aims at one creature you control.
#[test]
fn the_protector_covers_the_creature_it_named() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.put_onto_battlefield(PlayerId::One, cards::MIDVAST_PROTECTOR)
        .expect("cataloged");
    // The trigger picks its target first; the bear is the only creature the
    // Protector did not arrive as.
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let targeting = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the trigger asks for a target");
    let option = targeting
        .options
        .iter()
        .find(|option| option.card.map(|(card, _)| card) == Some(bear_id))
        .expect("the bear is offered")
        .id;
    game.apply(
        targeting.player,
        Action::ChooseDecision {
            decision: targeting.id,
            options: vec![option],
        },
    )
    .expect("the target is chosen");
    // Index 1 is blue.
    name_color(&mut game, 1);

    let covered = OFFERED
        .into_iter()
        .filter(|color| protected_from(&game, bear_id, *color))
        .collect::<Vec<_>>();
    assert_eq!(covered, vec![ManaColor::Blue], "one colour, the named one");
}

/// The Tomb repaints outright: the permanent becomes the chosen colour and
/// stops being whatever it was.
#[test]
fn the_tomb_repaints_a_permanent() {
    let mut game = ready();
    let tomb = creature(10_000, cards::ALCHORS_TOMB, PlayerId::One);
    let tomb_id = tomb.card.id;
    game.battlefield.push(tomb);
    let bear = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let colors = |game: &Game| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == bear_id)
            .expect("still there");
        game.permanent_colors(permanent)
    };
    let green = ManaColor::Green.color_index().expect("green is a colour");
    let red = ManaColor::Red.color_index().expect("red is a colour");
    assert!(colors(&game)[green], "a bear starts green");

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == tomb_id
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bear_id)))
            }
            _ => false,
        })
        .expect("the bear can be named");
    game.apply(PlayerId::One, action).expect("it is activated");
    // Index 3 is red.
    name_color(&mut game, 3);

    assert!(colors(&game)[red], "and ends red");
    assert!(!colors(&game)[green], "instead of green, not as well as");
}

#[test]
fn every_colour_choice_card_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::CARTEL_ARISTOCRAT,
        cards::MIDVAST_PROTECTOR,
        cards::BRAVE_THE_ELEMENTS,
        cards::ALCHORS_TOMB,
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
