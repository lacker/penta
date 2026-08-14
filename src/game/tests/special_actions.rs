use super::*;

static GAIN_LIFE_SPECIAL_ACTION: [AbilityDef; 1] = [AbilityDef::special_action(
    "{U}: You gain 2 life.",
    &[ZoneKind::Battlefield],
    &[AbilityCostDef::Mana(crate::mana_cost!("{U}"))],
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
)];
static END_AURA_SPECIAL_ACTION: [AbilityDef; 1] = [AbilityDef::special_action(
    "{U}: End this effect.",
    &[ZoneKind::Battlefield],
    &[AbilityCostDef::Mana(crate::mana_cost!("{U}"))],
    EffectDef::EndAuraEffect,
)];
static PARALLEL_LICID_TRANSFORMS: [AbilityDef; 2] = [
    AbilityDef::activated(
        "First Licid transform.",
        &[],
        EffectDef::BecomeAuraAndAttach {
            object: EffectRecipientDef::Source,
            end: &END_AURA_SPECIAL_ACTION[0],
        },
    ),
    AbilityDef::activated(
        "Second Licid transform.",
        &[],
        EffectDef::BecomeAuraAndAttach {
            object: EffectRecipientDef::Source,
            end: &END_AURA_SPECIAL_ACTION[0],
        },
    ),
];

fn add_test_card(game: &mut Game, id: CardDefinitionId, abilities: &'static [AbilityDef]) {
    let mut definition = CardDefinition::new(
        id,
        "Special action test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
}

fn activate_licid(
    game: &mut Game,
    source: GameObjectId,
    transform: AbilityOrigin,
    host: GameObjectId,
) {
    game.activate_ability(
        PlayerId::One,
        source,
        transform,
        vec![TargetSelection::new(
            TargetSlotId(0),
            vec![Target::Permanent(host)],
        )],
        None,
        0,
    );
}

fn special_action_effect_id(action: &Action) -> u64 {
    let Action::TakeSpecialAction {
        effect_id: Some(id),
        ..
    } = action
    else {
        panic!("expected an effect-specific special action");
    };
    *id
}

#[test]
fn special_action_is_payable_and_resolves_immediately_without_the_stack() {
    let definition = CardDefinitionId(10_090);
    let source = GameObjectId(10_000);
    let mut game = ready_game();
    add_test_card(&mut game, definition, &GAIN_LIFE_SPECIAL_ACTION);
    game.battlefield
        .push(creature(source.0, definition, PlayerId::One));

    assert!(
        !game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::TakeSpecialAction { source: candidate, .. } if *candidate == source)),
        "an unpayable special action is not advertised",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TakeSpecialAction { source: candidate, .. } if *candidate == source))
        .expect("the paid special action is legal");
    let stack_len = game.stack.len();
    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.players[PlayerId::One.index()].life, 22);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.total(), 0);
    assert_eq!(game.stack.len(), stack_len);
}

#[test]
fn end_aura_special_action_exists_only_for_a_licid_form() {
    let definition = CardDefinitionId(10_091);
    let source = GameObjectId(10_000);
    let host = GameObjectId(10_001);
    let mut game = ready_game();
    add_test_card(&mut game, definition, &END_AURA_SPECIAL_ACTION);
    game.battlefield.extend([
        creature(source.0, definition, PlayerId::Two),
        creature(host.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    assert!(
        !game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::TakeSpecialAction { source: candidate, .. } if *candidate == source)),
    );

    {
        let attachment = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
            .unwrap();
        attachment.attachment_form = Some(AttachmentForm::Licid);
        attachment.licid_effects.push(LicidEffect {
            id: ContinuousEffectTimestamp(99),
            ender: PlayerId::One,
            transform_action: AbilityOrigin::Printed {
                definition,
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            },
            end: END_AURA_SPECIAL_ACTION[0],
        });
        attachment.attached_to = Some(host);
    }
    let removal = spell_with_targets(
        20_000,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(source)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Any),
            duration: EffectDurationDef::UntilEndOfTurn,
        }),
        &removal,
        TriggerContext::empty(),
    );
    let attachment = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    assert!(game.effective_abilities(attachment).is_empty());
    game.priority = PlayerId::Two;
    assert!(
        !game.legal_actions(PlayerId::Two)
            .iter()
            .any(|action| matches!(action, Action::TakeSpecialAction { source: candidate, .. } if *candidate == source)),
        "the Licid's current controller does not inherit another player's permission",
    );
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TakeSpecialAction { source: candidate, .. } if *candidate == source))
        .expect("the player who created the Licid effect can end it");
    game.apply(PlayerId::One, action).unwrap();

    let source = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("ending the effect keeps the Licid on the battlefield");
    assert_eq!(source.attachment_form, None);
    assert_eq!(source.attached_to, None);
    assert!(game.stack.is_empty());
}

