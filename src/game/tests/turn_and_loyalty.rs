use super::*;

/// The top of a library is one place, and everything that reads it has to
/// agree. Drawing took from the end while the shared "top of library" helper
/// took from the front, so effects that looked at the top were quietly
/// reading the bottom of the deck.
#[test]
fn the_top_of_a_library_is_the_same_card_however_it_is_reached() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (19_000, cards::SAVANNAH_LIONS),
            (19_001, cards::LIGHTNING_BOLT),
            (19_002, cards::SERRA_ANGEL),
        ],
    );

    let taken = game.take_top_of_library(PlayerId::One, 2);
    assert_eq!(
        taken.iter().map(|card| card.definition).collect::<Vec<_>>(),
        vec![cards::SAVANNAH_LIONS, cards::LIGHTNING_BOLT],
        "the top cards come back top first"
    );

    // What is left is what a draw sees next.
    let drawn = game.draw_card(PlayerId::One).expect("a card to draw");
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .find(|card| card.id == drawn)
            .map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "and the draw continues from where they were lifted"
    );
    assert!(game.players[0].library.is_empty());
}

#[test]
#[allow(clippy::too_many_lines)]
fn pithing_needle_locks_the_named_card_but_not_its_mana() {
    let mut game = ready_game();
    game.battlefield.clear();
    // Mishra's Factory both animates and taps for mana, so one card shows
    // which half a Needle stops.
    let factory = game
        .put_onto_battlefield(PlayerId::One, cards::MISHRA_S_FACTORY)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let activations = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == factory))
            .count()
    };
    let mana_actions = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == factory))
            .count()
    };
    assert!(activations(&game) > 0, "the Factory starts unlocked");
    assert!(mana_actions(&game) > 0);

    game.put_onto_battlefield(PlayerId::One, cards::PITHING_NEEDLE)
        .expect("cataloged");
    let choice = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the Needle names a card as it enters");
    assert!(
        choice
            .options
            .iter()
            .any(|option| option.label == "Savannah Lions"),
        "the generic card-name list includes legal names without activated abilities",
    );
    assert!(
        choice
            .options
            .iter()
            .any(|option| option.label == "Garruk, the Veil-Cursed"),
        "independently nameable back faces are included",
    );
    assert!(
        !choice
            .options
            .iter()
            .any(|option| matches!(option.label.as_str(), "Dragon" | "Domri Rade emblem")),
        "created-token names and emblems are not legal card-name choices",
    );
    let factory_name = choice
        .options
        .iter()
        .find(|option| option.label == "Mishra's Factory")
        .expect("a card with an activated ability is offered");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![factory_name.id],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    assert_eq!(
        activations(&game),
        0,
        "the named card's activated abilities are locked"
    );
    assert!(
        mana_actions(&game) > 0,
        "but a mana ability is exempt from the lock"
    );

    let mut transformed = ready_game();
    transformed.battlefield.clear();
    let garruk = transformed
        .put_onto_battlefield(PlayerId::One, cards::GARRUK_RELENTLESS)
        .expect("cataloged");
    transformed.transform_permanent(garruk);
    transformed.turn = 2;
    transformed.step = Step::PrecombatMain;
    transformed.priority = PlayerId::One;
    transformed
        .put_onto_battlefield(PlayerId::One, cards::PITHING_NEEDLE)
        .expect("cataloged");
    let choice = transformed.pending_decisions[0].observation.clone();
    let back_face = choice
        .options
        .iter()
        .find(|option| option.label == "Garruk, the Veil-Cursed")
        .expect("the transformed face can be named");
    transformed
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: choice.id,
                options: vec![back_face.id],
            },
        )
        .unwrap();
    drain_pending(&mut transformed);
    assert!(
        !transformed.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == garruk)
        ),
        "naming the presented back face locks its activated abilities",
    );
}

