use super::*;
use crate::AbilityProgramDef;

/// Casts a modal spell by picking one mode. A selected mode's clause-local
/// primary target becomes runtime slot zero.
pub(super) fn cast_mode(card: GameObjectId, mode: ModeId, targets: Vec<Target>) -> Action {
    let mut choices = CastChoices::default().with_modes(vec![mode]);
    if !targets.is_empty() {
        choices = choices.with_targets(vec![TargetSelection::new(TargetSlotId(0), targets)]);
    }
    Action::CastSpell {
        card,
        choices,
        sacrifices: Vec::new(),
    }
}

#[test]
fn azorius_charm_gives_your_creatures_lifelink_but_not_theirs() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;

    game.apply(PlayerId::One, cast_mode(charm.id, ModeId(0), Vec::new()))
        .unwrap();
    pass_priority_pair(&mut game);

    let lifelink = |game: &Game, definition| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("still on the battlefield");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink)
    };
    assert!(lifelink(&game, cards::SAVANNAH_LIONS), "yours gains it");
    assert!(!lifelink(&game, cards::SERRA_ANGEL), "theirs does not");
}

#[test]
fn azorius_charm_can_instead_draw_a_card() {
    let mut game = ready_game();
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;
    let before = game.players[0].library.len();

    game.apply(PlayerId::One, cast_mode(charm.id, ModeId(1), Vec::new()))
        .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn izzet_charm_deals_two_damage_in_its_implemented_mode() {
    let mut game = ready_game();
    // Savannah Lions is 2/1, so two damage kills it.
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two));
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(1),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
}

#[test]
fn izzet_charm_loots_by_drawing_two_then_discarding_two_of_choice() {
    let mut game = ready_game();
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    // One card already in hand, so the two drawn cards leave a choice of
    // three rather than a forced discard of everything.
    let keeper = card(10_002, cards::BLACK_LOTUS, PlayerId::One);
    game.players[0].hand.push(keeper.clone());
    let library_before = game.players[0].library.len();

    game.apply(PlayerId::One, cast_mode(charm.id, ModeId(2), Vec::new()))
        .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[0].library.len(),
        library_before - 2,
        "both cards were drawn"
    );
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the discard is the caster's choice");
    assert_eq!((decision.minimum, decision.maximum), (2, 2));
    let discards = decision
        .options
        .iter()
        .filter(|option| {
            option.card
                != Some((
                    keeper.id,
                    ObjectCharacteristics::card(cards::BLACK_LOTUS, CardPartId::PRIMARY),
                ))
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(discards.len(), 2, "the drawn cards are discardable");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: discards,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].id, keeper.id, "the Lotus was kept");
    assert_eq!(
        game.players[0].graveyard.len(),
        3,
        "two cards and the charm"
    );
}

#[test]
fn a_discard_with_no_choice_left_needs_no_decision() {
    let mut game = ready_game();
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;

    game.apply(PlayerId::One, cast_mode(charm.id, ModeId(2), Vec::new()))
        .unwrap();
    pass_priority_pair(&mut game);

    // Exactly the two drawn cards are in hand, so there is nothing to choose.
    assert!(game.players[0].hand.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn selesnya_charm_pumps_and_grants_trample() {
    let catalog = poc::catalog().unwrap();
    let charm_definition = catalog.get(cards::SELESNYA_CHARM).unwrap();
    let DeclarativeAbilityDef::Spell(spell) =
        charm_definition.rules.ability_clauses()[0].definition
    else {
        panic!("Selesnya Charm should have a spell ability")
    };
    let mode = spell.mode(ModeId(0)).unwrap();
    let AbilityProgramDef::Effects(EffectDef::Apply {
        recipient,
        effect: AppliedEffectDef::Composite(components),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    }) = mode.effect.definition
    else {
        panic!("Selesnya Charm should apply one composite effect until end of turn")
    };
    assert_eq!(recipient.legal_target(), Some(TargetIndex::PRIMARY));
    assert!(matches!(
        components,
        [
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Modify {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
            )),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )),
        ] if ability.definition == DeclarativeAbilityDef::Keyword(KeywordAbility::Trample)
    ));

    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(0),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("still there");
    assert_eq!(game.power(lions), Some(4), "2/1 plus 2/2");
    assert_eq!(game.toughness(lions), Some(3));
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));
}

