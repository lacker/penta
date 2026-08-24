//! Printed "Activate only during ..." windows.
//!
//! The restriction narrows when an ability may be activated and says nothing
//! about priority, so these drive it the way a seat meets it: by asking what
//! the legal-action list offers in each step and on each player's turn.

use super::*;
use crate::ImplementationStatus;

fn offers(game: &Game, player: PlayerId, source: GameObjectId) -> bool {
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn scepter_game() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let scepter = creature(10_000, cards::DISRUPTING_SCEPTER, PlayerId::One);
    let scepter_id = scepter.card.id;
    game.battlefield.push(scepter);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.players[PlayerId::Two.index()]
        .hand
        .push(card(10_500, cards::SEDGE_TROLL, PlayerId::Two));
    (game, scepter_id)
}

/// "Only during your turn" is about whose turn it is, not which step, so it
/// holds all the way through a turn its controller is taking.
#[test]
fn a_your_turn_ability_is_offered_in_every_step_of_your_own_turn() {
    let (mut game, scepter_id) = scepter_game();
    // Declare-attackers is excluded because the active player owes the game
    // an attack declaration there before anyone holds priority; that is a
    // priority rule, not this window.
    for step in [
        Step::Upkeep,
        Step::PrecombatMain,
        Step::PostcombatMain,
        Step::End,
    ] {
        game.step = step;
        game.priority = PlayerId::One;
        assert!(
            offers(&game, PlayerId::One, scepter_id),
            "the window is open in {step:?}"
        );
    }
}

#[test]
fn a_your_turn_ability_is_not_offered_on_the_opposing_turn() {
    let (mut game, scepter_id) = scepter_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    assert!(
        !offers(&game, PlayerId::One, scepter_id),
        "the controller's own turn is the whole window"
    );
}

/// The restriction follows the ability, not the permanent: an opponent who
/// somehow held priority still could not use it, and neither can its
/// controller once the turn has passed.
#[test]
fn an_upkeep_ability_is_offered_only_in_your_own_upkeep() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let smith = creature(10_000, cards::DWARVEN_WEAPONSMITH, PlayerId::One);
    let smith_id = smith.card.id;
    game.battlefield.push(smith);
    // Mishra's Factory is a Land until it animates, so the artifact this
    // cost eats has to be one that is printed as such.
    game.battlefield
        .push(creature(10_001, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SEDGE_TROLL, PlayerId::One));

    game.step = Step::PrecombatMain;
    assert!(
        !offers(&game, PlayerId::One, smith_id),
        "a main phase is not an upkeep"
    );

    game.step = Step::Upkeep;
    assert!(
        offers(&game, PlayerId::One, smith_id),
        "its own upkeep is the window"
    );

    game.active_player = PlayerId::Two;
    assert!(
        !offers(&game, PlayerId::One, smith_id),
        "and it has to be a turn its controller is taking"
    );
}

/// The window gates the ability, not the card: everything else about the
/// activation still has to hold, so a tapped source is still unavailable in
/// the open window.
#[test]
fn the_window_does_not_excuse_the_rest_of_the_cost() {
    let (mut game, scepter_id) = scepter_game();
    game.step = Step::PrecombatMain;
    assert!(offers(&game, PlayerId::One, scepter_id));

    if let Some(scepter) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == scepter_id)
    {
        scepter.tapped = true;
    }
    assert!(
        !offers(&game, PlayerId::One, scepter_id),
        "an open window does not untap the source"
    );
}

#[test]
fn every_timing_restricted_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::DISRUPTING_SCEPTER,
        cards::DWARVEN_WEAPONSMITH,
        cards::SVYELUNITE_PRIEST,
        cards::GWENDLYN_DI_CORCI,
        cards::BRAIN_WEEVIL,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}

fn brain_weevil_game() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let weevil = creature(10_000, cards::BRAIN_WEEVIL, PlayerId::One);
    let weevil_id = weevil.card.id;
    game.battlefield.push(weevil);
    game.players[PlayerId::Two.index()].hand.extend([
        card(10_500, cards::SEDGE_TROLL, PlayerId::Two),
        card(10_501, cards::LIGHTNING_BOLT, PlayerId::Two),
    ]);
    (game, weevil_id)
}

fn brain_weevil_activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source: actual, targets, .. }
                    if *actual == source
                        && targets.iter().flat_map(TargetSelection::targets)
                            .any(|target| *target == Target::Player(PlayerId::Two))
            )
        })
}

