use super::*;

fn whisper_game() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let spell = card(90_000, cards::UNENDING_WHISPER, PlayerId::One);
    let id = spell.id;
    game.players[0].graveyard.push(spell);
    game.players[0].mana_pool.blue = 1;
    (game, id)
}

fn cast_with(game: &Game, spell: GameObjectId, tapped: &[GameObjectId]) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(|a| {
        matches!(a,
        Action::CastSpell {card, sacrifices, ..} if *card == spell && sacrifices == tapped)
    })
}

#[test]
fn harmonize_taps_one_new_creature_and_exiles_after_resolving() {
    let (mut game, spell) = whisper_game();
    let mut helper = creature(80_000, cards::CRAW_WURM, PlayerId::One);
    helper.entered_controller_turn = game.turns_started[0];
    let id = helper.card.id;
    game.battlefield.push(helper);
    let action = cast_with(&game, spell, &[id]).expect("six power pays five generic");
    let before = game.players[0].library.len();
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    let cast = game.stack.last().unwrap().cast.as_ref().unwrap();
    assert_eq!(cast.alternative, Some(AlternativeCastKindDef::Harmonize));
    assert!(!cast.via_flashback);
    assert!(cast.exiles_on_leaving_stack());
    assert!(!cast.for_spell_copy().exiles_on_leaving_stack());
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].exile[0].definition, cards::UNENDING_WHISPER);
}

#[test]
fn harmonize_tap_is_optional_and_cannot_pay_colored_mana_or_use_two_creatures() {
    let (mut game, spell) = whisper_game();
    game.players[0].mana_pool.colorless = 5;
    assert!(cast_with(&game, spell, &[]).is_some());
    game.players[0].mana_pool.colorless = 0;
    let a = creature(80_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let b = creature(80_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let ids = [a.card.id, b.card.id];
    game.battlefield.extend([a, b]);
    assert!(cast_with(&game, spell, &ids).is_none());
    assert!(cast_with(&game, spell, &ids[..1]).is_none());
    let big = creature(80_002, cards::CRAW_WURM, PlayerId::One);
    let id = big.card.id;
    game.battlefield.push(big);
    game.players[0].mana_pool.blue = 0;
    assert!(cast_with(&game, spell, &[id]).is_none());
}

#[test]
fn harmonize_cannot_tap_a_mana_source_twice() {
    let (mut game, spell) = whisper_game();
    let elf = creature(80_000, cards::LLANOWAR_ELVES, PlayerId::One);
    let id = elf.card.id;
    game.battlefield.push(elf);
    game.players[0].mana_pool.colorless = 3;
    assert!(
        cast_with(&game, spell, &[id]).is_none(),
        "one tap cannot reduce and produce mana"
    );
    game.players[0].mana_pool.colorless = 4;
    let action = cast_with(&game, spell, &[id]).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn harmonize_announced_x_is_discounted_without_changing_the_spell_value() {
    let mut game = ready_game();
    let spell = card(90_001, cards::NATURE_S_RHYTHM, PlayerId::One);
    let id = spell.id;
    game.players[0].graveyard.push(spell);
    game.players[0].mana_pool.green = 4;
    let helper = creature(80_000, cards::CRAW_WURM, PlayerId::One);
    let helper_id = helper.card.id;
    game.battlefield.push(helper);
    let actions = game.legal_actions(PlayerId::One);
    let action = actions.iter().find(|a| matches!(a,
        Action::CastSpell {card, choices, sacrifices} if *card == id && choices.x() == 6 && sacrifices == &[helper_id]
    )).unwrap().clone();
    assert!(!actions.iter().any(
        |a| matches!(a, Action::CastSpell {card, choices, ..} if *card == id && choices.x() > 6)
    ));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.stack.last().unwrap().cast.as_ref().unwrap().x, 6);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn harmonize_uses_current_power_and_rejects_tapped_or_opposing_creatures() {
    for power in [-2, 0, 3] {
        let (mut game, spell) = whisper_game();
        let helper = token_permanent(
            80_000,
            TokenCharacteristics::creature(&["Beast"], &[], power, 5),
            PlayerId::One,
        );
        let id = helper.card.id;
        game.battlefield.push(helper);
        game.players[0].mana_pool.colorless = u16::try_from(5 - power.max(0)).unwrap();
        assert!(cast_with(&game, spell, &[id]).is_some());
        game.players[0].mana_pool.colorless -= 1;
        assert!(cast_with(&game, spell, &[id]).is_none());
        game.players[0].mana_pool.colorless = 5;
        game.battlefield[0].tapped = true;
        assert!(cast_with(&game, spell, &[id]).is_none());
        game.battlefield[0].tapped = false;
        game.battlefield[0].controller = PlayerId::Two;
        assert!(cast_with(&game, spell, &[id]).is_none());
    }
}

#[test]
fn harmonize_exiles_on_counter_bounce_or_library_return() {
    for zone in [
        CounteredSpellZone::Graveyard,
        CounteredSpellZone::Hand,
        CounteredSpellZone::Library(ZonePlacement::Top),
    ] {
        let (mut game, spell) = whisper_game();
        game.players[0].mana_pool.colorless = 5;
        let action = cast_with(&game, spell, &[]).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        let id = game.stack.last().unwrap().id;
        game.counter_spell_into(id, zone);
        assert_eq!(game.players[0].exile[0].definition, cards::UNENDING_WHISPER);
    }
}

#[test]
fn harmonize_grant_uses_card_cost_and_can_tap_its_new_songcrafter() {
    let mut game = ready_game();
    let spell = card(90_000, cards::DIVINATION, PlayerId::One);
    let id = spell.id;
    game.players[0].graveyard.push(spell);
    let mage = card(90_001, cards::SONGCRAFTER_MAGE, PlayerId::One);
    let mage_id = mage.id;
    game.players[0].hand.push(mage);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;
    game.apply(PlayerId::One, cast_action(mage_id, vec![], vec![], 0))
        .unwrap();
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.players[0].mana_pool.blue = 1;
    let helper = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::SONGCRAFTER_MAGE)
        .unwrap()
        .card
        .id;
    let mut expired = game.clone();
    expired.cleanup();
    assert!(
        cast_with(&expired, id, &[helper]).is_none(),
        "the grant expires at cleanup"
    );
    let action = cast_with(&game, id, &[helper])
        .expect("granted harmonize costs just U with the Mage's tap");
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].exile[0].definition, cards::DIVINATION);
}

