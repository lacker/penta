//! Magic 2013 cards whose audit lines had gone stale.
//!
//! Exalted and regeneration were both built; the entering creature's power
//! is a trigger value that exists; and a mana ability can now read a
//! battlefield count as well as a counter count, since either is knowable
//! before the ability is activated.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Queues an arrival without answering anything, so the caller decides how
/// any resulting "you may" is answered.
fn arrive(game: &mut Game, id: u32, definition: CardDefinitionId, controller: PlayerId) {
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(id, definition, controller),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
}

/// Answers each waiting decision by taking the last option, which for a
/// "you may" is the branch that accepts.
fn drain_accepting(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

fn activate_printed(
    game: &mut Game,
    player: PlayerId,
    source: GameObjectId,
    ability_index: u8,
    target: Option<Target>,
) {
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } => {
                *actual == source
                    && *ability == AbilityId(ability_index)
                    && target.is_none_or(|wanted| {
                        targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|offered| *offered == wanted)
                    })
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("printed ability {ability_index} is offered"));
    game.apply(player, action).expect("the ability activates");
    drain_accepting(game);
}

#[test]
fn duty_bound_dead_has_exalted_and_can_regenerate() {
    let mut game = ready();
    let dead = creature(10_000, cards::DUTY_BOUND_DEAD, PlayerId::One);
    let dead_id = dead.card.id;
    game.battlefield.push(dead);

    // Exalted is a triggered ability rather than a keyword, and has its own
    // test module; what this one checks is the regeneration beside it.
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == dead_id)
        ),
        "one black is three short",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == dead_id)
        ),
        "four mana buys the regeneration",
    );
}

/// The Goliath reads the entering creature's power, not a constant, and
/// takes arrivals from either side.
#[test]
fn the_goliath_counts_the_entering_creatures_power() {
    let mut game = ready();
    let goliath = creature(10_000, cards::HAMLETBACK_GOLIATH, PlayerId::One);
    let goliath_id = goliath.card.id;
    game.battlefield.push(goliath);

    // Air Elemental is a 4/4.
    arrive(&mut game, 10_100, cards::AIR_ELEMENTAL, PlayerId::One);
    drain_accepting(&mut game);
    assert_eq!(counters(&game, goliath_id), 4);

    // Grizzly Bears is a 2/2, and theirs feeds it just the same.
    arrive(&mut game, 10_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    drain_accepting(&mut game);
    assert_eq!(counters(&game, goliath_id), 6, "four then two");
}

/// The Archdruid counts itself, so a lone one taps for a single green.
#[test]
fn the_archdruid_taps_for_one_green_per_elf() {
    let mut game = ready();
    let druid = creature(10_000, cards::ELVISH_ARCHDRUID, PlayerId::One);
    let druid_id = druid.card.id;
    game.battlefield.push(druid);

    let tap = |game: &mut Game| {
        let before = game.players[PlayerId::One.index()].mana_pool.green;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateManaAbility { source, .. } if *source == druid_id)
            })
            .expect("an untapped Archdruid offers its mana ability");
        game.apply(PlayerId::One, action).expect("tapping is free");
        drain_pending(game);
        game.players[PlayerId::One.index()].mana_pool.green - before
    };

    assert_eq!(tap(&mut game), 1, "itself and nothing else");

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == druid_id)
        .expect("still there")
        .tapped = false;
    for index in 0..2 {
        game.battlefield.push(creature(
            10_100 + index,
            cards::LLANOWAR_ELVES,
            PlayerId::One,
        ));
    }
    game.priority = PlayerId::One;

    assert_eq!(tap(&mut game), 3, "itself and two more");
}

/// "Other Elf creatures you control", so the Archdruid does not pump itself.
#[test]
fn the_archdruid_pumps_other_elves_only() {
    let mut game = ready();
    let druid = creature(10_000, cards::ELVISH_ARCHDRUID, PlayerId::One);
    let druid_id = druid.card.id;
    game.battlefield.push(druid);
    let elf = creature(10_100, cards::LLANOWAR_ELVES, PlayerId::One);
    let elf_id = elf.card.id;
    game.battlefield.push(elf);

    let stats = |game: &Game, id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(stats(&game, elf_id), (Some(2), Some(2)), "a 1/1 plus one");
    assert_eq!(stats(&game, druid_id), (Some(2), Some(2)), "printed size");
}

