//! Ajani, Nacatl Pariah: a Cat that turns into a planeswalker when its
//! friends die.

use super::*;

/// Answers every pending decision with its first option, then resolves
/// whatever is left on the stack.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // "Decline" is offered first; this helper always says yes.
            let options = decision
                .options
                .iter()
                .find(|option| option.label != "Decline")
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

fn ajani_on_battlefield() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let ajani = game
        .put_onto_battlefield(PlayerId::One, cards::AJANI_NACATL_PARIAH)
        .expect("cataloged");
    settle(&mut game);
    drain_pending(&mut game);
    (game, ajani)
}

fn permanent_named<'a>(game: &'a Game, name: &str) -> Option<&'a Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| game.effective_permanent_name(permanent).as_deref() == Some(name))
}

/// Entering makes a Cat Warrior, which is the body the death trigger needs.
#[test]
fn ajani_brings_a_cat_with_him() {
    let (game, _ajani) = ajani_on_battlefield();

    assert!(
        game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1)
        )),
        "the token is there",
    );
}

/// A Cat dying turns him over: he is exiled and comes back on the other
/// face, a planeswalker with its printed loyalty.
#[test]
fn another_cat_dying_returns_ajani_transformed() {
    let (mut game, ajani) = ajani_on_battlefield();
    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1),
            )
        })
        .map(|permanent| permanent.card.id)
        .expect("the token is there");

    game.move_permanents_to_graveyard(&[token]);
    settle(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != ajani),
        "the creature he was is gone",
    );
    let avenger =
        permanent_named(&game, "Ajani, Nacatl Avenger").expect("he came back on the other face");
    assert_eq!(
        avenger.counters(CounterKind::Loyalty),
        3,
        "with the loyalty the back face prints",
    );
    assert_eq!(avenger.controller, PlayerId::One);
}

/// Ajani's own death does not turn him over: the trigger names other Cats.
#[test]
fn ajanis_own_death_does_not_turn_him_over() {
    let (mut game, ajani) = ajani_on_battlefield();

    game.move_permanents_to_graveyard(&[ajani]);
    settle(&mut game);
    drain_pending(&mut game);

    assert!(
        permanent_named(&game, "Ajani, Nacatl Avenger").is_none(),
        "he stayed dead",
    );
}

/// Once he is a planeswalker, +2 grows every Cat -- the token he made and
/// nothing else.
#[test]
fn the_plus_two_grows_every_cat() {
    let (mut game, _ajani) = ajani_on_battlefield();
    game.battlefield
        .push(creature(91_000, cards::GRIZZLY_BEARS, PlayerId::One));
    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1),
            )
        })
        .map(|permanent| permanent.card.id)
        .expect("the token is there");
    let second_cat = token_permanent(
        91_001,
        tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1),
        PlayerId::One,
    );
    let second_cat_id = second_cat.card.id;
    game.battlefield.push(second_cat);

    game.move_permanents_to_graveyard(&[token]);
    settle(&mut game);
    drain_pending(&mut game);
    let avenger = permanent_named(&game, "Ajani, Nacatl Avenger")
        .map(|permanent| permanent.card.id)
        .expect("he turned over");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let ability = activated_ability_for(&game, avenger, 0);
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: avenger,
            ability,
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .expect("the loyalty ability activates");
    settle(&mut game);
    drain_pending(&mut game);

    let cat = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == second_cat_id)
        .expect("the other Cat is there");
    assert_eq!(cat.counters(CounterKind::PlusOnePlusOne), 1);
    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the bear is there");
    assert_eq!(
        bears.counters(CounterKind::PlusOnePlusOne),
        0,
        "a bear is not a Cat",
    );
}

/// Turns Ajani over and hands back the planeswalker he became.
fn avenger(game: &mut Game) -> GameObjectId {
    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1),
            )
        })
        .map(|permanent| permanent.card.id)
        .expect("the token is there");
    game.move_permanents_to_graveyard(&[token]);
    settle(game);
    drain_pending(game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    permanent_named(game, "Ajani, Nacatl Avenger")
        .map(|permanent| permanent.card.id)
        .expect("he turned over")
}

