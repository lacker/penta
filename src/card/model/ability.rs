use std::borrow::Cow;

use crate::ids::TargetIndex;

use super::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityEffectDef, AbilityProcedureDef,
    AbilityTargetDef, ActivatedAbilityDef, ActivationTimingDef, AlternativeCastAbilityDef,
    AlternativeCastKindDef, AlternativeCastManaCostDef, CardBehavior, ConditionDef,
    DeclarativeAbilityDef, EffectDef, EffectExecutionDef, ImplementationStatus, KeywordAbility,
    ManaCost, ModalSpellDef, OptionalAdditionalCostAbilityDef, PregameAbilityDef,
    PregameConditionDef, PregameTimingDef, ReplacementAbilityDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, SpecialActionDef, SpellAbilityDef,
    SpellAdditionalCostDef, SpellLifeCostDef, SpellResolutionDestinationDef, StaticAbilityDef,
    TriggerConditionDef, TriggerEventDef, TriggeredAbilityDef, ValueDef, ZoneKind,
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

    /// A spell that spends objects as it is cast, in addition to its mana.
    #[must_use]
    pub const fn spell_with_additional_cost(
        text: &'static str,
        targets: &'static [AbilityTargetDef],
        cost: SpellAdditionalCostDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(
                SpellAbilityDef::new()
                    .with_targets(targets)
                    .with_additional_cost(cost),
            ),
            effect,
        )
    }

    /// A spell that pays life as it is cast, in addition to its mana.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not a nonmodal spell, since nothing
    /// else carries a spell's additional cost.
    #[must_use]
    pub const fn with_spell_life_cost(mut self, cost: SpellLifeCostDef) -> Self {
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            panic!("only a spell has an additional cost");
        };
        self.definition = DeclarativeAbilityDef::Spell(spell.with_life_cost(cost));
        self
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

    /// Declares where this spell's card goes after successful resolution.
    /// Flashback still replaces that move with exile.
    ///
    /// # Panics
    ///
    /// Panics when called on an ability that is not a spell.
    #[must_use]
    pub const fn with_resolution_destination(
        mut self,
        destination: SpellResolutionDestinationDef,
    ) -> Self {
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            panic!("a resolution destination belongs on a spell")
        };
        self.definition =
            DeclarativeAbilityDef::Spell(spell.with_resolution_destination(destination));
        self
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

    /// "This ability costs {N} less to activate for each ...", printed on
    /// the ability itself. The discount travels with the ability rather than
    /// being read off the battlefield, which is what a channel cost needs:
    /// the card is in hand, where nothing it says is a static ability.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not an activated one, since nothing
    /// else has an activation cost to discount.
    #[must_use]
    pub const fn with_activation_cost_reduction(mut self, amount: ValueDef, minimum: u16) -> Self {
        let DeclarativeAbilityDef::Activated(activated) = self.definition else {
            panic!("only an activated ability has an activation cost");
        };
        self.definition =
            DeclarativeAbilityDef::Activated(activated.with_cost_reduction(amount, minimum));
        self
    }

    /// A cost paid as the whole spell is cast, on top of its mana. Escalate
    /// is the only one a modal spell prints.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not a spell.
    #[must_use]
    pub const fn with_spell_additional_cost(
        mut self,
        cost: &'static SpellAdditionalCostDef,
    ) -> Self {
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            panic!("only a spell has an additional cost to pay");
        };
        self.definition = DeclarativeAbilityDef::Spell(spell.with_additional_cost(*cost));
        self
    }

    /// "If <condition> as you cast this spell, you may choose two instead."
    /// The larger maximum applies where the spell is offered, which is what
    /// "as you cast" means; the minimum is unchanged, because choosing the
    /// extra mode is always optional.
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not a modal spell, since nothing else
    /// has a mode count to raise.
    #[must_use]
    pub const fn with_conditional_mode_maximum(
        mut self,
        condition: ConditionDef,
        maximum: u8,
    ) -> Self {
        let DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) = self.definition else {
            panic!("only a modal spell has a mode count");
        };
        self.definition = DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(
            modal.with_conditional_maximum(condition, maximum),
        ));
        self
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

    /// "{T}: Add {B}. Activate only if you control a Swamp or a Forest."
    /// The condition is read where the activation is offered, so a land
    /// whose condition is false simply does not produce that colour.
    #[must_use]
    pub const fn activated_mana_if(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        condition: &'static TriggerConditionDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::ActivatedMana(
                ActivatedAbilityDef::new(costs).only_if(condition),
            ),
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

    /// "Choose one --" on an activated ability, which chooses its modes as
    /// it is activated (CR 601.2b) rather than as it resolves. The ability
    /// does nothing of its own beyond the modes it prints.
    #[must_use]
    pub const fn modal_activated(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Activated(
                ActivatedAbilityDef::with_costs(AbilityCostList::borrowed(costs))
                    .with_modes(ModalSpellDef::new(modes, minimum, maximum, may_repeat)),
            ),
            EffectDef::None,
        )
    }

    /// "Choose one --" on a triggered ability, which chooses its mode as it
    /// is put onto the stack (CR 603.3c) rather than as it resolves. The
    /// ability does nothing of its own beyond the mode it prints.
    #[must_use]
    pub const fn modal_triggered(
        text: &'static str,
        event: TriggerEventDef,
        modes: &'static [AbilityDef],
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(event).with_modes(ModalSpellDef::choose_one(modes)),
            ),
            EffectDef::None,
        )
    }

    /// "Choose up to one --" on a triggered ability. The only difference
    /// from `modal_triggered` is that declining every mode is a legal
    /// answer, so the trigger goes onto the stack carrying nothing.
    #[must_use]
    pub const fn modal_triggered_up_to_one(
        text: &'static str,
        event: TriggerEventDef,
        modes: &'static [AbilityDef],
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(event).with_modes(ModalSpellDef::new(modes, 0, 1, false)),
            ),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn triggered(text: &'static str, event: TriggerEventDef, effect: EffectDef) -> Self {
        Self::triggered_with_targets(text, event, &[], effect)
    }

    /// "This ability triggers only once each turn."
    ///
    /// # Panics
    ///
    /// Panics for any ability that is not a triggered one, since nothing
    /// else has a triggering to cap.
    #[must_use]
    pub const fn triggering_at_most(self, times: u8) -> Self {
        let DeclarativeAbilityDef::Triggered(definition) = self.definition else {
            panic!("only a triggered ability caps how often it triggers");
        };
        Self {
            definition: DeclarativeAbilityDef::Triggered(definition.triggering_at_most(times)),
            ..self
        }
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
    pub const fn replacement(text: &'static str, effect: ReplacementEffectDef) -> Self {
        Self::replacement_for(text, ReplacementEventDef::SourceEntersBattlefield, effect)
    }

    /// Defines a replacement ability with a prospective-event program rather
    /// than a resolving stack effect.
    #[must_use]
    pub const fn replacement_for(
        text: &'static str,
        event: ReplacementEventDef,
        effect: ReplacementEffectDef,
    ) -> Self {
        Self::defined_replacement(text, ReplacementAbilityDef::new().with_event(event), effect)
    }

    #[must_use]
    pub const fn defined_replacement(
        text: &'static str,
        definition: ReplacementAbilityDef,
        effect: ReplacementEffectDef,
    ) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Replacement(definition),
            effect: AbilityEffectDef::replacement_program(effect),
            coverage: AbilityCoverageDef::complete(),
        }
    }

    /// Defines a replacement ability that modifies how its own source enters
    /// the battlefield.
    #[must_use]
    pub const fn as_enters(text: &'static str, effect: ReplacementEffectDef) -> Self {
        Self::replacement(text, effect)
    }

    /// The same, gated on a condition read as the permanent enters.
    #[must_use]
    pub const fn as_enters_if(
        text: &'static str,
        condition: ReplacementConditionDef,
        effect: ReplacementEffectDef,
    ) -> Self {
        Self::defined_replacement(
            text,
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::SourceEntersBattlefield)
                .with_condition(condition),
            effect,
        )
    }

    /// A cost a player may add to any legal way of casting this spell. The
    /// selected identity travels in `CostConfiguration::additional`, so the
    /// outcome can be frozen on the stack without conflating it with an
    /// alternative way of casting the card.
    #[must_use]
    pub const fn optional_additional_cost(
        text: &'static str,
        definition: OptionalAdditionalCostAbilityDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::OptionalAdditionalCost(definition),
            EffectDef::None,
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

    /// A rules-defined action taken before the first turn, without using the
    /// stack. Its timing is structural rather than inferred from Oracle text.
    #[must_use]
    pub const fn pregame(
        text: &'static str,
        timing: PregameTimingDef,
        condition: PregameConditionDef,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Pregame(
                PregameAbilityDef::new(timing)
                    .with_condition(condition)
                    .with_costs(costs),
            ),
            effect,
        )
    }

    #[must_use]
    pub const fn opening_hand(text: &'static str, effect: EffectDef) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::OpeningHand,
            PregameConditionDef::Always,
            &[],
            effect,
        )
    }

    #[must_use]
    pub const fn opening_hand_reveal(text: &'static str, effect: EffectDef) -> Self {
        let mut ability = Self::opening_hand(text, effect);
        let DeclarativeAbilityDef::Pregame(definition) = ability.definition else {
            unreachable!()
        };
        ability.definition = DeclarativeAbilityDef::Pregame(definition.revealing_source());
        ability
    }

    #[must_use]
    pub const fn opening_hand_with(
        text: &'static str,
        condition: PregameConditionDef,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::OpeningHand,
            condition,
            costs,
            effect,
        )
    }

    #[must_use]
    pub const fn mulligan_action(text: &'static str, effect: EffectDef) -> Self {
        Self::pregame(
            text,
            PregameTimingDef::Mulligan,
            PregameConditionDef::Always,
            &[],
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
        self.definition = match self.definition {
            DeclarativeAbilityDef::Activated(definition) => {
                DeclarativeAbilityDef::Activated(definition.with_timing(timing))
            }
            // A mana ability is enumerated elsewhere but is an activated
            // ability all the same, and "activate only during your turn" is
            // printed on some of them.
            DeclarativeAbilityDef::ActivatedMana(definition) => {
                DeclarativeAbilityDef::ActivatedMana(definition.with_timing(timing))
            }
            _ => panic!("only an activated ability has an activation window"),
        };
        self
    }

    /// A printed "Activate only as an instant" on a mana ability: it may be
    /// activated whenever its controller holds priority and never as part of
    /// paying for something.
    ///
    /// # Panics
    ///
    /// Panics for anything but a mana ability, which is the only place the
    /// clause is printed and the only place the distinction is visible.
    #[must_use]
    pub const fn only_as_instant(mut self) -> Self {
        let DeclarativeAbilityDef::ActivatedMana(definition) = self.definition else {
            panic!("only a mana ability is restricted to instant speed");
        };
        self.definition = DeclarativeAbilityDef::ActivatedMana(definition.only_as_instant());
        self
    }

    /// A printed "activate only if ..." restriction on an activated ability.
    ///
    /// # Panics
    ///
    /// Panics for any other ability category, which has no activation to gate.
    #[must_use]
    pub const fn with_activation_condition(
        mut self,
        condition: &'static TriggerConditionDef,
    ) -> Self {
        let DeclarativeAbilityDef::Activated(definition) = self.definition else {
            panic!("only an activated ability has an activation restriction");
        };
        self.definition = DeclarativeAbilityDef::Activated(definition.only_if(condition));
        self
    }

    /// Opens an activated ability to every player, for a printed "any player
    /// may activate this ability" clause. The permanent stays the source.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability.
    #[must_use]
    pub const fn open_to_any_player(mut self) -> Self {
        let DeclarativeAbilityDef::Activated(definition) = self.definition else {
            panic!("only an activated ability can be opened to other players");
        };
        self.definition = DeclarativeAbilityDef::Activated(definition.open_to_any_player());
        self
    }

    /// Caps an activated ability at one activation each turn, for a printed
    /// "only once each turn" clause.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability.
    #[must_use]
    pub const fn once_each_turn(self) -> Self {
        self.activations_each_turn(1)
    }

    /// Exhaust (CR 702.184a): this ability may be activated once from this
    /// object and never again, however many turns it survives.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability.
    #[must_use]
    pub const fn exhausting(mut self) -> Self {
        self.definition = match self.definition {
            DeclarativeAbilityDef::Activated(definition) => {
                DeclarativeAbilityDef::Activated(definition.exhausting())
            }
            DeclarativeAbilityDef::ActivatedMana(definition) => {
                DeclarativeAbilityDef::ActivatedMana(definition.exhausting())
            }
            _ => panic!("only an activated ability can be exhausted"),
        };
        self
    }

    /// The general form: "no more than twice each turn" and its relatives.
    ///
    /// # Panics
    ///
    /// Panics if the clause is not an activated ability.
    #[must_use]
    pub const fn activations_each_turn(mut self, limit: u8) -> Self {
        self.definition = match self.definition {
            DeclarativeAbilityDef::Activated(definition) => {
                DeclarativeAbilityDef::Activated(definition.with_activation_limit(limit))
            }
            DeclarativeAbilityDef::ActivatedMana(definition) => {
                DeclarativeAbilityDef::ActivatedMana(definition.with_activation_limit(limit))
            }
            _ => panic!("only an activated ability can be capped per turn"),
        };
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
            | DeclarativeAbilityDef::OptionalAdditionalCost(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Pregame(_)
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

    /// The printed "choose one --" of this clause, wherever it prints it.
    /// A spell carries its modes in its casting shape; an activated ability
    /// carries its own, chosen as it is activated (CR 601.2b). Everything
    /// downstream -- validation, grant numbering, mode selection -- treats
    /// the two identically, so it asks here rather than matching the kind.
    /// Which chapter of a Saga this ability is, if it is one. Read off the
    /// shape `abilities::saga_chapter` builds rather than stored beside it:
    /// the number the chapter waits for is the number it is.
    #[must_use]
    pub const fn saga_chapter(self) -> Option<u8> {
        let DeclarativeAbilityDef::Triggered(triggered) = self.definition else {
            return None;
        };
        let TriggerEventDef::While { event, condition } = triggered.event else {
            return None;
        };
        let TriggerEventDef::CountersPlaced { kind, .. } = *event else {
            return None;
        };
        let TriggerConditionDef::SourceCounters {
            kind: counted,
            amount,
            ..
        } = *condition
        else {
            return None;
        };
        if kind.is_lore() && counted.is_lore() {
            Some(amount)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn modal(self) -> Option<ModalSpellDef> {
        match self.definition {
            DeclarativeAbilityDef::Spell(spell) => spell.modal(),
            DeclarativeAbilityDef::Activated(activated) => activated.modes,
            _ => None,
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

    #[must_use]
    pub const fn declarative_replacement(self) -> Option<ReplacementEffectDef> {
        if self.is_executable() {
            self.effect.declarative_replacement()
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
            DeclarativeAbilityDef::OptionalAdditionalCost(definition)
                if definition.mana_cost.is_some() && self.text == definition.kind.label() =>
            {
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
            | DeclarativeAbilityDef::OptionalAdditionalCost(_)
            | DeclarativeAbilityDef::Pregame(_)
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
        if self.effect.declarative_definition() == Some(EffectDef::None) {
            modes
        } else {
            own.combine(modes)
        }
    }
}
include!("ability/alternative_casts.rs");
include!("ability/target_resolution.rs");
