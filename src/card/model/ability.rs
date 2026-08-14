use std::borrow::Cow;

use crate::ids::TargetIndex;

use super::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityEffectDef, AbilityProcedureDef,
    AbilityTargetDef, ActivatedAbilityDef, ActivationTimingDef, AlternativeCastAbilityDef,
    AlternativeCastKindDef, AlternativeCastManaCostDef, CardBehavior, DeclarativeAbilityDef,
    EffectDef, EffectExecutionDef, ImplementationStatus, KeywordAbility, ManaCost,
    ReplacementAbilityDef, ReplacementEffectDef, ReplacementEventDef, SpecialActionDef,
    SpellAbilityDef, StaticAbilityDef, TriggerConditionDef, TriggerEventDef, TriggeredAbilityDef,
    ZoneKind,
};

/// One printed rules clause and its implementation.
///
/// The category is explicit even when the implementation remains custom; the
/// engine never infers stack behavior from costs, targets, or effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityDef {
    /// Static text for ordinary clauses and the keyword label for clauses
    /// whose full Oracle-style text is rendered from structured metadata.
    /// Use [`Self::rules_text`] when presenting a clause.
    pub text: &'static str,
    pub definition: DeclarativeAbilityDef,
    pub effect: AbilityEffectDef,
    pub coverage: AbilityCoverageDef,
}

impl AbilityDef {
    #[must_use]
    pub const fn spell(text: &'static str, effect: EffectDef) -> Self {
        Self::spell_with_targets(text, &[], effect)
    }