#[test]
fn nefarox_makes_the_defending_player_choose_a_sacrifice() {
    let mut game = ready();
    let nefarox = creature(10_000, cards::NEFAROX_OVERLORD_OF_GRIXIS, PlayerId::One);
    let nefarox_id = nefarox.card.id;
    game.battlefield.push(nefarox);
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(10_101, cards::SAVANNAH_LIONS, PlayerId::Two));

    game.step = Step::DeclareAttackers;
    game.declare_attacker(nefarox_id, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_accepting(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .count(),
        1,
        "the defending player sacrificed exactly one creature",
    );
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 1);
}

#[test]
fn veilborn_ghoul_returns_for_its_controllers_swamp() {
    let mut game = ready();
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_000,
        cards::VEILBORN_GHOUL,
        PlayerId::One,
    ));

    arrive(&mut game, 10_100, cards::SWAMP, PlayerId::One);
    drain_accepting(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::VEILBORN_GHOUL),
        "accepting the graveyard trigger returns the Ghoul",
    );
    assert!(game.players[PlayerId::One.index()].graveyard.is_empty());
}

#[test]
fn each_opposing_attacker_wakes_slumbering_dragon_once() {
    let mut game = ready();
    let dragon = creature(10_000, cards::SLUMBERING_DRAGON, PlayerId::One);
    let dragon_id = dragon.card.id;
    game.battlefield.push(dragon);
    let first = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let first_id = first.card.id;
    game.battlefield.push(first);
    let second = creature(10_101, cards::SAVANNAH_LIONS, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.push(second);

    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.declare_attacker(first_id, AttackDefender::Player(PlayerId::One));
    game.declare_attacker(second_id, AttackDefender::Player(PlayerId::One));
    game.finish_declaring_attackers();
    drain_accepting(&mut game);

    assert_eq!(counters(&game, dragon_id), 2);
}

#[test]
fn slumbering_dragon_cannot_attack_until_it_has_five_counters() {
    let mut game = ready();
    let dragon = creature(10_000, cards::SLUMBERING_DRAGON, PlayerId::One);
    let dragon_id = dragon.card.id;
    game.battlefield.push(dragon);
    game.step = Step::DeclareAttackers;

    let attack = Action::DeclareAttacker {
        attacker: dragon_id,
        defender: AttackDefender::Player(PlayerId::Two),
    };
    assert!(!game.legal_actions(PlayerId::One).contains(&attack));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == dragon_id)
        .expect("the Dragon is there")
        .add_counters(CounterKind::PlusOnePlusOne, 5);
    assert!(game.legal_actions(PlayerId::One).contains(&attack));
}

#[test]
fn elixir_shuffles_itself_and_its_controllers_graveyard_on_resolution() {
    let mut game = ready();
    let elixir = creature(10_000, cards::ELIXIR_OF_IMMORTALITY, PlayerId::One);
    let elixir_id = elixir.card.id;
    game.battlefield.push(elixir);
    for (id, definition) in [(10_100, cards::PLAINS), (10_101, cards::GRIZZLY_BEARS)] {
        game.players[PlayerId::One.index()]
            .graveyard
            .push(card(id, definition, PlayerId::One));
    }
    let library_before = game.players[PlayerId::One.index()].library.len();
    game.players[PlayerId::One.index()].life = 12;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == elixir_id)
        })
        .expect("two mana offers the Elixir ability");
    game.apply(PlayerId::One, action)
        .expect("the Elixir ability activates");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == elixir_id),
        "the Elixir is part of the effect, not an activation cost",
    );

    drain_accepting(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 17);
    assert!(game.players[PlayerId::One.index()].graveyard.is_empty());
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library_before + 3,
        "the two graveyard cards and the Elixir moved to the library",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == elixir_id),
    );
}