#[test]
fn voice_of_resurgence_makes_a_token_that_counts_the_board() {
    let mut game = ready_game();
    game.battlefield.clear();
    let voice = game
        .put_onto_battlefield(PlayerId::One, cards::VOICE_OF_RESURGENCE)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;

    // An opponent's spell only triggers it during your turn.
    let opponent_casts = |game: &mut Game, active: PlayerId| {
        game.active_player = active;
        game.priority = PlayerId::Two;
        game.players[1].hand = vec![card(21_000, cards::LIGHTNING_BOLT, PlayerId::Two)];
        game.players[1].mana_pool = ManaPool {
            red: 1,
            ..ManaPool::default()
        };
        let cast = game
            .legal_actions(PlayerId::Two)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { .. }))
            .expect("the Bolt is castable");
        game.apply(PlayerId::Two, cast).unwrap();
        let triggered = game
            .stack
            .iter()
            .filter(|object| object.kind == StackObjectKind::TriggeredAbility)
            .count();
        game.pending_triggers.clear();
        game.stack.clear();
        triggered
    };
    assert_eq!(
        opponent_casts(&mut game, PlayerId::Two),
        0,
        "their turn, so nothing triggers"
    );
    assert_eq!(
        opponent_casts(&mut game, PlayerId::One),
        1,
        "your turn, so the Voice speaks"
    );

    game.active_player = PlayerId::One;
    game.destroy_permanent(voice);
    drain_pending(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, voice_elemental_token()))
        .expect("dying made a token")
        .clone();
    // The Voice is gone, so the token is the only creature: a 1/1.
    assert_eq!(game.power(&token), Some(1));
    assert_eq!(game.toughness(&token), Some(1));

    // Every creature added counts, including the token itself.
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let token = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, voice_elemental_token()))
        .expect("the token is still there")
        .clone();
    assert_eq!(game.power(&token), Some(3));
    assert_eq!(game.toughness(&token), Some(3));
}

#[test]
fn turn_strips_a_creature_and_burn_finishes_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    // Serra Angel is a 4/4 with flying and vigilance, so all three of power,
    // abilities, and colour are visible before and after.
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    let before = game.battlefield[0].clone();
    assert_eq!(game.power(&before), Some(4));
    assert!(game.permanent_has_executable_keyword(&before, KeywordAbility::Flying));

    let turn = spell_with_targets(
        22_000,
        cards::TURN_BURN,
        PlayerId::One,
        vec![Target::Permanent(angel)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&TURN_TEST_CHARACTERISTICS),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        }),
        &turn,
        TriggerContext::empty(),
    );

    let turned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel)
        .expect("still on the battlefield")
        .clone();
    assert_eq!(game.power(&turned), Some(0), "base power is replaced");
    assert_eq!(game.toughness(&turned), Some(1));
    assert!(
        !game.permanent_has_executable_keyword(&turned, KeywordAbility::Flying),
        "and the printed abilities are gone"
    );
    let subtypes = game.effective_subtypes(&turned);
    assert_eq!(
        subtypes.as_ref(),
        &["Weird"],
        "the printed creature types gave way rather than being added to"
    );

    // Two damage now finishes a 0/1 that used to be a 4/4.
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == angel)
    {
        permanent.damage = 2;
    }
    game.check_state_based_actions();
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "Burn kills what Turn shrank"
    );
}

static TURN_TEST_CHARACTERISTICS: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Weird"])),
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
];

#[test]
fn flames_of_the_firebrand_splits_its_three_damage() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.players[0].hand = vec![card(23_000, cards::FLAMES_OF_THE_FIREBRAND, PlayerId::One)];
    game.players[0].mana_pool = ManaPool {
        red: 1,
        colorless: 2,
        ..ManaPool::default()
    };
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { .. }))
        .collect::<Vec<_>>();
    // Every split is offered: all three to one target, or two and one, or one
    // each. Piling all three onto one creature is a single target, not three.
    let shares = |action: &Action| {
        let Action::CastSpell { choices, .. } = action else {
            return Vec::new();
        };
        choices
            .targets()
            .iter()
            .flat_map(|selection| selection.amounts().to_vec())
            .collect::<Vec<_>>()
    };
    assert!(
        casts.iter().any(|action| shares(action) == vec![3]),
        "three to a single target"
    );
    assert!(
        casts.iter().any(|action| shares(action) == vec![2, 1]),
        "two and one"
    );
    assert!(
        casts
            .iter()
            .all(|action| shares(action).iter().sum::<u16>() == 3),
        "every split spends exactly three"
    );
    assert!(
        casts
            .iter()
            .all(|action| shares(action).iter().all(|share| *share > 0)),
        "and no target is chosen for nothing"
    );

    // Two to the Lions kills them; one to the Angel does not.
    let split = casts
        .into_iter()
        .find(|action| {
            let Action::CastSpell { choices, .. } = action else {
                return false;
            };
            choices.targets().iter().any(|selection| {
                selection.amount_for(Target::Permanent(lions)) == Some(2)
                    && selection.amount_for(Target::Permanent(angel)) == Some(1)
            })
        })
        .expect("two to the Lions and one to the Angel is a legal split");
    game.apply(PlayerId::One, split).unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions),
        "the Lions took lethal"
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel)
        .expect("the Angel survived");
    assert_eq!(angel.damage, 1, "and took only its share");
}

