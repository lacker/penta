use super::*;

pub(super) fn dust_to_dust_targets(game: &mut Game, mut spell: StackObject) {
    spell.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(
            vec![
                Target::Permanent(CardInstanceId(10_000)),
                Target::Permanent(CardInstanceId(10_001)),
            ],
            0,
        ),
    ));
    game.resolve_spell_effect(&spell, CardBehavior::DustToDust);
}

#[test]
fn wrath_and_supreme_verdict_use_equivalent_declarative_creature_sweepers() {
    let game = ready_game();
    for (definition, can_regenerate, cannot_be_countered) in [
        (cards::WRATH_OF_GOD, false, false),
        (cards::SUPREME_VERDICT, true, true),
    ] {
        let definition = game.catalog.get(definition).unwrap();
        assert_eq!(definition.rules.special_behavior(), None);
        assert!(
            definition
                .rules
                .ability_clauses()
                .iter()
                .all(|ability| ability.declarative_effect().is_some())
        );
        assert!(definition.rules.ability_clauses().iter().any(|ability| {
            let EffectDef::Destroy {
                object:
                    EffectRecipientDef::MatchingObjects {
                        object,
                        zones,
                        controller,
                    },
                can_regenerate: actual,
            } = ability.effect.definition
            else {
                return false;
            };
            object == ObjectPredicateDef::HasType(CardType::Creature)
                && zones == [ZoneKind::Battlefield]
                && controller == PlayerRelation::Any
                && actual == can_regenerate
        }));
        assert_eq!(
            definition.rules.ability_clauses().iter().any(|ability| {
                matches!(
                    ability.effect.definition,
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::CannotBeCountered,
                        ..
                    }
                )
            }),
            cannot_be_countered,
        );
    }
}

#[test]
fn nevinyrrals_disk_declares_shared_costs_and_a_global_destroy_effect() {
    let game = ready_game();
    let definition = game
        .catalog
        .get(cards::NEVINYRRALS_DISK)
        .expect("Nevinyrral's Disk is in the catalog");
    assert_eq!(definition.rules.special_behavior(), None);

    let ability = definition
        .rules
        .ability_clauses()
        .iter()
        .find(|ability| matches!(ability.definition, DeclarativeAbilityDef::Activated(_)))
        .expect("the Disk has an activated ability");
    let DeclarativeAbilityDef::Activated(activated) = ability.definition else {
        unreachable!("the selected ability is activated")
    };
    assert_eq!(activated.procedure, AbilityProcedureDef::Shared);
    assert_eq!(
        activated.costs.as_slice(),
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ]
    );

    let EffectDef::Destroy {
        object:
            EffectRecipientDef::MatchingObjects {
                object,
                zones,
                controller,
            },
        can_regenerate,
    } = ability.effect.definition
    else {
        panic!("the Disk uses the shared global destruction effect")
    };
    assert_eq!(
        object,
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])
    );
    assert_eq!(zones, [ZoneKind::Battlefield]);
    assert_eq!(controller, PlayerRelation::Any);
    assert!(can_regenerate);
}

#[test]
fn nevinyrrals_disk_uses_the_shared_stack_and_destroys_every_named_type() {
    let mut game = ready_game();
    let disk = creature(10_000, cards::NEVINYRRALS_DISK, PlayerId::One);
    let disk_id = disk.card.id;
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let lions = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let moat = creature(10_003, cards::MOAT, PlayerId::Two);
    let mountain = creature(10_004, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    let mut troll = creature(10_005, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    troll.regeneration_shields = 1;
    game.battlefield
        .extend([disk, mox, lions, moat, mountain, troll]);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let activation = Action::ActivateAbility {
        source: disk_id,
        ability: activated_ability_for(&game, disk_id, 0),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default()
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == disk_id)
            .is_some_and(|permanent| permanent.tapped),
        "the Disk taps but remains on the battlefield while its ability is on the stack"
    );
    assert!(matches!(
        game.stack
            .last()
            .and_then(|object| object.ability.as_ref())
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Declarative(_))
    ));

    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![mountain_id, troll_id],
        "the land and regenerated creature are the only survivors"
    );
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("the Troll regenerated");
    assert!(troll.tapped);
    assert_eq!(troll.regeneration_shields, 0);
}

#[test]
fn regeneration_shields_stop_destroy_but_not_wrath() {
    let mut game = ready_game();
    let mut troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    troll.regeneration_shields = 1;
    game.battlefield.push(troll);
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.battlefield[0].regeneration_shields, 0);

    game.battlefield[0].regeneration_shields = 1;

    let wrath = spell(10_001, cards::WRATH_OF_GOD, PlayerId::Two, 0);
    let effect = game
        .catalog
        .get(cards::WRATH_OF_GOD)
        .expect("Wrath of God is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect
        .definition;
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &wrath,
        TriggerContext::empty(),
    );
    assert!(game.battlefield.is_empty());
}

