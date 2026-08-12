use super::*;

#[test]
fn a_random_discard_spell_hits_the_player_it_targets() {
    let mut game = ready_game();
    let hymn = card(10_000, cards::HYMN_TO_TOURACH, PlayerId::One);
    game.players[0].hand.clear();
    game.players[0].hand.push(hymn.clone());
    for id in [10_001, 10_002, 10_003] {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.players[1].hand.clear();
    for id in [10_010, 10_011, 10_012] {
        game.players[1]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::Two));
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);

    // Both players are legal targets, which is the whole point: the old
    // resolver always took from the opponent.
    let at_self = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        hymn.id,
        Target::Player(PlayerId::One),
    );
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, choices, .. }
            if *card == hymn.id
                && choices.iter_targets().copied().eq(std::iter::once(
                    Target::Player(PlayerId::Two)
                )))
        ),
        "the opponent is offered too"
    );

    game.apply(PlayerId::One, at_self).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.pending_decisions.is_empty(),
        "random discard resolves without asking the targeted player to choose"
    );

    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the caster discarded two of their own three lands"
    );
    assert_eq!(
        game.players[1].hand.len(),
        3,
        "and the opponent, who was not targeted, kept everything"
    );
}

#[test]
fn giant_growth_can_pump_a_creature_you_do_not_control() {
    let mut game = ready_game();
    let growth = card(10_000, cards::GIANT_GROWTH, PlayerId::One);
    game.players[0].hand.push(growth.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two));

    let at_theirs = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        growth.id,
        Target::Permanent(GameObjectId(10_002)),
    );
    game.apply(PlayerId::One, at_theirs).unwrap();
    drain_pending(&mut game);

    let theirs = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_002))
        .expect("still there");
    assert_eq!(
        (game.power(theirs), game.toughness(theirs)),
        (Some(5), Some(4)),
        "the card says target creature, not target creature you control"
    );
    let mine = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert_eq!((game.power(mine), game.toughness(mine)), (Some(2), Some(1)));
}

#[test]
fn regrowth_returns_the_card_you_choose_rather_than_the_last_one_buried() {
    let mut game = ready_game();
    let regrowth = card(10_000, cards::REGROWTH, PlayerId::One);
    game.players[0].hand.push(regrowth.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.players[0].graveyard = vec![
        card(10_001, cards::BLACK_LOTUS, PlayerId::One),
        card(10_002, cards::MOUNTAIN, PlayerId::One),
    ];
    // An opponent's graveyard is off limits.
    game.players[1].graveyard = vec![card(10_003, cards::BLACK_LOTUS, PlayerId::Two)];

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, choices, .. }
                if *card == regrowth.id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Card(GameObjectId(10_003))))
        ),
        "only your own graveyard is a legal source"
    );

    // The Lotus is under the Mountain, so a positional resolver would take
    // the Mountain instead.
    let take_lotus = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        regrowth.id,
        Target::Card(GameObjectId(10_001)),
    );
    game.apply(PlayerId::One, take_lotus).unwrap();
    drain_pending(&mut game);

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::BLACK_LOTUS],
        "the chosen card came back"
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::MOUNTAIN, cards::REGROWTH],
        "and the one on top stayed put"
    );
}

#[test]
fn argothian_pixies_ignore_artifact_creatures_entirely() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut pixies = creature(10_000, cards::ARGOTHIAN_PIXIES, PlayerId::One);
    pixies.attacking = true;
    game.battlefield.push(pixies);
    // Su-Chi is a 4/4 artifact creature: lethal to a 2/1 if the damage lands.
    let mut su_chi = creature(10_001, cards::SU_CHI, PlayerId::Two);
    su_chi.blocking = Some(GameObjectId(10_000));
    game.battlefield.push(su_chi);

    game.deal_combat_damage();
    drain_pending(&mut game);

    let pixies = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("the Pixies shrugged it off");
    assert_eq!(pixies.damage, 0, "artifact creatures cannot hurt them");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(10_001))
            .expect("still there")
            .damage,
        2,
        "and the Pixies still hit back"
    );
}

#[test]
fn argothian_pixies_still_take_damage_from_an_ordinary_creature() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut pixies = creature(10_000, cards::ARGOTHIAN_PIXIES, PlayerId::One);
    pixies.attacking = true;
    game.battlefield.push(pixies);
    let mut lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    lions.blocking = Some(GameObjectId(10_000));
    game.battlefield.push(lions);

    game.deal_combat_damage();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "the prevention only names artifact creatures"
    );
}

