// Mana-activation characteristics used by the payment planner.
//
// Included textually into `mana_planning.rs`, so the imports here are the
// parent module's.

impl Game {
    /// Whether an ability turns its own source into a creature.
    pub(super) fn effect_animates_source(effect: Option<EffectDef>) -> bool {
        match effect {
            Some(EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect,
                ..
            }) => Self::applied_effect_adds_creature_type(effect),
            Some(EffectDef::Sequence(effects)) => effects
                .iter()
                .any(|effect| Self::effect_animates_source(Some(*effect))),
            Some(EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            }) => {
                Self::effect_animates_source(Some(*on_success))
                    || Self::effect_animates_source(Some(*on_failure))
            }
            Some(EffectDef::Choose(choice)) => Self::effect_animates_source(Some(*choice.then)),
            Some(EffectDef::PayOr(payment)) => payment
                .if_paid
                .iter()
                .chain(payment.otherwise.iter())
                .any(|effect| Self::effect_animates_source(Some(**effect))),
            _ => false,
        }
    }

    fn applied_effect_adds_creature_type(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_adds_creature_type),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                SetOperationDef::Add(types) | SetOperationDef::Set(types),
            )) => types.contains(CardType::Creature),
            _ => false,
        }
    }

    pub(super) fn activated_ability_mana_cost(
        definition: &ActivatedAbilityDef,
    ) -> Option<ManaCost> {
        let mut cost = ManaCost::default();
        let mut has_mana_cost = false;
        for ability_cost in definition.costs.as_slice() {
            if let AbilityCostDef::Mana(mana) = ability_cost {
                cost = add_mana_cost(cost, *mana);
                has_mana_cost = true;
            }
        }
        has_mana_cost.then_some(cost)
    }

    /// The complete mana portion of an activation after its object cost has
    /// been chosen. Ordinary printed mana is static; a linked card cost is
    /// deliberately priced only now, so each graveyard choice can expose a
    /// different legal activation.
    pub(super) fn activated_ability_mana_cost_for(
        &self,
        definition: &ActivatedAbilityDef,
        targets: &[TargetSelection],
        cost_objects: &[GameObjectId],
    ) -> Option<ManaCost> {
        let mut cost = ManaCost::default();
        let mut has_mana_cost = false;
        for ability_cost in definition.costs.as_slice() {
            match ability_cost {
                AbilityCostDef::Mana(mana) => {
                    cost = add_mana_cost(cost, *mana);
                    has_mana_cost = true;
                }
                AbilityCostDef::ManaCostOf(ObjectRefDef::Binding(binding)) => {
                    let _movement = definition.costs.iter().find_map(|cost| {
                        let AbilityCostDef::MoveToZone(movement) = cost else {
                            return None;
                        };
                        (movement.binding == Some(*binding)).then_some(movement)
                    })?;
                    let chosen = *cost_objects.first()?;
                    let (_, card) = self.card_in_nonbattlefield_zone(chosen)?;
                    let mana = self.catalog.get(card.definition)?.rules.mana_cost()?;
                    cost = add_mana_cost(cost, mana);
                    has_mana_cost = true;
                }
                AbilityCostDef::ManaCostOf(_) => return None,
                AbilityCostDef::ManaValueOfTarget { target, multiplier } => {
                    let slot = TargetSlotId::from_index(target.index())?;
                    let selected = targets
                        .iter()
                        .find(|selection| selection.slot() == slot)?
                        .targets()
                        .first()?;
                    let object = match selected {
                        Target::Card(object)
                        | Target::Permanent(object)
                        | Target::Spell(object) => *object,
                        Target::Player(_) => return None,
                    };
                    let amount = self
                        .current_or_last_known_mana_value(object)?
                        .saturating_mul(u16::from(*multiplier));
                    cost = add_mana_cost(cost, ManaCost::new(amount, 0));
                    has_mana_cost = true;
                }
                _ => {}
            }
        }
        has_mana_cost.then_some(cost)
    }
}