#[test]
fn harmonize_checkpoint_preserves_its_stack_exit_replacement() {
    let (mut game, spell) = whisper_game();
    for state in &mut game.players {
        state.library.clear();
    }
    game.players[0].mana_pool.colorless = 5;
    let action = cast_with(&game, spell, &[]).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let observation = game.observe(game.priority);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &actions,
    );
    let hidden = serde_json::json!({
        "hands": {"p1": [], "p2": []}, "libraries": {"p1": [], "p2": []},
        "outsideGame": {"p1": [], "p2": []},
    });
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 123)
            .unwrap();
    let stack = restored.stack.last().unwrap();
    assert_eq!(
        stack.cast.as_ref().unwrap().alternative,
        Some(AlternativeCastKindDef::Harmonize)
    );
    let id = stack.id;
    restored.counter_spell_into(id, CounteredSpellZone::Hand);
    assert_eq!(
        restored.players[0].exile[0].definition,
        cards::UNENDING_WHISPER
    );
}

#[test]
fn harmonize_does_not_change_timing_or_the_cast_from_hand() {
    let (mut game, spell) = whisper_game();
    game.players[0].mana_pool.colorless = 5;
    game.active_player = PlayerId::Two;
    assert!(cast_with(&game, spell, &[]).is_none());
    game.active_player = PlayerId::One;
    let card = game.players[0].graveyard.pop().unwrap();
    game.players[0].hand.push(card);
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|a| matches!(a, Action::CastSpell {card, ..} if *card == spell))
        .collect();
    assert_eq!(casts.len(), 1);
    assert!(
        matches!(&casts[0], Action::CastSpell {choices, ..} if choices.costs().alternative().is_none())
    );
}

#[test]
fn harmonize_discount_uses_modified_power_and_offsets_cost_increases() {
    let (mut game, spell) = whisper_game();
    let mut helper = creature(80_000, cards::GRIZZLY_BEARS, PlayerId::One);
    helper.add_counters(CounterKind::PlusOnePlusOne, 4);
    let id = helper.card.id;
    game.battlefield.push(helper);
    game.battlefield
        .push(creature(80_001, cards::SPHERE_OF_RESISTANCE, PlayerId::Two));
    let action = cast_with(&game, spell, &[id])
        .expect("six effective power pays all six generic including the tax");
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn harmonize_reference_and_prepared_execution_agree() {
    let run = |prepared| {
        let (mut game, spell) = whisper_game();
        game.set_prepared_engine_enabled(prepared);
        let helper = creature(80_000, cards::CRAW_WURM, PlayerId::One);
        let id = helper.card.id;
        game.battlefield.push(helper);
        let action = cast_with(&game, spell, &[id]).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        game
    };
    let reference = run(false);
    let prepared = run(true);
    assert_eq!(reference.players, prepared.players);
    assert_eq!(reference.battlefield, prepared.battlefield);
    assert_eq!(reference.events, prepared.events);
    assert_eq!(
        reference.legal_actions(PlayerId::One),
        prepared.legal_actions(PlayerId::One)
    );
}

#[test]
fn harmonize_can_tap_then_sacrifice_the_same_creature_for_an_additional_cost() {
    let mut game = ready_game();
    let spell = card(90_000, cards::VILLAGE_RITES, PlayerId::One);
    let id = spell.id;
    game.players[0].graveyard.push(spell);
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object: id,
            ability: crate::card::sets::tarkir_dragonstorm::harmonize(&[CostDef::ManaCostOf(
                ObjectRefDef::Source,
            )]),
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        });
    let helper = creature(80_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let helper_id = helper.card.id;
    game.battlefield.push(helper);
    game.players[0].mana_pool.black = 1;
    let action = cast_with(&game, id, &[helper_id, helper_id]).unwrap();
    let before = game.players[0].library.len();
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.battlefield.is_empty());
    assert_eq!(
        game.players[0].graveyard[0].definition,
        cards::GRIZZLY_BEARS
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].library.len(), before - 2);
    assert_eq!(game.players[0].exile[0].definition, cards::VILLAGE_RITES);
}