#[test]
fn black_vise_squeezes_only_the_player_it_chose() {
    let mut game = ready_game();
    // One Vise per side, each pointed at its own controller's opponent.
    for (id, controller) in [(10_000, PlayerId::One), (10_001, PlayerId::Two)] {
        let mut vise = creature(id, cards::BLACK_VISE, controller);
        vise.chosen_player = Some(controller.opponent());
        game.battlefield.push(vise);
    }
    for index in 0..7 {
        game.players[0]
            .hand
            .push(card(20_000 + index, cards::MOUNTAIN, PlayerId::One));
        game.players[1]
            .hand
            .push(card(20_100 + index, cards::MOUNTAIN, PlayerId::Two));
    }

    game.turn = 2;
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].life, 17,
        "the Vise aimed at player one fired on their upkeep"
    );
    assert_eq!(
        game.players[1].life, 20,
        "and the one aimed at player two waited for theirs"
    );
}

#[test]
fn a_forked_copy_is_red_whatever_it_copies() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));

    // A black Terror on the stack, aimed at something it can legally hit.
    let terror = card(10_001, cards::TERROR, PlayerId::Two);
    game.players[1].hand.push(terror.clone());
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Black, 2);
    game.priority = PlayerId::Two;
    let cast = acceptance_cast_action_targeting(
        &game,
        PlayerId::Two,
        terror.id,
        Target::Permanent(GameObjectId(10_000)),
    );
    game.apply(PlayerId::Two, cast).unwrap();
    let original = game.stack.last().expect("Terror is on the stack").id;
    assert_eq!(
        game.object_colors(original),
        [false, false, true, false, false],
        "Terror itself is black"
    );

    let fork = card(10_002, cards::FORK, PlayerId::One);
    game.players[0].hand.push(fork.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.priority = PlayerId::One;
    let cast_fork =
        acceptance_cast_action_targeting(&game, PlayerId::One, fork.id, Target::Spell(original));
    game.apply(PlayerId::One, cast_fork).unwrap();
    // Resolve the Fork itself, which puts the copy on the stack.
    for _ in 0..8 {
        if game
            .stack
            .iter()
            .any(|object| object.id != original && object.card.definition == cards::TERROR)
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: decision
                        .options
                        .iter()
                        .map(|option| option.id)
                        .take(decision.minimum.max(1))
                        .collect(),
                },
            )
            .unwrap();
            continue;
        }
        if game.apply(game.priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let copy = game
        .stack
        .iter()
        .find(|object| object.id != original && object.card.definition == cards::TERROR)
        .expect("the copy is on the stack");
    assert_eq!(
        game.object_colors(copy.id),
        [false, false, false, true, false],
        "the copy is red, not the black of what it copied"
    );
}

#[test]
fn hypnotic_specter_takes_exactly_one_card_per_connection() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut specter = creature(10_000, cards::HYPNOTIC_SPECTER, PlayerId::One);
    specter.attacking = true;
    game.battlefield.push(specter);
    game.players[1].hand.clear();
    for index in 0..3 {
        game.players[1]
            .hand
            .push(card(10_001 + index, cards::MOUNTAIN, PlayerId::Two));
    }

    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        2,
        "one card at random, not one per path through the combat step"
    );
}

#[test]
fn whirling_dervish_grows_at_the_end_step_only_after_drawing_blood() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::WHIRLING_DERVISH, PlayerId::One));

    // A quiet turn leaves it alone.
    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    let dervish = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("still there");
    assert_eq!(dervish.counters(CounterKind::PlusOnePlusOne), 0);

    // Damage from anything at all counts, not just an attack.
    game.damage_target_from(
        Some(GameObjectId(10_000)),
        Some(Target::Player(PlayerId::Two)),
        1,
    );
    game.begin_step_triggers();
    drain_pending(&mut game);
    let dervish = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("still there");
    assert_eq!(
        dervish.counters(CounterKind::PlusOnePlusOne),
        1,
        "it drew blood this turn"
    );
    assert_eq!(game.power(dervish), Some(2));
}

