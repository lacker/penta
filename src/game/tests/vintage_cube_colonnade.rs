//! Celestial Colonnade: a land that costs you a turn and then attacks for
//! four in the air without tapping.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let colonnade = game
        .put_onto_battlefield(PlayerId::One, cards::CELESTIAL_COLONNADE)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == colonnade)
    {
        permanent.tapped = false;
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    (game, colonnade)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Activates the animation.
fn animate(game: &mut Game, colonnade: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == colonnade))
        .expect("five mana animates it");
    game.apply(PlayerId::One, animate).expect("it activates");
    resolve(game);
}

/// It taps for either of its colours and nothing else.
#[test]
fn it_taps_for_white_or_blue() {
    let (game, colonnade) = staged();
    let colors = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == colonnade => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(colors.contains(&ManaColor::White));
    assert!(colors.contains(&ManaColor::Blue));
    assert_eq!(colors.len(), 2);
}

/// Unanimated it is a land and nothing else: no body, and no attacking.
#[test]
fn a_land_is_not_a_creature() {
    let (mut game, colonnade) = staged();
    let types = game
        .permanent_types(permanent(&game, colonnade))
        .expect("it has types");

    assert!(types.contains(CardType::Land));
    assert!(!types.contains(CardType::Creature));

    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(
                action,
                Action::DeclareAttacker { attacker, .. } if *attacker == colonnade
            )),
        "a land does not attack",
    );
}

/// Animated it is a 4/4 Elemental that is still a land.
#[test]
fn animating_it_makes_a_four_four_that_is_still_a_land() {
    let (mut game, colonnade) = staged();
    animate(&mut game, colonnade);

    let types = game
        .permanent_types(permanent(&game, colonnade))
        .expect("it has types");
    assert!(types.contains(CardType::Creature), "a creature now");
    assert!(types.contains(CardType::Land), "and still a land");
    assert_eq!(game.power(permanent(&game, colonnade)), Some(4));
    assert_eq!(game.toughness(permanent(&game, colonnade)), Some(4));
}

/// It comes with flying and vigilance, which is what makes it a threat
/// rather than a body.
#[test]
fn the_animation_grants_flying_and_vigilance() {
    let (mut game, colonnade) = staged();
    animate(&mut game, colonnade);

    for keyword in [KeywordAbility::Flying, KeywordAbility::Vigilance] {
        assert!(
            game.permanent_has_executable_keyword(permanent(&game, colonnade), keyword),
            "{keyword:?} comes with the animation",
        );
    }
}

/// The animation lasts until end of turn and no longer.
#[test]
fn the_body_goes_away_at_end_of_turn() {
    let (mut game, colonnade) = staged();
    animate(&mut game, colonnade);
    assert!(
        game.permanent_types(permanent(&game, colonnade))
            .is_some_and(|types| types.contains(CardType::Creature)),
    );

    game.active_player = PlayerId::One;
    game.step = Step::End;
    game.advance_step();
    drain_pending(&mut game);

    assert!(
        game.permanent_types(permanent(&game, colonnade))
            .is_some_and(|types| !types.contains(CardType::Creature)),
        "a land again once the turn is over",
    );
}

/// The price of the land: it arrives tapped and does nothing the turn it
/// comes down.
#[test]
fn it_enters_tapped() {
    let mut game = ready_game();
    game.battlefield.clear();
    let colonnade = game
        .build_zone(PlayerId::One, &[cards::CELESTIAL_COLONNADE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = colonnade.id;
    game.players[0].hand.push(colonnade);
    game.priority = PlayerId::One;
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == id))
        .expect("a land drop is available");
    game.apply(PlayerId::One, play)
        .expect("the land is playable");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::CELESTIAL_COLONNADE)
            .is_some_and(|permanent| permanent.tapped),
    );
}

/// "Summoning sickness cares about when that permanent came under your
/// control, not when it became a creature." A Colonnade that arrived this
/// turn animates and still cannot attack -- or tap for its own mana.
#[test]
fn a_land_that_arrived_this_turn_is_summoning_sick() {
    let (mut game, colonnade) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == colonnade)
    {
        permanent.entered_controller_turn = game.turns_started[0];
    }

    animate(&mut game, colonnade);
    assert_eq!(
        game.power(permanent(&game, colonnade)),
        Some(4),
        "it is a 4/4 all the same",
    );

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(
                action,
                Action::DeclareAttacker { attacker, .. } if *attacker == colonnade
            )),
        "but it did not begin the turn here, so it does not attack",
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == colonnade
            )),
        "and a creature's tap is a creature's tap, mana ability or not",
    );
}

/// "When a land becomes a creature, that doesn't count as having a creature
/// enter." A Champion of Lambholt watching for arrivals sees nothing.
#[test]
fn animating_it_is_not_a_creature_entering() {
    let (mut game, colonnade) = staged();
    let champion = game
        .put_onto_battlefield(PlayerId::One, cards::CHAMPION_OF_LAMBHOLT)
        .expect("cataloged");
    drain_pending(&mut game);
    let before = permanent(&game, champion).counters(CounterKind::PlusOnePlusOne);

    animate(&mut game, colonnade);

    assert_eq!(
        permanent(&game, champion).counters(CounterKind::PlusOnePlusOne),
        before,
        "the land was already there; it only changed what it is",
    );
}

