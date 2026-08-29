//! Identities unblocked by earlier passes rather than by new machinery.
//!
//! Each of these pairs something built earlier in this session with something
//! that already existed. The tests drive the pairing rather than either half:
//! a counter-conditional grant on an unleash creature, and a sacrifice
//! ability reading the power of the creature it just spent.

use super::*;

/// Chaos Imps has trample only while it carries the unleash counter, so the
/// two halves of the card interact rather than sitting side by side.
#[test]
fn chaos_imps_gains_trample_only_with_its_counter() {
    let mut game = ready_game();
    let imps = creature(10_000, cards::CHAOS_IMPS, PlayerId::One);
    let imps_id = imps.card.id;
    game.battlefield.push(imps);

    let without = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == imps_id)
        .expect("there");
    assert!(
        !game.permanent_has_executable_keyword(without, KeywordAbility::Trample),
        "no counter, no trample"
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == imps_id)
        .expect("there")
        .add_counters(CounterKind::PlusOnePlusOne, 1);

    let with = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == imps_id)
        .expect("there");
    assert!(
        game.permanent_has_executable_keyword(with, KeywordAbility::Trample),
        "the unleash counter is what turns it on"
    );
}

/// Hellhole Flailer sacrifices itself and then reads its own power, which
/// only works from last known information.
#[test]
fn hellhole_flailer_deals_its_power_after_sacrificing_itself() {
    let mut game = ready_game();
    let flailer = creature(10_000, cards::HELLHOLE_FLAILER, PlayerId::One);
    let flailer_id = flailer.card.id;
    game.battlefield.push(flailer);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == flailer_id)
        .expect("there")
        // Unleashed, so it is a 4/3 rather than the printed 3/2.
        .add_counters(CounterKind::PlusOnePlusOne, 1);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.black = 1;
    pool.red = 1;
    pool.colorless = 2;
    let before = game.players[PlayerId::Two.index()].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == flailer_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("it can be sacrificed at the other player");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == flailer_id),
        "it spent itself"
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        before - 4,
        "and dealt the four power it had when it left"
    );
}

/// "Each creature you control with a +1/+1 counter on it" is read live, so a
/// creature that gains a counter picks the grant up and one that never had
/// one never does.
#[test]
fn a_counter_is_what_earns_the_granted_keyword() {
    let mut game = ready_game();
    let drake = creature(10_000, cards::SAPPHIRE_DRAKE, PlayerId::One);
    game.battlefield.push(drake);
    let grounded = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let grounded_id = grounded.card.id;
    game.battlefield.push(grounded);
    let theirs = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    for id in [grounded_id, theirs_id] {
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
            .expect("there")
            .add_counters(CounterKind::PlusOnePlusOne, 1);
    }

    let flies = |game: &Game, id| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying)
    };
    assert!(flies(&game, grounded_id), "a counter earns it flying");
    assert!(
        !flies(&game, theirs_id),
        "but only among creatures you control"
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == grounded_id)
        .expect("there")
        .remove_counters(CounterKind::PlusOnePlusOne, 1);
    assert!(
        !flies(&game, grounded_id),
        "and losing the counter loses the grant"
    );
}

/// "Create a tapped ... token" has to arrive tapped, and an ordinary token
/// still arrives untapped.
#[test]
fn a_tapped_token_arrives_tapped() {
    let mut game = ready_game();
    let necromancer = creature(10_000, cards::XATHRID_NECROMANCER, PlayerId::One);
    let necromancer_id = necromancer.card.id;
    game.battlefield.push(necromancer);

    game.destroy_permanent(necromancer_id);
    game.check_state_based_actions();
    game.priority = PlayerId::One;
    drain_pending(&mut game);

    let zombies: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            )
        })
        .collect();
    assert_eq!(zombies.len(), 1, "its own death triggers it");
    assert!(zombies[0].tapped, "and the token arrives tapped");
}

/// A cost that taps a chosen permanent: only untapped Gates its controller
/// controls qualify, and paying taps the one chosen rather than the source.
#[test]
fn tapping_a_chosen_gate_pays_for_the_ability() {
    let mut game = ready_game();
    let shade = creature(10_000, cards::GATEWAY_SHADE, PlayerId::One);
    let shade_id = shade.card.id;
    game.battlefield.push(shade);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, cost_objects, .. }
                if *source == shade_id && !cost_objects.is_empty())
        }),
        "with no Gate there is nothing to tap"
    );

    let guildgate = creature(10_001, cards::AZORIUS_GUILDGATE, PlayerId::One);
    let gate_id = guildgate.card.id;
    game.battlefield.push(guildgate);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, cost_objects, .. }
                if *source == shade_id && cost_objects.as_slice() == [gate_id])
        })
        .expect("the Gate can pay for it");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let guildgate = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == gate_id)
        .expect("still there");
    assert!(guildgate.tapped, "the chosen Gate paid");
    let shade = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == shade_id)
        .expect("still there");
    assert!(!shade.tapped, "and the source did not");
    assert_eq!(
        (game.power(shade), game.toughness(shade)),
        (Some(3), Some(3))
    );
}
