use super::model::{
    AbilityLocator, AppliedEffectLocator, KeywordSnapshot, ManaPayloadLocator,
    ReplacementEffectLocator, ScopedEffectSnapshot,
};
use super::model_animation::AnimationSnapshot;
use super::{Mana, ScopedEffect};
use crate::CardCatalog;
use crate::card::{
    AbilityDef, AddManaEffectDef, AnimationDef, AppliedEffectDef, DeclarativeAbilityDef, EffectDef,
    KeywordAbility, ManaColor, ManaSpendEffectDef, ReplacementEffectDef, SpellAbilityDef,
};

pub(super) fn ability_locator(
    catalog: &CardCatalog,
    mut matches: impl FnMut(&AbilityDef) -> bool,
) -> Option<AbilityLocator> {
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let mut nested = Vec::new();
                if locate_ability(&attached.definition, &mut matches, &mut nested) {
                    return Some(AbilityLocator {
                        definition: definition.id.0,
                        part_id: part.id.0,
                        ability_id: attached.id.0,
                        nested,
                    });
                }
            }
        }
    }
    None
}

pub(super) fn catalog_ability(
    catalog: &CardCatalog,
    locator: &AbilityLocator,
) -> Option<AbilityDef> {
    let mut current = *catalog
        .get(crate::CardDefinitionId(locator.definition))?
        .part(crate::CardPartId(locator.part_id))?
        .rules
        .ability(crate::AbilityId(locator.ability_id))?;
    for &index in &locator.nested {
        current = **child_abilities(&current).get(index)?;
    }
    Some(current)
}

