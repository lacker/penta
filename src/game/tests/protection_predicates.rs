//! Protection qualities expressed as composable object predicates.
//!
//! These cards cover card types, spell status, color count, a negated subtype,
//! and a player stored on the protected permanent. Together they pin all four
//! parts of protection -- damage, enchanting, blocking, and targeting -- plus
//! temporary grants and frozen recipient sets.

use super::*;

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn has_protection(game: &Game, id: GameObjectId, predicate: &'static ObjectPredicateDef) -> bool {
    game.permanent_has_executable_keyword(
        permanent(game, id),
        KeywordAbility::ProtectionFrom(predicate),
    )
}

#[test]
fn spare_from_evil_grants_temporary_protection_to_the_creatures_it_resolves_over() {
    let mut game = ready_game();
    let protected = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let protected_id = protected.card.id;
    let opposing = creature(20_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let opposing_id = opposing.card.id;
    game.battlefield.extend([protected, opposing]);

    let spare = card(20_002, cards::SPARE_FROM_EVIL, PlayerId::One);
    game.players[0].hand.push(spare.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(spare.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Spare from Evil is castable");
    drain_pending(&mut game);

    assert!(has_protection(
        &game,
        protected_id,
        &ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
        ]),
    ));
    assert!(
        !has_protection(
            &game,
            opposing_id,
            &ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
            ]),
        ),
        "only the caster's creatures receive the grant",
    );

    let late = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("the Bear is cataloged");
    assert!(
        !has_protection(
            &game,
            late,
            &ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
            ]),
        ),
        "a resolving group grant freezes its recipients",
    );

    let human = token_permanent(
        20_003,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
        PlayerId::Two,
    );
    let human_id = human.card.id;
    game.battlefield.push(human);

    assert!(
        !game.permanent_can_be_targeted_by(
            permanent(&game, protected_id),
            PlayerId::Two,
            opposing_id,
            false,
        ),
        "a non-Human creature cannot target the protected creature",
    );
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, protected_id),
        PlayerId::Two,
        human_id,
        false,
    ));
    assert!(game.combat_is_protected(
        permanent(&game, opposing_id),
        permanent(&game, protected_id),
    ));
    assert!(!game.combat_is_protected(permanent(&game, human_id), permanent(&game, protected_id),));

    game.damage_target_from(Some(opposing_id), Some(Target::Permanent(protected_id)), 2);
    assert_eq!(permanent(&game, protected_id).damage, 0);
    game.damage_target_from(Some(human_id), Some(Target::Permanent(protected_id)), 1);
    assert_eq!(permanent(&game, protected_id).damage, 1);

    game.finish_cleanup();
    assert!(!has_protection(
        &game,
        protected_id,
        &ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
        ]),
    ));
}

#[test]
fn azorius_first_wing_applies_every_part_of_protection_from_enchantments() {
    let mut game = ready_game();
    let first_wing = creature(20_100, cards::AZORIUS_FIRST_WING, PlayerId::One);
    let first_wing_id = first_wing.card.id;
    let enchantment_creature = creature(20_101, cards::ENDURING_INNOCENCE, PlayerId::Two);
    let enchantment_creature_id = enchantment_creature.card.id;
    let bear = creature(20_102, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    game.battlefield
        .extend([first_wing, enchantment_creature, bear]);

    let aura_spell = card(20_103, cards::HOLY_STRENGTH, PlayerId::Two);
    let aura_spell_id = aura_spell.id;
    game.players[1].hand.push(aura_spell);
    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, first_wing_id),
        PlayerId::Two,
        aura_spell_id,
        true,
    ));

    let mut attached = creature(20_104, cards::HOLY_STRENGTH, PlayerId::Two);
    attached.attached_to = Some(first_wing_id);
    let attached_id = attached.card.id;
    game.battlefield.push(attached);
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attached_id),
        "an attached enchantment becomes illegal and falls off",
    );

    game.damage_target_from(
        Some(enchantment_creature_id),
        Some(Target::Permanent(first_wing_id)),
        2,
    );
    assert_eq!(permanent(&game, first_wing_id).damage, 0);
    assert!(game.combat_is_protected(
        permanent(&game, enchantment_creature_id),
        permanent(&game, first_wing_id),
    ));

    game.damage_target_from(Some(bear_id), Some(Target::Permanent(first_wing_id)), 1);
    assert_eq!(permanent(&game, first_wing_id).damage, 1);
    assert!(!game.combat_is_protected(permanent(&game, bear_id), permanent(&game, first_wing_id),));
}

#[test]
fn beloved_chaplain_is_protected_from_creatures_of_every_kind() {
    let mut game = ready_game();
    let chaplain = creature(20_200, cards::BELOVED_CHAPLAIN, PlayerId::One);
    let chaplain_id = chaplain.card.id;
    let artifact_creature = creature(20_201, cards::ORNITHOPTER, PlayerId::Two);
    let artifact_creature_id = artifact_creature.card.id;
    game.battlefield.extend([chaplain, artifact_creature]);

    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, chaplain_id),
        PlayerId::Two,
        artifact_creature_id,
        false,
    ));
    assert!(game.combat_is_protected(
        permanent(&game, artifact_creature_id),
        permanent(&game, chaplain_id),
    ));
    game.damage_target_from(
        Some(artifact_creature_id),
        Some(Target::Permanent(chaplain_id)),
        1,
    );
    assert_eq!(permanent(&game, chaplain_id).damage, 0);
}

