//! Which entry replacements could possibly apply, before any of them do.
//!
//! Separated from applying them because it answers a different question: a
//! prospective permanent has to know whether anything at all might replace
//! its entry before it commits to asking, and that survey reads printed and
//! granted abilities rather than a pending event.

use super::super::{
    AbilityDef, AbilityOperationDef, AppliedEffectDef, CharacteristicOperationDef, ControlFlow,
    DeclarativeAbilityDef, EffectDef, Game, Permanent, ReplacementEventDef, ZoneKind,
};

impl Game {
    pub(super) fn is_source_entry_replacement(ability: &AbilityDef) -> bool {
        matches!(
            (ability.definition, ability.declarative_replacement()),
            (
                DeclarativeAbilityDef::Replacement(definition),
                Some(_),
            ) if definition.event == ReplacementEventDef::SourceEntersBattlefield
        )
    }

    pub(super) fn is_external_entry_replacement(ability: &AbilityDef) -> bool {
        matches!(
            ability.definition,
            DeclarativeAbilityDef::Replacement(definition)
                if definition.source_zones.contains(&ZoneKind::Battlefield)
                    && matches!(
                        definition.event,
                        ReplacementEventDef::ObjectEntersBattlefield { .. }
                    )
        ) && ability.declarative_replacement().is_some()
    }

    pub(super) fn applied_grant_entry_replacement_possibilities(
        effect: AppliedEffectDef,
    ) -> (bool, bool) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                effects
                    .iter()
                    .fold((false, false), |(source, external), effect| {
                        let found = Self::applied_grant_entry_replacement_possibilities(*effect);
                        (source || found.0, external || found.1)
                    })
            }
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )) => (
                Self::is_source_entry_replacement(ability),
                Self::is_external_entry_replacement(ability),
            ),
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => (false, false),
        }
    }

    /// Whether a static effect could grant a source-entry replacement or an
    /// external object-entry replacement, respectively.
    pub(super) fn granted_entry_replacement_possibilities(effect: EffectDef) -> (bool, bool) {
        match effect {
            EffectDef::Sequence(effects) => {
                effects
                    .iter()
                    .fold((false, false), |(source, external), effect| {
                        let found = Self::granted_entry_replacement_possibilities(*effect);
                        (source || found.0, external || found.1)
                    })
            }
            EffectDef::ConditionalStatic(conditional) => {
                Self::applied_grant_entry_replacement_possibilities(conditional.then.effect)
            }
            EffectDef::StaticApply { effect, .. } => {
                Self::applied_grant_entry_replacement_possibilities(effect)
            }
            _ => (false, false),
        }
    }

    pub(super) fn static_grant_entry_replacement_possibilities(
        ability: &AbilityDef,
    ) -> (bool, bool) {
        if matches!(ability.definition, DeclarativeAbilityDef::Static(_))
            && let Some(effect) = ability.declarative_effect()
        {
            Self::granted_entry_replacement_possibilities(effect)
        } else {
            (false, false)
        }
    }

    pub(super) fn prospective_permanent_may_supply_source_entry_replacement(
        &self,
        permanent: &Permanent,
    ) -> bool {
        let may_supply = |ability: &AbilityDef| {
            Self::is_source_entry_replacement(ability)
                || Self::static_grant_entry_replacement_possibilities(ability).0
        };
        self.effective_rules(permanent)
            .is_some_and(|rules| rules.ability_clauses().iter().any(&may_supply))
            || permanent.active_copy_values().into_iter().any(|copy| {
                copy.added_abilities
                    .iter()
                    .any(|added| may_supply(&added.definition))
            })
    }

    /// Returns whether an existing static source might grant the prospective
    /// permanent a source-entry replacement and whether the battlefield might
    /// supply an external object-entry replacement. Recipient mismatches may
    /// yield conservative false positives, never false negatives.
    pub(super) fn battlefield_entry_replacement_possibilities(&self) -> (bool, bool) {
        let mut source = false;
        let mut external = false;
        for permanent in &self.battlefield {
            let result = self.visit_effective_abilities(permanent, |effective| {
                let ability = &effective.ability;
                let granted = Self::static_grant_entry_replacement_possibilities(ability);
                source |= granted.0;
                external |= Self::is_external_entry_replacement(ability) || granted.1;
                if source && external {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            if source && external {
                debug_assert!(result.is_break());
                break;
            }
        }
        if !source {
            for player in [crate::PlayerId::One, crate::PlayerId::Two] {
                for card in &self.players[player.index()].graveyard {
                    self.for_each_printed_card_ability(
                        card,
                        &super::super::CharacteristicContext::Graveyard,
                        |effective| {
                            let ability = &effective.ability;
                            let DeclarativeAbilityDef::Static(definition) = ability.definition
                            else {
                                return;
                            };
                            if definition.source_zones.contains(&ZoneKind::Graveyard) {
                                source |=
                                    Self::static_grant_entry_replacement_possibilities(ability).0;
                            }
                        },
                    );
                    if source {
                        break;
                    }
                }
                if source {
                    break;
                }
            }
        }
        (source, external)
    }
}