#[test]
fn shatter_does_not_destroy_darksteel_ingot() {
    let mut game = ready_game();
    let ingot = creature(10_000, cards::DARKSTEEL_INGOT, PlayerId::Two);
    let ingot_id = ingot.card.id;
    let shatter = card(10_001, cards::SHATTER, PlayerId::One);
    game.battlefield.push(ingot);
    game.players[0].hand.push(shatter.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.red = 1;

    let cast = cast_action(shatter.id, vec![Target::Permanent(ingot_id)], Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "indestructible does not make the artifact an illegal target",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let ingot = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ingot_id)
        .expect("Darksteel Ingot survives Shatter");
    assert!(game.permanent_has_executable_keyword(ingot, KeywordAbility::Indestructible));
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SHATTER),
        "Shatter resolved and went to its owner's graveyard",
    );
}

#[test]
fn indestructible_stops_destruction_and_lethal_damage_but_not_other_death() {
    let mut game = ready_game();
    let mut lions = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    lions
        .temporary_keywords
        .push(KeywordAbility::Indestructible);
    game.battlefield.push(lions);

    game.destroy_permanent_without_regeneration(lions_id);
    assert_eq!(
        game.battlefield.len(),
        1,
        "no-regeneration destroy still fails"
    );

    game.battlefield[0].damage = 1;
    game.check_state_based_actions();
    assert_eq!(
        game.battlefield.len(),
        1,
        "lethal damage does not destroy it"
    );

    game.battlefield[0].toughness_bonus = -1;
    game.check_state_based_actions();
    assert!(
        game.battlefield.is_empty(),
        "zero toughness puts it into the graveyard without destroying it",
    );

    let mut angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    angel
        .temporary_keywords
        .push(KeywordAbility::Indestructible);
    angel.damage = 1;
    angel.deathtouch_damage = true;
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    game.check_state_based_actions();
    assert_eq!(
        game.battlefield.len(),
        1,
        "deathtouch damage does not destroy it"
    );

    game.sacrifice_permanent(angel_id);
    assert!(
        game.battlefield.is_empty(),
        "indestructible can be sacrificed"
    );
}

#[test]
fn moat_prevents_nonfliers_and_argothian_pixies_dodge_artifact_blockers() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.battlefield
        .push(creature(10_000, cards::MOAT, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERENDIB_EFREET, PlayerId::One));
    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_001),
        defender: AttackDefender::Player(PlayerId::Two),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_002),
        defender: AttackDefender::Player(PlayerId::Two),
    }));

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    let mut pixies = creature(10_003, cards::ARGOTHIAN_PIXIES, PlayerId::One);
    pixies.attacking = true;
    game.battlefield.push(pixies);
    game.battlefield
        .push(creature(10_004, cards::SU_CHI, PlayerId::Two));
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_004),
                attacker: CardInstanceId(10_003),
            })
    );
}

