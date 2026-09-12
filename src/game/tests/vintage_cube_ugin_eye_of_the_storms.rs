//! Ugin, Eye of the Storms: seven mana that answers something as it is
//! cast, again for every colorless spell after it, and pays for the next
//! one himself.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers every pending decision, naming `wanted` where it is offered and
/// otherwise taking the smallest legal answer.
fn settle_naming(game: &mut Game, wanted: Option<GameObjectId>) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = match wanted {
                Some(wanted) => decision
                    .options
                    .iter()
                    .filter(|option| option.card.is_some_and(|(object, _)| object == wanted))
                    .map(|option| option.id)
                    .take(1)
                    .collect(),
                None => decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum)
                    .collect(),
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn ugin(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::UGIN_EYE_OF_THE_STORMS)
        .expect("he is on the battlefield")
}

fn loyalty_action(game: &Game, source: GameObjectId, ability: u8) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: activated,
                ability: AbilityOrigin::Printed { ability: id, .. },
                ..
            } => *activated == source && *id == AbilityId(ability),
            _ => false,
        })
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// Casting him exiles a colored permanent before he even arrives.
#[test]
fn casting_him_exiles_a_colored_permanent() {
    let mut game = staged();
    let bears = creature(150_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let ugin = card(150_001, cards::UGIN_EYE_OF_THE_STORMS, PlayerId::One);
    let ugin_id = ugin.id;
    game.players[0].hand.push(ugin);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 7);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ugin_id))
        .expect("seven mana pays for him");
    game.apply(PlayerId::One, cast).expect("he is cast");
    settle_naming(&mut game, Some(bears_id));

    assert!(
        !on_battlefield(&game, cards::GRIZZLY_BEARS),
        "the cast trigger answered it on the way in",
    );
    assert!(on_battlefield(&game, cards::UGIN_EYE_OF_THE_STORMS));
}

/// A colorless spell cast afterwards does it again; a colored one does not,
/// and a colorless permanent is never a legal target.
#[test]
fn every_colorless_spell_answers_something() {
    let mut game = staged();
    game.put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bears = creature(150_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let ornithopter = card(150_101, cards::ORNITHOPTER, PlayerId::One);
    let ornithopter_id = ornithopter.id;
    game.players[0].hand.push(ornithopter);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ornithopter_id))
        .expect("a free artifact is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle_naming(&mut game, Some(bears_id));

    assert!(
        !on_battlefield(&game, cards::GRIZZLY_BEARS),
        "the colorless spell triggered him",
    );
    assert!(on_battlefield(&game, cards::ORNITHOPTER));
}

/// The plus gains three and draws.
#[test]
fn the_plus_gains_three_and_draws() {
    let mut game = staged();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .expect("cataloged");
    drain_pending(&mut game);
    let card = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].library.push(card);
    game.players[0].life = 20;

    let plus = loyalty_action(&game, source, 2).expect("the plus is activatable");
    game.apply(PlayerId::One, plus).expect("it activates");
    settle_naming(&mut game, None);

    assert_eq!(game.players[0].life, 23);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(ugin(&game).counters(CounterKind::Loyalty), 9);
}

/// The zero uses the stack despite adding mana, and it is still the
/// one loyalty ability he may use this turn.
#[test]
fn the_zero_uses_the_stack_even_though_it_adds_mana() {
    let mut game = staged();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .expect("cataloged");
    drain_pending(&mut game);

    let ability = loyalty_action(&game, source, 3).expect("the loyalty ability is available");
    game.apply(PlayerId::One, ability).expect("it activates");
    assert_eq!(game.stack.len(), 1, "loyalty abilities use the stack");
    assert_eq!(game.players[0].mana.len(), 0);
    settle_naming(&mut game, None);
    assert_eq!(game.players[0].mana.len(), 3);
    assert_eq!(
        ugin(&game).counters(CounterKind::Loyalty),
        7,
        "zero loyalty"
    );
    assert!(
        loyalty_action(&game, source, 2).is_none(),
        "and it was his one loyalty ability for the turn",
    );
}

/// The ultimate empties the library of colorless nonland cards into exile
/// and lets them be cast for free for the rest of the turn.
#[test]
fn the_ultimate_exiles_the_library_and_frees_it() {
    let mut game = staged();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
    {
        permanent.set_counters(CounterKind::Loyalty, 11);
    }
    drain_pending(&mut game);
    let mut thopter = None;
    for definition in [cards::ORNITHOPTER, cards::MOUNTAIN, cards::GRIZZLY_BEARS] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        if definition == cards::ORNITHOPTER {
            thopter = Some(card.id);
        }
        game.players[0].library.push(card);
    }

    let ultimate = loyalty_action(&game, source, 4).expect("eleven loyalty pays for it");
    game.apply(PlayerId::One, ultimate).expect("it activates");
    settle_naming(&mut game, thopter);

    assert_eq!(
        game.players[0]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::ORNITHOPTER],
        "only the colorless nonland card was a legal choice",
    );
    let exiled = game.players[0].exile[0].id;
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == exiled)),
        "and it is castable from exile with no mana at all",
    );
}

/// Ultimates with `library` under Player One and takes `wanted` out of it,
/// returning the exiled card's id.
fn ultimate_taking(
    game: &mut Game,
    library: &[CardDefinitionId],
    wanted: CardDefinitionId,
) -> GameObjectId {
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
    {
        permanent.set_counters(CounterKind::Loyalty, 11);
    }
    drain_pending(game);
    let mut target = None;
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        if *definition == wanted {
            target = Some(card.id);
        }
        game.players[0].library.push(card);
    }
    let ultimate = loyalty_action(game, source, 4).expect("eleven loyalty pays for it");
    game.apply(PlayerId::One, ultimate).expect("it activates");
    settle_naming(game, target);
    // Exiling mints a new object, so what may be cast from there is found by
    // its card rather than by the id it had in the library.
    game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == wanted)
        .expect("the wanted card was exiled")
        .id
}

/// "If a spell has {X} in its mana cost, you must choose 0 as the value of X
/// when casting it without paying its mana cost." A Walking Ballista off the
/// ultimate is a free 0/0 and nothing more, so it dies where it lands.
#[test]
fn a_free_x_spell_is_cast_for_zero() {
    let mut game = staged();
    let ballista = ultimate_taking(
        &mut game,
        &[cards::WALKING_BALLISTA],
        cards::WALKING_BALLISTA,
    );

    let offered: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == ballista => Some(choices.x()),
            _ => None,
        })
        .collect();
    assert_eq!(offered, vec![0], "zero is the only X on offer");

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ballista))
        .expect("it is castable for nothing");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle_naming(&mut game, None);
    game.check_state_based_actions();

    assert!(
        !on_battlefield(&game, cards::WALKING_BALLISTA),
        "an X of zero is a 0/0, and a 0/0 does not stay",
    );
}

/// "You must follow the normal timing permissions and restrictions of each
/// spell you cast using the permission granted by Ugin's last ability." The
/// permission is about the mana, not the timing: an artifact off the
/// ultimate still waits for a main phase with an empty stack.
#[test]
fn the_free_cast_still_obeys_sorcery_timing() {
    let mut game = staged();
    let thopter = ultimate_taking(&mut game, &[cards::ORNITHOPTER], cards::ORNITHOPTER);
    let castable = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == thopter))
    };
    assert!(castable(&game), "your own main phase is its window");

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(!castable(&game), "a combat step is not");

    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::Two;
    assert!(!castable(&game), "and neither is their turn");
}
