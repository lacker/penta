use super::*;

fn divine_reckoning_effect(game: &Game) -> EffectDef {
    game.catalog
        .get(cards::DIVINE_RECKONING)
        .expect("Divine Reckoning is cataloged")
        .rules
        .ability_clauses()[0]
        .declarative_effect()
        .expect("Divine Reckoning uses a resolving effect program")
}

#[test]
fn chooses_for_each_player_then_uses_standard_destroy() {
    let game = ready_game();
    let EffectDef::ChooseForEachPlayer(choice) = divine_reckoning_effect(&game) else {
        panic!("Divine Reckoning starts by choosing for each player");
    };
    assert_eq!(choice.player, EffectRecipientDef::EachPlayer);
    assert_eq!(
        choice.candidates,
        ObjectPredicateDef::HasType(CardType::Creature)
    );
    assert_eq!(
        choice.selection,
        PerPlayerSelectionDef::OneOfEach(&[ObjectPredicateDef::Any])
    );
    assert_ne!(choice.chosen, choice.unchosen);
    assert_eq!(
        *choice.then,
        EffectDef::Destroy {
            object: EffectRecipientDef::objects(ObjectSetDef::Binding(choice.unchosen)),
            can_regenerate: true,
            then: None,
        }
    );
}

fn choose_permanent(game: &mut Game, permanent: GameObjectId) {
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("a survivor choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == permanent))
        .expect("the chosen creature is offered")
        .id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the survivor choice is legal");
}

#[test]
fn choices_are_frozen_before_the_rest_are_destroyed() {
    let mut game = ready_game();
    let yours_kept = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let yours_kept_id = yours_kept.card.id;
    let yours_destroyed = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let yours_destroyed_id = yours_destroyed.card.id;
    let theirs_kept = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    let theirs_kept_id = theirs_kept.card.id;
    let theirs_destroyed = creature(10_003, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_destroyed_id = theirs_destroyed.card.id;
    let artifact = creature(10_004, cards::SOL_RING, PlayerId::Two);
    let artifact_id = artifact.card.id;
    game.battlefield.extend([
        yours_kept,
        yours_destroyed,
        theirs_kept,
        theirs_destroyed,
        artifact,
    ]);

    let reckoning = card(10_005, cards::DIVINE_RECKONING, PlayerId::One);
    game.players[0].hand.push(reckoning.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(reckoning.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Divine Reckoning is cast");
    pass_priority_pair(&mut game);

    assert_eq!(game.decision_player(), Some(PlayerId::One));
    choose_permanent(&mut game, yours_kept_id);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    assert_eq!(
        game.battlefield.len(),
        5,
        "the active player's answer is locked without destroying anything yet"
    );

    choose_permanent(&mut game, theirs_kept_id);
    assert!(game.pending_decisions.is_empty());
    let survivors = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    assert!(survivors.contains(&yours_kept_id));
    assert!(survivors.contains(&theirs_kept_id));
    assert!(survivors.contains(&artifact_id));
    assert!(!survivors.contains(&yours_destroyed_id));
    assert!(!survivors.contains(&theirs_destroyed_id));
}

#[test]
fn an_unchosen_creature_can_regenerate() {
    let mut game = ready_game();
    let kept = creature(10_010, cards::SERRA_ANGEL, PlayerId::One);
    let kept_id = kept.card.id;
    let mut troll = creature(10_011, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    troll.regeneration_shields = 1;
    game.battlefield.extend([kept, troll]);

    let source = spell(10_012, cards::DIVINE_RECKONING, PlayerId::One, 0);
    let effect = divine_reckoning_effect(&game);
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &source,
        TriggerContext::empty(),
    );
    choose_permanent(&mut game, kept_id);

    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("the regeneration shield saves the unchosen Troll");
    assert!(troll.tapped);
    assert_eq!(troll.regeneration_shields, 0);
}

#[test]
fn flashback_casts_for_seven_mana_and_exiles_the_spell() {
    let mut game = ready_game();
    let reckoning = card(10_020, cards::DIVINE_RECKONING, PlayerId::One);
    game.players[0].graveyard.push(reckoning.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 5;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == reckoning.id
                        && choices.costs().alternative() == Some(AlternativeCostId(1))
            )
        })
        .expect("the printed flashback cost is offered from the graveyard");
    game.apply(PlayerId::One, action)
        .expect("Divine Reckoning is flashed back");
    pass_priority_pair(&mut game);

    assert!(game.players[0].graveyard.is_empty());
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::DIVINE_RECKONING)
    );
}
