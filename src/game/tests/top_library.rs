//! Shared top-card access, conditional permissions and selected play consequences.
use super::*;

pub(super) fn staged(sources: &[CardDefinitionId], library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    for source in sources {
        game.put_onto_battlefield(PlayerId::One, *source).unwrap();
    }
    drain_pending(&mut game);
    game.turns_started = [3, 3];
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game
}
pub(super) fn top(game: &Game) -> GameObjectId {
    game.players[0].library.last().unwrap().id
}
pub(super) fn mana(game: &mut Game) {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 8);
    }
}
pub(super) fn casts(game: &Game) -> Vec<Action> {
    let top = top(game);
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == top))
        .collect()
}
pub(super) fn settle(game: &mut Game) {
    for _ in 0..60 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("did not settle");
}
pub(super) fn restored(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
        .expect("top library checkpoint")
}

#[test]
fn frenzy_prohibits_hand_plays_but_allows_top_plays() {
    let mut game = staged(&[cards::EXPERIMENTAL_FRENZY], &[cards::FOREST]);
    let hand = game
        .build_zone(PlayerId::One, &[cards::ISLAND, cards::LIGHTNING_BOLT])
        .unwrap();
    let ids = hand.iter().map(|card| card.id).collect::<Vec<_>>();
    game.players[0].hand = hand;
    mana(&mut game);
    let actions = game.legal_actions(PlayerId::One);
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if *card == top(&game)))
    );
    assert!(!actions.iter().any(|action| matches!(action, Action::PlayLand { card, .. } | Action::CastSpell { card, .. } if ids.contains(card))));
}

#[test]
fn coven_counts_distinct_live_powers_and_verge_compares_land_counts() {
    let mut game = staged(
        &[
            cards::AUGUR_OF_AUTUMN,
            cards::LLANOWAR_ELVES,
            cards::BIRDS_OF_PARADISE,
        ],
        &[cards::GRIZZLY_BEARS],
    );
    mana(&mut game);
    assert!(!casts(&game).is_empty());
    game.battlefield
        .retain(|p| p.card.definition != cards::BIRDS_OF_PARADISE);
    assert!(casts(&game).is_empty());
    game.put_onto_battlefield(PlayerId::One, cards::LLANOWAR_ELVES)
        .unwrap();
    assert!(
        casts(&game).is_empty(),
        "duplicate powers do not satisfy coven"
    );
    let mut game = staged(&[cards::VERGE_RANGERS], &[cards::FOREST]);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::PlayLand { .. }))
    );
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::PlayLand { .. }))
    );
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .unwrap();
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::PlayLand { .. }))
    );
}

#[test]
fn channelers_rechecks_the_top_for_stats_and_granted_mana() {
    let mut game = staged(
        &[cards::MUL_DAYA_CHANNELERS],
        &[cards::FOREST, cards::GRIZZLY_BEARS],
    );
    let id = game.battlefield[0].card.id;
    assert_eq!(game.power(&game.battlefield[0]), Some(5));
    assert!(!game.legal_actions(PlayerId::One).iter().any(|action| matches!(action, Action::ActivateManaAbility { source: permanent, .. } if *permanent == id)));
    game.players[0].library.pop();
    assert_eq!(game.power(&game.battlefield[0]), Some(2));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateManaAbility { source: permanent, .. } if *permanent == id))
        .expect("land grants mana ability");
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].mana_pool.total(), 2);
    game.players[0].library.clear();
    assert_eq!(game.power(&game.battlefield[0]), Some(2));
}

#[test]
fn glarb_tests_chosen_x_and_citadel_forces_zero_x() {
    let mut game = staged(&[cards::GLARB_CALAMITY_S_AUGUR], &[cards::FIREBALL]);
    mana(&mut game);
    let actions = casts(&game);
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, Action::CastSpell { choices, .. } if choices.x() == 3))
    );
    assert!(
        actions
            .iter()
            .all(|a| matches!(a, Action::CastSpell { choices, .. } if choices.x() >= 3))
    );
    let game = staged(&[cards::BOLASS_CITADEL], &[cards::FIREBALL]);
    assert!(!casts(&game).is_empty());
    assert!(
        casts(&game)
            .iter()
            .all(|a| matches!(a, Action::CastSpell { choices, .. } if choices.x() == 0))
    );
}

