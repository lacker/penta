use super::{
    CardCatalog, CatalogError, GrantedAbilityValidationError, validate_ability_targets,
    validate_replacement_ability_targets, validate_semantic_spell_presentation,
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityEffectDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, AdditionalCostDef, AlternateSpellKind, AlternativeCastKindDef,
    AlternativeCostDef, AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef,
    CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId, CardSet,
    CardStructure, CardType, ChoiceVisibilityDef, ChooseDef, DamageEventMatcherDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DeclarativeAbilityDef, DoubleFacedKind,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaCost, ModeDef, ModeSetDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayActionMatcherDef, PlayOptionDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, PrintedManaCost, ReplacementAbilityDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, SpellForm, TargetChooserDef,
    TargetConditionDef, TargetPredicate, TargetSlotDef, TokenCharacteristics, TriggerConditionDef,
    TriggerEventDef, TurnKindDef, TurnStepDef, ValueDef, ZoneKind, ZoneMoveCauseDef,
};
use crate::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, Format, GrantId,
    MeldRecipeId, ModeId, PlayOptionId, TargetIndex, TargetSlotId,
};

fn definition(id: u64, name: &str, set: CardSet) -> CardDefinition {
    CardDefinition::new(
        CardDefinitionId::new(id),
        name,
        set,
        crate::card::CardRules::unsupported(),
    )
}

fn target(id: u8, minimum: u8, maximum: u8) -> TargetSlotDef {
    TargetSlotDef {
        id: TargetSlotId(id),
        label: "any target".into(),
        predicate: TargetPredicate::AnyTarget,
        minimum,
        maximum,
        divided_total: None,
    }
}

fn mode(id: u8, targets: Vec<TargetSlotDef>) -> ModeDef {
    ModeDef {
        id: ModeId(id),
        label: "test mode".into(),
        additional_mana_cost: None,
        targets,
        effect_status: CardEffectStatus::Unsupported,
    }
}

fn semantic_target(minimum: u8, maximum: u8) -> AbilityTargetDef {
    AbilityTargetDef {
        predicate: AbilityTargetPredicate::AnyTarget,
        minimum,
        maximum,
        exact_count: None,
        divided_total: None,
        another: false,
        excludes_source: false,
        chooser: TargetChooserDef::Controller,
    }
}

fn semantic_mode(targets: Vec<AbilityTargetDef>) -> AbilityDef {
    AbilityDef::spell_with_targets(
        "test mode",
        Box::leak(targets.into_boxed_slice()),
        EffectDef::None,
    )
}

fn semantic_modal_definition(
    semantic_modes: Vec<AbilityDef>,
    presentation_modes: Option<ModeSetDef>,
) -> CardDefinition {
    let semantic_modes = Box::leak(semantic_modes.into_boxed_slice());
    semantic_spell_definition(
        &AbilityDef::modal_spell("Choose one.", semantic_modes),
        presentation_modes,
    )
}

fn semantic_spell_definition(
    ability: &AbilityDef,
    mut presentation_modes: Option<ModeSetDef>,
) -> CardDefinition {
    let abilities = Box::leak(vec![*ability].into_boxed_slice());
    let rules = crate::CardRules::new_instant(ManaCost::default()).with_abilities(abilities);
    let mut card = definition(1, "Test Modal Spell", CardSet::Alpha);
    set_primary_rules(&mut card, &rules);
    card.play_options = vec![PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Test Modal Spell",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::default(),
        CardEffectStatus::Implemented,
    )];
    if let Some(modes) = &mut presentation_modes {
        for mode in &mut modes.modes {
            mode.effect_status = CardEffectStatus::Implemented;
        }
    }
    card.play_options[0].modes = presentation_modes;
    card
}

fn split_definition(fused: Option<PlayOptionId>) -> CardDefinition {
    let mut card = definition(1, "Left // Right", CardSet::Alpha);
    let spell_rules = crate::CardRules::new_instant(ManaCost::default());
    card.rules = spell_rules;
    card.parts[0].rules = spell_rules;
    card.parts
        .push(CardPart::new(CardPartId(1), "Right", spell_rules));
    card.structure = CardStructure::Split {
        parts: vec![CardPartId::PRIMARY, CardPartId(1)],
        fused,
    };
    card.play_options[0].label = "Left".into();
    card.play_options[0].mana_cost = Some(ManaCost::default());
    card.play_options.push(PlayOptionDef::cast(
        PlayOptionId(1),
        "Right",
        SpellForm::Part(CardPartId(1)),
        ManaCost::default(),
        CardEffectStatus::Implemented,
    ));
    card
}

fn error(card: CardDefinition) -> CatalogError {
    CardCatalog::new([card]).unwrap_err()
}

fn set_primary_rules(card: &mut CardDefinition, rules: &crate::CardRules) {
    card.rules = *rules;
    let primary = card.primary_part_id();
    card.parts
        .iter_mut()
        .find(|part| part.id == primary)
        .expect("the test definition has a primary part")
        .rules = *rules;
    let status = match rules.implementation_status() {
        crate::ImplementationStatus::Complete => CardEffectStatus::Implemented,
        crate::ImplementationStatus::Unsupported => CardEffectStatus::Unsupported,
    };
    for option in &mut card.play_options {
        option.effect_status = status;
    }
}

fn definition_granting(granted: &'static AbilityDef) -> CardDefinition {
    let abilities = Box::leak(
        vec![AbilityDef::static_ability(
            "This object grants an ability.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(granted),
            },
        )]
        .into_boxed_slice(),
    );
    let mut card = definition(1, "Test Card", CardSet::Alpha);
    let rules = card.rules.with_abilities(abilities);
    set_primary_rules(&mut card, &rules);
    card
}

mod abilities_grants;
mod composition;
mod identity_printings;
mod names;
mod presentation;
