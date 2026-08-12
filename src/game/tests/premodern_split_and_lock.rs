use super::*;

#[test]
fn fire_divides_two_damage_and_ice_taps_then_draws() {
    let mut game = ready_game();
    let fire_ice = card(10_000, cards::FIRE_ICE, PlayerId::One);
    let fire_ice_id = fire_ice.id;
    game.players[PlayerId::One.index()].hand.push(fire_ice);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let fire = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            let Action::CastSpell { card, choices, .. } = action else {
                return false;
            };
            *card == fire_ice_id
                && choices.play_option() == PlayOptionId::DEFAULT
                && choices
                    .targets()
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>()
                    == [Target::Player(PlayerId::One), Target::Player(PlayerId::Two)]
                && choices
                    .targets()
                    .iter()
                    .flat_map(TargetSelection::amounts)
                    .copied()
                    .collect::<Vec<_>>()
                    == [1, 1]
        })
        .expect("Fire offers a one-and-one division between both players");
    game.apply(PlayerId::One, fire).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 19);
    assert_eq!(game.players[PlayerId::Two.index()].life, 19);

    let ice = card(10_001, cards::FIRE_ICE, PlayerId::One);
    let ice_id = ice.id;
    game.players[PlayerId::One.index()].hand.push(ice);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.players[PlayerId::One.index()]
        .library
        .push(card(10_002, cards::MOUNTAIN, PlayerId::One));
    let target = creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let ice = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == ice_id
                        && choices.play_option() == PlayOptionId(1)
                        && choices.targets().iter().flat_map(TargetSelection::targets).copied().eq([Target::Permanent(target_id)])
            )
        })
        .expect("Ice is an executable split-card play option");
    game.apply(PlayerId::One, ice).unwrap();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .expect("the target remains")
            .tapped
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN)
    );
}

#[test]
fn goblin_pyromancer_pumps_goblins_then_destroys_them_at_end_step() {
    let mut game = ready_game();
    let sharpshooter = creature(10_000, cards::GOBLIN_SHARPSHOOTER, PlayerId::One);
    let sharpshooter_id = sharpshooter.card.id;
    let lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lions_id = lions.card.id;
    game.battlefield.extend([sharpshooter, lions]);
    let pyromancer_id = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_PYROMANCER)
        .expect("Goblin Pyromancer is cataloged");
    drain_pending(&mut game);

    let sharpshooter = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sharpshooter_id)
        .expect("Sharpshooter remains through the entry trigger");
    assert_eq!(game.power(sharpshooter), Some(4));
    let pyromancer = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == pyromancer_id)
        .expect("Pyromancer entered");
    assert_eq!(game.power(pyromancer), Some(5));

    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    assert!(game.battlefield.iter().all(|permanent| {
        permanent.card.id != sharpshooter_id && permanent.card.id != pyromancer_id
    }));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions_id),
        "the non-Goblin survives"
    );
}

#[test]
fn tsabos_web_draws_and_keeps_only_utility_lands_tapped() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()]
        .library
        .push(card(10_000, cards::MOUNTAIN, PlayerId::One));
    let web = card(10_001, cards::TSABOS_WEB, PlayerId::One);
    game.players[PlayerId::One.index()].hand.push(web.clone());
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(web.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    drain_pending(&mut game);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "the entry trigger drew a card"
    );

    let mut wasteland = creature(10_002, cards::WASTELAND, PlayerId::One);
    wasteland.tapped = true;
    let wasteland_id = wasteland.card.id;
    let mut plains = creature(10_003, cards::PLAINS, PlayerId::One);
    plains.tapped = true;
    let plains_id = plains.card.id;
    game.battlefield.extend([wasteland, plains]);
    game.active_player = PlayerId::Two;
    game.next_regular_player = PlayerId::One;
    game.start_next_turn();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == wasteland_id)
            .expect("Wasteland remains")
            .tapped,
        "a land with a nonmana activated ability stays tapped"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == plains_id)
            .expect("Plains remains")
            .tapped,
        "a land with only mana abilities untaps"
    );
}

#[test]
fn kor_haven_prevents_damage_from_the_attacker_but_not_to_it() {
    let mut game = ready_game();
    let haven = creature(10_000, cards::KOR_HAVEN, PlayerId::One);
    let haven_id = haven.card.id;
    let mut attacker = creature(10_001, cards::MOGG_FANATIC, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    blocker.blocking = Some(attacker_id);
    let blocker_id = blocker.card.id;
    game.battlefield.extend([haven, attacker, blocker]);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: haven_id,
            ability: activated_ability_for(&game, haven_id, 0),
            targets: activated_targets(Target::Permanent(attacker_id)),
            cost_object: None,
            x: 0,
        },
    )
    .unwrap();
    drain_pending(&mut game);
    game.begin_combat_damage_assignment();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attacker_id),
        "the blocker still deals lethal combat damage to the attacker"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker_id),
        "the prevented attacker deals no return damage"
    );
}