#[test]
fn moorland_haunt_pays_with_a_creature_card_from_its_own_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    let haunt = game
        .put_onto_battlefield(PlayerId::One, cards::MOORLAND_HAUNT)
        .expect("cataloged");
    game.players[0].mana_pool = ManaPool {
        white: 1,
        blue: 1,
        ..ManaPool::default()
    };
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let token_activations = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| {
                matches!(action, Action::ActivateAbility { source, ability, .. }
                    if *source == haunt
                        && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(1)))
            })
            .collect::<Vec<_>>()
    };
    assert!(
        token_activations(&game).is_empty(),
        "an empty graveyard cannot pay the cost"
    );

    // A noncreature card is not a legal payment either.
    game.players[0]
        .graveyard
        .push(card(24_000, cards::PLAINS, PlayerId::One));
    assert!(token_activations(&game).is_empty(), "and neither is a land");

    game.players[0]
        .graveyard
        .push(card(24_001, cards::SAVANNAH_LIONS, PlayerId::One));
    let activations = token_activations(&game);
    assert_eq!(
        activations.len(),
        1,
        "one activation for the one creature card that can pay"
    );
    game.apply(PlayerId::One, activations[0].clone()).unwrap();
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the creature card paid the cost"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::PLAINS),
        "and the land stayed put"
    );
    assert!(
        game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            token_with_flying(tokens::creature(&["Spirit"], &[ManaColor::White], 1, 1))
        )),
        "and a Spirit arrived"
    );
}

#[test]
fn bonfire_burns_a_player_and_everything_they_control() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    // Your own creature is not theirs, so it is untouched.
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[0].hand = vec![card(25_000, cards::BONFIRE_OF_THE_DAMNED, PlayerId::One)];
    game.players[0].mana_pool = ManaPool {
        red: 1,
        colorless: 6,
        ..ManaPool::default()
    };
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    // A creature is not a legal target; a player is.
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { .. }))
        .collect::<Vec<_>>();
    assert!(
        !casts.iter().any(|action| {
            let Action::CastSpell { choices, .. } = action else {
                return false;
            };
            choices
                .iter_targets()
                .any(|target| *target == Target::Permanent(mine))
        }),
        "a creature is not a player or a planeswalker"
    );

    let three_at_them = casts
        .into_iter()
        .find(|action| {
            let Action::CastSpell { choices, .. } = action else {
                return false;
            };
            choices.x() == 3
                && choices
                    .iter_targets()
                    .any(|target| *target == Target::Player(PlayerId::Two))
        })
        .expect("three damage at the opponent is castable");
    game.apply(PlayerId::One, three_at_them).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 17, "the player took three");
    assert!(
        !game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::Two
                && permanent.card.definition == cards::SAVANNAH_LIONS
        }),
        "their 2/1 died"
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("their 4/4 survived three damage");
    assert_eq!(angel.damage, 3, "but it took the same three");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == mine),
        "your own creature was never in range"
    );
}

