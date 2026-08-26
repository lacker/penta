use super::*;

#[test]
fn night_terrors_exiles_the_nonland_card_the_caster_chooses() {
    let mut game = ready_game();
    let terrors = card(10_020, cards::NIGHT_TERRORS, PlayerId::One);
    game.players[0].hand.push(terrors.clone());
    game.players[0].mana_pool.black = 3;
    game.players[1].hand.extend([
        card(10_021, cards::MOUNTAIN, PlayerId::Two),
        card(10_022, cards::SAVANNAH_LIONS, PlayerId::Two),
        card(10_023, cards::LIGHTNING_BOLT, PlayerId::Two),
    ]);

    game.apply(
        PlayerId::One,
        cast_action(
            terrors.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("Night Terrors can target the opponent");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses from the revealed hand");
    assert_eq!(decision.options.len(), 2, "the land is not offered");
    let lions = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::SAVANNAH_LIONS)
            })
        })
        .expect("the creature is a legal nonland choice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![lions.id],
        },
    )
    .expect("the revealed card can be chosen");

    assert_eq!(game.players[1].exile.len(), 1);
    assert_eq!(game.players[1].exile[0].definition, cards::SAVANNAH_LIONS);
    assert!(
        game.players[1].graveyard.is_empty(),
        "exiled, not discarded"
    );
}

#[test]
fn despise_discards_only_a_creature_or_planeswalker_card() {
    let mut game = ready_game();
    let despise = card(10_030, cards::DESPISE, PlayerId::One);
    game.players[0].hand.push(despise.clone());
    game.players[0].mana_pool.black = 1;
    game.players[1].hand.extend([
        card(10_031, cards::MOUNTAIN, PlayerId::Two),
        card(10_032, cards::LIGHTNING_BOLT, PlayerId::Two),
        card(10_033, cards::SAVANNAH_LIONS, PlayerId::Two),
        card(10_034, cards::GRIZZLY_BEARS, PlayerId::Two),
    ]);

    game.apply(
        PlayerId::One,
        cast_action(
            despise.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("Despise can target the opponent");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses from the revealed hand");
    assert_eq!(decision.options.len(), 2);
    let lions = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::SAVANNAH_LIONS)
            })
        })
        .expect("the white creature is a legal choice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![lions.id],
        },
    )
    .expect("the revealed creature can be chosen");

    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::SAVANNAH_LIONS
    );
    assert!(game.players[1].exile.is_empty(), "discarded, not exiled");
}
