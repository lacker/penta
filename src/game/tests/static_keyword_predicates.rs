//! Identities whose only gap was a keyword predicate blind to static grants.
//!
//! Each of these cards asks "does this creature have flying?" from a place
//! that used to read printed and resolved abilities alone: a static power and
//! toughness effect, a blocking restriction, target legality, or a damage
//! sweep. `Game::keyword_mask` now answers all of them from the same ability
//! set the combat rules use, so every test here pairs the printed case with a
//! creature whose flying comes from a live static effect.
//!
//! Galerider Sliver ("Sliver creatures you control have flying") is the
//! granter throughout, and Blur Sliver is the 2/2 Sliver that receives it —
//! its own static ability hands out haste, so it perturbs neither power nor
//! flying.

use super::modal_effects::cast_mode;
use super::search_and_reveal::stack_library;
use super::*;

/// The battlefield every flying test wants: a receiver whose flying is
/// printed, one whose flying is granted by a static effect, and one with
/// neither. `granter` decides whether the static grant is actually live.
fn sliver_board(game: &mut Game, controller: PlayerId, granter: bool) {
    let base = match controller {
        PlayerId::One => 10_000,
        PlayerId::Two => 10_010,
    };
    game.battlefield
        .push(creature(base, cards::BLUR_SLIVER, controller));
    game.battlefield
        .push(creature(base + 1, cards::SAVANNAH_LIONS, controller));
    game.battlefield
        .push(creature(base + 2, cards::SERRA_ANGEL, controller));
    if granter {
        game.battlefield
            .push(creature(base + 3, cards::GALERIDER_SLIVER, controller));
    }
}

fn stats(game: &Game, id: u32) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(id))
        .expect("still on the battlefield");
    (
        game.power(permanent).expect("a creature has power"),
        game.toughness(permanent).expect("a creature has toughness"),
    )
}

fn damage(game: &Game, id: u32) -> Option<u16> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(id))
        .map(|permanent| permanent.damage)
}

/// Every permanent a card in hand can legally be aimed at, read off the real
/// cast actions rather than the card's declaration.
fn castable_targets(game: &Game, player: PlayerId, spell: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(player)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => {
                choices.iter_targets().find_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    _ => None,
                })
            }
            _ => None,
        })
        .collect()
}

fn can_block(game: &Game, blocker: u32) -> bool {
    game.legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::DeclareBlocker { blocker: b, .. } if *b == CardInstanceId(blocker)))
}

/// Sets up an attack by player one so player two's blocker list can be read.
fn attacking_game(attacker: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut permanent = creature(10_500, attacker, PlayerId::One);
    permanent.attacking = true;
    permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(permanent);
    game
}

// AVR 51 — Favorable Winds

#[test]
fn favorable_winds_pumps_your_fliers_including_a_statically_granted_one() {
    for granter in [false, true] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_100, cards::FAVORABLE_WINDS, PlayerId::One));
        sliver_board(&mut game, PlayerId::One, granter);
        // An opposing flier is never yours, granted or not.
        game.battlefield
            .push(creature(10_200, cards::SERRA_ANGEL, PlayerId::Two));

        assert_eq!(
            stats(&game, 10_000),
            if granter { (3, 3) } else { (2, 2) },
            "a 2/2 Sliver is pumped exactly when Galerider Sliver is granting it flying"
        );
        assert_eq!(stats(&game, 10_001), (2, 1), "a ground creature is not");
        assert_eq!(stats(&game, 10_002), (5, 5), "a printed flier always is");
        assert_eq!(stats(&game, 10_200), (4, 4), "an opponent's flier is not");
    }
}

// AVR 159 — Thunderbolt