    #[must_use]
    pub const fn spell_with_targets(
        text: &'static str,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::new().with_targets(targets)),
            effect,
        )
    }

    #[must_use]
    pub const fn aura_spell(
        text: &'static str,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::aura(targets)),
            effect,
        )
    }

    /// A one-target counterspell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn counter_target(text: &'static str, target: &'static AbilityTargetDef) -> Self {
        Self::spell_with_targets(
            text,
            core::slice::from_ref(target),
            EffectDef::counter_target(TargetIndex::PRIMARY),
        )
    }

    /// A one-target destroy spell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn destroy_target(
        text: &'static str,
        target: &'static AbilityTargetDef,
        can_regenerate: bool,
    ) -> Self {
        Self::spell_with_targets(
            text,
            core::slice::from_ref(target),
            EffectDef::destroy_target(TargetIndex::PRIMARY, can_regenerate),
        )
    }

    #[must_use]
    pub const fn unimplemented_spell(text: &'static str, explanation: &'static str) -> Self {
        Self::spell(text, EffectDef::None)
            .with_coverage(AbilityCoverageDef::metadata_only(explanation))
    }

    #[must_use]
    pub const fn modal_spell(
        text: &'static str,
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::modal_spell(
                modes, minimum, maximum, may_repeat,
            )),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn choose_one_spell(text: &'static str, modes: &'static [AbilityDef]) -> Self {
        Self::modal_spell(text, modes, 1, 1, false)
    }

    #[must_use]
    pub const fn activated_mana(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::ActivatedMana(ActivatedAbilityDef::new(costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered_mana(
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::TriggeredMana(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn activated(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::activated_with_targets(text, costs, &[], effect)
    }

    #[must_use]
    pub const fn activated_with_targets(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::activated_with_cost_list_and_targets(
            text,
            AbilityCostList::borrowed(costs),
            targets,
            effect,
        )
    }

    #[must_use]
    pub(crate) const fn activated_with_cost_list_and_targets(
        text: &'static str,
        costs: AbilityCostList,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Activated(
                ActivatedAbilityDef::with_costs(costs).with_targets(targets),
            ),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered(text: &'static str, event: TriggerEventDef, effect: EffectDef) -> Self {
        Self::triggered_with_targets(text, event, &[], effect)
    }

    #[must_use]
    pub const fn triggered_with_targets(
        text: &'static str,
        event: TriggerEventDef,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(TriggeredAbilityDef::new(event).with_targets(targets)),
            effect,
        )
    }

    /// A trigger with an intervening-if condition, for "at the beginning of
    /// your upkeep, if ...".
    #[must_use]
    pub const fn triggered_if(
        text: &'static str,
        event: TriggerEventDef,
        condition: &'static TriggerConditionDef,
        effect: EffectDef,
    ) -> Self {
        Self::triggered_if_with_targets(text, event, condition, &[], effect)
    }

    #[must_use]
    pub const fn triggered_if_with_targets(
        text: &'static str,
        event: TriggerEventDef,
        condition: &'static TriggerConditionDef,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(event)
                    .with_condition(condition)
                    .with_targets(targets),
            ),
            effect,
        )
    }

    #[must_use]
    pub const fn static_ability(text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Static(StaticAbilityDef::new()),
            effect,
        )
    }

    /// A printed clause that shapes how the card is cast rather than what it
    /// does on resolution: a timing restriction, a limit on which mana may
    /// pay for it, an extra cost per target. There is nothing to execute when
    /// the spell resolves, because the play option has already applied it.
    #[must_use]
    pub const fn enforced_when_cast(text: &'static str, explanation: &'static str) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Static(StaticAbilityDef::new()),
            effect: AbilityEffectDef::declarative(EffectDef::None),
            coverage: AbilityCoverageDef::explained_complete(explanation),
        }
    }

    #[must_use]
    pub const fn keyword(text: &'static str, ability: KeywordAbility) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Keyword(ability),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn replacement(text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Replacement(ReplacementAbilityDef::new()),
            effect,
        )
    }

    /// Defines a replacement ability that modifies how its own source enters
    /// the battlefield.
    #[must_use]
    pub const fn as_enters(text: &'static str, effect: ReplacementEffectDef) -> Self {
        Self::replacement_for(
            text,
            ReplacementEventDef::SourceEntersBattlefield,
            EffectDef::Replacement(effect),
        )
    }

    #[must_use]
    pub const fn replacement_for(
        text: &'static str,
        event: ReplacementEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Replacement(ReplacementAbilityDef::new().with_event(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn alternative_cast(
        mana_cost: ManaCost,
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::Fixed(mana_cost),
                kind,
                targets: &[],
                stack_text,
            }),
            effect,
        )
    }

    /// Builds an alternative-casting ability whose cost is the mana cost of
    /// the card carrying the ability. This is resolved only after a concrete
    /// spell form has been selected.
    #[must_use]
    pub const fn alternative_cast_for_card_mana_cost(
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::ThisCardManaCost,
                kind,
                targets: &[],
                stack_text,
            }),
            effect,
        )
    }

    /// Builds an alternative casting procedure whose spell characteristics
    /// supply their own target declaration, as bestow does.
    #[must_use]
    pub const fn alternative_cast_with_targets(
        mana_cost: ManaCost,
        kind: AlternativeCastKindDef,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::Fixed(mana_cost),
                kind,
                targets,
                stack_text: None,
            }),
            effect,
        )
    }

    #[must_use]
    pub const fn special_action(
        text: &'static str,
        source_zones: &'static [ZoneKind],
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::SpecialAction(SpecialActionDef::new(source_zones, costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn defined(
        text: &'static str,
        definition: DeclarativeAbilityDef,
        effect: EffectDef,
    ) -> Self {
        Self {
            text,
            definition,
            effect: AbilityEffectDef::declarative(effect),
            coverage: AbilityCoverageDef::complete(),
        }
    }

    #[must_use]
    pub const fn custom_full(
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None)
                .with_execution(EffectExecutionDef::Custom(behavior)),
            coverage: AbilityCoverageDef::explained_complete(explanation),
        }
    }

    #[must_use]
    pub const fn custom_partial(
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None)
                .with_execution(EffectExecutionDef::Custom(behavior)),
            coverage: AbilityCoverageDef::partial(explanation),
        }
    }

    #[must_use]
    pub const fn not_implemented(text: &'static str, explanation: &'static str) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None),
            coverage: AbilityCoverageDef::metadata_only(explanation),
        }
    }

    #[must_use]
    pub const fn with_effect_execution(mut self, execution: EffectExecutionDef) -> Self {
        self.effect.execution = execution;
        self
    }

    #[must_use]
    pub const fn with_coverage(mut self, coverage: AbilityCoverageDef) -> Self {
        self.coverage = coverage;
        self
    }

    /// Narrows when an activated ability may be activated, for a printed
    /// "Activate only during ..." clause.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability, since nothing else
    /// carries an activation window.
    #[must_use]
    pub const fn with_activation_timing(mut self, timing: ActivationTimingDef) -> Self {
        let DeclarativeAbilityDef::Activated(definition) = self.definition else {
            panic!("only an activated ability has an activation window");
        };
        self.definition = DeclarativeAbilityDef::Activated(definition.with_timing(timing));
        self
    }

    /// Caps an activated ability at one activation each turn, for a printed
    /// "only once each turn" clause.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability.
    #[must_use]
    pub const fn once_each_turn(mut self) -> Self {
        let DeclarativeAbilityDef::Activated(definition) = self.definition else {
            panic!("only an activated ability can be capped per turn");
        };
        self.definition = DeclarativeAbilityDef::Activated(definition.with_once_each_turn());
        self
    }

    #[must_use]
    /// Selects the legacy procedure for an activated or triggered ability.
    ///
    /// # Panics
    ///
    /// Panics when called on an ability category that has no selectable procedure.
    pub const fn with_legacy_procedure(mut self) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                definition.procedure = AbilityProcedureDef::Legacy;
            }
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => {
                definition.procedure = AbilityProcedureDef::Legacy;
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::AlternativeCast(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::Legacy => {
                panic!("only activated and triggered abilities have a selectable procedure")
            }
        }
        self
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        self.coverage.is_executable()
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        if self.is_executable() {
            self.effect.custom_behavior()
        } else {
            None
        }
    }

    #[must_use]
    pub const fn declarative_effect(self) -> Option<EffectDef> {
        if self.is_executable() {
            self.effect.declarative_definition()
        } else {
            None
        }
    }

    /// Renders the complete printed clause. Most abilities borrow their
    /// canonical static text; structured alternative-casting keywords insert
    /// their owned mana cost into canonical reminder text.
    #[must_use]
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.definition {
            DeclarativeAbilityDef::AlternativeCast(definition) => {
                Cow::Owned(definition.rules_text())
            }
            _ => Cow::Borrowed(self.text),
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Static(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Replacement(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::SpecialAction(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::AlternativeCast(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::Legacy => {}
        }
        self
    }

    #[must_use]
    pub const fn uses_stack(self) -> bool {
        matches!(
            self.definition,
            DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
        )
    }

    fn own_implementation_status(self) -> ImplementationStatus {
        self.coverage.status
    }

    pub(super) fn implementation_status(self) -> ImplementationStatus {
        let own = self.own_implementation_status();
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            return own;
        };
        let Some(modal) = spell.modal() else {
            return own;
        };
        if !self.is_executable() {
            return own;
        }
        let mut statuses = modal
            .modes
            .iter()
            .copied()
            .map(AbilityDef::own_implementation_status);
        let modes = statuses.next().map_or(own, |first| {
            statuses.fold(first, ImplementationStatus::combine)
        });
        if self.effect.execution == EffectExecutionDef::Declarative
            && self.effect.definition == EffectDef::None
        {
            modes
        } else {
            own.combine(modes)
        }
    }
}