#[test]
fn aurelias_fury_taps_what_it_burns_and_locks_who_it_hits() {
    let mut game = ready_game();
    game.battlefield.clear();
    // A 4/4, so one damage leaves it alive to show the tap.
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.players[0].hand = vec![card(27_000, cards::AURELIAS_FURY, PlayerId::One)];
    game.players[0].mana_pool = ManaPool {
        red: 1,
        white: 1,
        colorless: 2,
        ..ManaPool::default()
    };
    // Something for the locked player to try casting afterwards.
    game.players[1].hand = vec![
        card(27_001, cards::LIGHTNING_BOLT, PlayerId::Two),
        // A creature with flash, so the only thing stopping it would be the
        // lock rather than sorcery timing.
        card(27_002, cards::RESTORATION_ANGEL, PlayerId::Two),
        card(27_003, cards::PLAINS, PlayerId::Two),
    ];
    game.players[1].mana_pool = ManaPool {
        red: 1,
        white: 1,
        colorless: 3,
        ..ManaPool::default()
    };
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    // X of two, split one at the player and one at their creature.
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            let Action::CastSpell { choices, .. } = action else {
                return false;
            };
            choices.x() == 2
                && choices.targets().iter().any(|selection| {
                    selection.amount_for(Target::Player(PlayerId::Two)) == Some(1)
                        && selection.amount_for(Target::Permanent(angel)) == Some(1)
                })
        })
        .expect("X of two split between the player and their creature");
    game.apply(PlayerId::One, cast).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 19, "the player took its share");
    let burned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel)
        .expect("a 4/4 survives one damage");
    assert_eq!(burned.damage, 1);
    assert!(burned.tapped, "and every creature it burned is tapped");

    // The burned player keeps their creatures but loses their burn.
    game.priority = PlayerId::Two;
    let casts = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(card),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        !casts.contains(&GameObjectId(27_001)),
        "a noncreature spell is locked out for the rest of the turn"
    );
    assert!(
        casts.contains(&GameObjectId(27_002)),
        "but a creature spell is not"
    );
    let plains = game.players[1]
        .hand
        .iter()
        .find(|card| card.id == GameObjectId(27_003))
        .expect("the land remains in hand");
    let land_option = game
        .catalog
        .get(cards::PLAINS)
        .and_then(|definition| definition.play_options.first())
        .expect("Plains has a land-play option");
    assert!(
        !game.play_is_prohibited(plains, PlayerId::Two, land_option),
        "the cast-only rule leaves land plays untouched"
    );
}

#[test]
fn garruk_turns_over_when_his_own_ability_wounds_him() {
    let mut game = ready_game();
    game.battlefield.clear();
    let garruk = game
        .put_onto_battlefield(PlayerId::One, cards::GARRUK_RELENTLESS)
        .expect("cataloged");
    // A 2/1 hits back for two, taking Garruk from three to one. Its own
    // power is read after it dies, which is what last-known information is
    // for.
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let front = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == garruk)
        .expect("Garruk is there");
    assert_eq!(front.counters(CounterKind::Loyalty), 3);
    assert_eq!(front.presented, CardPartId::PRIMARY, "he starts face up");

    let fight = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == garruk
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(1))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(bear)))
        })
        .expect("the damage ability is offered");
    game.apply(PlayerId::One, fight).unwrap();
    drain_pending(&mut game);

    let turned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == garruk)
        .expect("Garruk survived");
    assert_eq!(
        turned.counters(CounterKind::Loyalty),
        1,
        "the creature hit back for two"
    );
    assert_ne!(
        turned.presented,
        CardPartId::PRIMARY,
        "two or fewer loyalty turned him over"
    );
    assert_eq!(
        game.effective_subtypes(turned).as_ref(),
        &["Garruk"],
        "and the object is the same permanent, now showing its other face"
    );
}

#[test]
fn huntmaster_turns_on_a_quiet_turn_and_back_on_a_busy_one() {
    let mut game = ready_game();
    game.battlefield.clear();
    let huntmaster = game
        .put_onto_battlefield(PlayerId::One, cards::HUNTMASTER_OF_THE_FELLS)
        .expect("cataloged");
    drain_pending(&mut game);
    let front = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == huntmaster)
        .expect("it is there")
        .presented;

    // Entering already made a Wolf and gained two life.
    assert_eq!(game.players[0].life, 22);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2)
            ))
            .count(),
        1
    );

    // A turn with a spell cast keeps it human.
    game.spells_cast_last_turn = [1, 0];
    game.turn = 2;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == huntmaster)
            .expect("it is there")
            .presented,
        front,
        "somebody cast something, so it stays human"
    );

    // A quiet turn turns it over, and transforming makes another Wolf.
    game.spells_cast_last_turn = [0, 0];
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    let back = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == huntmaster)
        .expect("it is there")
        .presented;
    assert_ne!(back, front, "a quiet turn turned it over");
    // Turning into the wolf is not turning into the Huntmaster, so no Wolf
    // and no life. The other face bites the opponent instead.
    assert_eq!(game.players[0].life, 22);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2)
            ))
            .count(),
        1
    );
    assert_eq!(game.players[1].life, 18, "the wolf bit somebody");

    // One player casting twice turns it back, and coming home makes a Wolf.
    game.spells_cast_last_turn = [0, 2];
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == huntmaster)
            .expect("it is there")
            .presented,
        front,
        "two spells from one player turned it back"
    );
    assert_eq!(game.players[0].life, 24);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2)
            ))
            .count(),
        2
    );
}

include!("turn_and_loyalty/domri.rs");