#[test]
fn thunderbolt_hits_a_player_for_three_or_a_flier_for_four() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let bolt = card(10_100, cards::THUNDERBOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 2;

    // The flying mode reaches the granted flier and the printed one, but not
    // the Lions or the Sliver lord's ground-bound neighbours.
    let flying_mode_targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == bolt.id && choices.modes() == [ModeId(1)] =>
            {
                choices.iter_targets().find_map(|target| match target {
                    Target::Permanent(id) => Some(id.0),
                    _ => None,
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        flying_mode_targets,
        vec![10_010, 10_012, 10_013],
        "the Sliver with granted flying, Serra Angel, and Galerider Sliver itself"
    );

    game.apply(
        PlayerId::One,
        cast_mode(bolt.id, ModeId(0), vec![Target::Player(PlayerId::Two)]),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 17, "the player mode deals three");
}

#[test]
fn thunderbolts_flying_mode_deals_four_to_a_granted_flier() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let bolt = card(10_100, cards::THUNDERBOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        cast_mode(
            bolt.id,
            ModeId(1),
            vec![Target::Permanent(CardInstanceId(10_010))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        damage(&game, 10_010),
        None,
        "four damage kills the 2/2 the grant made a legal target"
    );
}

// AVR 170 — Bower Passage

#[test]
fn bower_passage_stops_fliers_from_blocking_your_creatures() {
    for granter in [false, true] {
        let mut game = attacking_game(cards::SAVANNAH_LIONS);
        game.battlefield
            .push(creature(10_100, cards::BOWER_PASSAGE, PlayerId::One));
        sliver_board(&mut game, PlayerId::Two, granter);

        assert!(
            can_block(&game, 10_011),
            "a ground creature blocks as usual"
        );
        assert!(!can_block(&game, 10_012), "a printed flier cannot block");
        assert_eq!(
            can_block(&game, 10_010),
            !granter,
            "the 2/2 Sliver loses the ability to block exactly when it is granted flying"
        );
    }
}

#[test]
fn bower_passage_only_protects_its_controllers_creatures() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    // Player two owns the Passage, so player one's attacker is unprotected.
    game.battlefield
        .push(creature(10_100, cards::BOWER_PASSAGE, PlayerId::Two));
    let mut attacker = creature(10_500, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);
    sliver_board(&mut game, PlayerId::Two, true);

    assert!(can_block(&game, 10_012), "a flier may still block this");
    assert!(can_block(&game, 10_010), "and so may the granted flier");
}

// GTC 79 — Smog Elemental

#[test]
fn smog_elemental_shrinks_opposing_fliers_including_granted_ones() {
    for granter in [false, true] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_100, cards::SMOG_ELEMENTAL, PlayerId::Two));
        sliver_board(&mut game, PlayerId::One, granter);

        assert_eq!(
            stats(&game, 10_000),
            if granter { (1, 1) } else { (2, 2) },
            "a granted flier is shrunk by a static effect keyed on flying"
        );
        assert_eq!(stats(&game, 10_001), (2, 1), "a ground creature is not");
        assert_eq!(stats(&game, 10_002), (3, 3), "a printed flier is");
        assert_eq!(
            stats(&game, 10_100),
            (3, 3),
            "the Elemental flies but is not its own opponent"
        );
    }
}

// GTC 135 — Spire Tracer

#[test]
fn spire_tracer_is_blocked_only_by_flying_or_reach() {
    for granter in [false, true] {
        let mut game = attacking_game(cards::SPIRE_TRACER);
        sliver_board(&mut game, PlayerId::Two, granter);
        game.battlefield
            .push(creature(10_100, cards::GIANT_SPIDER, PlayerId::Two));

        assert!(!can_block(&game, 10_011), "a ground creature cannot");
        assert!(can_block(&game, 10_012), "a printed flier can");
        assert!(can_block(&game, 10_100), "and so can reach");
        assert_eq!(
            can_block(&game, 10_010),
            granter,
            "the 2/2 Sliver may block exactly when it is granted flying"
        );
    }
}

// GTC 151 — Clan Defiance

#[test]
fn clan_defiance_can_choose_all_three_modes_at_once() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let defiance = card(10_100, cards::CLAN_DEFIANCE, PlayerId::One);
    game.players[0].hand.push(defiance.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.green = 3;

    let choices = CastChoices::default()
        .with_modes(vec![ModeId(0), ModeId(1), ModeId(2)])
        .with_x(2)
        .with_targets(vec![
            // Slot order follows the selected modes: flier, nonflier, player.
            TargetSelection::single(TargetSlotId(0), Target::Permanent(CardInstanceId(10_012))),
            TargetSelection::single(TargetSlotId(1), Target::Permanent(CardInstanceId(10_011))),
            TargetSelection::single(TargetSlotId(2), Target::Player(PlayerId::Two)),
        ]);
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: defiance.id,
            choices,
            sacrifices: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(damage(&game, 10_012), Some(2), "the flier took X");
    assert_eq!(
        damage(&game, 10_011),
        None,
        "two damage kills the 2/1 nonflier"
    );
    assert_eq!(game.players[1].life, 18, "and the player took X");
}