#[test]
fn firebreathing_is_offered_while_the_mana_is_still_in_the_land() {
    for definition in [
        cards::DRAGON_WHELP,
        cards::GOBLIN_BALLOON_BRIGADE,
        cards::GRANITE_GARGOYLE,
    ] {
        let mut game = ready_game();
        let source = creature(10_000, definition, PlayerId::One);
        let source_id = source.card.id;
        game.battlefield.push(source);
        game.battlefield
            .push(creature(10_001, cards::MOUNTAIN, PlayerId::One));
        assert_eq!(game.players[0].mana_pool.red, 0);

        let activation = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if targets.is_empty() && *source == source_id)
            })
            .expect("the ability is offered with an untapped Mountain and an empty pool");

        let before = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        while !game.stack.is_empty() {
            game.apply(PlayerId::One, Action::PassPriority).unwrap();
            game.apply(PlayerId::Two, Action::PassPriority).unwrap();
        }

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::MOUNTAIN && permanent.tapped),
            "activating tapped the land for you",
        );
        assert_eq!(
            game.players[0].mana_pool.red, 0,
            "and spent exactly the red it produced",
        );
        let after = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
        match definition {
            cards::DRAGON_WHELP => {
                assert_eq!(
                    after.0,
                    before.0.map(|value| value + 1),
                    "Dragon Whelp grew"
                );
            }
            cards::GOBLIN_BALLOON_BRIGADE => {
                assert!(!before.2);
                assert!(after.2, "Goblin Balloon Brigade gained flying");
            }
            cards::GRANITE_GARGOYLE => {
                assert_eq!(
                    after.1,
                    before.1.map(|value| value + 1),
                    "Granite Gargoyle gained toughness",
                );
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn negate_and_essence_scatter_split_the_stack_by_card_kind() {
    let mut game = ready_game();
    // A creature spell and a noncreature spell, both waiting to resolve.
    game.stack
        .push(spell(10_001, cards::SAVANNAH_LIONS, PlayerId::Two, 0));
    game.stack
        .push(spell(10_002, cards::LIGHTNING_BOLT, PlayerId::Two, 0));

    let spells_hit = |game: &Game, behavior| -> Vec<StackObjectId> {
        game.legal_target_lists(behavior, PlayerId::One, None)
            .into_iter()
            .filter_map(|choice| match choice.first() {
                Some(Target::Spell(id)) => Some(*id),
                _ => None,
            })
            .collect()
    };

    let scatter = spells_hit(&game, CardBehavior::EssenceScatter);
    assert_eq!(
        scatter,
        vec![StackObjectId(10_001)],
        "Essence Scatter sees only the creature spell"
    );
    let negate = spells_hit(&game, CardBehavior::Negate);
    assert_eq!(
        negate,
        vec![StackObjectId(10_002)],
        "Negate sees only the noncreature spell"
    );

    let counter = spell_with_targets(
        10_003,
        cards::NEGATE,
        PlayerId::One,
        vec![Target::Spell(StackObjectId(10_002))],
        0,
    );
    game.resolve_spell_effect(&counter, CardBehavior::Negate);
    assert!(
        !game
            .stack
            .iter()
            .any(|object| object.id == StackObjectId(10_002)),
        "the countered spell left the stack"
    );
    assert!(
        game.stack
            .iter()
            .any(|object| object.id == StackObjectId(10_001)),
        "and the creature spell is untouched"
    );
}

#[test]
fn sign_in_blood_draws_two_and_costs_two_life_without_dealing_damage() {
    let mut game = ready_game();
    let before_hand = game.players[0].hand.len();
    let before_life = game.players[0].life;
    let sign = card(10_000, cards::SIGN_IN_BLOOD, PlayerId::One);
    game.players[0].hand.push(sign.clone());
    game.players[0].mana_pool.black = 2;
    game.apply(
        PlayerId::One,
        cast_action(sign.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), before_hand + 2);
    assert_eq!(game.players[0].life, before_life - 2);
    // Losing life is not being dealt damage: nothing that triggers on damage
    // may see this, so it must not be logged as damage either.
    assert!(
        game.events
            .iter()
            .any(|event| matches!(event, GameEvent::LifeLost { amount: 2, .. })),
        "the loss is recorded as life loss"
    );
    assert!(
        !game
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::DamageDealt { .. })),
        "and never as damage"
    );
}

#[test]
fn duress_takes_a_noncreature_nonland_card_of_the_casters_choosing() {
    let mut game = ready_game();
    game.players[1].hand.extend([
        card(10_001, cards::SAVANNAH_LIONS, PlayerId::Two), // creature: off limits
        card(10_002, cards::MOUNTAIN, PlayerId::Two),       // land: off limits
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two), // fair game
    ]);

    let cast = spell_with_targets(
        10_000,
        cards::DURESS,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::Duress);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses");
    assert_eq!(
        decision.options.len(),
        1,
        "only the instant is a legal choice"
    );
    // The hand is revealed, so the choice is public rather than hidden.
    assert_eq!(decision.visibility, DecisionVisibility::Public);

    let choice = decision.options[0].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![choice],
        },
    )
    .expect("choosing the revealed card is legal");

    assert_eq!(game.players[1].hand.len(), 2, "one card was discarded");
    assert!(
        !game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and it was the one the caster named"
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
}

#[test]
fn mulch_keeps_the_lands_and_bins_the_rest() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (10_001, cards::MOUNTAIN),
            (10_002, cards::LIGHTNING_BOLT),
            (10_003, cards::MOUNTAIN),
            (10_004, cards::SAVANNAH_LIONS),
            (10_005, cards::BLACK_LOTUS), // fifth card is untouched
        ],
    );
    let before_hand = game.players[0].hand.len();

    let cast = spell(10_000, cards::MULCH, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::Mulch);

    assert_eq!(
        game.players[0].hand.len(),
        before_hand + 2,
        "two lands kept"
    );
    assert_eq!(game.players[0].graveyard.len(), 2, "two nonlands binned");
    assert_eq!(
        game.players[0].library.len(),
        1,
        "only the top four were revealed"
    );
}

