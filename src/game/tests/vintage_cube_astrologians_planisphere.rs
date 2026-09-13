//! Astrologian's Planisphere: a two-mana Equipment that brings its own body
//! and grows it on the turns a blue deck was having anyway.

use super::*;

/// The Planisphere on the battlefield with its Hero, plus `hand` in hand and
/// five mana up.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..8 {
        game.players[0]
            .library
            .push(card(97_000 + index, cards::ISLAND, PlayerId::One));
    }
    let planisphere = game
        .put_onto_battlefield(PlayerId::One, cards::ASTROLOGIAN_S_PLANISPHERE)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);
    for definition in hand {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].hand.push(card);
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for color in [ManaColor::Blue, ManaColor::Red, ManaColor::Green] {
        game.add_unrestricted_mana(PlayerId::One, color, 3);
    }
    (game, planisphere)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
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

/// The Hero the Job select clause made.
fn hero(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the Hero is on the battlefield")
}

fn counters_on_hero(game: &Game) -> u16 {
    hero(game).counters(CounterKind::PlusOnePlusOne)
}

fn cast(game: &mut Game, card: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card))
        .expect("there is mana for it");
    game.apply(PlayerId::One, action).expect("it is cast");
    settle(game);
}

/// Job select: a 1/1 Hero arrives and the Equipment goes straight onto it.
#[test]
fn it_arrives_wearing_its_own_hero() {
    let (game, planisphere) = staged(&[]);

    let hero = hero(&game);
    assert_eq!(game.power(hero), Some(1));
    assert_eq!(game.toughness(hero), Some(1));
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == planisphere)
            .expect("the Equipment is there")
            .attached_to,
        Some(hero.card.id),
        "attached as it entered",
    );
}

/// "A Wizard in addition to its other types": the Hero keeps being a Hero.
#[test]
fn the_hero_becomes_a_wizard_as_well() {
    let (game, _planisphere) = staged(&[]);
    let subtypes = game.effective_subtypes(hero(&game));

    assert!(
        subtypes.contains(crate::card::Subtype::named("Hero")),
        "still a Hero"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Wizard")),
        "and a Wizard now"
    );
}

/// A noncreature spell puts a counter on the creature wearing it.
#[test]
fn a_noncreature_spell_grows_it() {
    let (mut game, _planisphere) = staged(&[cards::LIGHTNING_BOLT]);
    let bolt = game.players[0].hand[0].id;
    assert_eq!(counters_on_hero(&game), 0);

    cast(&mut game, bolt);

    assert_eq!(counters_on_hero(&game), 1, "one spell, one counter");
    assert_eq!(game.power(hero(&game)), Some(2), "a 2/2 now");
}

/// A creature spell is not a noncreature spell.
#[test]
fn a_creature_spell_does_not() {
    let (mut game, _planisphere) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = game.players[0].hand[0].id;

    cast(&mut game, bears);

    assert_eq!(counters_on_hero(&game), 0, "the clause says noncreature");
}

/// The third card of the turn, and only the third.
#[test]
fn the_third_draw_grows_it_once() {
    let (mut game, _planisphere) = staged(&[]);

    game.draw_card(PlayerId::One);
    game.draw_card(PlayerId::One);
    settle(&mut game);
    assert_eq!(counters_on_hero(&game), 0, "two is not three");

    game.draw_card(PlayerId::One);
    settle(&mut game);
    assert_eq!(counters_on_hero(&game), 1, "the third one does it");

    game.draw_card(PlayerId::One);
    settle(&mut game);
    assert_eq!(counters_on_hero(&game), 1, "and the fourth does not");
}

/// The two halves are one ability and count separately: a spell and a third
/// draw in the same turn are two counters.
#[test]
fn both_halves_fire_in_one_turn() {
    let (mut game, _planisphere) = staged(&[cards::LIGHTNING_BOLT]);
    let bolt = game.players[0].hand[0].id;

    cast(&mut game, bolt);
    for _ in 0..3 {
        game.draw_card(PlayerId::One);
    }
    settle(&mut game);

    assert_eq!(counters_on_hero(&game), 2, "one each");
}

/// Equip moves the whole clause: the new wearer grows and the Hero stops.
#[test]
fn equipping_someone_else_moves_the_ability() {
    let (mut game, planisphere) = staged(&[cards::LIGHTNING_BOLT]);
    let bolt = game.players[0].hand[0].id;
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    let equip =
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    *source == planisphere
                        && targets.iter().any(|selection| {
                            selection.targets().iter().any(
                                |target| matches!(target, Target::Permanent(id) if *id == bears),
                            )
                        })
                }
                _ => false,
            })
            .expect("two mana equips it to the Bear");
    game.apply(PlayerId::One, equip).expect("it activates");
    settle(&mut game);

    cast(&mut game, bolt);

    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("the Bear is there");
    assert_eq!(bear.counters(CounterKind::PlusOnePlusOne), 1, "it grew");
    assert!(
        game.effective_subtypes(bear)
            .contains(crate::card::Subtype::named("Wizard")),
        "and it is a Wizard while it wears it",
    );
    assert_eq!(counters_on_hero(&game), 0, "the Hero kept nothing");
}

/// "If the Hero token is destroyed, the Equipment stays on the battlefield."
#[test]
fn losing_the_hero_leaves_the_equipment() {
    let (mut game, planisphere) = staged(&[]);
    let hero = hero(&game).card.id;

    game.move_permanents_to_graveyard(&[hero]);
    settle(&mut game);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == planisphere),
        "the Equipment is still there, wearing nobody",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == planisphere)
            .expect("it is there")
            .attached_to
            .is_none(),
        "and attached to nothing",
    );
}