#[test]
fn overlapping_library_permissions_offer_both_payment_methods() {
    let mut game = staged(
        &[cards::BOLASS_CITADEL, cards::FUTURE_SIGHT],
        &[cards::GRIZZLY_BEARS],
    );
    mana(&mut game);
    let actions = casts(&game);
    let sources = actions
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { choices, .. } => choices.costs().permission_source(),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(sources.len(), 2);
    for source in sources {
        let mut branch = game.clone();
        let action = actions.iter().find(|action| matches!(action, Action::CastSpell { choices, .. } if choices.costs().permission_source() == Some(source))).unwrap().clone();
        let life = branch.players[0].life;
        let citadel = branch
            .battlefield
            .iter()
            .any(|p| p.card.id == source && p.card.definition == cards::BOLASS_CITADEL);
        branch.apply(PlayerId::One, action).unwrap();
        assert_eq!(branch.players[0].life, life - if citadel { 2 } else { 0 });
    }
}

#[test]
fn selected_counter_rider_survives_source_removal_and_checkpoint() {
    let mut game = staged(
        &[cards::MIKEY_DON_PARTY_PLANNERS],
        &[cards::NINJA_OF_THE_DEEP_HOURS],
    );
    mana(&mut game);
    let action = casts(&game).into_iter().next().unwrap();
    game.apply(PlayerId::One, action).unwrap();
    game.battlefield.clear();
    let mut game = restored(&game);
    settle(&mut game);
    let ninja = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::NINJA_OF_THE_DEEP_HOURS)
        .unwrap();
    assert_eq!(ninja.counters(CounterKind::PlusOnePlusOne), 1);
}

#[test]
fn doctor_cast_spends_one_use_and_creates_a_reflexive_food_trigger() {
    let mut game = staged(
        &[cards::THE_FOURTH_DOCTOR],
        &[cards::SOL_RING, cards::SOL_RING],
    );
    mana(&mut game);
    let action = casts(&game).into_iter().next().unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let mut game = restored(&game);
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    assert!(casts(&game).is_empty());
}

#[test]
fn snoop_freezes_the_granted_ability_when_the_top_changes() {
    let mut game = staged(
        &[cards::CONSPICUOUS_SNOOP, cards::LLANOWAR_ELVES],
        &[cards::FOREST, cards::KIKI_JIKI_MIRROR_BREAKER],
    );
    let snoop = game.battlefield[0].card.id;
    let elf = game.battlefield[1].card.id;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility { source: permanent, targets, .. } if *permanent == snoop && targets.iter().any(|selection| selection.targets().contains(&Target::Permanent(elf))))).expect("Snoop inherits Kiki-Jiki's activation");
    game.apply(PlayerId::One, action).unwrap();
    game.draw_cards(PlayerId::One, 1);
    let mut game = restored(&game);
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |a| matches!(a, Action::ActivateAbility { source: permanent, .. } if *permanent == snoop)
    ));
}

#[test]
fn doctor_land_choice_preserves_an_unused_permission_and_round_trips() {
    let mut game = staged(
        &[cards::THE_FOURTH_DOCTOR, cards::FUTURE_SIGHT],
        &[cards::SOL_RING, cards::ANCIENT_DEN],
    );
    let doctor = game.battlefield[0].card.id;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::PlayLand { card, .. } if *card == top(&game)))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let mut game = restored(&game);
    let decision = game.pending_decisions[0].observation.clone();
    let choice = decision
        .options
        .iter()
        .find(|o| o.label.contains("Future Sight"))
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![choice],
        },
    )
    .unwrap();
    settle(&mut game);
    assert_eq!(game.play_permission_uses(doctor), 0);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
    mana(&mut game);
    assert!(casts(&game).iter().any(|a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().permission_source() == Some(doctor))));
}

#[test]
fn fblthp_plots_the_top_as_a_special_action_and_rejects_lands() {
    let mut game = staged(
        &[cards::FBLTHP_LOST_ON_THE_RANGE],
        &[cards::FOREST, cards::LIGHTNING_BOLT],
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let card = top(&game);
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::Plot { card })
    );
    game.apply(PlayerId::One, Action::Plot { card }).unwrap();
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::LIGHTNING_BOLT)
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::Plot { .. }))
    );
}