#[test]
fn the_abyss_lets_each_player_pick_which_of_their_own_creatures_it_takes() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::THE_ABYSS, PlayerId::One));
    // The player whose upkeep it is has a choice; the other player's
    // creatures are not candidates.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::One));
    // An artifact creature is safe from it.
    game.battlefield
        .push(creature(10_004, cards::SU_CHI, PlayerId::Two));

    game.turn = 2;
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    // The trigger uses the stack, so it has to resolve before anyone is
    // asked anything.
    for _ in 0..12 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        if game.apply(game.priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the Abyss asks its victim");
    assert_eq!(
        decision
            .options
            .iter()
            .filter_map(|option| option.card.map(|(card, _)| card))
            .collect::<Vec<_>>(),
        vec![GameObjectId(10_001), GameObjectId(10_002)],
        "only their own nonartifact creatures are candidates"
    );

    // They keep the Angel and feed it the Lions.
    let lions = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == GameObjectId(10_001))
        })
        .expect("the Lions are offered")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![lions],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    let survivors = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    assert!(!survivors.contains(&GameObjectId(10_001)), "the Lions went");
    assert!(
        survivors.contains(&GameObjectId(10_002)),
        "the Angel they chose to keep stayed"
    );
    assert!(
        survivors.contains(&GameObjectId(10_003)),
        "the other player's creature was never at risk"
    );
    assert!(survivors.contains(&GameObjectId(10_004)), "nor the Su-Chi");
}

#[test]
fn copy_artifact_may_decline_and_never_targets() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SOL_RING, PlayerId::Two));
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    // Nothing about it is chosen while it is a spell, so there is exactly one
    // way to cast it however many artifacts are around.
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == copy.id))
            .count(),
        1,
        "the copy is picked as it enters, not targeted"
    );

    game.apply(
        PlayerId::One,
        cast_action(copy.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("entering asks what to copy");
    let decline = decision
        .options
        .iter()
        .find(|option| option.card.is_none())
        .expect("entering as itself is always allowed")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline],
        },
    )
    .unwrap();

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("it entered either way");
    assert!(
        entered.copy_effect.is_none(),
        "declining leaves an ordinary Copy Artifact"
    );
}

#[test]
fn mana_drain_pays_out_at_its_controllers_next_main_phase() {
    let mut game = ready_game();
    let angel = card(10_000, cards::SERRA_ANGEL, PlayerId::One);
    game.players[0].hand.push(angel.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 5);
    let drain = card(10_001, cards::MANA_DRAIN, PlayerId::Two);
    game.players[1].hand.push(drain.clone());
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    game.apply(
        PlayerId::One,
        cast_action(angel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let on_stack = game.stack.last().expect("the Angel is on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::Two,
        cast_action(drain.id, vec![Target::Spell(on_stack)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty(), "the Angel was countered");
    assert_eq!(
        game.players[1].mana_pool.colorless, 0,
        "the mana is not paid on the spot"
    );

    // Their own next main phase is what the card waits for, not the caster's.
    game.finish_cleanup();
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    game.step = Step::Draw;
    game.advance_step();
    assert_eq!(
        game.players[1].mana_pool.colorless, 5,
        "five for the Angel's mana value"
    );
}

#[test]
fn hypnotic_specter_notices_damage_it_did_not_deal_in_combat() {
    let mut game = ready_game();
    game.step = Step::PrecombatMain;
    game.battlefield
        .push(creature(10_000, cards::HYPNOTIC_SPECTER, PlayerId::One));
    game.players[1].hand.clear();
    for index in 0..3 {
        game.players[1]
            .hand
            .push(card(10_001 + index, cards::MOUNTAIN, PlayerId::Two));
    }

    // Damage from anything the Specter is the source of counts, which is what
    // the card says and what a combat-only trigger missed.
    game.damage_target_from(
        Some(GameObjectId(10_000)),
        Some(Target::Player(PlayerId::Two)),
        1,
    );
    drain_pending(&mut game);
    assert_eq!(game.players[1].hand.len(), 2, "it took a card");

    // Its controller taking damage from it is not an opponent being hit.
    game.players[0].hand.clear();
    for index in 0..3 {
        game.players[0]
            .hand
            .push(card(10_010 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.damage_target_from(
        Some(GameObjectId(10_000)),
        Some(Target::Player(PlayerId::One)),
        1,
    );
    drain_pending(&mut game);
    assert_eq!(
        (game.players[0].hand.len(), game.players[1].hand.len()),
        (3, 2),
        "the card says an opponent, so hitting its own controller takes nothing"
    );
}

#[test]
fn drain_life_gains_only_what_the_target_had_to_give() {
    // A player on 3 can only give 3, however much the drain deals.
    let mut game = ready_game();
    game.players[1].life = 3;
    let drain = card(10_000, cards::DRAIN_LIFE, PlayerId::One);
    game.players[0].hand.push(drain.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 8);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == drain.id
                && choices.x() == 6
                && choices.iter_targets().copied().eq(std::iter::once(
                    Target::Player(PlayerId::Two)
                )))
        })
        .expect("six is affordable");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, -3, "all six landed");
    assert_eq!(
        game.players[0].life, 23,
        "but only the three they had came back"
    );
}

#[test]
fn drain_life_spends_only_black_mana_on_x() {
    // Three black and five green. The B symbol takes one black and the green
    // covers the generic, so only two black are left for X -- not the six the
    // pool could otherwise afford.
    let mut game = ready_game();
    let drain = card(10_000, cards::DRAIN_LIFE, PlayerId::One);
    game.players[0].hand.push(drain.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 5);

    let offered: Vec<u16> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == drain.id
                    && choices
                        .iter_targets()
                        .copied()
                        .eq(std::iter::once(Target::Player(PlayerId::Two))) =>
            {
                Some(choices.x())
            }
            _ => None,
        })
        .collect();
    assert_eq!(offered, vec![0, 1, 2], "green cannot be spent on X");

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == drain.id
                && choices.x() == 2
                && choices.iter_targets().copied().eq(std::iter::once(
                    Target::Player(PlayerId::Two)
                )))
        })
        .expect("two is affordable");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 18, "two damage landed");
    assert_eq!(
        game.players[0].mana_pool.amount(ManaColor::Black),
        0,
        "all three black went to the symbol and to X"
    );
    assert_eq!(
        game.players[0].mana_pool.amount(ManaColor::Green),
        4,
        "and the green paid only the one generic"
    );
}

