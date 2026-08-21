// Long because the effect vocabulary is wide, not because the function
// does several things: every arm is one variant walked the same way.
fn collect_program_ability_grants(program: AbilityProgramDef, grants: &mut Vec<&AbilityDef>) {
    match program {
        AbilityProgramDef::Effects(effect) => collect_ability_grants(effect, grants),
        AbilityProgramDef::Replacement(effect) => {
            collect_replacement_ability_grants(effect, grants);
        }
    }
}

#[allow(clippy::too_many_lines)]
fn collect_ability_grants(effect: EffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                collect_ability_grants(*effect, grants);
            }
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            collect_ability_grants(*on_success, grants);
            collect_ability_grants(*on_failure, grants);
        }
        EffectDef::Choose(choice) => collect_ability_grants(*choice.then, grants),
        EffectDef::RevealAtRandomFromHand { then, .. }
        | EffectDef::ChooseCardName { then, .. }
        | EffectDef::SearchZone {
            then: Some(then), ..
        }
        | EffectDef::BindMatching { then, .. } => {
            collect_ability_grants(*then, grants);
        }
        EffectDef::PayOr(payment) => {
            for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                collect_ability_grants(**effect, grants);
            }
        }
        EffectDef::SplitIntoPiles(partition) => {
            collect_ability_grants(*partition.then, grants);
        }
        EffectDef::CreateToken {
            created: Some(created),
            ..
        } => collect_ability_grants(*created.then, grants),
        EffectDef::May { effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::ExileTopAndMayCast {
            otherwise: Some(effect),
            ..
        }
        | EffectDef::Mill {
            then: Some(effect), ..
        }
        | EffectDef::MillUntil {
            then: Some(effect), ..
        }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. } => {
            collect_ability_grants(*effect, grants);
        }
        EffectDef::InstallTrigger(trigger) => {
            collect_program_ability_grants(trigger.ability.effect.definition, grants);
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            collect_ability_grants(*then, grants);
            collect_ability_grants(*otherwise, grants);
        }
        EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => collect_ability_grants(*effect, grants),
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            if let Some(effect) = selection.then {
                collect_ability_grants(*effect, grants);
            }
        }
        EffectDef::StaticApply { effect, .. }
        | EffectDef::Apply { effect, .. }
        | EffectDef::DealDamageAndApply {
            applied: effect, ..
        } => {
            collect_applied_ability_grants(effect, grants);
        }
        EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::AddPoisonCounters { .. }
            | EffectDef::AddEnergyCounters { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::WinTheGame { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::DestroyAtEndOfCombat { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::DoubleCounters { .. }
            | EffectDef::RemoveAllCounters { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventDamage { .. }
        | EffectDef::Attach { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::ReturnAttached { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Unattach { .. }
        | EffectDef::PairWithSource { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::CreateToken { created: None, .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeKeepingOnePerType { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::ExileTopOfLibraryToPlay { .. }
        | EffectDef::Mill { then: None, .. }
        | EffectDef::ExileTopAndMayCast { otherwise: None, .. }
        | EffectDef::MayCastTargetWithoutPaying { .. }
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::MillUntil { then: None, .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::ManifestDread { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::Explore { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::RevealHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::Counter { .. }
        | EffectDef::ReturnSpellToHand { .. }
        | EffectDef::PutSpellIntoOwnersLibrary { .. }
        | EffectDef::CopyResolvingSpell { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::RemoveCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::ChooseColor { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::ReturnWithHasteAndFinality { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::PutSourceOntoBattlefieldAttacking
            | EffectDef::BecomeMonarch { .. }
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::ExchangeControl { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::IncreaseMatchingAbilityCostBy { .. }
            | EffectDef::ReduceMatchingAbilityCostBy { .. }
        | EffectDef::IncreaseMatchingSpellCostBy { .. }
        | EffectDef::ReduceMatchingSpellCostBy { .. }
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::PutIntoLibraryBeneathTop { .. }
            | EffectDef::MoveToZone { .. }
        | EffectDef::Special(_) => {}
    }
}

fn collect_replacement_ability_grants(effect: ReplacementEffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                collect_replacement_ability_grants(*effect, grants);
            }
        }
        ReplacementEffectDef::Perform(effect) => collect_ability_grants(*effect, grants),
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                collect_replacement_ability_grants(*effect, grants);
            }
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                collect_replacement_ability_grants(*effect, grants);
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => {}
    }
}

fn collect_applied_ability_grants(effect: AppliedEffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_ability_grants(*effect, grants);
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => grants.push(ability),
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}

fn program_ability_grant_sites(program: AbilityProgramDef) -> usize {
    match program {
        AbilityProgramDef::Effects(effect) => ability_grant_sites(effect),
        AbilityProgramDef::Replacement(effect) => replacement_ability_grant_sites(effect),
    }
}

// One arm per effect that can carry a grant; the list is long because the
// vocabulary is, not because the function does much.
#[allow(clippy::too_many_lines)]
fn ability_grant_sites(effect: EffectDef) -> usize {
    match effect {
        EffectDef::Sequence(effects) => effects
            .iter()
            .map(|effect| ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => ability_grant_sites(*on_success).saturating_add(ability_grant_sites(*on_failure)),
        EffectDef::Choose(choice) => ability_grant_sites(*choice.then),
        EffectDef::RevealAtRandomFromHand { then, .. }
        | EffectDef::ChooseCardName { then, .. }
        | EffectDef::SearchZone {
            then: Some(then), ..
        }
        | EffectDef::BindMatching { then, .. } => ability_grant_sites(*then),
        EffectDef::PayOr(payment) => payment
            .if_paid
            .iter()
            .chain(payment.otherwise.iter())
            .map(|effect| ability_grant_sites(**effect))
            .fold(0, usize::saturating_add),
        EffectDef::SplitIntoPiles(partition) => ability_grant_sites(*partition.then),
        EffectDef::CreateToken {
            created: Some(created),
            ..
        } => ability_grant_sites(*created.then),
        EffectDef::May { effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::ExileTopAndMayCast {
            otherwise: Some(effect),
            ..
        }
        | EffectDef::Mill {
            then: Some(effect), ..
        }
        | EffectDef::MillUntil {
            then: Some(effect), ..
        }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
        | EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => ability_grant_sites(*effect),
        EffectDef::InstallTrigger(trigger) => {
            program_ability_grant_sites(trigger.ability.effect.definition)
        }
        EffectDef::LookAtTopAndSelect { selection, .. } => selection
            .then
            .map_or(0, |effect| ability_grant_sites(*effect)),
        EffectDef::IfFormat {
            then, otherwise, ..
        } => ability_grant_sites(*then).max(ability_grant_sites(*otherwise)),
        EffectDef::StaticApply { effect, .. }
        | EffectDef::Apply { effect, .. }
        | EffectDef::DealDamageAndApply {
            applied: effect, ..
        } => applied_ability_grant_sites(effect),
        EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::AddPoisonCounters { .. }
            | EffectDef::AddEnergyCounters { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::WinTheGame { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::DestroyAtEndOfCombat { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::DoubleCounters { .. }
            | EffectDef::RemoveAllCounters { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventDamage { .. }
        | EffectDef::Attach { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::ReturnAttached { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Unattach { .. }
        | EffectDef::PairWithSource { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::CreateToken { created: None, .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeKeepingOnePerType { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::ExileTopOfLibraryToPlay { .. }
        | EffectDef::Mill { then: None, .. }
        | EffectDef::ExileTopAndMayCast { otherwise: None, .. }
        | EffectDef::MayCastTargetWithoutPaying { .. }
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::MillUntil { then: None, .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::ManifestDread { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::Explore { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::RevealHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::Counter { .. }
        | EffectDef::ReturnSpellToHand { .. }
        | EffectDef::PutSpellIntoOwnersLibrary { .. }
        | EffectDef::CopyResolvingSpell { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::RemoveCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::ChooseColor { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::GainClassLevel { .. }
            | EffectDef::CannotBeForcedToDiscard
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::ReturnWithHasteAndFinality { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::PutSourceOntoBattlefieldAttacking
            | EffectDef::BecomeMonarch { .. }
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::ExchangeControl { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::IncreaseMatchingAbilityCostBy { .. }
            | EffectDef::ReduceMatchingAbilityCostBy { .. }
        | EffectDef::IncreaseMatchingSpellCostBy { .. }
        | EffectDef::ReduceMatchingSpellCostBy { .. }
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::PutIntoLibraryBeneathTop { .. }
            | EffectDef::MoveToZone { .. }
        | EffectDef::Special(_) => 0,
    }
}

fn replacement_ability_grant_sites(effect: ReplacementEffectDef) -> usize {
    match effect {
        ReplacementEffectDef::Sequence(effects) => effects
            .iter()
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::Perform(effect) => ability_grant_sites(*effect),
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => if_true
            .iter()
            .chain(if_false.iter())
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => if_paid
            .iter()
            .chain(if_declined.iter())
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => 0,
    }
}

fn applied_ability_grant_sites(effect: AppliedEffectDef) -> usize {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .map(|effect| applied_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(_),
        )) => 1,
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => 0,
    }
}