#[test]
fn independently_resolving_licid_effects_have_distinct_end_actions() {
    let source = GameObjectId(10_000);
    let host_a = GameObjectId(10_001);
    let host_b = GameObjectId(10_002);
    let transform = AbilityOrigin::Printed {
        definition: cards::DOMINATING_LICID,
        part: CardPartId::PRIMARY,
        ability: AbilityId(0),
    };
    let mut game = ready_game();
    game.battlefield.extend([
        creature(source.0, cards::DOMINATING_LICID, PlayerId::One),
        creature(host_a.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(host_b.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    activate_licid(&mut game, source, transform, host_a);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap()
        .tapped = false;
    activate_licid(&mut game, source, transform, host_b);
    assert_eq!(game.stack.len(), 2);

    game.resolve_stack_top();
    game.resolve_stack_top();
    let licid = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    assert_eq!(licid.attachment_form, Some(AttachmentForm::Licid));
    assert_eq!(licid.attached_to, Some(host_a));
    assert_eq!(licid.licid_effects.len(), 2);
    assert_ne!(licid.licid_effects[0].id, licid.licid_effects[1].id);

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    game.priority = PlayerId::One;
    let mut end_actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(
                action,
                Action::TakeSpecialAction {
                    source: candidate,
                    effect_id: Some(_),
                    ..
                } if *candidate == source
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(end_actions.len(), 2);
    let first_id = special_action_effect_id(&end_actions[0]);
    let second_id = special_action_effect_id(&end_actions[1]);
    assert_ne!(first_id, second_id);

    game.apply(PlayerId::One, end_actions.remove(0)).unwrap();
    let licid = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    assert_eq!(licid.attachment_form, Some(AttachmentForm::Licid));
    assert_eq!(licid.attached_to, Some(host_a));
    assert_eq!(licid.licid_effects.len(), 1);

    let last = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::TakeSpecialAction {
                    source: candidate,
                    effect_id: Some(id),
                    ..
                } if *candidate == source && *id == second_id
            )
        })
        .unwrap();
    game.apply(PlayerId::One, last).unwrap();
    let licid = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    assert_eq!(licid.attachment_form, None);
    assert_eq!(licid.attached_to, None);
    assert!(licid.licid_effects.is_empty());
}

#[test]
fn licid_form_suppresses_only_the_transform_origin_that_created_it() {
    let definition = CardDefinitionId(10_092);
    let source = GameObjectId(10_000);
    let first = AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: AbilityId(0),
    };
    let second = AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: AbilityId(1),
    };
    let mut game = ready_game();
    add_test_card(&mut game, definition, &PARALLEL_LICID_TRANSFORMS);
    let mut licid = creature(source.0, definition, PlayerId::One);
    licid.attachment_form = Some(AttachmentForm::Licid);
    licid.licid_effects.push(LicidEffect {
        id: ContinuousEffectTimestamp(99),
        ender: PlayerId::One,
        transform_action: first,
        end: END_AURA_SPECIAL_ACTION[0],
    });
    game.battlefield.push(licid);

    let transforms = game
        .effective_abilities(&game.battlefield[0])
        .into_iter()
        .filter(|effective| {
            effective
                .ability
                .declarative_effect()
                .is_some_and(|effect| matches!(effect, EffectDef::BecomeAuraAndAttach { .. }))
        })
        .map(|effective| effective.origin)
        .collect::<Vec<_>>();
    assert_eq!(transforms, vec![second]);
}
