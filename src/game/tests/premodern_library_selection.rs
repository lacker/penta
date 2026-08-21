use super::*;

fn cast_library_spell(game: &mut Game, definition: CardDefinitionId, mana: ManaPool) {
    let spell = card(14_000, definition, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool = mana;
    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(game);
}

#[test]
fn impulse_offers_the_top_four_and_bottoms_the_rest() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (14_100, cards::SAVANNAH_LIONS),
            (14_101, cards::LIGHTNING_BOLT),
            (14_102, cards::SERRA_ANGEL),
            (14_103, cards::JUZAM_DJINN),
            (14_104, cards::SWORDS_TO_PLOWSHARES),
        ],
    );

    cast_library_spell(
        &mut game,
        cards::IMPULSE,
        ManaPool {
            blue: 1,
            colorless: 1,
            ..ManaPool::default()
        },
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    let offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![
            cards::SAVANNAH_LIONS,
            cards::LIGHTNING_BOLT,
            cards::SERRA_ANGEL,
            cards::JUZAM_DJINN,
        ]
    );

    choose_decision_by_label(&mut game, PlayerId::One, "Lightning Bolt");

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT)
    );
    assert_eq!(
        game.players[0].library.last().unwrap().definition,
        cards::SWORDS_TO_PLOWSHARES
    );
    let mut bottomed = game.players[0].library[..3]
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    bottomed.sort_unstable();
    let mut expected = vec![
        cards::SAVANNAH_LIONS,
        cards::SERRA_ANGEL,
        cards::JUZAM_DJINN,
    ];
    expected.sort_unstable();
    assert_eq!(bottomed, expected);
}

#[test]
fn sleight_of_hand_keeps_one_card_and_bottoms_the_other() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (14_200, cards::LIGHTNING_BOLT),
            (14_201, cards::SERRA_ANGEL),
            (14_202, cards::SAVANNAH_LIONS),
        ],
    );

    cast_library_spell(
        &mut game,
        cards::SLEIGHT_OF_HAND,
        ManaPool {
            blue: 1,
            ..ManaPool::default()
        },
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Serra Angel");

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL)
    );
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT, cards::SAVANNAH_LIONS]
    );
}

#[test]
fn opt_draws_after_the_scry_choice() {
    let mut keep_game = ready_game();
    keep_game.players[0].library.clear();
    stack_library(
        &mut keep_game,
        &[
            (14_300, cards::LIGHTNING_BOLT),
            (14_301, cards::SERRA_ANGEL),
        ],
    );
    cast_library_spell(
        &mut keep_game,
        cards::OPT,
        ManaPool {
            blue: 1,
            ..ManaPool::default()
        },
    );
    let keep = keep_game.observe(PlayerId::One).decision.unwrap();
    keep_game
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: keep.id,
                options: Vec::new(),
            },
        )
        .unwrap();
    assert!(
        keep_game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT)
    );

    let mut bottom_game = ready_game();
    bottom_game.players[0].library.clear();
    stack_library(
        &mut bottom_game,
        &[
            (14_400, cards::LIGHTNING_BOLT),
            (14_401, cards::SERRA_ANGEL),
        ],
    );
    cast_library_spell(
        &mut bottom_game,
        cards::OPT,
        ManaPool {
            blue: 1,
            ..ManaPool::default()
        },
    );
    choose_decision_by_label(&mut bottom_game, PlayerId::One, "Lightning Bolt");

    assert!(
        bottom_game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL)
    );
    assert_eq!(
        bottom_game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT]
    );
}

#[test]
fn enlightened_tutor_reveals_an_artifact_or_enchantment_after_shuffling() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (14_500, cards::LIGHTNING_BOLT),
            (14_501, cards::BLACK_VISE),
            (14_502, cards::PRESENCE_OF_THE_MASTER),
            (14_503, cards::SERRA_ANGEL),
        ],
    );
    cast_library_spell(
        &mut game,
        cards::ENLIGHTENED_TUTOR,
        ManaPool {
            white: 1,
            ..ManaPool::default()
        },
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let offered = decision
        .options
        .iter()
        .map(|option| option.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(offered.len(), 2);
    assert!(offered.contains(&"Black Vise"));
    assert!(offered.contains(&"Presence of the Master"));
    choose_decision_by_label(&mut game, PlayerId::One, "Black Vise");

    let top = game.players[0].library.last().unwrap();
    assert_eq!(top.definition, cards::BLACK_VISE);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            player: PlayerId::One,
            definition: cards::BLACK_VISE,
            ..
        }
    )));
}

#[test]
fn worldly_tutor_puts_only_a_creature_on_top() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (14_600, cards::LIGHTNING_BOLT),
            (14_601, cards::SERRA_ANGEL),
            (14_602, cards::BLACK_VISE),
        ],
    );
    cast_library_spell(
        &mut game,
        cards::WORLDLY_TUTOR,
        ManaPool {
            green: 1,
            ..ManaPool::default()
        },
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.options.len(), 1);
    assert_eq!(decision.options[0].label, "Serra Angel");
    choose_decision_by_label(&mut game, PlayerId::One, "Serra Angel");

    assert_eq!(
        game.players[0].library.last().unwrap().definition,
        cards::SERRA_ANGEL
    );
}