#[test]
fn selesnya_charm_can_instead_make_a_knight() {
    let mut game = ready_game();
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(PlayerId::One, cast_mode(charm.id, ModeId(2), Vec::new()))
        .unwrap();
    pass_priority_pair(&mut game);

    let knight = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                token_with_vigilance(tokens::creature(&["Knight"], &[ManaColor::White], 2, 2)),
            )
        })
        .expect("a Knight token arrived");
    assert_eq!(game.power(knight), Some(2));
    assert_eq!(game.toughness(knight), Some(2));
    assert_eq!(knight.controller, PlayerId::One);
    assert!(game.permanent_has_executable_keyword(knight, KeywordAbility::Vigilance));
}

#[test]
fn selesnya_charm_exiles_only_a_big_creature() {
    // Juzam Djinn is 5/5 and qualifies; Serra Angel is 4/4 and does not.
    for (definition, legal) in [(cards::JUZAM_DJINN, true), (cards::SERRA_ANGEL, false)] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, definition, PlayerId::Two));
        let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
        game.players[0].hand.push(charm.clone());
        game.players[0].mana_pool.green = 1;
        game.players[0].mana_pool.white = 1;

        let action = cast_mode(
            charm.id,
            ModeId(1),
            vec![Target::Permanent(CardInstanceId(10_000))],
        );
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{definition:?} should be {}",
            if legal { "exilable" } else { "too small" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[1].exile[0].definition, definition);
    }
}

#[test]
fn selesnya_charm_reads_current_power_not_printed_power() {
    // A 4/4 pumped to 6/6 by the charm's own first mode qualifies for the
    // second, which is why the predicate reads live power.
    let mut game = ready_game();
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    game.battlefield.push(angel);
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(2),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    let action = cast_mode(
        charm.id,
        ModeId(1),
        vec![Target::Permanent(CardInstanceId(10_000))],
    );
    assert!(
        game.legal_actions(PlayerId::One).contains(&action),
        "a 4/4 pumped to 6/6 is now big enough"
    );
}

#[test]
fn boros_charm_burns_a_player_for_four() {
    let mut game = ready_game();
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(0), vec![Target::Player(PlayerId::Two)]),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 16);
    assert_eq!(
        game.players[0].life, 20,
        "it is a targeted burn, not a sweep"
    );
}

#[test]
fn boros_charm_burns_a_planeswalker_for_four() {
    let mut game = ready_game();
    let mut domri = creature(10_000, cards::DOMRI_RADE, PlayerId::Two);
    domri.set_counters(CounterKind::Loyalty, 3);
    let domri_id = domri.card.id;
    game.battlefield.push(domri);
    let lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    let charm = card(10_002, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    let cast = cast_mode(charm.id, ModeId(0), vec![Target::Permanent(domri_id)]);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "the printed player-or-planeswalker target includes Domri",
    );
    assert!(
        !game.legal_actions(PlayerId::One).contains(&cast_mode(
            charm.id,
            ModeId(0),
            vec![Target::Permanent(lions_id)],
        )),
        "the broader catalog projection does not make a creature a legal target",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != domri_id),
        "four damage removes a planeswalker with three loyalty",
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::DOMRI_RADE),
    );
    assert_eq!(game.players[1].life, 20, "the planeswalker took the damage");
}

#[test]
fn boros_charm_protects_only_your_current_permanents_until_cleanup() {
    let mut game = ready_game();
    let own_creature = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let own_creature_id = own_creature.card.id;
    let own_artifact = creature(10_001, cards::SOL_RING, PlayerId::One);
    let own_artifact_id = own_artifact.card.id;
    let opposing_creature = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let opposing_creature_id = opposing_creature.card.id;
    game.battlefield
        .extend([own_creature, own_artifact, opposing_creature]);
    let charm = card(10_003, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    let cast = cast_mode(charm.id, ModeId(1), Vec::new());
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "the target-free Indestructible mode is executable",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let has_indestructible = |game: &Game, id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .is_some_and(|permanent| {
                game.permanent_has_executable_keyword(permanent, KeywordAbility::Indestructible)
            })
    };
    assert!(has_indestructible(&game, own_creature_id));
    assert!(
        has_indestructible(&game, own_artifact_id),
        "the mode protects every permanent type, not only creatures",
    );
    assert!(!has_indestructible(&game, opposing_creature_id));

    let later_artifact = creature(10_004, cards::FELLWAR_STONE, PlayerId::One);
    let later_artifact_id = later_artifact.card.id;
    game.battlefield.push(later_artifact);
    assert!(
        !has_indestructible(&game, later_artifact_id),
        "the resolving spell snapshots the permanents it grants to",
    );

    game.destroy_permanent_without_regeneration(own_creature_id);
    game.destroy_permanent_without_regeneration(own_artifact_id);
    game.destroy_permanent_without_regeneration(opposing_creature_id);
    assert!(has_indestructible(&game, own_creature_id));
    assert!(has_indestructible(&game, own_artifact_id));
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_creature_id),
        "the opposing permanent was never protected",
    );

    game.finish_cleanup();
    assert!(!has_indestructible(&game, own_creature_id));
    assert!(!has_indestructible(&game, own_artifact_id));
    game.destroy_permanent_without_regeneration(own_creature_id);
    game.destroy_permanent_without_regeneration(own_artifact_id);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| { ![own_creature_id, own_artifact_id].contains(&permanent.card.id) }),
        "the grant expires during cleanup",
    );
}