/// "As a sorcery" is the conjunction of three restrictions: your turn, a
/// main phase, and an empty stack. Brain Weevil uses the shared activation
/// window, so each closed boundary withholds the action altogether.
#[test]
fn brain_weevil_is_offered_only_in_a_sorcery_window() {
    let (mut game, weevil_id) = brain_weevil_game();
    assert!(
        brain_weevil_activation(&game, weevil_id).is_some(),
        "the empty main phase on its controller's turn is open",
    );

    game.active_player = PlayerId::Two;
    assert!(
        brain_weevil_activation(&game, weevil_id).is_none(),
        "the opponent's turn is closed",
    );

    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    assert!(
        brain_weevil_activation(&game, weevil_id).is_none(),
        "its controller's upkeep is not a main phase",
    );

    game.step = Step::PrecombatMain;
    game.stack
        .push(spell(20_000, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    assert!(
        brain_weevil_activation(&game, weevil_id).is_none(),
        "a nonempty stack closes the sorcery window",
    );
}

#[test]
fn brain_weevil_pays_its_sacrifice_and_discards_two() {
    let (mut game, weevil_id) = brain_weevil_game();
    let activate = brain_weevil_activation(&game, weevil_id)
        .expect("the sorcery-speed activation aimed at the opponent is offered");

    game.apply(PlayerId::One, activate)
        .expect("the offered activation is legal");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == weevil_id),
        "the Weevil is sacrificed as the cost",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::BRAIN_WEEVIL),
    );

    drain_pending(&mut game);

    assert!(game.players[PlayerId::Two.index()].hand.is_empty());
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        2,
        "the targeted player chose and discarded both cards",
    );
}

/// Printed "only once each turn" caps. The engine already counted every
/// activation per ability and cleared the counts each turn, so the cap is a
/// read of existing state rather than new bookkeeping -- which is what these
/// check, including that the allowance really does return.
mod once_each_turn {
    use super::*;

    fn drake_game() -> (Game, GameObjectId) {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let drake = creature(10_000, cards::FIRE_DRAKE, PlayerId::One);
        let drake_id = drake.card.id;
        game.battlefield.push(drake);
        game.players[PlayerId::One.index()].mana_pool.red = 5;
        (game, drake_id)
    }

    fn pump(game: &mut Game, drake: GameObjectId) {
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == drake)
            })
            .expect("the pump is offered");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(game);
    }

    #[test]
    fn a_capped_ability_is_offered_once_and_then_withheld() {
        let (mut game, drake_id) = drake_game();
        assert!(offers(&game, PlayerId::One, drake_id));

        pump(&mut game, drake_id);

        assert!(
            !offers(&game, PlayerId::One, drake_id),
            "the allowance is spent even though the mana is not"
        );
        let drake = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == drake_id)
            .expect("the Drake is on the battlefield");
        assert_eq!(game.power(drake), Some(2), "and the one activation landed");
    }

    /// The cap is per turn, so the next turn returns it. Cleanup alone cannot
    /// erase turn history because an effect may insert another phase before
    /// the next turn actually begins.
    #[test]
    fn the_allowance_returns_with_the_turn() {
        let (mut game, drake_id) = drake_game();
        pump(&mut game, drake_id);
        assert!(!offers(&game, PlayerId::One, drake_id));

        game.finish_cleanup();
        game.start_next_turn();
        game.priority = PlayerId::One;
        game.players[PlayerId::One.index()].mana_pool.red = 5;

        assert!(
            offers(&game, PlayerId::One, drake_id),
            "a new turn is a new allowance"
        );
    }

    /// "No more than twice each turn" is the same cap with a different
    /// number, which is what shows the limit is counted rather than flagged.
    #[test]
    fn a_cap_of_two_allows_exactly_two() {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let bats = creature(10_000, cards::VAMPIRE_BATS, PlayerId::One);
        let bats_id = bats.card.id;
        game.battlefield.push(bats);
        game.players[PlayerId::One.index()].mana_pool.black = 5;

        for expected in 1..=2 {
            assert!(offers(&game, PlayerId::One, bats_id));
            pump(&mut game, bats_id);
            let bats = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == bats_id)
                .expect("still there");
            assert_eq!(game.power(bats), Some(expected));
        }

        assert!(
            !offers(&game, PlayerId::One, bats_id),
            "two is the whole allowance, and the mana is still there"
        );
    }

    /// One activation, two riders: the cap applies to the ability rather than
    /// to either half of what it does.
    #[test]
    fn a_capped_ability_still_applies_all_of_its_riders() {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let mage = creature(10_000, cards::BEETLEFORM_MAGE, PlayerId::One);
        let mage_id = mage.card.id;
        game.battlefield.push(mage);
        let pool = &mut game.players[PlayerId::One.index()].mana_pool;
        pool.green = 4;
        pool.blue = 4;

        pump(&mut game, mage_id);

        let mage = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == mage_id)
            .expect("still there");
        assert_eq!((game.power(mage), game.toughness(mage)), (Some(4), Some(4)));
        assert!(game.permanent_has_executable_keyword(mage, KeywordAbility::Flying));
        assert!(!offers(&game, PlayerId::One, mage_id));
    }

    /// Gate to Phyrexia carries both restrictions, so it is the check that
    /// they compose rather than one masking the other.
    #[test]
    fn a_window_and_a_cap_both_apply() {
        let mut game = ready_game();
        let phyrexia = creature(10_000, cards::GATE_TO_PHYREXIA, PlayerId::One);
        let gate_id = phyrexia.card.id;
        game.battlefield.push(phyrexia);
        game.battlefield
            .push(creature(10_001, cards::SEDGE_TROLL, PlayerId::One));
        game.battlefield
            .push(creature(10_002, cards::SEDGE_TROLL, PlayerId::One));
        game.battlefield
            .push(creature(10_003, cards::SOL_RING, PlayerId::Two));

        game.step = Step::PrecombatMain;
        assert!(
            !offers(&game, PlayerId::One, gate_id),
            "the window is shut outside upkeep"
        );

        game.step = Step::Upkeep;
        assert!(offers(&game, PlayerId::One, gate_id));

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == gate_id)
            })
            .expect("the ability is offered in the open window");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert!(
            !offers(&game, PlayerId::One, gate_id),
            "the cap holds inside the open window, with a second creature to spare"
        );
    }

    #[test]
    fn every_capped_identity_reports_complete_coverage() {
        let catalog = poc::catalog().expect("catalog builds");
        for definition in [
            cards::GATE_TO_PHYREXIA,
            cards::FIRE_DRAKE,
            cards::DARKTHICKET_WOLF,
            cards::VAMPIRE_BATS,
            cards::BEETLEFORM_MAGE,
        ] {
            let card = catalog.get(definition).expect("the card is cataloged");
            assert_eq!(
                card.rules.implementation_status(),
                ImplementationStatus::Complete,
                "{} should be fully executable",
                card.name,
            );
        }
    }
}