#[test]
fn drain_life_is_capped_by_a_creatures_toughness() {
    let mut game = ready_game();
    // Savannah Lions is a 2/1, so a big drain still only gains one.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    let drain = card(10_000, cards::DRAIN_LIFE, PlayerId::One);
    game.players[0].hand.push(drain.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 8);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == drain.id
                && choices.x() == 6
                && choices.iter_targets().copied().eq(std::iter::once(
                    Target::Permanent(GameObjectId(10_001))
                )))
        })
        .expect("the Lions are a legal target");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_001)),
        "six damage killed it"
    );
    assert_eq!(
        game.players[0].life, 21,
        "and its one toughness is all it had to give"
    );
}

#[test]
fn berserk_doubles_any_creature_and_only_kills_one_that_attacked() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    // An attacking creature the caster does not control, which the old
    // targeting refused.
    let mut angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    angel.attacking = true;
    angel.attacked_this_turn = true;
    game.battlefield.push(angel);
    // And one of their own sitting at home.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));

    let berserk = card(10_002, cards::BERSERK, PlayerId::One);
    game.players[0].hand.push(berserk.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        berserk.id,
        Target::Permanent(GameObjectId(10_000)),
    );
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("still there");
    assert_eq!(game.power(angel), Some(8), "a 4/4 doubles to 8/4");
    assert_eq!(game.toughness(angel), Some(4));
    assert!(game.permanent_has_executable_keyword(angel, KeywordAbility::Trample));

    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "it attacked, so the end step collected it"
    );
}

#[test]
fn berserk_cannot_be_cast_once_combat_damage_arrives() {
    // The restriction is the whole reason Berserk is a decision the defender
    // can play around: it has to be committed before damage, not held back
    // until the attack has already connected.
    let mut game = ready_game();
    game.attackers_declared = true;
    game.blockers_declared = true;
    let mut lions = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    lions.attacking = true;
    lions.attacked_this_turn = true;
    game.battlefield.push(lions);
    let berserk = card(10_001, cards::BERSERK, PlayerId::One);
    game.players[0].hand.push(berserk.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let offered = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == berserk.id))
    };

    game.step = Step::DeclareBlockers;
    assert!(
        offered(&game),
        "blockers are declared and damage is still ahead"
    );

    game.step = Step::CombatDamage;
    assert!(
        !offered(&game),
        "the combat damage step is too late to pump the attacker"
    );

    game.step = Step::PostcombatMain;
    assert!(!offered(&game), "and so is the rest of the turn");
}

#[test]
fn berserk_spares_a_creature_that_never_attacked() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    let berserk = card(10_001, cards::BERSERK, PlayerId::One);
    game.players[0].hand.push(berserk.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        berserk.id,
        Target::Permanent(GameObjectId(10_000)),
    );
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("it never attacked, so nothing came for it");
    assert_eq!(game.power(lions), Some(4), "a 2/1 doubles to 4/1");
}