#[test]
fn harmonize_combines_other_discounts_with_x_even_without_tapping() {
    for tap in [false, true] {
        let mut game = ready_game();
        let spell = card(90_000, cards::NATURE_S_RHYTHM, PlayerId::One);
        let id = spell.id;
        game.players[0].graveyard.push(spell);
        game.players[0].mana_pool.green = 4;
        game.battlefield
            .push(creature(80_000, cards::HELM_OF_AWAKENING, PlayerId::One));
        let helper = creature(80_001, cards::CRAW_WURM, PlayerId::One);
        let helper_id = helper.card.id;
        game.battlefield.push(helper);
        let x = if tap { 7 } else { 1 };
        let objects = if tap { vec![helper_id] } else { vec![] };
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
            Action::CastSpell {card, choices, sacrifices} if *card == id && choices.x() == x && *sacrifices == objects
        )).expect("Helm reduces the announced X alongside the optional harmonize tap");
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn harmonize_glacial_dragonhunt_targets_only_after_a_nonland_discard() {
    for discarded in [cards::FOREST, cards::GRIZZLY_BEARS] {
        let mut game = ready_game();
        game.players[0].library.clear();
        let drawn = card(90_002, discarded, PlayerId::One);
        game.players[0].library.push(drawn);
        game.players[0]
            .hand
            .push(card(90_003, cards::ISLAND, PlayerId::One));
        let spell = card(90_000, cards::GLACIAL_DRAGONHUNT, PlayerId::One);
        let id = spell.id;
        game.players[0].graveyard.push(spell);
        game.players[0].mana_pool.blue = 1;
        game.players[0].mana_pool.red = 1;
        let helper = creature(80_000, cards::CRAW_WURM, PlayerId::One);
        let helper_id = helper.card.id;
        game.battlefield.push(helper);
        let victim = creature(80_001, cards::CRAW_WURM, PlayerId::Two);
        let victim_id = victim.card.id;
        game.battlefield.push(victim);
        let action = cast_with(&game, id, &[helper_id]).unwrap();
        assert!(
            matches!(&action, Action::CastSpell { choices, .. } if choices.targets().is_empty())
        );
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        let drawn_id = game.players[0].hand.last().unwrap().id;
        let may = game.observe(PlayerId::One).decision.unwrap();
        let yes = may.options.iter().find(|o| o.label == "Do it").unwrap().id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: may.id,
                options: vec![yes],
            },
        )
        .unwrap();
        let discard = game.observe(PlayerId::One).decision.unwrap();
        let chosen = discard
            .options
            .iter()
            .find(|o| o.card.is_some_and(|(id, _)| id == drawn_id))
            .unwrap()
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: discard.id,
                options: vec![chosen],
            },
        )
        .unwrap();
        assert_eq!(
            game.players[0].exile[0].definition,
            cards::GLACIAL_DRAGONHUNT
        );
        if discarded == cards::FOREST {
            assert!(game.stack.is_empty());
            assert!(game.pending_decisions.is_empty());
        } else {
            let target = game.observe(PlayerId::One).decision.unwrap();
            let chosen = target
                .options
                .iter()
                .find(|o| o.card.is_some_and(|(id, _)| id == victim_id))
                .unwrap()
                .id;
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: target.id,
                    options: vec![chosen],
                },
            )
            .unwrap();
            assert_eq!(
                game.stack.len(),
                1,
                "the damage trigger uses the stack after the spell has left"
            );
            pass_priority_pair(&mut game);
            assert_eq!(
                game.battlefield
                    .iter()
                    .find(|p| p.card.id == victim_id)
                    .unwrap()
                    .damage,
                3
            );
        }
    }
}