#[test]
fn boros_charm_grants_double_strike_until_cleanup() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(2),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(10_000))
        .unwrap();
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::DoubleStrike));

    game.finish_cleanup();
    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(10_000))
        .unwrap();
    assert!(!game.permanent_has_executable_keyword(lions, KeywordAbility::DoubleStrike));
}

#[test]
fn primeval_bounty_makes_a_beast_only_for_its_controller() {
    for (caster, expect_token) in [(PlayerId::One, true), (PlayerId::Two, false)] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::PRIMEVAL_BOUNTY)
            .expect("cataloged");
        let lions = card(10_001, cards::SAVANNAH_LIONS, caster);
        game.players[caster.index()].hand.push(lions.clone());
        game.players[caster.index()].mana_pool.white = 1;
        // A creature spell is sorcery-speed, so the caster needs the turn.
        game.active_player = caster;
        game.priority = caster;

        game.apply(caster, cast_action(lions.id, Vec::new(), Vec::new(), 0))
            .unwrap();
        for _ in 0..8 {
            if game
                .battlefield
                .iter()
                .any(|p| is_token_with(p, tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3)))
            {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let made_token = game
            .battlefield
            .iter()
            .any(|p| is_token_with(p, tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3)));
        assert_eq!(
            made_token,
            expect_token,
            "a creature cast by {caster} should {} a Beast",
            if expect_token { "make" } else { "not make" }
        );
    }
}

#[test]
fn primeval_bounty_gains_life_only_for_its_own_lands() {
    for (lander, expect_life) in [(PlayerId::One, 23), (PlayerId::Two, 20)] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::PRIMEVAL_BOUNTY)
            .expect("cataloged");
        game.players[lander.index()]
            .hand
            .push(card(10_002, cards::FOREST, lander));
        game.players[lander.index()].land_played_this_turn = false;
        game.play_land(lander, CardInstanceId(10_002), PlayOptionId::DEFAULT);
        for _ in 0..8 {
            if game.players[0].life != 20 {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
        assert_eq!(
            game.players[0].life,
            expect_life,
            "a land played by {lander} should {} life",
            if expect_life > 20 { "gain" } else { "not gain" }
        );
    }
}

#[test]
fn vault_of_the_archangel_arms_only_your_creatures() {
    let mut game = ready_game();
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::VAULT_OF_THE_ARCHANGEL)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == vault)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 2;

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == vault))
        .expect("the vault ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let armed = |game: &Game, definition| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("still there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Deathtouch)
            && game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink)
    };
    assert!(armed(&game, cards::SAVANNAH_LIONS), "yours gets both");
    assert!(!armed(&game, cards::SERRA_ANGEL), "theirs gets neither");
}

#[test]
fn gavony_township_grows_only_your_creatures() {
    let mut game = ready_game();
    let township = game
        .put_onto_battlefield(PlayerId::One, cards::GAVONY_TOWNSHIP)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == township)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == township)
        })
        .expect("the township ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let power_of = |game: &Game, controller| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| {
                permanent.card.definition == cards::SAVANNAH_LIONS
                    && permanent.controller == controller
            })
            .expect("still there");
        game.power(permanent)
    };
    assert_eq!(
        power_of(&game, PlayerId::One),
        Some(3),
        "2/1 plus a counter"
    );
    assert_eq!(
        power_of(&game, PlayerId::Two),
        Some(2),
        "theirs is untouched"
    );
}

#[test]
fn shadowborn_demon_cannot_point_its_trigger_at_a_demon() {
    // Juzam Djinn is a Djinn and a legal target; Desecration Demon is not.
    // The other Demon deliberately has no entry trigger of its own, so this
    // test has exactly one trigger to answer.
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::JUZAM_DJINN)
        .expect("cataloged");
    let other_demon = game
        .put_onto_battlefield(PlayerId::Two, cards::DESECRATION_DEMON)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SHADOWBORN_DEMON)
        .expect("cataloged");

    // A captured trigger reaches the stack as the game processes actions.
    for _ in 0..6 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry trigger asks for a target");
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(id, _)| id))
        .collect();
    assert!(
        !offered.contains(&CardInstanceId(other_demon.0)),
        "another Demon is not a legal target"
    );
    assert_eq!(offered.len(), 1, "only the Djinn qualifies");

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();
    for _ in 0..6 {
        if !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::JUZAM_DJINN)
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::JUZAM_DJINN),
        "the Djinn was destroyed"
    );
}

