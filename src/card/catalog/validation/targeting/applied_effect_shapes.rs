// Keep the applied vocabulary's exhaustive shape validation in one match.
#[allow(clippy::too_many_lines)]
fn validate_applied_effect_shapes(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
    targets: &[AbilityTargetDef],
    static_effect: bool,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                validate_applied_effect_shapes(recipient, *effect, targets, static_effect)?;
            }
            Ok(())
        }
        // A grant over a whole graveyard names the player whose graveyard it
        // is, the same way the permissions below name a player. A timing
        // permission names its player for the same reason: no object has a
        // casting window of its own.
        AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
            object,
            ..
        }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(permission)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(permission.object, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlay(permission)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_query_shape(permission.cards, targets)?;
            validate_play_permission_shapes(recipient, permission.restriction, permission.benefit, targets, static_effect)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::KnownCards(query) | AppliedRuleDef::MayPlot { cards: query, .. }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_query_shape(query, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::ModifyTriggers(modification)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_trigger_event_references(
                modification.cause,
                targets.len(),
                BindingScope::empty(&BindingRegistry::default()),
            )?;
            modification.permanent.map_or(Ok(()), |predicate| {
                validate_object_predicate_shape(predicate, targets)
            })
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(restriction)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(restriction.object, targets)?;
            if static_effect && !static_play_rule_recipient_supported(recipient) {
                return Err(
                    GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient {
                        recipient: Box::new(recipient),
                    },
                );
            }
            Ok(())
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(rule)) => {
            validate_player_rule_shape(recipient, rule, targets, static_effect)
        }
        // Each names a player and carries nothing else.
        AppliedEffectDef::Rule(
            AppliedRuleDef::Ascend
            | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
            | AppliedRuleDef::MayPlayAdditionalLands(_)
            | AppliedRuleDef::MayPlayAnyNumberOfLands
            | AppliedRuleDef::CannotDrawMoreThanEachTurn(_)
            | AppliedRuleDef::RevealsDrawnCards
            | AppliedRuleDef::CannotGainLife
            | AppliedRuleDef::DoublesTokensCreated,
        ) => validate_recipient_shape(recipient, targets, RecipientExpectation::Player),
        // The cap names the players it applies to; the predicate picks out
        // which of their permanents it covers.
        AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(predicate)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(predicate, targets)
        }
        // The protection names the player who has it and the quality it is
        // from, which is an ordinary object predicate read against whatever
        // would damage, target, or enchant them.
        AppliedEffectDef::Rule(AppliedRuleDef::PlayerProtectionFrom(quality)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(quality, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            validate_attack_restriction_shape(recipient, restriction, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(restriction)) => {
            validate_block_restriction_shape(recipient, restriction, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(matcher)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Any)?;
            validate_damage_matcher_shape(matcher, targets)
        }
        // A limit protects a player, so unlike prevention its recipient is
        // the player whose damage is capped.
        AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage { matcher, .. }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_damage_matcher_shape(matcher, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::RedirectDamageFromTo {
            source,
            destination,
        }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_reference_shape(source, targets)?;
            validate_object_reference_shape(destination, targets)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        )) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)?;
            validate_value_shape(power, targets)?;
            validate_value_shape(toughness, targets)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::AddActivatedAbilitiesOf { cards, object },
        )) => {
            if let crate::card::ActivatedAbilityCardsDef::Query(query) = cards {
                validate_query_shape(query, targets)?;
            }
            validate_object_predicate_shape(object, targets)?;
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)
        }
    }
}