pub(super) fn mana_payload_locator(
    catalog: &CardCatalog,
    mana: Mana,
) -> Option<ManaPayloadLocator> {
    if mana.restrictions.is_empty() && mana.spend_effects.is_empty() {
        return None;
    }
    let ability = ability_locator(catalog, |candidate| {
        mana_effects(candidate)
            .iter()
            .any(|effect| mana_effect_matches(*effect, mana))
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = mana_effects(&definition)
        .iter()
        .position(|effect| mana_effect_matches(*effect, mana))?;
    Some(ManaPayloadLocator {
        ability,
        effect_index,
    })
}

pub(super) fn catalog_mana_payload(
    catalog: &CardCatalog,
    locator: &ManaPayloadLocator,
) -> Option<AddManaEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    mana_effects(&ability).get(locator.effect_index).copied()
}

pub(super) fn applied_effect_locator(
    catalog: &CardCatalog,
    expected: AppliedEffectDef,
) -> Option<AppliedEffectLocator> {
    let ability = ability_locator(catalog, |candidate| {
        applied_effects(candidate).contains(&expected)
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = applied_effects(&definition)
        .iter()
        .position(|effect| *effect == expected)?;
    Some(AppliedEffectLocator {
        ability,
        effect_index,
    })
}

pub(super) fn catalog_applied_effect(
    catalog: &CardCatalog,
    locator: &AppliedEffectLocator,
) -> Option<AppliedEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    applied_effects(&ability).get(locator.effect_index).copied()
}

pub(super) fn applied_effects(ability: &AbilityDef) -> Vec<AppliedEffectDef> {
    let mut found = Vec::new();
    collect_applied_effects_from_effect(ability.effect.definition, &mut found);
    for mana in mana_effects(ability) {
        for spend in mana.spend_effects {
            if let ManaSpendEffectDef::ApplyToPaidSpell(effect) = *spend {
                collect_applied_effect(effect, &mut found);
            }
        }
    }
    found
}

fn collect_applied_effects_from_effect(effect: EffectDef, found: &mut Vec<AppliedEffectDef>) {
    if let EffectDef::Apply { effect, .. } = effect {
        collect_applied_effect(effect, found);
    }
    for child in child_effects(effect) {
        collect_applied_effects_from_effect(child, found);
    }
}

fn collect_applied_effect(effect: AppliedEffectDef, found: &mut Vec<AppliedEffectDef>) {
    found.push(effect);
    if let AppliedEffectDef::Composite(children) = effect {
        for child in children {
            collect_applied_effect(*child, found);
        }
    }
}

pub(super) fn scoped_effect_snapshot(
    ability: &AbilityDef,
    effect: ScopedEffect,
) -> Option<ScopedEffectSnapshot> {
    let mut path = Vec::new();
    locate_effect(ability.effect.definition, effect.effect, &mut path).then_some(
        ScopedEffectSnapshot {
            path,
            target_base: effect.target_base,
        },
    )
}

pub(super) fn catalog_scoped_effect(
    catalog: &CardCatalog,
    ability: &AbilityLocator,
    snapshot: &ScopedEffectSnapshot,
) -> Option<ScopedEffect> {
    let ability = catalog_ability(catalog, ability)?;
    let mut effect = ability.effect.definition;
    for &index in &snapshot.path {
        effect = *child_effects(effect).get(index)?;
    }
    Some(ScopedEffect {
        effect,
        target_base: snapshot.target_base,
    })
}

pub(super) fn replacement_effect_locator(
    catalog: &CardCatalog,
    expected: ReplacementEffectDef,
) -> Option<ReplacementEffectLocator> {
    let ability = ability_locator(catalog, |candidate| {
        replacement_effects(candidate)
            .into_iter()
            .any(|effect| effect == expected)
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = replacement_effects(&definition)
        .into_iter()
        .position(|effect| effect == expected)?;
    Some(ReplacementEffectLocator {
        ability,
        effect_index,
    })
}

pub(super) fn catalog_replacement_effect(
    catalog: &CardCatalog,
    locator: &ReplacementEffectLocator,
) -> Option<ReplacementEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    replacement_effects(&ability)
        .get(locator.effect_index)
        .copied()
}

pub(super) fn replacement_effects(ability: &AbilityDef) -> Vec<ReplacementEffectDef> {
    let mut effects = Vec::new();
    collect_replacement_effects_from_effect(ability.effect.definition, &mut effects);
    effects
}

fn collect_replacement_effects_from_effect(
    effect: EffectDef,
    found: &mut Vec<ReplacementEffectDef>,
) {
    if let EffectDef::Replacement(replacement) = effect {
        collect_replacement_effects(replacement, found);
    }
    for child in child_effects(effect) {
        collect_replacement_effects_from_effect(child, found);
    }
}

fn collect_replacement_effects(
    effect: ReplacementEffectDef,
    found: &mut Vec<ReplacementEffectDef>,
) {
    found.push(effect);
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::OptionalPayment {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::None | ReplacementEffectDef::ModifyBattlefieldEntry(_) => {}
    }
}

fn locate_effect(current: EffectDef, needle: EffectDef, path: &mut Vec<usize>) -> bool {
    if current == needle {
        return true;
    }
    for (index, child) in child_effects(current).into_iter().enumerate() {
        path.push(index);
        if locate_effect(child, needle, path) {
            return true;
        }
        path.pop();
    }
    false
}

pub(super) fn child_effects(effect: EffectDef) -> Vec<EffectDef> {
    match effect {
        EffectDef::Sequence(effects) => effects.to_vec(),
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => vec![*on_success, *on_failure],
        EffectDef::OptionalPayment { if_paid, .. } => vec![*if_paid],
        EffectDef::UnlessPaid { otherwise, .. }
        | EffectDef::May {
            effect: otherwise, ..
        }
        | EffectDef::ReplaceNextDrawThisTurn {
            effect: otherwise, ..
        }
        | EffectDef::IfCondition {
            then: otherwise, ..
        }
        | EffectDef::AtNextStep {
            effect: otherwise, ..
        }
        | EffectDef::ChoosePermanent {
            then: otherwise, ..
        } => vec![*otherwise],
        EffectDef::IfFormat {
            then, otherwise, ..
        } => vec![*then, *otherwise],
        EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => vec![*effect],
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            selection.then.into_iter().copied().collect()
        }
        _ => Vec::new(),
    }
}

fn mana_effect_matches(effect: AddManaEffectDef, mana: Mana) -> bool {
    effect.restrictions == mana.restrictions
        && effect.spend_effects == mana.spend_effects
        && match effect.mana {
            crate::card::ManaSelectionDef::One(color) => color == mana.color,
            crate::card::ManaSelectionDef::Choice(colors) => colors.contains(&mana.color),
        }
}

pub(super) fn mana_effects(ability: &AbilityDef) -> Vec<AddManaEffectDef> {
    let mut effects = Vec::new();
    collect_mana_effects(ability.effect.definition, &mut effects);
    effects
}

fn collect_mana_effects(effect: EffectDef, found: &mut Vec<AddManaEffectDef>) {
    match effect {
        EffectDef::AddMana(mana) => found.push(mana),
        EffectDef::Sequence(effects) => {
            for effect in effects {
                collect_mana_effects(*effect, found);
            }
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            collect_mana_effects(*on_success, found);
            collect_mana_effects(*on_failure, found);
        }
        EffectDef::OptionalPayment {
            if_paid: effect, ..
        }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May { effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. }
        | EffectDef::ChoosePermanent { then: effect, .. }
        | EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => collect_mana_effects(*effect, found),
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            collect_mana_effects(*then, found);
            collect_mana_effects(*otherwise, found);
        }
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            if let Some(effect) = selection.then {
                collect_mana_effects(*effect, found);
            }
        }
        _ => {}
    }
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

pub(super) fn child_abilities(ability: &AbilityDef) -> Vec<&AbilityDef> {
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
        | EffectDef::May { effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
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
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            collect_effect_abilities(*then, abilities);
            collect_effect_abilities(*otherwise, abilities);
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
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
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

pub(super) fn animation_snapshot(animation: &AnimationDef) -> AnimationSnapshot {
    AnimationSnapshot {
        power: animation.power,
        toughness: animation.toughness,
        types: animation.types.type_name().clone(),
        subtypes: animation
            .subtypes
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        all_creature_types: animation.all_creature_types,
        replaces_subtypes: animation.replaces_subtypes,
        loses_abilities: animation.loses_abilities,
        colors: animation.colors.map(crate::card::ColorSet::to_flags),
    }
}

pub(super) const fn keyword_snapshot(keyword: KeywordAbility) -> KeywordSnapshot {
    match keyword {
        KeywordAbility::Flying => KeywordSnapshot::Flying,
        KeywordAbility::Trample => KeywordSnapshot::Trample,
        KeywordAbility::Haste => KeywordSnapshot::Haste,
        KeywordAbility::FirstStrike => KeywordSnapshot::FirstStrike,
        KeywordAbility::DoubleStrike => KeywordSnapshot::DoubleStrike,
        KeywordAbility::Banding => KeywordSnapshot::Banding,
        KeywordAbility::Vigilance => KeywordSnapshot::Vigilance,
        KeywordAbility::Defender => KeywordSnapshot::Defender,
        KeywordAbility::Deathtouch => KeywordSnapshot::Deathtouch,
        KeywordAbility::Lifelink => KeywordSnapshot::Lifelink,
        KeywordAbility::Reach => KeywordSnapshot::Reach,
        KeywordAbility::Flash => KeywordSnapshot::Flash,
        KeywordAbility::Hexproof => KeywordSnapshot::Hexproof,
        KeywordAbility::Shroud => KeywordSnapshot::Shroud,
        KeywordAbility::Intimidate => KeywordSnapshot::Intimidate,
        KeywordAbility::Undying => KeywordSnapshot::Undying,
        KeywordAbility::Indestructible => KeywordSnapshot::Indestructible,
        KeywordAbility::AttacksEachCombatIfAble => KeywordSnapshot::AttacksEachCombatIfAble,
        KeywordAbility::Mountainwalk => KeywordSnapshot::Mountainwalk,
        KeywordAbility::Forestwalk => KeywordSnapshot::Forestwalk,
        KeywordAbility::ProtectionFrom(ManaColor::White) => KeywordSnapshot::ProtectionFromWhite,
        KeywordAbility::ProtectionFrom(ManaColor::Blue) => KeywordSnapshot::ProtectionFromBlue,
        KeywordAbility::ProtectionFrom(ManaColor::Black) => KeywordSnapshot::ProtectionFromBlack,
        KeywordAbility::ProtectionFrom(ManaColor::Red) => KeywordSnapshot::ProtectionFromRed,
        KeywordAbility::ProtectionFrom(ManaColor::Green) => KeywordSnapshot::ProtectionFromGreen,
        KeywordAbility::ProtectionFrom(ManaColor::Colorless) => {
            KeywordSnapshot::ProtectionFromColorless
        }
    }
}

pub(super) const fn parse_keyword(value: KeywordSnapshot) -> KeywordAbility {
    match value {
        KeywordSnapshot::Flying => KeywordAbility::Flying,
        KeywordSnapshot::Trample => KeywordAbility::Trample,
        KeywordSnapshot::Haste => KeywordAbility::Haste,
        KeywordSnapshot::FirstStrike => KeywordAbility::FirstStrike,
        KeywordSnapshot::DoubleStrike => KeywordAbility::DoubleStrike,
        KeywordSnapshot::Banding => KeywordAbility::Banding,
        KeywordSnapshot::Vigilance => KeywordAbility::Vigilance,
        KeywordSnapshot::Defender => KeywordAbility::Defender,
        KeywordSnapshot::Deathtouch => KeywordAbility::Deathtouch,
        KeywordSnapshot::Lifelink => KeywordAbility::Lifelink,
        KeywordSnapshot::Reach => KeywordAbility::Reach,
        KeywordSnapshot::Flash => KeywordAbility::Flash,
        KeywordSnapshot::Hexproof => KeywordAbility::Hexproof,
        KeywordSnapshot::Shroud => KeywordAbility::Shroud,
        KeywordSnapshot::Intimidate => KeywordAbility::Intimidate,
        KeywordSnapshot::Undying => KeywordAbility::Undying,
        KeywordSnapshot::Indestructible => KeywordAbility::Indestructible,
        KeywordSnapshot::AttacksEachCombatIfAble => KeywordAbility::AttacksEachCombatIfAble,
        KeywordSnapshot::Mountainwalk => KeywordAbility::Mountainwalk,
        KeywordSnapshot::Forestwalk => KeywordAbility::Forestwalk,
        KeywordSnapshot::ProtectionFromWhite => KeywordAbility::ProtectionFrom(ManaColor::White),
        KeywordSnapshot::ProtectionFromBlue => KeywordAbility::ProtectionFrom(ManaColor::Blue),
        KeywordSnapshot::ProtectionFromBlack => KeywordAbility::ProtectionFrom(ManaColor::Black),
        KeywordSnapshot::ProtectionFromRed => KeywordAbility::ProtectionFrom(ManaColor::Red),
        KeywordSnapshot::ProtectionFromGreen => KeywordAbility::ProtectionFrom(ManaColor::Green),
        KeywordSnapshot::ProtectionFromColorless => {
            KeywordAbility::ProtectionFrom(ManaColor::Colorless)
        }
    }
}

pub(super) fn catalog_animation(
    catalog: &CardCatalog,
    key: &AnimationSnapshot,
) -> Option<&'static AnimationDef> {
    catalog
        .definitions()
        .into_iter()
        .flat_map(|definition| &definition.parts)
        .flat_map(|part| part.rules.indexed_abilities())
        .find_map(|attached| animation_in_ability(&attached.definition, key))
}

fn animation_in_ability(
    ability: &AbilityDef,
    key: &AnimationSnapshot,
) -> Option<&'static AnimationDef> {
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

fn animation_in_effect(
    effect: EffectDef,
    key: &AnimationSnapshot,
) -> Option<&'static AnimationDef> {
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
        | EffectDef::May {
            effect: otherwise, ..
        }
        | EffectDef::ReplaceNextDrawThisTurn {
            effect: otherwise, ..
        }
        | EffectDef::IfCondition {
            then: otherwise, ..
        }
        | EffectDef::AtNextStep {
            effect: otherwise, ..
        } => animation_in_effect(*otherwise, key),
        EffectDef::IfFormat {
            then, otherwise, ..
        } => animation_in_effect(*then, key).or_else(|| animation_in_effect(*otherwise, key)),
        EffectDef::TriggerUntilYourNextTurn { ability } => animation_in_ability(ability, key),
        EffectDef::Apply { effect, .. } => animation_in_applied(effect, key),
        _ => None,
    }
}

fn animation_in_applied(
    effect: AppliedEffectDef,
    key: &AnimationSnapshot,
) -> Option<&'static AnimationDef> {
    match effect {
        AppliedEffectDef::Animate(animation) if animation_snapshot(animation) == *key => {
            Some(animation)
        }
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .find_map(|effect| animation_in_applied(*effect, key)),
        AppliedEffectDef::GrantAbility(ability) => animation_in_ability(ability, key),
        _ => None,
    }
}