#[test]
fn order_of_leitbur_can_pump_itself() {
    let mut game = ready_game();
    let order = game
        .put_onto_battlefield(PlayerId::One, cards::ORDER_OF_LEITBUR)
        .expect("cataloged");
    game.players[0].mana_pool.white = 2;
    let pump = activated_ability_for(&game, order, 1);

    // The Order also grants itself first strike for {W}; the pump is the
    // later printed clause.
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability,
                    ..
                } if *source == order && *ability == pump
            )
        })
        .expect("the pump is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let order = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == order)
        .expect("still there");
    assert_eq!(game.power(order), Some(3), "2/1 plus one power");
    assert_eq!(game.toughness(order), Some(1), "toughness is unchanged");
}

#[test]
fn encroaching_wastes_spares_basic_lands() {
    // A Mountain is Basic and safe; a dual land is not.
    for (definition, legal) in [(cards::MOUNTAIN, false), (cards::TUNDRA, true)] {
        let mut game = ready_game();
        let wastes = game
            .put_onto_battlefield(PlayerId::One, cards::ENCROACHING_WASTES)
            .expect("cataloged");
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == wastes)
            .unwrap()
            .entered_controller_turn = game.turns_started[0] - 1;
        let target = game
            .put_onto_battlefield(PlayerId::Two, definition)
            .expect("cataloged");
        game.players[0].mana_pool.colorless = 4;

        let offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } if source == wastes => Some(
                    targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .copied()
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(
            offered.contains(&Target::Permanent(target)),
            legal,
            "{definition:?} should be {}",
            if legal {
                "destroyable"
            } else {
                "protected by Basic"
            }
        );
    }
}

#[test]
fn goblin_digging_team_only_hits_walls() {
    let mut game = ready_game();
    let team = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_DIGGING_TEAM)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == team)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    // Savannah Lions is a Cat, not a Wall, so there is nothing to point at.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == team)
        }),
        "with no Wall in play the ability has no legal target"
    );

    let wall = game
        .put_onto_battlefield(PlayerId::Two, cards::WALL_OF_STONE)
        .expect("cataloged");
    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == team => Some(
                targets
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(targets, vec![vec![Target::Permanent(wall)]]);
}

#[test]
fn ruric_thar_burns_whoever_cast_the_noncreature_spell() {
    // It hits its own controller too, which is the point of the card.
    for caster in [PlayerId::One, PlayerId::Two] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::RURIC_THAR_THE_UNBOWED)
            .expect("cataloged");
        let bolt = card(10_001, cards::LIGHTNING_BOLT, caster);
        game.players[caster.index()].hand.push(bolt.clone());
        game.players[caster.index()].mana_pool.red = 1;
        game.active_player = caster;
        game.priority = caster;
        let before = [game.players[0].life, game.players[1].life];

        game.apply(
            caster,
            cast_action(
                bolt.id,
                vec![Target::Player(caster.opponent())],
                Vec::new(),
                0,
            ),
        )
        .unwrap();
        for _ in 0..8 {
            if game.players[caster.index()].life < before[caster.index()] {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        assert_eq!(
            game.players[caster.index()].life,
            before[caster.index()] - 6,
            "the caster {caster} takes six"
        );
    }
}

#[test]
fn assemble_the_legion_musters_one_more_soldier_every_upkeep() {
    let mut game = ready_game();
    game.turn = 2;
    game.put_onto_battlefield(PlayerId::One, cards::ASSEMBLE_THE_LEGION)
        .expect("cataloged");

    let mut mustered = Vec::new();
    for _ in 0..3 {
        loop {
            game.start_next_turn();
            if game.active_player == PlayerId::One {
                break;
            }
        }
        for _ in 0..8 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .take(decision.minimum.max(1))
                    .map(|option| option.id)
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
        mustered.push(
            game.battlefield
                .iter()
                .filter(|permanent| {
                    is_token_with(
                        permanent,
                        token_with_haste(tokens::creature(
                            &["Soldier"],
                            &[ManaColor::Red, ManaColor::White],
                            1,
                            1,
                        )),
                    )
                })
                .count(),
        );
    }

    // One counter is added before the tokens are made, so the first upkeep
    // already musters a Soldier and each later one musters one more.
    assert_eq!(mustered, vec![1, 3, 6]);
}