/// "It doesn't need to have been attached when the first or second card is
/// drawn. As long as a creature you control has the granted ability when you
/// draw your third card, that ability will trigger."
#[test]
fn the_first_two_draws_need_not_have_been_watched() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..8 {
        game.players[0]
            .library
            .push(card(97_500 + index, cards::ISLAND, PlayerId::One));
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.cards_drawn_this_turn = [0; 2];

    // Two cards drawn with nothing watching.
    game.draw_card(PlayerId::One);
    game.draw_card(PlayerId::One);
    game.put_onto_battlefield(PlayerId::One, cards::ASTROLOGIAN_S_PLANISPHERE)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);
    assert_eq!(counters_on_hero(&game), 0, "it arrived to nothing");

    game.draw_card(PlayerId::One);
    settle(&mut game);

    assert_eq!(
        counters_on_hero(&game),
        1,
        "the third card of the turn is the third card of the turn",
    );
}

/// "The Hero token enters as a 1/1 creature, then the Equipment becomes
/// attached to it. Abilities that trigger when a creature enters the
/// battlefield see that a 1/1 creature entered." A Midnight Guard beside it
/// is woken by the Hero.
#[test]
fn the_hero_arriving_is_a_creature_entering() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let guard = game
        .put_onto_battlefield(PlayerId::One, cards::MIDNIGHT_GUARD)
        .expect("cataloged");
    drain_pending(&mut game);
    game.tap_permanent(guard);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == guard)
            .is_some_and(|permanent| permanent.tapped),
        "the Guard is asleep",
    );

    game.put_onto_battlefield(PlayerId::One, cards::ASTROLOGIAN_S_PLANISPHERE)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);

    let hero = hero(&game);
    assert_eq!(
        (game.power(hero), game.toughness(hero)),
        (Some(1), Some(1)),
        "a 1/1 arrived",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == guard)
            .is_some_and(|permanent| !permanent.tapped),
        "and the Guard saw a creature enter",
    );
}

/// The clause is the equipped creature's, so an Equipment wearing nobody
/// grows nothing: the spells go by and the counters have nowhere to land.
#[test]
fn an_equipment_wearing_nobody_grows_nothing() {
    let (mut game, planisphere) = staged(&[cards::LIGHTNING_BOLT]);
    let hero = hero(&game).card.id;
    game.move_permanents_to_graveyard(&[hero]);
    settle(&mut game);
    game.check_state_based_actions();
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let bolt = game.players[0]
        .hand
        .iter()
        .find(|card| card.definition == cards::LIGHTNING_BOLT)
        .expect("the Bolt is in hand")
        .id;
    cast(&mut game, bolt);
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == planisphere),
        "the Equipment is still there",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == bears)
            .expect("the bear is there")
            .counters(CounterKind::PlusOnePlusOne),
        0,
        "and a creature it is not attached to is no business of its clause",
    );
}

/// "Whenever *you* cast a noncreature spell": the clause belongs to the
/// creature's controller, so their Bolt is not one of yours.
#[test]
fn their_noncreature_spell_grows_nothing() {
    let (mut game, _planisphere) = staged(&[]);
    let bolt = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::One))
            }
            _ => false,
        })
        .expect("they can aim a Bolt at your face");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        17,
        "their spell resolved",
    );
    assert_eq!(
        counters_on_hero(&game),
        0,
        "and grew nothing of yours doing it",
    );
}

/// "Whenever *you* draw your third card each turn" reads the same way: their
/// draws are counted on their own side of the table.
#[test]
fn their_third_draw_grows_nothing() {
    let (mut game, _planisphere) = staged(&[]);
    for _ in 0..4 {
        game.draw_card(PlayerId::Two);
    }
    settle(&mut game);

    assert_eq!(
        counters_on_hero(&game),
        0,
        "four of their cards is none of yours",
    );

    for _ in 0..3 {
        game.draw_card(PlayerId::One);
    }
    settle(&mut game);
    assert_eq!(
        counters_on_hero(&game),
        1,
        "and your own third is still your own third, counted separately",
    );
}

/// Equip is an ordinary equip ability under the flavour name, and carries
/// the ordinary two restrictions with it: "target creature you control", and
/// only when you could cast a sorcery. Their bear is never a legal wearer,
/// and on their turn nobody is.
#[test]
fn equip_names_only_your_own_creatures_and_only_on_your_turn() {
    let (mut game, planisphere) = staged(&[]);
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);

    let equips_at = |game: &Game, wanted: GameObjectId| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    source == planisphere
                        && targets.iter().any(|selection| {
                            selection.targets().iter().any(
                                |target| matches!(target, Target::Permanent(id) if *id == wanted),
                            )
                        })
                }
                _ => false,
            })
    };

    assert!(equips_at(&game, mine), "your own bear may wear it");
    assert!(
        !equips_at(&game, theirs),
        "and their Angel never may, however much mana is up",
    );

    // Their turn, same board and the same five mana.
    game.active_player = PlayerId::Two;
    game.turns_started = [5, 6];
    game.priority = PlayerId::One;

    assert!(
        !equips_at(&game, mine),
        "equip waits for a turn of yours, like the sorcery it is timed as",
    );
}