#[test]
fn conditional_static_counts_read_battlefield_and_graveyard_queries() {
    let mut game = ready();
    let phantasm = creature(11_000, cards::JACE_S_PHANTASM, PlayerId::One);
    let phantasm_id = phantasm.card.id;
    game.battlefield.push(phantasm);
    for index in 0..9 {
        game.players[PlayerId::Two.index()].graveyard.push(card(
            11_100 + index,
            cards::PLAINS,
            PlayerId::Two,
        ));
    }
    let phantasm = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == phantasm_id)
        .expect("the Phantasm is there");
    assert_eq!(game.power(phantasm), Some(1));
    game.players[PlayerId::Two.index()]
        .graveyard
        .push(card(11_200, cards::ISLAND, PlayerId::Two));
    assert_eq!(
        game.power(phantasm),
        Some(5),
        "the tenth card turns on +4/+4"
    );

    let serpent = creature(11_300, cards::HARBOR_SERPENT, PlayerId::One);
    let serpent_id = serpent.card.id;
    game.battlefield.push(serpent);
    for index in 0..4 {
        game.battlefield
            .push(creature(11_400 + index, cards::ISLAND, PlayerId::One));
    }
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let attack = Action::DeclareAttacker {
        attacker: serpent_id,
        defender: AttackDefender::Player(PlayerId::Two),
    };
    assert!(!game.legal_actions(PlayerId::One).contains(&attack));
    game.battlefield
        .push(creature(11_500, cards::ISLAND, PlayerId::Two));
    assert!(
        game.legal_actions(PlayerId::One).contains(&attack),
        "the fifth Island on either side releases the Serpent",
    );
}

#[test]
fn courtly_provocateur_grants_each_combat_requirement_for_the_turn() {
    let mut game = ready();
    let provocateur = creature(12_000, cards::COURTLY_PROVOCATEUR, PlayerId::One);
    let provocateur_id = provocateur.card.id;
    game.battlefield.push(provocateur);
    let compelled = creature(12_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let compelled_id = compelled.card.id;
    game.battlefield.push(compelled);

    activate_printed(
        &mut game,
        PlayerId::One,
        provocateur_id,
        0,
        Some(Target::Permanent(compelled_id)),
    );
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::FinishDeclaringAttackers),
        "the targeted creature cannot stay home while it can attack",
    );
    game.apply(
        PlayerId::Two,
        Action::DeclareAttacker {
            attacker: compelled_id,
            defender: AttackDefender::Player(PlayerId::One),
        },
    )
    .expect("the compelled creature attacks");
    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::FinishDeclaringAttackers),
    );

    let mut game = ready();
    let provocateur = creature(12_100, cards::COURTLY_PROVOCATEUR, PlayerId::One);
    let provocateur_id = provocateur.card.id;
    game.battlefield.push(provocateur);
    let attacker = creature(12_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let compelled = creature(12_102, cards::SAVANNAH_LIONS, PlayerId::Two);
    let compelled_id = compelled.card.id;
    game.battlefield.push(compelled);
    activate_printed(
        &mut game,
        PlayerId::One,
        provocateur_id,
        1,
        Some(Target::Permanent(compelled_id)),
    );
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    game.blockers_declared = false;
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("the attacker is there");
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::FinishDeclaringBlockers),
        "the targeted creature cannot decline a legal block",
    );
}