#[test]
fn library_visibility_is_independent_of_play_restrictions() {
    for (source, public) in [
        (cards::FBLTHP_LOST_ON_THE_RANGE, false),
        (cards::CONSPICUOUS_SNOOP, true),
        (cards::AUGUR_OF_AUTUMN, false),
        (cards::BENJAMIN_SISKO_BESIEGED, true),
        (cards::BOLASS_CITADEL, false),
        (cards::COURSER_OF_KRUPHIX, true),
        (cards::CRYSTAL_SKULL_ISU_SPYGLASS, false),
        (cards::EXPERIMENTAL_FRENZY, false),
        (cards::FUTURE_SIGHT, true),
        (cards::GLARB_CALAMITY_S_AUGUR, false),
        (cards::ISU_THE_ABOMINABLE, false),
        (cards::MAGUS_OF_THE_FUTURE, true),
        (cards::MIKEY_DON_PARTY_PLANNERS, false),
        (cards::MUL_DAYA_CHANNELERS, true),
        (cards::ORACLE_OF_MUL_DAYA, true),
        (cards::THE_FOURTH_DOCTOR, false),
        (cards::THE_REALITY_CHIP, false),
        (cards::TRAVELING_CHOCOBO, false),
        (cards::VERGE_RANGERS, false),
    ] {
        let mut game = staged(&[source], &[cards::LIGHTNING_BOLT]);
        let shown = Some((top(&game), cards::LIGHTNING_BOLT));
        assert_eq!(
            game.observe(PlayerId::One).revealed_library_top,
            shown,
            "{source:?}"
        );
        assert_eq!(
            game.observe(PlayerId::Two).opponent_revealed_library_top,
            if public { shown } else { None },
            "{source:?}"
        );
        assert_eq!(
            restored(&game).observe(PlayerId::One).revealed_library_top,
            shown
        );
        game.players[0].library.clear();
        assert_eq!(game.observe(PlayerId::One).revealed_library_top, None);
        assert!(!game.legal_actions(PlayerId::One).iter().any(|a| matches!(
            a,
            Action::PlayLand { .. } | Action::CastSpell { .. } | Action::Plot { .. }
        )));
    }
}

#[test]
fn library_permissions_match_the_spell_being_cast_and_keep_normal_timing() {
    for (source, accepted, rejected) in [
        (
            cards::BENJAMIN_SISKO_BESIEGED,
            cards::GRIZZLY_BEARS,
            cards::SOL_RING,
        ),
        (
            cards::CONSPICUOUS_SNOOP,
            cards::KIKI_JIKI_MIRROR_BREAKER,
            cards::GRIZZLY_BEARS,
        ),
        (
            cards::ISU_THE_ABOMINABLE,
            cards::BOREAL_DRUID,
            cards::LLANOWAR_ELVES,
        ),
        (
            cards::CRYSTAL_SKULL_ISU_SPYGLASS,
            cards::SOL_RING,
            cards::LIGHTNING_BOLT,
        ),
        (
            cards::THE_FOURTH_DOCTOR,
            cards::SOL_RING,
            cards::LIGHTNING_BOLT,
        ),
        (
            cards::MIKEY_DON_PARTY_PLANNERS,
            cards::NINJA_OF_THE_DEEP_HOURS,
            cards::GRIZZLY_BEARS,
        ),
        (
            cards::TRAVELING_CHOCOBO,
            cards::BIRDS_OF_PARADISE,
            cards::GRIZZLY_BEARS,
        ),
    ] {
        let mut game = staged(&[source], &[rejected, accepted]);
        mana(&mut game);
        assert!(!casts(&game).is_empty(), "{source:?}");
        game.step = Step::Upkeep;
        assert!(
            casts(&game).is_empty(),
            "{source:?} must keep sorcery timing"
        );
        game.step = Step::PrecombatMain;
        game.players[0].library.pop();
        assert!(casts(&game).is_empty(), "{source:?} must reject this card");
    }
    for source in [
        cards::FUTURE_SIGHT,
        cards::MAGUS_OF_THE_FUTURE,
        cards::EXPERIMENTAL_FRENZY,
    ] {
        let mut game = staged(&[source], &[cards::LIGHTNING_BOLT]);
        mana(&mut game);
        game.step = Step::Upkeep;
        assert!(!casts(&game).is_empty());
    }
}

