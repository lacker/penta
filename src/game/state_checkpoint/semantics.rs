use serde_json::{Value, json};

use crate::CardCatalog;
use crate::card::{
    AbilityDef, AnimationDef, AppliedEffectDef, DeclarativeAbilityDef, EffectDef, KeywordAbility,
    ManaColor, SpellAbilityDef,
};

pub(super) fn ability_locator_json(
    catalog: &CardCatalog,
    mut matches: impl FnMut(&AbilityDef) -> bool,
) -> Option<Value> {
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let mut nested = Vec::new();
                if locate_ability(&attached.definition, &mut matches, &mut nested) {
                    return Some(json!({
                        "definition": definition.id.0,
                        "partId": part.id.0,
                        "abilityId": attached.id.0,
                        "nested": nested,
                    }));
                }
            }
        }
    }
    None
}

pub(super) fn catalog_ability(catalog: &CardCatalog, locator: &Value) -> Option<AbilityDef> {
    let definition = locator
        .get("definition")?
        .as_u64()
        .and_then(|value| u16::try_from(value).ok())?;
    let part = locator
        .get("partId")?
        .as_u64()
        .and_then(|value| u8::try_from(value).ok())?;
    let ability = locator
        .get("abilityId")?
        .as_u64()
        .and_then(|value| u8::try_from(value).ok())?;
    let mut current = *catalog
        .get(crate::CardDefinitionId(definition))?
        .part(crate::CardPartId(part))?
        .rules
        .ability(crate::AbilityId(ability))?;
    for index in locator.get("nested")?.as_array()? {
        let index = index
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())?;
        current = **child_abilities(&current).get(index)?;
    }
    Some(current)
}

fn locate_ability(
    ability: &AbilityDef,
    matches: &mut impl FnMut(&AbilityDef) -> bool,
    path: &mut Vec<usize>,
) -> bool {
    if matches(ability) {
        return true;
    }
    for (index, child) in child_abilities(ability).into_iter().enumerate() {
        path.push(index);
        if locate_ability(child, matches, path) {
            return true;
        }
        path.pop();
    }
    false
}

fn child_abilities(ability: &AbilityDef) -> Vec<&AbilityDef> {
    let mut children = Vec::new();
    if let DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) = ability.definition {
        children.extend(modal.modes);
    }
    collect_effect_abilities(ability.effect.definition, &mut children);
    children
}

fn collect_effect_abilities(effect: EffectDef, abilities: &mut Vec<&'static AbilityDef>) {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                collect_effect_abilities(*effect, abilities);
            }
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            collect_effect_abilities(*on_success, abilities);
            collect_effect_abilities(*on_failure, abilities);
        }
        EffectDef::OptionalPayment {
            if_paid: effect, ..
        }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May(effect)
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. }
        | EffectDef::ChoosePermanent { then: effect, .. }
        | EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => collect_effect_abilities(*effect, abilities),
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            if let Some(effect) = selection.then {
                collect_effect_abilities(*effect, abilities);
            }
        }
        EffectDef::Apply { effect, .. } => collect_applied_abilities(effect, abilities),
        EffectDef::TriggerUntilYourNextTurn { ability } => abilities.push(ability),
        EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
        | EffectDef::Attach { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchLibrary { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => {}
    }
}

fn collect_applied_abilities(effect: AppliedEffectDef, abilities: &mut Vec<&'static AbilityDef>) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_abilities(*effect, abilities);
            }
        }
        AppliedEffectDef::GrantAbility(ability) => abilities.push(ability),
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::Special(_) => {}
    }
}

pub(super) fn animation_json(animation: &AnimationDef) -> Value {
    json!({
        "power": animation.power,
        "toughness": animation.toughness,
        "types": animation.types.type_name(),
        "subtypes": animation.subtypes,
        "allCreatureTypes": animation.all_creature_types,
        "replacesSubtypes": animation.replaces_subtypes,
        "losesAbilities": animation.loses_abilities,
        "colors": animation.colors.map(crate::card::ColorSet::to_flags),
    })
}

pub(super) fn keyword_json(keyword: KeywordAbility) -> Value {
    Value::from(match keyword {
        KeywordAbility::Flying => "flying",
        KeywordAbility::Trample => "trample",
        KeywordAbility::Haste => "haste",
        KeywordAbility::FirstStrike => "firstStrike",
        KeywordAbility::DoubleStrike => "doubleStrike",
        KeywordAbility::Banding => "banding",
        KeywordAbility::Vigilance => "vigilance",
        KeywordAbility::Defender => "defender",
        KeywordAbility::Deathtouch => "deathtouch",
        KeywordAbility::Lifelink => "lifelink",
        KeywordAbility::Reach => "reach",
        KeywordAbility::Flash => "flash",
        KeywordAbility::Hexproof => "hexproof",
        KeywordAbility::Shroud => "shroud",
        KeywordAbility::Intimidate => "intimidate",
        KeywordAbility::Undying => "undying",
        KeywordAbility::Indestructible => "indestructible",
        KeywordAbility::AttacksEachCombatIfAble => "attacksEachCombatIfAble",
        KeywordAbility::Mountainwalk => "mountainwalk",
        KeywordAbility::Forestwalk => "forestwalk",
        KeywordAbility::ProtectionFrom(ManaColor::White) => "protectionFromWhite",
        KeywordAbility::ProtectionFrom(ManaColor::Blue) => "protectionFromBlue",
        KeywordAbility::ProtectionFrom(ManaColor::Black) => "protectionFromBlack",
        KeywordAbility::ProtectionFrom(ManaColor::Red) => "protectionFromRed",
        KeywordAbility::ProtectionFrom(ManaColor::Green) => "protectionFromGreen",
        KeywordAbility::ProtectionFrom(ManaColor::Colorless) => "protectionFromColorless",
    })
}