#[test]
fn devoted_caretaker_grants_protection_from_spells_but_not_permanent_abilities() {
    let mut game = ready_game();
    let caretaker = creature(20_300, cards::DEVOTED_CARETAKER, PlayerId::One);
    let caretaker_id = caretaker.card.id;
    let protected = creature(20_301, cards::SAVANNAH_LIONS, PlayerId::One);
    let protected_id = protected.card.id;
    let permanent_source = creature(20_302, cards::ATOG, PlayerId::Two);
    let permanent_source_id = permanent_source.card.id;
    game.battlefield
        .extend([caretaker, protected, permanent_source]);
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: caretaker_id,
            ability: activated_ability_for(&game, caretaker_id, 0),
            targets: activated_targets(Target::Permanent(protected_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .expect("the Caretaker can protect its controller's permanent");
    pass_priority_pair(&mut game);

    let instant = card(20_303, cards::LIGHTNING_BOLT, PlayerId::Two);
    let instant_id = instant.id;
    game.players[1].hand.push(instant);
    let sorcery = card(20_304, cards::FLAME_SLASH, PlayerId::Two);
    let sorcery_id = sorcery.id;
    game.players[1].hand.push(sorcery);
    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, protected_id),
        PlayerId::Two,
        instant_id,
        true,
    ));
    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, protected_id),
        PlayerId::Two,
        sorcery_id,
        true,
    ));
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, protected_id),
        PlayerId::Two,
        permanent_source_id,
        false,
    ));

    let spell_source = spell(20_305, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let spell_source_id = spell_source.id;
    game.stack.push(spell_source);
    game.damage_target_from(
        Some(spell_source_id),
        Some(Target::Permanent(protected_id)),
        3,
    );
    assert_eq!(permanent(&game, protected_id).damage, 0);

    game.finish_cleanup();
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, protected_id),
        PlayerId::Two,
        instant_id,
        true,
    ));
}

#[test]
fn stonecoil_serpent_enters_for_x_and_uses_color_count_for_protection() {
    let mut game = ready_game();
    let stonecoil = card(20_400, cards::STONECOIL_SERPENT, PlayerId::One);
    game.players[0].hand.push(stonecoil.clone());
    game.players[0].mana_pool.colorless = 3;
    game.apply(
        PlayerId::One,
        cast_action(stonecoil.id, Vec::new(), Vec::new(), 3),
    )
    .expect("Stonecoil Serpent is castable for X equals three");
    drain_pending(&mut game);

    let serpent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::STONECOIL_SERPENT)
        .expect("Stonecoil resolved");
    let serpent_id = serpent.card.id;
    assert_eq!(serpent.counters(CounterKind::PlusOnePlusOne), 3);
    assert_eq!(game.power(serpent), Some(3));
    assert!(game.permanent_has_executable_keyword(serpent, KeywordAbility::Reach));
    assert!(game.permanent_has_executable_keyword(serpent, KeywordAbility::Trample));

    let multicolored = creature(20_401, cards::LOXODON_SMITER, PlayerId::Two);
    let multicolored_id = multicolored.card.id;
    let monocolored = creature(20_402, cards::GRIZZLY_BEARS, PlayerId::Two);
    let monocolored_id = monocolored.card.id;
    game.battlefield.extend([multicolored, monocolored]);

    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, serpent_id),
        PlayerId::Two,
        multicolored_id,
        false,
    ));
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, serpent_id),
        PlayerId::Two,
        monocolored_id,
        false,
    ));
    game.damage_target_from(
        Some(multicolored_id),
        Some(Target::Permanent(serpent_id)),
        3,
    );
    assert_eq!(permanent(&game, serpent_id).damage, 0);
    game.damage_target_from(Some(monocolored_id), Some(Target::Permanent(serpent_id)), 1);
    assert_eq!(permanent(&game, serpent_id).damage, 1);
}

#[test]
fn true_name_nemesis_uses_the_player_chosen_as_it_enters() {
    let mut game = ready_game();
    let definition = game
        .catalog
        .get(cards::TRUE_NAME_NEMESIS)
        .expect("True-Name Nemesis is cataloged");
    assert_eq!(
        definition.rules.ability_clauses()[1].text,
        "This creature has protection from the chosen player.",
    );
    let nemesis_id = game
        .put_onto_battlefield(PlayerId::One, cards::TRUE_NAME_NEMESIS)
        .expect("True-Name Nemesis is cataloged");
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("its controller chooses a player before it enters");
    assert_eq!(
        decision
            .options
            .iter()
            .map(|option| option.label.as_str())
            .collect::<Vec<_>>(),
        vec!["You", "Opponent"],
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Opponent");

    assert_eq!(
        permanent(&game, nemesis_id).chosen_player,
        Some(PlayerId::Two)
    );
    let chosen_players_source = creature(20_501, cards::GRIZZLY_BEARS, PlayerId::Two);
    let chosen_players_source_id = chosen_players_source.card.id;
    let other_source = creature(20_502, cards::GRIZZLY_BEARS, PlayerId::One);
    let other_source_id = other_source.card.id;
    game.battlefield
        .extend([chosen_players_source, other_source]);

    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, nemesis_id),
        PlayerId::Two,
        chosen_players_source_id,
        false,
    ));
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, nemesis_id),
        PlayerId::One,
        other_source_id,
        false,
    ));
    assert!(game.combat_is_protected(
        permanent(&game, chosen_players_source_id),
        permanent(&game, nemesis_id),
    ));
    assert!(!game.combat_is_protected(
        permanent(&game, other_source_id),
        permanent(&game, nemesis_id),
    ));
    game.damage_target_from(
        Some(chosen_players_source_id),
        Some(Target::Permanent(nemesis_id)),
        2,
    );
    assert_eq!(permanent(&game, nemesis_id).damage, 0);
    game.damage_target_from(
        Some(other_source_id),
        Some(Target::Permanent(nemesis_id)),
        1,
    );
    assert_eq!(permanent(&game, nemesis_id).damage, 1);
}