/// "Once Celestial Colonnade has attacked, tapping it for mana won't remove
/// it from combat." Vigilance leaves it untapped to be tapped, and it is
/// still an attacker afterwards.
#[test]
fn tapping_it_for_mana_does_not_take_it_out_of_combat() {
    let (mut game, colonnade) = staged();
    animate(&mut game, colonnade);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: colonnade,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a 4/4 with vigilance attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    assert!(
        !permanent(&game, colonnade).tapped,
        "vigilance kept it untapped",
    );

    game.empty_mana_pools();
    let white = Action::ActivateManaAbility {
        source: colonnade,
        ability: mana_ability_for(&game, colonnade, ManaColor::White),
        color: ManaColor::White,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, white)
        .expect("an untapped land taps for mana, attacking or not");

    assert_eq!(game.players[0].mana_pool.white, 1);
    let attacking = permanent(&game, colonnade);
    assert!(attacking.tapped, "it is tapped now");
    assert!(
        attacking.attacking,
        "and still attacking: tapping an attacker does not call it home",
    );
    assert_eq!(
        attacking.attack_defender,
        Some(AttackDefender::Player(PlayerId::Two)),
    );
}

/// "A 4/4 white and blue Elemental creature": a land is colourless until it
/// is woken, and what wakes it says which colours it is.
#[test]
fn the_body_it_wakes_up_as_is_white_and_blue() {
    let (mut game, colonnade) = staged();
    let colors = |game: &Game| game.permanent_colors(permanent(game, colonnade));
    assert!(
        colors(&game).iter().all(|present| !present),
        "a land is colourless",
    );

    animate(&mut game, colonnade);

    for color in ManaColor::COLORS {
        let index = color.color_index().expect("a colour has an index");
        assert_eq!(
            colors(&game)[index],
            color == ManaColor::White || color == ManaColor::Blue,
            "{color:?} on the animated land",
        );
    }
    assert!(
        game.effective_subtypes(permanent(&game, colonnade))
            .contains(crate::card::Subtype::named("Elemental")),
        "and it is an Elemental while it lasts",
    );
}

/// The animation sets a base rather than adding to one, and the ability
/// carries no restriction on how often it is bought: five mana twice over
/// is the same 4/4, five mana poorer.
#[test]
fn animating_it_twice_is_still_a_four_four() {
    let (mut game, colonnade) = staged();
    animate(&mut game, colonnade);
    assert_eq!(game.power(permanent(&game, colonnade)), Some(4));

    animate(&mut game, colonnade);

    assert_eq!(
        (
            game.power(permanent(&game, colonnade)),
            game.toughness(permanent(&game, colonnade))
        ),
        (Some(4), Some(4)),
        "the second animation sets the same base rather than stacking",
    );
}

/// Summoning sickness stops attacking and tapping, and nothing else: a
/// Colonnade that arrived this turn may still be woken up on their turn to
/// block, which is the mode the card is really held open for.
#[test]
fn a_freshly_played_colonnade_may_still_block() {
    let (mut game, colonnade) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == colonnade)
    {
        permanent.entered_controller_turn = game.turns_started[0];
    }
    let attacker = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
    {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);

    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::Two;
    game.declare_attacker(attacker, AttackDefender::Player(PlayerId::One));
    game.finish_declaring_attackers();
    resolve(&mut game);
    game.priority = PlayerId::One;

    animate(&mut game, colonnade);
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(
                action,
                Action::DeclareBlocker { blocker, .. } if *blocker == colonnade
            )),
        "blocking asks nothing about how long it has been here",
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == colonnade
            )),
        "while it is summoning sick still, its own tap being the proof",
    );
}

/// The price of waking it: it is a creature now, so it dies like one, and
/// the land dies with it. Four damage is exactly its toughness.
#[test]
fn four_damage_kills_it_and_takes_the_land_with_it() {
    let (mut game, colonnade) = staged();

    animate(&mut game, colonnade);
    game.damage_target_from_kind(None, Some(Target::Permanent(colonnade)), 3, false);
    game.check_state_based_actions();
    assert_eq!(
        game.power(permanent(&game, colonnade)),
        Some(4),
        "three is not enough",
    );

    game.damage_target_from_kind(None, Some(Target::Permanent(colonnade)), 1, false);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != colonnade),
        "the fourth point killed the creature",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CELESTIAL_COLONNADE),
        "and the land it still was went with it",
    );
}

/// "{3}{W}{U}": the two pips are real, and five colourless will not wake it.
/// What can pay one of them is the Colonnade itself -- it taps for white or
/// blue like any other turn -- and the price of that is a 4/4 that is
/// already tapped and cannot attack with the body it just bought.
#[test]
fn it_can_tap_itself_for_a_pip_and_is_tapped_for_it() {
    let (mut game, colonnade) = staged();
    let offered = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::ActivateAbility { source, .. } if source == colonnade))
    };

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);
    assert!(
        !offered(&game),
        "five colourless and a land that makes one pip is still a pip short",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    assert!(
        offered(&game),
        "with the white in hand the Colonnade taps for the blue itself",
    );

    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == colonnade))
        .expect("just checked");
    game.apply(PlayerId::One, animate).expect("it activates");
    resolve(&mut game);

    let woken = permanent(&game, colonnade);
    assert_eq!(game.power(woken), Some(4), "a 4/4 all the same");
    assert!(
        woken.tapped,
        "and tapped, having paid a pip of its own animation",
    );

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::DeclareAttacker { attacker, .. } if attacker == colonnade)),
        "so the body it bought stays home this turn",
    );
}
