//! Focused coverage for Mirrodin Besieged declarations that exercise shared
//! rules-engine abstractions rather than card-owned procedures.

use super::*;

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

fn staged_hero_of_bladehold() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let hero = game
        .put_onto_battlefield(PlayerId::One, cards::HERO_OF_BLADEHOLD)
        .expect("Hero of Bladehold is cataloged");
    let walker = game
        .put_onto_battlefield(PlayerId::Two, cards::VRASKA_THE_UNSEEN)
        .expect("Vraska is cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == hero)
        .expect("Hero is on the battlefield")
        .entered_controller_turn = 0;
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;

    (game, hero, walker)
}

fn resolve_hero_tokens_before_battle_cry(game: &mut Game) {
    let order = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("Hero's two attack triggers need an order");
    assert_eq!(order.kind, DecisionKind::TriggerOrder);
    let trigger = |text: &str| {
        order
            .options
            .iter()
            .find(|option| {
                option
                    .ability_text
                    .as_deref()
                    .is_some_and(|ability| ability.contains(text))
            })
            .unwrap_or_else(|| panic!("{text} is one ordering option"))
            .id
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: vec![
                trigger("create two 1/1 white Soldier"),
                trigger("Battle cry"),
            ],
        },
    )
    .expect("the Soldiers may resolve before battle cry");
    resolve(game);
}

fn choose_arriving_attacker_defender(game: &mut Game, defender: AttackDefender) {
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the Soldier chooses what it is attacking");
    let option = decision
        .options
        .iter()
        .find(|option| match defender {
            AttackDefender::Player(_) => option.card.is_none(),
            AttackDefender::Planeswalker(walker) => option.card.is_some_and(|(id, _)| id == walker),
        })
        .expect("the chosen defender is legal")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the Soldier may attack that defender");
}

fn soldier_tokens(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1),
            )
        })
        .collect()
}

#[test]
fn hero_of_bladehold_splits_her_soldiers_and_battle_cry_can_boost_them() {
    let (mut game, hero, walker) = staged_hero_of_bladehold();

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: hero,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("Hero may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the attack declaration finishes");

    resolve_hero_tokens_before_battle_cry(&mut game);
    choose_arriving_attacker_defender(&mut game, AttackDefender::Planeswalker(walker));
    choose_arriving_attacker_defender(&mut game, AttackDefender::Player(PlayerId::Two));
    resolve(&mut game);

    let soldiers = soldier_tokens(&game);
    assert_eq!(soldiers.len(), 2, "Hero creates two Soldiers");
    assert!(
        soldiers
            .iter()
            .all(|soldier| soldier.tapped && soldier.attacking),
        "both Soldiers enter tapped and attacking",
    );
    assert_eq!(
        soldiers
            .iter()
            .filter(|soldier| {
                soldier.attack_defender == Some(AttackDefender::Planeswalker(walker))
            })
            .count(),
        1,
        "one Soldier attacks the planeswalker",
    );
    assert_eq!(
        soldiers
            .iter()
            .filter(|soldier| {
                soldier.attack_defender == Some(AttackDefender::Player(PlayerId::Two))
            })
            .count(),
        1,
        "and the other attacks the player",
    );
    assert!(
        soldiers
            .iter()
            .all(|soldier| (game.power(soldier), game.toughness(soldier)) == (Some(2), Some(1))),
        "battle cry resolves after token creation and boosts both Soldiers",
    );
    let hero = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == hero)
        .expect("Hero remains on the battlefield");
    assert_eq!(
        (game.power(hero), game.toughness(hero)),
        (Some(3), Some(4)),
        "battle cry excludes its source",
    );
}

#[test]
fn leonin_relic_warder_returns_what_it_exiled_when_it_leaves() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let artifact = game
        .put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .expect("Sol Ring is cataloged");
    let warder = game
        .build_zone(PlayerId::One, &[cards::LEONIN_RELIC_WARDER])
        .expect("Leonin Relic-Warder is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let warder_card = warder.id;
    game.players[0].hand.push(warder);
    game.turns_started = [1, 1];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == warder_card))
        .expect("two white mana casts Leonin Relic-Warder");
    game.apply(PlayerId::One, cast).expect("the Warder casts");
    resolve(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enters trigger offers an artifact or enchantment");
    let target = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == artifact))
        .expect("Sol Ring is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target],
        },
    )
    .expect("the target is legal");
    resolve(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    resolve(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "the Warder exiles the artifact",
    );
    let warder_permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LEONIN_RELIC_WARDER)
        .expect("the Warder remains on the battlefield")
        .card
        .id;
    game.move_permanents_to_graveyard(&[warder_permanent]);
    resolve(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SOL_RING),
        "the linked artifact returns when the Warder leaves",
    );
}

#[test]
fn myr_welder_gains_activated_abilities_from_a_linked_noncreature_artifact() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1].graveyard.clear();
    let sol_ring = game
        .build_zone(PlayerId::Two, &[cards::SOL_RING])
        .expect("Sol Ring is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let sol_ring_id = sol_ring.id;
    game.players[1].graveyard.push(sol_ring);
    let welder = game
        .put_onto_battlefield(PlayerId::One, cards::MYR_WELDER)
        .expect("Myr Welder is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == welder)
        .expect("Myr Welder is on the battlefield")
        .entered_controller_turn = 0;
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == welder
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Card(sol_ring_id)))
            }
            _ => false,
        })
        .expect("Myr Welder can exile the artifact card");
    game.apply(PlayerId::One, action)
        .expect("the imprint ability activates");
    resolve(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == welder)
        .expect("Myr Welder remains on the battlefield");
    let mut texts = Vec::new();
    let _ = game.visit_effective_abilities(permanent, |effective| {
        if matches!(
            effective.ability.definition,
            crate::card::DeclarativeAbilityDef::Activated(_)
                | crate::card::DeclarativeAbilityDef::ActivatedMana(_)
        ) {
            texts.push(effective.ability.text);
        }
        std::ops::ControlFlow::Continue(())
    });

    assert!(
        texts.contains(&"{T}: Add {C}{C}."),
        "the linked noncreature artifact grants its activated ability: {texts:?}",
    );
}
