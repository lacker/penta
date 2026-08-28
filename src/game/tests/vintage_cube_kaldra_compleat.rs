//! Kaldra Compleat: seven mana that arrives as its own 5/5, and exiles
//! whatever it hits.

use super::*;

/// Kaldra on the battlefield, having brought its Germ, with `theirs` under
/// player Two.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut ids = Vec::new();
    for definition in theirs {
        ids.push(
            game.put_onto_battlefield(PlayerId::Two, *definition)
                .expect("cataloged"),
        );
    }
    let kaldra = game
        .put_onto_battlefield(PlayerId::One, cards::KALDRA_COMPLEAT)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, kaldra, ids)
}

/// Answers whatever combat asks: a decision, or a trampler's assignment,
/// which the engine only queues when there is a real choice about how much
/// spills past the blocker.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(assignment) = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find_map(|player| {
                game.legal_actions(player)
                    .into_iter()
                    .find(|action| matches!(action, Action::AssignCombatDamage { .. }))
                    .map(|action| (player, action))
            })
        {
            let (player, action) = assignment;
            game.apply(player, action).expect("the assignment is legal");
            continue;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// The Germ the living weapon brought, which is what Kaldra is wearing.
fn germ(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the Germ is on the battlefield")
}

fn has(game: &Game, permanent: &Permanent, keyword: KeywordAbility) -> bool {
    game.permanent_has_executable_keyword(permanent, keyword)
}

/// Living weapon: it brings a Germ and equips itself to it, and the 0/0
/// survives because it is a 5/5.
#[test]
fn it_arrives_wearing_its_own_germ() {
    let (game, kaldra, _) = staged(&[]);

    let germ = germ(&game);
    assert_eq!(germ.attached_to, None, "the Germ wears the Equipment");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == kaldra)
            .expect("Kaldra is there")
            .attached_to,
        Some(germ.card.id),
    );
    assert_eq!(game.power(germ), Some(5));
    assert_eq!(game.toughness(germ), Some(5));
}

/// Everything the static clause hands over.
#[test]
fn the_germ_has_the_whole_list() {
    let (game, _kaldra, _) = staged(&[]);
    let germ = germ(&game);

    assert!(has(&game, germ, KeywordAbility::FirstStrike));
    assert!(has(&game, germ, KeywordAbility::Trample));
    assert!(has(&game, germ, KeywordAbility::Indestructible));
    assert!(has(&game, germ, KeywordAbility::Haste));
}

/// The Equipment is indestructible itself, not only its wearer.
#[test]
fn the_equipment_is_indestructible() {
    let (game, kaldra, _) = staged(&[]);
    let equipment = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == kaldra)
        .expect("Kaldra is there");

    assert!(has(&game, equipment, KeywordAbility::Indestructible));
}

/// Combat damage to a blocker exiles it: not destroyed, exiled, so nothing
/// about indestructible or regeneration saves it.
#[test]
fn what_it_hits_is_exiled() {
    let (mut game, _kaldra, theirs) = staged(&[cards::WORLDSPINE_WURM]);
    let blocker = theirs[0];
    let germ = germ(&game).card.id;

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(germ, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::DeclareBlockers;
    game.declare_blocker(blocker, germ);
    game.finish_declaring_blockers();
    settle(&mut game);
    for _ in 0..12 {
        if game.step == Step::PostcombatMain {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker),
        "the blocker is gone",
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::WORLDSPINE_WURM),
        "and it is in exile, not the graveyard",
    );
    assert!(
        !game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::WORLDSPINE_WURM),
    );
}

/// If combat damage kills the creature, state-based actions move it before
/// Kaldra's damage trigger resolves. That graveyard card is a new object, so
/// the ordinary "that creature" reference cannot follow it and exile it.
#[test]
fn lethal_damage_does_not_exile_the_graveyard_successor() {
    let (mut game, _kaldra, theirs) = staged(&[cards::SERRA_ANGEL]);
    let blocker = theirs[0];
    let germ = germ(&game).card.id;

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(germ, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::DeclareBlockers;
    game.declare_blocker(blocker, germ);
    game.finish_declaring_blockers();
    settle(&mut game);
    for _ in 0..12 {
        if game.step == Step::PostcombatMain {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }

    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the new graveyard object remains there",
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .all(|card| card.definition != cards::SERRA_ANGEL),
        "Kaldra does not gain a zone-change exception",
    );
}

/// Damage to a player is not damage to a creature, so nothing is exiled.
#[test]
fn unblocked_damage_exiles_nothing() {
    let (mut game, _kaldra, _) = staged(&[]);
    let germ = germ(&game).card.id;
    let before = game.players[1].exile.len();

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(germ, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    for _ in 0..12 {
        if game.step == Step::PostcombatMain {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }

    assert_eq!(game.players[1].life, 15, "five trampling damage");
    assert_eq!(game.players[1].exile.len(), before, "and nothing exiled");
}

/// Move the Equipment and the whole package moves with it: the Germ becomes
/// a 0/0 and dies where it stands.
#[test]
fn the_germ_dies_when_the_equipment_leaves_it() {
    let (mut game, kaldra, _) = staged(&[cards::GRIZZLY_BEARS]);
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 7);

    let equip = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == kaldra
                        && targets
                            .iter()
                            .any(|slot| slot.targets().contains(&Target::Permanent(mine)))
            )
        })
        .expect("equip is activatable");
    game.apply(PlayerId::One, equip).expect("it equips");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == ObjectKind::Token),
        "a 0/0 Germ with nothing on it dies",
    );
    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mine)
        .expect("the new wearer is there");
    assert_eq!(game.power(lions), Some(7), "2 plus five");
}
