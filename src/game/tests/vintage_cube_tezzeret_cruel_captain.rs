//! Tezzeret, Cruel Captain: three colourless that an artifact deck keeps
//! topping up, and a free untap every turn.

use super::*;

/// Tezzeret on the battlefield since last turn, with `board` under Player
/// One beside him.
fn staged(board: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    let tezzeret = game
        .put_onto_battlefield(PlayerId::One, cards::TEZZERET_CRUEL_CAPTAIN)
        .expect("cataloged");
    let mut ids = Vec::new();
    for definition in board {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, tezzeret, ids)
}

fn loyalty(game: &Game, tezzeret: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == tezzeret)
        .map_or(0, |permanent| permanent.counters(CounterKind::Loyalty))
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Activates the loyalty ability whose cost is `change`, aimed at `target`
/// when it takes one.
fn activate(game: &mut Game, tezzeret: GameObjectId, change: i8, target: Option<GameObjectId>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability,
                targets,
                ..
            } => {
                *source == tezzeret
                    && loyalty_cost(game, tezzeret, *ability) == Some(change)
                    && target.is_none_or(|wanted| {
                        targets.iter().any(|selection| {
                            selection.targets().contains(&Target::Permanent(wanted))
                        })
                    })
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("the {change} ability is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(game);
}

/// The loyalty an offered activation would pay, read off the ability itself.
fn loyalty_cost(game: &Game, tezzeret: GameObjectId, ability: AbilityOrigin) -> Option<i8> {
    let permanent = permanent(game, tezzeret);
    game.find_effective_ability(permanent, |effective| effective.origin == ability)
        .and_then(|effective| match effective.ability.definition {
            DeclarativeAbilityDef::Activated(definition) => {
                definition.costs.iter().find_map(|cost| match cost {
                    CostDef::Loyalty(change) => Some(*change),
                    _ => None,
                })
            }
            _ => None,
        })
}

/// Every artifact of yours that lands makes him bigger.
#[test]
fn an_artifact_of_yours_entering_grows_him() {
    let (mut game, tezzeret, _) = staged(&[]);
    assert_eq!(loyalty(&game, tezzeret), 4, "he starts on four");

    game.put_onto_battlefield(PlayerId::One, cards::SOL_RING)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(loyalty(&game, tezzeret), 5, "the Ring counted");

    game.put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(loyalty(&game, tezzeret), 5, "and theirs did not");
}

/// The zero untaps, and grows what it untapped when that is an artifact
/// creature.
#[test]
fn the_zero_untaps_and_grows_an_artifact_creature() {
    let (mut game, tezzeret, ids) = staged(&[cards::ORNITHOPTER]);
    let thopter = ids[0];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == thopter)
        .expect("it is here")
        .tapped = true;
    // Five rather than four: the Thopter is an artifact, so putting it out
    // in front of him already grew him.
    let before = loyalty(&game, tezzeret);

    activate(&mut game, tezzeret, 0, Some(thopter));

    let thopter = permanent(&game, thopter);
    assert!(!thopter.tapped, "untapped");
    assert_eq!(
        thopter.counters(CounterKind::PlusOnePlusOne),
        1,
        "and grown, because it is an artifact creature",
    );
    assert_eq!(loyalty(&game, tezzeret), before, "the zero costs nothing");
}

/// A creature that is not an artifact is untapped and nothing more.
#[test]
fn a_plain_creature_is_only_untapped() {
    let (mut game, tezzeret, ids) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = ids[0];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is here")
        .tapped = true;

    activate(&mut game, tezzeret, 0, Some(bears));

    let bears = permanent(&game, bears);
    assert!(!bears.tapped, "untapped");
    assert_eq!(
        bears.counters(CounterKind::PlusOnePlusOne),
        0,
        "and left alone otherwise",
    );
}

/// The minus three finds a one-mana artifact and nothing dearer.
#[test]
fn the_minus_three_fetches_a_cheap_artifact() {
    let (mut game, tezzeret, _) = staged(&[]);
    for (index, definition) in [cards::SERRA_ANGEL, cards::SOL_RING]
        .into_iter()
        .enumerate()
    {
        game.players[0].library.push(card(
            97_000 + u32::try_from(index).expect("a small library"),
            definition,
            PlayerId::One,
        ));
    }

    activate(&mut game, tezzeret, -3, None);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "the Ring is in hand",
    );
    assert_eq!(loyalty(&game, tezzeret), 1, "and the ability cost three");
}