#[test]
fn oracle_adds_one_land_play_and_courser_triggers_for_library_lands() {
    let mut game = staged(
        &[cards::ORACLE_OF_MUL_DAYA, cards::COURSER_OF_KRUPHIX],
        &[cards::FOREST, cards::FOREST, cards::FOREST],
    );
    let life = game.players[0].life;
    for _ in 0..2 {
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::PlayLand { .. }))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
    }
    assert_eq!(game.players[0].life, life + 2);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::PlayLand { .. }))
    );
}

#[test]
fn isu_can_pay_each_listed_color_and_only_triggers_for_other_controlled_snow() {
    for color in [ManaColor::Green, ManaColor::White, ManaColor::Blue] {
        let mut game = staged(&[cards::ISU_THE_ABOMINABLE], &[cards::SNOW_COVERED_FOREST]);
        game.add_unrestricted_mana(PlayerId::One, color, 1);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::PlayLand { .. }))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        let decision = game.pending_decisions[0].observation.clone();
        let paid = decision
            .options
            .iter()
            .find(|option| option.id != 0)
            .unwrap()
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![paid],
            },
        )
        .unwrap();
        settle(&mut game);
        assert_eq!(game.battlefield[0].counters(CounterKind::PlusOnePlusOne), 1);
        game.put_onto_battlefield(PlayerId::Two, cards::SNOW_COVERED_ISLAND)
            .unwrap();
        assert!(game.pending_triggers.is_empty());
    }
}

#[test]
fn fblthp_uses_the_shared_plotted_designation_and_free_cast_rules() {
    for prepared in [false, true] {
        let mut game = staged(
            &[cards::FBLTHP_LOST_ON_THE_RANGE],
            &[cards::FOREST, cards::LIGHTNING_BOLT],
        );
        game.set_prepared_engine_enabled(prepared);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        game.apply(PlayerId::One, Action::Plot { card: top(&game) })
            .unwrap();
        let exiled = game.players[0].exile[0].id;
        assert!(game.plotted_cards.contains_key(&exiled));
        assert!(game.stack.is_empty());
        assert!(
            !game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|a| matches!(a, Action::CastSpell { card, .. } if *card == exiled))
        );
        game.battlefield.clear();
        game.turns_started[0] += 1;
        let mut game = restored(&game);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == exiled))
            .expect("designation survives its grant and checkpoint");
        game.apply(PlayerId::One, action).unwrap();
        assert!(game.players[0].exile.is_empty());
        assert_eq!(game.players[0].mana_pool.total(), 0);
    }
}

#[test]
fn fblthp_preserves_a_cards_printed_plot_cost_as_an_alternative() {
    let mut game = staged(
        &[cards::FBLTHP_LOST_ON_THE_RANGE],
        &[cards::PILLAGE_THE_BOG],
    );
    mana(&mut game);
    game.apply(PlayerId::One, Action::Plot { card: top(&game) })
        .unwrap();
    assert_eq!(game.pending_decisions[0].observation.options.len(), 2);
    let mut game = restored(&game);
    settle(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::PILLAGE_THE_BOG)
    );
}

#[test]
fn reality_chip_permission_follows_reconfigure_and_skull_produces_blue_mana() {
    let mut game = staged(
        &[cards::THE_REALITY_CHIP, cards::GRIZZLY_BEARS],
        &[cards::LIGHTNING_BOLT],
    );
    mana(&mut game);
    assert!(casts(&game).is_empty());
    let chip = game.battlefield[0].card.id;
    let host = game.battlefield[1].card.id;
    let attach = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility { source, targets, .. } if *source == chip && targets.iter().any(|t| t.targets().contains(&Target::Permanent(host))))).unwrap();
    game.apply(PlayerId::One, attach).unwrap();
    settle(&mut game);
    assert!(!casts(&game).is_empty());
    assert!(
        !game
            .permanent_types(&game.battlefield[0])
            .unwrap()
            .contains(CardType::Creature)
    );
    let unattach = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility { source, targets, .. } if *source == chip && targets.iter().all(|t| t.targets().is_empty()))).unwrap();
    game.apply(PlayerId::One, unattach).unwrap();
    settle(&mut game);
    assert!(casts(&game).is_empty());
    let mut game = staged(&[cards::CRYSTAL_SKULL_ISU_SPYGLASS], &[cards::FOREST]);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateManaAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.blue, 1);
}