pub(super) fn parse_keyword(value: &Value) -> Result<KeywordAbility, String> {
    match value.as_str() {
        Some("flying") => Ok(KeywordAbility::Flying),
        Some("trample") => Ok(KeywordAbility::Trample),
        Some("haste") => Ok(KeywordAbility::Haste),
        Some("firstStrike") => Ok(KeywordAbility::FirstStrike),
        Some("doubleStrike") => Ok(KeywordAbility::DoubleStrike),
        Some("banding") => Ok(KeywordAbility::Banding),
        Some("vigilance") => Ok(KeywordAbility::Vigilance),
        Some("defender") => Ok(KeywordAbility::Defender),
        Some("deathtouch") => Ok(KeywordAbility::Deathtouch),
        Some("lifelink") => Ok(KeywordAbility::Lifelink),
        Some("reach") => Ok(KeywordAbility::Reach),
        Some("flash") => Ok(KeywordAbility::Flash),
        Some("hexproof") => Ok(KeywordAbility::Hexproof),
        Some("shroud") => Ok(KeywordAbility::Shroud),
        Some("intimidate") => Ok(KeywordAbility::Intimidate),
        Some("undying") => Ok(KeywordAbility::Undying),
        Some("indestructible") => Ok(KeywordAbility::Indestructible),
        Some("attacksEachCombatIfAble") => Ok(KeywordAbility::AttacksEachCombatIfAble),
        Some("mountainwalk") => Ok(KeywordAbility::Mountainwalk),
        Some("forestwalk") => Ok(KeywordAbility::Forestwalk),
        Some("protectionFromWhite") => Ok(KeywordAbility::ProtectionFrom(ManaColor::White)),
        Some("protectionFromBlue") => Ok(KeywordAbility::ProtectionFrom(ManaColor::Blue)),
        Some("protectionFromBlack") => Ok(KeywordAbility::ProtectionFrom(ManaColor::Black)),
        Some("protectionFromRed") => Ok(KeywordAbility::ProtectionFrom(ManaColor::Red)),
        Some("protectionFromGreen") => Ok(KeywordAbility::ProtectionFrom(ManaColor::Green)),
        Some("protectionFromColorless") => Ok(KeywordAbility::ProtectionFrom(ManaColor::Colorless)),
        Some(other) => Err(format!("unknown keyword {other}")),
        None => Err("keyword must be a string".into()),
    }
}

pub(super) fn catalog_animation(
    catalog: &CardCatalog,
    key: &Value,
) -> Option<&'static AnimationDef> {
    catalog
        .definitions()
        .into_iter()
        .flat_map(|definition| &definition.parts)
        .flat_map(|part| part.rules.indexed_abilities())
        .find_map(|attached| animation_in_ability(&attached.definition, key))
}

fn animation_in_ability(ability: &AbilityDef, key: &Value) -> Option<&'static AnimationDef> {
    if let DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) = ability.definition
        && let Some(animation) = modal
            .modes
            .iter()
            .find_map(|mode| animation_in_ability(mode, key))
    {
        return Some(animation);
    }
    animation_in_effect(ability.effect.definition, key)
}

fn animation_in_effect(effect: EffectDef, key: &Value) -> Option<&'static AnimationDef> {
    match effect {
        EffectDef::Sequence(effects) => effects
            .iter()
            .find_map(|effect| animation_in_effect(*effect, key)),
        EffectDef::SacrificeOfChoice { then, .. } => {
            then.and_then(|effect| animation_in_effect(*effect, key))
        }
        EffectDef::LookAtTopAndSelect { selection, .. } => selection
            .then
            .and_then(|effect| animation_in_effect(*effect, key)),
        EffectDef::OptionalPayment { if_paid, .. } => animation_in_effect(*if_paid, key),
        EffectDef::UnlessPaid { otherwise, .. }
        | EffectDef::May(otherwise)
        | EffectDef::IfCondition {
            then: otherwise, ..
        }
        | EffectDef::AtNextStep {
            effect: otherwise, ..
        } => animation_in_effect(*otherwise, key),
        EffectDef::TriggerUntilYourNextTurn { ability } => animation_in_ability(ability, key),
        EffectDef::Apply { effect, .. } => animation_in_applied(effect, key),
        _ => None,
    }
}

fn animation_in_applied(effect: AppliedEffectDef, key: &Value) -> Option<&'static AnimationDef> {
    match effect {
        AppliedEffectDef::Animate(animation) if animation_json(animation) == *key => {
            Some(animation)
        }
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .find_map(|effect| animation_in_applied(*effect, key)),
        AppliedEffectDef::GrantAbility(ability) => animation_in_ability(ability, key),
        _ => None,
    }
}
