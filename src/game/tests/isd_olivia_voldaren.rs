//! Olivia Voldaren's linked damage rider and source-bound control change.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let olivia = game
        .put_onto_battlefield(PlayerId::One, cards::OLIVIA_VOLDAREN)
        .expect("Olivia Voldaren is cataloged");
    drain_pending(&mut game);
    (game, olivia)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn activation(game: &Game, source: GameObjectId, index: usize, target: GameObjectId) -> Action {
    Action::ActivateAbility {
        source,
        ability: activated_ability_for(game, source, index),
        targets: activated_targets(Target::Permanent(target)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    }
}

fn bear(id: u32, controller: PlayerId) -> Permanent {
    token_permanent(
        id,
        tokens::creature(&["Bear"], &[ManaColor::Green], 2, 2),
        controller,
    )
}

fn vampire(id: u32, controller: PlayerId) -> Permanent {
    token_permanent(
        id,
        tokens::creature(&["Vampire"], &[ManaColor::Black], 2, 2),
        controller,
    )
}

#[test]
fn the_bite_cannot_target_olivia_and_permanently_adds_vampire_before_the_counter() {
    let (mut game, olivia) = staged();
    let victim = GameObjectId(10_100);
    game.battlefield.push(bear(victim.0, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&activation(&game, olivia, 0, olivia)),
        "another target creature excludes Olivia herself",
    );
    let bite = activation(&game, olivia, 0, victim);
    assert!(game.legal_actions(PlayerId::One).contains(&bite));
    game.apply(PlayerId::One, bite)
        .expect("Olivia may bite another creature");
    pass_priority_pair(&mut game);

    let bitten = permanent(&game, victim);
    assert_eq!(bitten.damage, 1);
    let subtypes = game.effective_subtypes(bitten);
    assert!(subtypes.contains(&"Bear"));
    assert!(subtypes.contains(&"Vampire"));
    assert_eq!(
        permanent(&game, olivia).counters(CounterKind::PlusOnePlusOne),
        1,
    );
    assert_eq!(
        (
            game.power(permanent(&game, olivia)),
            game.toughness(permanent(&game, olivia)),
        ),
        (Some(4), Some(4)),
    );

    game.move_permanents_to_graveyard(&[olivia]);
    game.check_state_based_actions();
    assert!(
        game.effective_subtypes(permanent(&game, victim))
            .contains(&"Vampire"),
        "the subtype change does not depend on Olivia remaining",
    );
}

#[test]
fn preventing_the_damage_does_not_prevent_the_vampire_rider_or_counter() {
    let (mut game, olivia) = staged();
    let victim = GameObjectId(10_100);
    game.battlefield.push(bear(victim.0, PlayerId::Two));
    let shield = spell_with_targets(
        20_000,
        cards::FOG,
        PlayerId::Two,
        vec![Target::Permanent(victim)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PreventDamage {
            prevention: crate::card::DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        }),
        &shield,
        TriggerContext::empty(),
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);

    game.apply(PlayerId::One, activation(&game, olivia, 0, victim))
        .expect("Olivia may bite the shielded creature");
    pass_priority_pair(&mut game);

    assert_eq!(permanent(&game, victim).damage, 0);
    assert!(
        game.effective_subtypes(permanent(&game, victim))
            .contains(&"Vampire"),
    );
    assert_eq!(
        permanent(&game, olivia).counters(CounterKind::PlusOnePlusOne),
        1,
    );
}

#[test]
fn the_second_ability_targets_only_vampires_and_control_ends_when_olivia_leaves() {
    let (mut game, olivia) = staged();
    let bear_id = GameObjectId(10_100);
    let vampire_id = GameObjectId(10_101);
    game.battlefield.extend([
        bear(bear_id.0, PlayerId::Two),
        vampire(vampire_id.0, PlayerId::Two),
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&activation(&game, olivia, 1, bear_id)),
        "the control ability cannot target a non-Vampire",
    );
    let steal = activation(&game, olivia, 1, vampire_id);
    assert!(game.legal_actions(PlayerId::One).contains(&steal));
    game.apply(PlayerId::One, steal)
        .expect("Olivia may take control of a Vampire");
    pass_priority_pair(&mut game);
    assert_eq!(permanent(&game, vampire_id).controller, PlayerId::One);

    game.move_permanents_to_graveyard(&[olivia]);
    game.check_state_based_actions();
    assert_eq!(
        permanent(&game, vampire_id).controller,
        PlayerId::Two,
        "the Vampire returns when Olivia leaves the battlefield",
    );
}

#[test]
fn losing_control_of_olivia_before_the_second_ability_resolves_prevents_the_theft() {
    let (mut game, olivia) = staged();
    let vampire_id = GameObjectId(10_100);
    game.battlefield.push(vampire(vampire_id.0, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);

    game.apply(PlayerId::One, activation(&game, olivia, 1, vampire_id))
        .expect("Olivia's control ability is activated");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == olivia)
        .expect("Olivia remains on the battlefield")
        .controller = PlayerId::Two;
    pass_priority_pair(&mut game);

    assert_eq!(
        permanent(&game, vampire_id).controller,
        PlayerId::Two,
        "the source-bound effect has no lasting control change",
    );
}