#[test]
fn liliana_chooses_the_minus_three_branch_and_x_on_resolution() {
    let mut game = ready();
    let mut liliana = creature(13_000, cards::LILIANA_OF_THE_DARK_REALMS, PlayerId::One);
    liliana.set_counters(CounterKind::Loyalty, 3);
    let liliana_id = liliana.card.id;
    game.battlefield.push(liliana);
    game.battlefield
        .push(creature(13_100, cards::SWAMP, PlayerId::One));
    let bears = creature(13_200, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                modes,
                ..
            } => {
                *source == liliana_id
                    && *ability == AbilityId(1)
                    && modes.is_empty()
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bears_id))
            }
            _ => false,
        })
        .expect("Liliana's -3 is activated by choosing only its target");
    game.apply(PlayerId::One, activation)
        .expect("the loyalty ability activates");
    assert!(
        game.pending_decisions.is_empty(),
        "the pump-or-shrink choice is not made while activating",
    );

    // A response can change the Swamp count before the ability resolves.
    game.battlefield
        .push(creature(13_101, cards::SWAMP, PlayerId::One));
    for _ in 0..4 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority passes toward resolution");
    }
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the branch is chosen during resolution");
    let shrink = decision
        .options
        .iter()
        .find(|option| option.label == "Give -X/-X")
        .map(|option| option.id)
        .expect("the resolving choice offers the shrink branch");
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![shrink],
        },
    )
    .expect("the resolving branch is chosen");
    game.check_state_based_actions();
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "choosing -X/-X with two Swamps kills a 2/2",
    );
}

#[test]
fn lilianas_emblem_grants_four_black() {
    let mut game = ready();
    let mut liliana = creature(13_300, cards::LILIANA_OF_THE_DARK_REALMS, PlayerId::One);
    liliana.set_counters(CounterKind::Loyalty, 6);
    let liliana_id = liliana.card.id;
    game.battlefield.push(liliana);
    let swamp = creature(13_301, cards::SWAMP, PlayerId::One);
    let swamp_id = swamp.card.id;
    game.battlefield.push(swamp);
    activate_printed(&mut game, PlayerId::One, liliana_id, 2, None);
    let emblem_mana = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source,
                    ability: AbilityOrigin::EmblemGranted { .. },
                    ..
                } if *source == swamp_id
            )
        })
        .expect("the emblem grants the Swamp a mana ability");
    game.apply(PlayerId::One, emblem_mana)
        .expect("the granted mana ability activates");
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.black, 4);
}

#[test]
fn xathrid_gorgon_petrifies_a_creature_permanently() {
    let mut game = ready();
    let gorgon = creature(14_000, cards::XATHRID_GORGON, PlayerId::One);
    let gorgon_id = gorgon.card.id;
    game.battlefield.push(gorgon);
    let elf = creature(14_001, cards::LLANOWAR_ELVES, PlayerId::Two);
    let elf_id = elf.card.id;
    game.battlefield.push(elf);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    activate_printed(
        &mut game,
        PlayerId::One,
        gorgon_id,
        1,
        Some(Target::Permanent(elf_id)),
    );

    let elf = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == elf_id)
        .expect("the petrified creature remains");
    assert_eq!(elf.counters(CounterKind::named("petrification")), 1);
    assert!(
        game.permanent_types(elf)
            .is_some_and(|types| types.contains(CardType::Artifact)),
    );
    assert_eq!(game.permanent_colors(elf), [false; 5]);
    assert!(game.permanent_has_executable_keyword(elf, KeywordAbility::Defender));
    game.priority = PlayerId::Two;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, .. }
                    | Action::ActivateAbility { source, .. }
                    if *source == elf_id
            )
        }),
        "neither mana nor nonmana activated abilities can be activated",
    );
}

#[test]
fn elderscale_wurm_sets_and_then_defends_the_seven_life_floor() {
    let mut game = ready();
    game.players[PlayerId::One.index()].life = 4;
    arrive(&mut game, 15_000, cards::ELDERSCALE_WURM, PlayerId::One);
    drain_accepting(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 7);

    let burner = creature(15_001, cards::SOL_RING, PlayerId::Two);
    let burner_id = burner.card.id;
    game.battlefield.push(burner);
    game.damage_target_from(Some(burner_id), Some(Target::Player(PlayerId::One)), 10);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        7,
        "damage cannot cross the floor while the controller is at seven",
    );

    game.players[PlayerId::One.index()].life = 6;
    game.damage_target_from(Some(burner_id), Some(Target::Player(PlayerId::One)), 1);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        5,
        "the damage limit is off below seven life",
    );
}