#[test]
fn grisly_salvage_may_keep_one_creature_or_land() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One), // not eligible
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One), // creature
        card(10_003, cards::MOUNTAIN, PlayerId::One),       // land
        card(10_004, cards::BLACK_LOTUS, PlayerId::One),    // not eligible
        card(10_005, cards::COUNTERSPELL, PlayerId::One),   // not eligible
    ]);

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::GrislySalvage);

    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    assert_eq!(decision.options.len(), 2, "the creature and the land");
    assert_eq!(
        decision.minimum, 0,
        "'you may' means keeping nothing is legal"
    );

    let keep = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == CardInstanceId(10_003))
        })
        .expect("the land is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![keep],
        },
    )
    .unwrap();

    // A zone change mints a new object id, so the card is identified by what
    // it is rather than by the id it had in the library.
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.players[0].hand[0].definition,
        cards::MOUNTAIN,
        "the chosen land reached hand"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        4,
        "the other four are binned"
    );
    assert!(game.players[0].library.is_empty());
}

#[test]
fn grisly_salvage_can_decline_and_bin_everything() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0]
        .library
        .extend((0..5).map(|i| card(10_100 + i, cards::SAVANNAH_LIONS, PlayerId::One)));

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::GrislySalvage);
    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("declining is legal");

    assert!(game.players[0].hand.is_empty(), "nothing was kept");
    assert_eq!(
        game.players[0].graveyard.len(),
        5,
        "and no revealed card was lost on the way"
    );
}

#[test]
fn sphinxs_revelation_scales_life_and_cards_with_x() {
    let mut game = ready_game();
    let before_life = game.players[0].life;
    let before_hand = game.players[0].hand.len();

    let cast = spell(10_000, cards::SPHINXS_REVELATION, PlayerId::One, 3);
    game.resolve_spell_effect(&cast, CardBehavior::SphinxsRevelation);

    assert_eq!(game.players[0].life, before_life + 3);
    assert_eq!(game.players[0].hand.len(), before_hand + 3);
}

#[test]
fn the_mana_creatures_tap_for_their_colour() {
    // Their whole printed text is a mana ability the engine already models,
    // so they are complete rather than staged.
    for (definition, expected) in [
        (cards::AVACYNS_PILGRIM, ManaColor::White),
        (cards::ELVISH_MYSTIC, ManaColor::Green),
    ] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_001, definition, PlayerId::One));
        assert!(
            game.legal_actions(PlayerId::One)
                .iter()
                .any(|action| matches!(
                    action,
                    Action::ActivateManaAbility { color, .. } if *color == expected
                )),
            "{definition:?} taps for {expected:?}"
        );
    }
}

#[test]
fn deathtouch_kills_whatever_it_touches_and_lifelink_pays_its_controller() {
    // Vampire Nighthawk is a 2/3 flying deathtouch lifelink. Before these
    // keywords were read, it was a 2/3 flier and nothing more.
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = Some(CardInstanceId(10_001));
    game.battlefield.extend([hawk, wall]);
    let before_life = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two deathtouch damage is lethal to a 4/4"
    );
    assert_eq!(
        game.players[0].life,
        before_life + 2,
        "and lifelink paid its controller for the damage dealt"
    );
}

#[test]
fn lifelink_pays_for_damage_to_a_player_too() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    game.battlefield.push(hawk);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert_eq!(game.players[1].life, 18, "unblocked, it hits for two");
    assert_eq!(game.players[0].life, before + 2, "and gains that much");
}

#[test]
fn an_ordinary_creature_does_not_gain_life_or_kill_through_toughness() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One); // 2/1 vanilla
    lions.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = Some(CardInstanceId(10_001));
    game.battlefield.extend([lions, wall]);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two ordinary damage does not kill a 4/4"
    );
    assert_eq!(game.players[0].life, before, "and gains nobody any life");
}

#[test]
fn reach_blocks_fliers_without_flying() {
    // Ruric Thar has reach; a plain ground creature does not.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut flier = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    flier.attacking = true;
    game.battlefield.push(flier);
    game.battlefield.push(creature(
        10_002,
        cards::RURIC_THAR_THE_UNBOWED,
        PlayerId::Two,
    ));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two));

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "reach can block a flier"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a ground creature still cannot"
    );
}

#[test]
fn intimidate_only_lets_artifacts_and_matching_colours_block() {
    // Lifebane Zombie is black; only black or artifact creatures may block it.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut zombie = creature(10_001, cards::LIFEBANE_ZOMBIE, PlayerId::One);
    zombie.attacking = true;
    game.battlefield.push(zombie);
    game.battlefield
        .push(creature(10_002, cards::JUZAM_DJINN, PlayerId::Two)); // black
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two)); // white
    game.battlefield
        .push(creature(10_004, cards::JUGGERNAUT, PlayerId::Two)); // artifact

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "a black creature shares a colour and may block"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a white creature may not"
    );
    assert!(
        blockers.contains(&CardInstanceId(10_004)),
        "an artifact creature may block regardless of colour",
    );
}