fn activate(game: &mut Game, source: GameObjectId, index: usize, targets: Vec<TargetSelection>) {
    let ability = activated_ability_for(game, source, index);
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability,
            targets,
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .expect("the loyalty ability activates");
    settle(game);
    drain_pending(game);
}

/// The zero makes a Cat, and with a red permanent out it burns for the
/// number of creatures you control -- counted after the token has arrived.
#[test]
fn the_zero_burns_when_you_control_a_red_permanent() {
    let (mut game, _ajani) = ajani_on_battlefield();
    // A Mountain is not a red permanent: lands have no colour. It takes an
    // actually red object to switch the burn on.
    game.battlefield
        .push(creature(91_100, cards::GRAY_OGRE, PlayerId::One));
    let avenger = avenger(&mut game);
    let before = game.players[1].life;

    activate(
        &mut game,
        avenger,
        1,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
    );

    let cats = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1),
            )
        })
        .count();
    assert_eq!(cats, 1, "the zero made a Cat");
    assert_eq!(
        game.players[1].life,
        before - 2,
        "the Ogre and the new Cat, counted after the token arrived",
    );
}

/// Without a red permanent the token still arrives and nothing burns.
#[test]
fn the_zero_makes_a_cat_and_nothing_else_without_red() {
    let (mut game, _ajani) = ajani_on_battlefield();
    let avenger = avenger(&mut game);
    let before = game.players[1].life;

    activate(
        &mut game,
        avenger,
        1,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
    );

    assert!(
        game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            tokens::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1)
        )),
        "the Cat is there either way",
    );
    assert_eq!(game.players[1].life, before, "and nothing burned");
}

/// The ultimate lets the opponent keep one of each named type and takes the
/// rest, leaving their lands alone.
#[test]
fn the_ultimate_uses_simultaneous_choice_then_standard_sacrifice() {
    let game = ready_game();
    let avenger = game
        .catalog
        .get(cards::AJANI_NACATL_PARIAH)
        .expect("Ajani is cataloged")
        .part(CardPartId(1))
        .expect("Ajani's back face is cataloged");
    let EffectDef::SimultaneousChoose(choice) = avenger.rules.ability_clauses()[2]
        .declarative_effect()
        .expect("Ajani's ultimate is declarative")
    else {
        panic!("Ajani's ultimate starts with a simultaneous choice");
    };
    assert_eq!(choice.one_of_each.len(), 4);
    assert_eq!(
        *choice.then,
        EffectDef::Sacrifice {
            object: EffectRecipientDef::objects(ObjectSetDef::Binding(choice.unchosen)),
        }
    );
}

#[test]
fn the_ultimate_spares_one_of_each_type_and_every_land() {
    let (mut game, _ajani) = ajani_on_battlefield();
    for id in 91_200..91_203 {
        game.battlefield
            .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::Two));
    }
    for id in 91_210..91_212 {
        game.battlefield
            .push(creature(id, cards::BLACK_LOTUS, PlayerId::Two));
    }
    game.battlefield
        .push(creature(91_220, cards::MOUNTAIN, PlayerId::Two));
    let avenger = avenger(&mut game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == avenger)
    {
        permanent.set_counters(CounterKind::Loyalty, 9);
    }

    activate(&mut game, avenger, 2, Vec::new());

    let theirs = |definition| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == PlayerId::Two && permanent.card.definition == definition
            })
            .count()
    };
    assert_eq!(theirs(cards::GRIZZLY_BEARS), 1, "one creature survives");
    assert_eq!(theirs(cards::BLACK_LOTUS), 1, "one artifact survives");
    assert_eq!(theirs(cards::MOUNTAIN), 1, "lands are not touched at all");
}
