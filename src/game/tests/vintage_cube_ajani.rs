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
            mana_payment: None,
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
            mana_payment: None,
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
fn the_ultimate_chooses_for_each_player_then_uses_standard_sacrifice() {
    let game = ready_game();
    let avenger = game
        .catalog
        .get(cards::AJANI_NACATL_PARIAH)
        .expect("Ajani is cataloged")
        .part(CardPartId(1))
        .expect("Ajani's back face is cataloged");
    let EffectDef::ChooseForEachPlayer(choice) = avenger.rules.ability_clauses()[2]
        .declarative_effect()
        .expect("Ajani's ultimate is declarative")
    else {
        panic!("Ajani's ultimate starts by choosing for each player");
    };
    let PerPlayerSelectionDef::OneOfEach(selectors) = choice.selection else {
        panic!("Ajani's ultimate chooses one of each permanent type");
    };
    assert_eq!(selectors.len(), 4);
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

/// "A permanent with more than one type may be chosen as any of its types.
/// For example, an artifact creature may be chosen as the artifact, the
/// creature, or both." A board whose only nonland permanent is an artifact
/// creature loses nothing: the Juggernaut answers both headings by itself.
/// And an opponent with none of a type still chooses among the types they
/// do control -- there is no enchantment or planeswalker here to name.
#[test]
fn one_artifact_creature_answers_both_headings() {
    let (mut game, _ajani) = ajani_on_battlefield();
    let juggernaut = creature(91_300, cards::JUGGERNAUT, PlayerId::Two);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);
    game.battlefield
        .push(creature(91_301, cards::MOUNTAIN, PlayerId::Two));
    let avenger = avenger(&mut game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == avenger)
    {
        permanent.set_counters(CounterKind::Loyalty, 9);
    }

    activate(&mut game, avenger, 2, Vec::new());

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == juggernaut_id),
        "the artifact creature is kept under both headings at once",
    );
    assert!(
        game.players[1].graveyard.is_empty(),
        "so nothing was sacrificed at all",
    );
}

/// With more to choose from, one of each heading survives: the Juggernaut
/// covers the artifact and a Bears covers the creature.
#[test]
fn the_ultimate_keeps_one_under_each_heading() {
    let (mut game, _ajani) = ajani_on_battlefield();
    game.battlefield
        .push(creature(91_310, cards::JUGGERNAUT, PlayerId::Two));
    for id in 91_311..91_314 {
        game.battlefield
            .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::Two));
    }
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
    assert_eq!(theirs(cards::JUGGERNAUT), 1, "the artifact they kept");
    assert_eq!(theirs(cards::GRIZZLY_BEARS), 1, "and the creature");
    assert_eq!(
        game.players[1].graveyard.len(),
        2,
        "the other two Bears went",
    );
}

/// Its ruling: "you can activate one of Ajani, Nacatl Avenger's loyalty
/// abilities the turn he enters the battlefield. However, you may do so only
/// during one of your main phases when the stack is empty. For example, if
/// he enters during combat, there will be an opportunity for your opponent
/// to remove him before you can activate one of his abilities."
#[test]
fn the_avenger_waits_for_a_main_phase_with_an_empty_stack() {
    let (mut game, _ajani) = ajani_on_battlefield();
    let avenger = avenger(&mut game);
    let loyalty_abilities = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(
                |action| matches!(action, Action::ActivateAbility { source, .. } if *source == avenger),
            )
            .count()
    };

    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(
        loyalty_abilities(&game) > 0,
        "his own main phase is when they are offered, the turn he arrived or not",
    );

    game.step = Step::DeclareAttackers;
    assert_eq!(
        loyalty_abilities(&game),
        0,
        "and combat is no time for a loyalty ability",
    );

    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::Two;
    assert_eq!(
        loyalty_abilities(&game),
        0,
        "nor is their main phase, however empty the stack",
    );

    game.active_player = PlayerId::One;
    game.players[0]
        .hand
        .push(card(112_900, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let bolt = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(112_900))
        })
        .expect("one red casts it");
    game.apply(PlayerId::One, bolt).expect("it is cast");
    assert_eq!(
        loyalty_abilities(&game),
        0,
        "and a spell on the stack closes the window too",
    );
}

/// "The mana value of a transforming double-faced card is the mana value of
/// its front face, no matter which face is up." The Avenger has no mana cost
/// printed on him at all, and he is worth the Pariah's two.
#[test]
fn the_avenger_is_worth_the_pariahs_mana_value() {
    let (mut game, ajani) = ajani_on_battlefield();
    let pariah_value = game.permanent_mana_value(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == ajani)
            .expect("he is out"),
    );
    assert_eq!(pariah_value, 2, "{{1}}{{W}} is two");

    let avenger = avenger(&mut game);

    assert_eq!(
        game.permanent_mana_value(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == avenger)
                .expect("he turned over"),
        ),
        2,
        "and turning him over does not change what he cost",
    );
}

/// "In every zone other than the battlefield, consider only the
/// characteristics of its front face." A card in the graveyard is the
/// Pariah: a creature card, and no planeswalker at all.
#[test]
fn off_the_battlefield_he_is_the_front_face_only() {
    let (mut game, ajani) = ajani_on_battlefield();
    let buried = game
        .build_zone(PlayerId::One, &[cards::AJANI_NACATL_PARIAH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(buried);
    let card = game.players[0].graveyard.last().expect("he is lying there");

    assert!(
        game.card_object_matches(
            ObjectPredicateDef::HasType(CardType::Creature),
            card,
            ZoneKind::Graveyard,
            ajani,
        ),
        "the front face is a Cat Warrior",
    );
    assert!(
        !game.card_object_matches(
            ObjectPredicateDef::HasType(CardType::Planeswalker),
            card,
            ZoneKind::Graveyard,
            ajani,
        ),
        "and the back face is no part of him there",
    );
}