/// Two identities the window unblocked without any further engine work. Both
/// are worth driving because the window is doing real work in each: the
/// Colossus is otherwise permanently tapped after it attacks, and the
/// Caretaker's upkeep restriction is the whole reason it is fair.
mod unblocked_by_the_window {
    use super::*;

    /// The Colossus does not untap on its own, so the upkeep ability is the
    /// only way back. That makes it a check that a static untap restriction
    /// and an upkeep-only untap coexist on one permanent.
    #[test]
    fn the_colossus_stays_tapped_until_its_upkeep_ability_pays() {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let mut colossus = creature(10_000, cards::COLOSSUS_OF_SARDIA, PlayerId::One);
        colossus.tapped = true;
        let colossus_id = colossus.card.id;
        game.battlefield.push(colossus);

        // The untap step untaps everything eligible; the Colossus is not.
        game.choose_untap(PlayerId::One, &[colossus_id]);
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == colossus_id)
                .expect("the Colossus is on the battlefield")
                .tapped,
            "the untap step passes it by"
        );

        game.step = Step::Upkeep;
        game.players[PlayerId::One.index()].mana_pool.colorless = 9;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == colossus_id)
            })
            .expect("nine mana in its own upkeep is the way back");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert!(
            !game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == colossus_id)
                .expect("the Colossus is on the battlefield")
                .tapped,
        );
    }

    #[test]
    fn hells_caretaker_trades_a_creature_for_one_in_the_graveyard() {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        game.step = Step::Upkeep;
        let caretaker = creature(10_000, cards::HELLS_CARETAKER, PlayerId::One);
        let caretaker_id = caretaker.card.id;
        game.battlefield.push(caretaker);
        let fodder = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
        let fodder_id = fodder.card.id;
        game.battlefield.push(fodder);
        let buried = card(10_002, cards::SERRA_ANGEL, PlayerId::One);
        let buried_id = buried.id;
        game.players[PlayerId::One.index()].graveyard.push(buried);

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, cost_objects, .. }
                        if *source == caretaker_id && cost_objects.as_slice() == [fodder_id]
                )
            })
            .expect("the Caretaker offers the trade in its own upkeep");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
            "the graveyard creature came back"
        );
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == fodder_id),
            "and the sacrificed one paid for it"
        );
        let _ = buried_id;
    }

    #[test]
    fn both_identities_report_complete_coverage() {
        let catalog = poc::catalog().expect("catalog builds");
        for definition in [cards::COLOSSUS_OF_SARDIA, cards::HELLS_CARETAKER] {
            let card = catalog.get(definition).expect("the card is cataloged");
            assert_eq!(
                card.rules.implementation_status(),
                ImplementationStatus::Complete,
                "{} should be fully executable",
                card.name,
            );
        }
    }
}