/// The emblem turns an artifact into a 3/3 Robot at the beginning of
/// combat: the counters go on first, and the body underneath them is 0/0.
#[test]
fn the_emblem_makes_a_robot_out_of_an_artifact() {
    let (mut game, tezzeret, ids) = staged(&[cards::SOL_RING]);
    let ring = ids[0];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tezzeret)
        .expect("he is here")
        .add_counters(CounterKind::Loyalty, 3);

    activate(&mut game, tezzeret, -7, None);
    assert_eq!(game.emblems.len(), 1, "the emblem is there");

    game.step = Step::BeginningOfCombat;
    game.begin_step_triggers();
    drain_pending(&mut game);

    let ring = permanent(&game, ring);
    assert_eq!(ring.counters(CounterKind::PlusOnePlusOne), 3);
    assert_eq!(game.power(ring), Some(3), "a 0/0 with three counters");
    assert_eq!(game.toughness(ring), Some(3));
    assert!(
        game.permanent_types(ring)
            .is_some_and(|types| types.contains(CardType::Creature)),
        "and it is a creature now",
    );
}

/// "The resulting artifact creature will be able to attack if it's been
/// under your control continuously since the turn began. That is, it doesn't
/// matter how long it's been a creature, just how long it's been on the
/// battlefield." The Robot the emblem makes attacks the turn it is made.
#[test]
fn the_robot_attacks_the_turn_it_is_animated() {
    let (mut game, tezzeret, ids) = staged(&[cards::SOL_RING]);
    let ring = ids[0];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tezzeret)
        .expect("he is here")
        .add_counters(CounterKind::Loyalty, 3);
    activate(&mut game, tezzeret, -7, None);

    game.step = Step::BeginningOfCombat;
    game.begin_step_triggers();
    drain_pending(&mut game);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: ring,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the Ring has been here all turn, whatever it has been");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    drain_pending(&mut game);

    assert!(
        permanent(&game, ring).attacking,
        "so the Robot is attacking",
    );
}

/// "If a card in your library has {X} in its mana cost, X is 0 for the
/// purpose of determining its mana value." A Walking Ballista costs {X}{X}
/// and is therefore a one-drop the search may take.
#[test]
fn the_minus_three_counts_an_x_cost_as_zero() {
    let (mut game, tezzeret, _) = staged(&[]);
    for (index, definition) in [cards::SERRA_ANGEL, cards::WALKING_BALLISTA]
        .into_iter()
        .enumerate()
    {
        game.players[0].library.push(card(
            97_100 + u32::try_from(index).expect("a small library"),
            definition,
            PlayerId::One,
        ));
    }

    activate(&mut game, tezzeret, -3, None);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::WALKING_BALLISTA),
        "an {{X}}{{X}} artifact is a mana value of nothing at all",
    );
}

/// "Whenever an artifact *you control* enters": one of theirs landing is
/// nothing to him.
#[test]
fn an_artifact_of_theirs_does_not_grow_him() {
    let (mut game, tezzeret, _) = staged(&[]);
    let before = loyalty(&game, tezzeret);

    game.put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        loyalty(&game, tezzeret),
        before,
        "he counts his own side of the table",
    );
}

/// "The emblem's triggered ability doesn't remove any abilities, types,
/// subtypes, or supertypes the artifact has." A Darksteel Plate made into a
/// Robot is still an indestructible Equipment artifact underneath it.
#[test]
fn the_emblem_adds_a_body_without_taking_anything_away() {
    let (mut game, tezzeret, ids) = staged(&[cards::DARKSTEEL_PLATE]);
    let plate = ids[0];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tezzeret)
        .expect("he is here")
        .add_counters(CounterKind::Loyalty, 3);

    activate(&mut game, tezzeret, -7, None);
    game.step = Step::BeginningOfCombat;
    game.begin_step_triggers();
    drain_pending(&mut game);

    let animated = permanent(&game, plate);
    let types = game.permanent_types(animated).expect("it has types");
    assert!(types.contains(CardType::Creature), "it is a creature now");
    assert!(types.contains(CardType::Artifact), "and still an artifact");
    let subtypes = game.effective_subtypes(animated);
    assert!(
        subtypes.contains(&"Robot"),
        "with the Robot the emblem names: {subtypes:?}",
    );
    assert!(
        subtypes.contains(&"Equipment"),
        "and the Equipment it already was: {subtypes:?}",
    );
    assert!(
        game.permanent_has_executable_keyword(animated, KeywordAbility::Indestructible),
        "and the keyword it printed",
    );
}