#[test]
fn clan_defiances_flying_modes_split_on_a_static_grant() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let defiance = card(10_100, cards::CLAN_DEFIANCE, PlayerId::One);
    game.players[0].hand.push(defiance.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.green = 3;

    let mode_targets = |game: &Game, mode: ModeId| {
        let mut targets = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. }
                    if card == defiance.id && choices.modes() == [mode] =>
                {
                    choices.iter_targets().find_map(|target| match target {
                        Target::Permanent(id) => Some(id.0),
                        _ => None,
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        targets.sort_unstable();
        targets.dedup();
        targets
    };

    assert_eq!(
        mode_targets(&game, ModeId(0)),
        vec![10_010, 10_012, 10_013],
        "the granted flier joins the printed ones"
    );
    assert_eq!(
        mode_targets(&game, ModeId(1)),
        vec![10_011],
        "and leaves the nonflying mode with only the Lions"
    );
}

// M13 140 — Magmaquake

#[test]
fn magmaquake_sweeps_nonfliers_and_planeswalkers_but_not_players() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let mut domri = creature(10_100, cards::DOMRI_RADE, PlayerId::Two);
    domri.set_counters(CounterKind::Loyalty, 3);
    game.battlefield.push(domri);
    let quake = card(10_200, cards::MAGMAQUAKE, PlayerId::One);
    game.players[0].hand.push(quake.clone());
    game.players[0].mana_pool.red = 4;

    game.apply(
        PlayerId::One,
        cast_action(quake.id, Vec::new(), Vec::new(), 2),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        damage(&game, 10_010),
        Some(0),
        "the granted flier is excluded"
    );
    assert_eq!(damage(&game, 10_012), Some(0), "so is the printed flier");
    assert_eq!(damage(&game, 10_011), None, "two damage kills the 2/1");
    let loyalty = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(10_100))
        .map(|permanent| permanent.counters(CounterKind::Loyalty));
    assert_eq!(loyalty, Some(1), "a planeswalker loses X loyalty");
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[1].life, 20, "no player is dealt damage");
}

// M13 177 — Mwonvuli Beast Tracker

#[test]
fn mwonvuli_beast_tracker_finds_only_the_four_named_keywords() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (11_000, cards::SAVANNAH_LIONS),
            (11_001, cards::GIANT_SPIDER),
            (11_002, cards::SERRA_ANGEL),
            (11_003, cards::DEADLY_RECLUSE),
            (11_004, cards::LIGHTNING_BOLT),
        ],
    );
    let tracker = card(11_100, cards::MWONVULI_BEAST_TRACKER, PlayerId::One);
    game.players[0].hand.push(tracker.clone());
    game.players[0].mana_pool.green = 2;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(tracker.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let mut offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    offered.sort_unstable_by_key(|definition| definition.get());
    assert_eq!(
        offered,
        vec![cards::GIANT_SPIDER, cards::DEADLY_RECLUSE],
        "reach and deathtouch qualify; a vanilla creature, a flier, and an instant do not"
    );

    let spider = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::GIANT_SPIDER)
            })
        })
        .unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![spider.id],
        },
    )
    .unwrap();

    assert_eq!(
        game.players[0].library.last().map(|card| card.definition),
        Some(cards::GIANT_SPIDER),
        "the found card is put on top after the shuffle"
    );
}

// M13 179 — Plummet

#[test]
fn plummet_can_target_only_a_creature_with_flying() {
    for granter in [false, true] {
        let mut game = ready_game();
        sliver_board(&mut game, PlayerId::Two, granter);
        let plummet = card(10_100, cards::PLUMMET, PlayerId::One);
        game.players[0].hand.push(plummet.clone());
        game.players[0].mana_pool.green = 2;

        let mut targets = castable_targets(&game, PlayerId::One, plummet.id)
            .into_iter()
            .map(|id| id.0)
            .collect::<Vec<_>>();
        targets.sort_unstable();
        assert_eq!(
            targets,
            if granter {
                vec![10_010, 10_012, 10_013]
            } else {
                vec![10_012]
            },
            "the static grant makes the Slivers legal targets"
        );
    }
}

#[test]
fn plummet_destroys_a_creature_whose_flying_is_granted() {
    let mut game = ready_game();
    sliver_board(&mut game, PlayerId::Two, true);
    let plummet = card(10_100, cards::PLUMMET, PlayerId::One);
    game.players[0].hand.push(plummet.clone());
    game.players[0].mana_pool.green = 2;

    game.apply(
        PlayerId::One,
        cast_action(
            plummet.id,
            vec![Target::Permanent(CardInstanceId(10_010))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(10_010)),
        "the granted flier is destroyed"
    );
}

// M13 191 — Silklash Spider

#[test]
fn silklash_spider_deals_x_to_every_flier_and_never_to_itself() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_100, cards::SILKLASH_SPIDER, PlayerId::One));
    sliver_board(&mut game, PlayerId::Two, true);
    game.players[0].mana_pool.green = 4;

    let spider = game.battlefield[0].clone();
    let ability = game.activated_ability_origin(&spider, 0);
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: spider.card.id,
            ability,
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 2,
            modes: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        damage(&game, 10_010),
        None,
        "two damage kills the 2/2 the grant put in range"
    );
    assert_eq!(damage(&game, 10_012), Some(2), "a printed flier takes X");
    assert_eq!(damage(&game, 10_011), Some(0), "a ground creature does not");
    assert_eq!(
        damage(&game, 10_100),
        Some(0),
        "reach is not flying, so the Spider spares itself"
    );
}