#[test]
fn frenzy_destroys_itself_and_restores_hand_access() {
    let mut game = staged(&[cards::EXPERIMENTAL_FRENZY], &[cards::FOREST]);
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .unwrap();
    mana(&mut game);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.battlefield.is_empty());
    assert!(game.legal_actions(PlayerId::One).iter().any(
        |a| matches!(a, Action::CastSpell { card, .. } if *card == game.players[0].hand[0].id)
    ));
}

#[test]
fn fblthp_can_be_sacrificed_for_the_mana_to_pay_the_selected_plot_cost() {
    let mut game = staged(
        &[cards::FBLTHP_LOST_ON_THE_RANGE, cards::PHYREXIAN_ALTAR],
        &[cards::LIGHTNING_BOLT],
    );
    let card = top(&game);
    game.apply(PlayerId::One, Action::Plot { card }).unwrap();
    settle(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::LIGHTNING_BOLT)
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::FBLTHP_LOST_ON_THE_RANGE)
    );
}

#[test]
fn coven_and_conditional_top_card_stats_share_live_characteristics() {
    for prepared in [false, true] {
        let mut game = staged(
            &[
                cards::AUGUR_OF_AUTUMN,
                cards::MUL_DAYA_CHANNELERS,
                cards::LLANOWAR_ELVES,
            ],
            &[cards::GRIZZLY_BEARS],
        );
        game.set_prepared_engine_enabled(prepared);
        mana(&mut game);
        assert!(!casts(&game).is_empty());
        assert_eq!(game.power(&game.battlefield[1]), Some(5));
    }
}

#[test]
fn citadel_keeps_additional_costs_but_disallows_other_alternative_costs() {
    let mut game = staged(&[cards::BOLASS_CITADEL], &[cards::DAUNTLESS_UNITY]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    let action = casts(&game).into_iter().find(|a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().alternative().is_some())).expect("legacy kicker remains an additional cost");
    let life = game.players[0].life;
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].life, life - 2);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    let mut game = staged(&[cards::BOLASS_CITADEL], &[cards::FORCE_OF_WILL]);
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::COUNTERSPELL])
        .unwrap();
    // Put a target on the stack so Force has a legal target.
    let target = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .unwrap()
        .remove(0);
    game.players[1].hand.push(target.clone());
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == target.id))
        .unwrap();
    game.apply(PlayerId::Two, action).unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let actions = casts(&game);
    assert!(!actions.is_empty());
    assert!(actions.iter().all(|a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().alternative().is_none())));
}

#[test]
fn library_permission_checks_face_down_characteristics_before_announcing_a_spell() {
    for (source, can_morph) in [
        (cards::GLARB_CALAMITY_S_AUGUR, false),
        (cards::FUTURE_SIGHT, true),
    ] {
        let mut game = staged(&[source], &[cards::EXALTED_ANGEL]);
        mana(&mut game);
        let actions = casts(&game);
        assert!(!actions.is_empty());
        assert_eq!(actions.iter().any(|a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().alternative().is_some())), can_morph);
    }
}

#[test]
fn the_next_library_card_is_not_revealed_during_interrupted_cast_payment() {
    let mut game = staged(
        &[
            cards::FUTURE_SIGHT,
            cards::UGINS_NEXUS,
            cards::REST_IN_PEACE,
        ],
        &[cards::FOREST, cards::KULDOTHA_REBIRTH],
    );
    mana(&mut game);
    let action = casts(&game).into_iter().next().unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(!game.pending_decisions.is_empty());
    assert_eq!(game.observe(PlayerId::One).revealed_library_top, None);
    assert_eq!(
        game.observe(PlayerId::Two).opponent_revealed_library_top,
        None
    );
    settle(&mut game);
    assert_eq!(
        game.observe(PlayerId::One).revealed_library_top,
        Some((top(&game), cards::FOREST))
    );
}
