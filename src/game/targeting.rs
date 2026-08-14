use super::{
    AlternativeCastKindDef, CardBehavior, CardDefinition, CardDefinitionId, CardSupertype,
    CardType, CardTypeSet, CharacteristicContext, Cow, DeclarativeAbilityDef, Game, GameObjectId,
    ManaCost, ModeId, PlayRestriction, PlayerId, StackObject, StackObjectKind, Step, Target,
    TargetPredicate, TargetSelection, TriggerEventObject, applicable_part_ids,
};

impl Game {
    pub(super) fn targets_matching(&self, predicate: TargetPredicate) -> Vec<Target> {
        match predicate {
            TargetPredicate::AnyTarget => self.damage_targets(),
            TargetPredicate::Player => {
                vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)]
            }
            TargetPredicate::Permanent => self
                .battlefield
                .iter()
                .map(|permanent| Target::Permanent(permanent.card.id))
                .collect(),
            TargetPredicate::CreaturePermanent => self
                .battlefield
                .iter()
                .filter(|permanent| self.power(permanent).is_some())
                .map(|permanent| Target::Permanent(permanent.card.id))
                .collect(),
            TargetPredicate::Spell => self
                .stack
                .iter()
                .filter(|object| object.kind == StackObjectKind::Spell)
                .map(|object| Target::Spell(object.id))
                .collect(),
            TargetPredicate::NoncreatureSpell => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self
                            .stack_spell_types(object)
                            .is_some_and(|types| !types.is_creature())
                })
                .map(|object| Target::Spell(object.id))
                .collect(),
        }
    }

    pub(super) fn stack_spell_types(&self, object: &StackObject) -> Option<CardTypeSet> {
        let definition = self.catalog.get(object.card.definition)?;
        let signature = object.signature.as_ref()?;
        let option = definition.play_option(signature.play_option())?;
        let types = Self::play_option_types(definition, option)?;
        if self.selected_alternative_kind(definition, option, object.id, signature.costs())
            == Some(AlternativeCastKindDef::Bestow)
        {
            Some(
                types
                    .without(CardType::Creature)
                    .with(CardType::Enchantment),
            )
        } else {
            Some(types)
        }
    }

    pub(super) fn stack_trigger_event_object(
        &self,
        object: &StackObject,
    ) -> Option<TriggerEventObject> {
        let signature = object.signature.as_ref()?;
        let mut event = self.printed_trigger_event_object(
            object.id,
            object.card.definition,
            object.controller,
            &CharacteristicContext::Stack {
                form: signature.form().clone(),
            },
        )?;
        let definition = self.catalog.get(object.card.definition)?;
        let option = definition.play_option(signature.play_option())?;
        if self.selected_alternative_kind(definition, option, object.id, signature.costs())
            == Some(AlternativeCastKindDef::Bestow)
        {
            event.types = event
                .types
                .without(CardType::Creature)
                .with(CardType::Enchantment);
            event.subtypes = Cow::Borrowed(&["Aura"]);
            event.power = None;
            event.toughness = None;
        }
        Some(event)
    }

    pub(super) fn printed_trigger_event_object(
        &self,
        id: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        context: &CharacteristicContext,
    ) -> Option<TriggerEventObject> {
        let definition = self.catalog.get(definition)?;
        let parts = applicable_part_ids(definition, context).ok()?;
        let mut types = CardTypeSet::empty();
        let mut colors = [false; 5];
        let mut subtypes = Vec::new();
        let mut mana_value = 0;
        let mut power = None;
        let mut toughness = None;
        let mut supertypes = [false; CardSupertype::COUNT];
        let mut keywords = 0;
        for part in parts {
            let part = definition.part(part)?;
            types = types.union(part.rules.types());
            for ability in part.rules.ability_clauses() {
                if ability.is_executable()
                    && let DeclarativeAbilityDef::Keyword(keyword) = ability.definition
                    && let Some(index) = keyword.simple_index()
                {
                    keywords |= 1 << index;
                }
            }
            for (combined, present) in colors.iter_mut().zip(part.rules.colors()) {
                *combined |= present;
            }
            for subtype in part.rules.subtypes() {
                if !subtypes.contains(subtype) {
                    subtypes.push(*subtype);
                }
            }
            mana_value += part.rules.mana_cost().map_or(0, ManaCost::mana_value);
            if let Some(stats) = part.rules.creature_stats() {
                power = Some(stats.power);
                toughness = Some(stats.toughness);
            }
            for supertype in CardSupertype::ALL {
                supertypes[supertype.index()] |= part.rules.has_supertype(supertype);
            }
        }
        Some(TriggerEventObject {
            id,
            token: self.is_token(definition.id),
            types,
            controller,
            colors,
            subtypes: Cow::Owned(subtypes),
            // A card or a spell is nowhere near combat.
            attacking_or_blocking: false,
            keywords,
            mana_value,
            power,
            toughness,
            supertypes,
            attacking: false,
            // A card outside the battlefield is not a tapped permanent.
            tapped: false,
            attacked_this_turn: false,
        })
    }

    /// Whether the step this spell would be cast in satisfies its own timing
    /// restriction. "Before the combat damage step" means combat damage has
    /// not started; once it has, the window is gone for the rest of the turn
    /// even in a later step.
    pub(super) fn play_timing_allows(&self, restriction: PlayRestriction) -> bool {
        match restriction {
            PlayRestriction::Normal | PlayRestriction::FromHandOnly => true,
            PlayRestriction::BeforeCombatDamage => !matches!(
                self.step,
                Step::CombatDamage
                    | Step::EndOfCombat
                    | Step::PostcombatMain
                    | Step::End
                    | Step::Cleanup
            ),
        }
    }

    /// Every legal target list, with hexproof and protection applied once at
    /// the end rather than in each of the several dozen per-card filters
    /// below. Doing it here is not just tidier: protection used to be spelled
    /// out arm by arm, and the arms that forgot -- Terror among them -- were
    /// simply wrong.
    pub(super) fn legal_target_lists(
        &self,
        behavior: CardBehavior,
        player: PlayerId,
        exact_count: Option<usize>,
    ) -> Vec<Vec<Target>> {
        self.printed_target_lists(behavior, player, exact_count)
            .into_iter()
            .filter(|choice| {
                choice.iter().all(|target| match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                        .is_none_or(|permanent| {
                            // Hexproof stops opponents only; you can always
                            // target your own. Protection stops everyone,
                            // including the permanent's own controller.
                            (permanent.controller == player || !self.has_hexproof(permanent))
                                && !self
                                    .is_protected_from_colors(permanent, behavior.rules().colors())
                        }),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => true,
                })
            })
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn printed_target_lists(
        &self,
        behavior: CardBehavior,
        player: PlayerId,
        exact_count: Option<usize>,
    ) -> Vec<Vec<Target>> {
        match behavior {
            CardBehavior::Duress => vec![vec![Target::Player(player.opponent())]],
            CardBehavior::ChainLightning
            | CardBehavior::PillarOfFlame
            | CardBehavior::GoblinGrenade => self
                .damage_targets()
                .into_iter()
                .map(|target| vec![target])
                .collect(),
            CardBehavior::Fireball => {
                let targets = self.damage_targets();
                // "Any number of targets" starts at none (CR 601.2c).
                let counts: Vec<_> =
                    exact_count.map_or_else(|| (0..=targets.len()).collect(), |count| vec![count]);
                counts
                    .into_iter()
                    .flat_map(|count| target_combinations(&targets, count))
                    .collect()
            }
            CardBehavior::DustToDust => {
                let artifacts: Vec<_> = self
                    .battlefield
                    .iter()
                    .filter(|permanent| self.is_artifact_permanent(permanent))
                    .map(|permanent| Target::Permanent(permanent.card.id))
                    .collect();
                target_combinations(&artifacts, 2)
            }
            CardBehavior::Fork => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self.stack_spell_types(object).is_some_and(|types| {
                            types.contains(CardType::Instant) || types.contains(CardType::Sorcery)
                        })
                })
                .map(|object| vec![Target::Spell(object.id)])
                .collect(),
            // Both read the spell's kind off its chosen play option, so a
            // split or modal card counts as whatever it was actually cast as.
            CardBehavior::Negate | CardBehavior::EssenceScatter => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self
                            .stack_spell_types(object)
                            .is_some_and(|types| match behavior {
                                CardBehavior::EssenceScatter => types.is_creature(),
                                _ => !types.is_creature(),
                            })
                })
                .map(|object| vec![Target::Spell(object.id)])
                .collect(),
            _ => vec![Vec::new()],
        }
    }
}

/// Every way to split `total` into exactly `parts` positive whole numbers,
/// in order. This is what "divided as you choose" enumerates.
pub(super) fn positive_compositions(total: u8, parts: usize) -> Vec<Vec<u16>> {
    if parts == 0 {
        return if total == 0 {
            vec![Vec::new()]
        } else {
            Vec::new()
        };
    }
    let mut result = Vec::new();
    for first in 1..=total.saturating_sub(u8::try_from(parts - 1).unwrap_or(u8::MAX)) {
        for mut rest in positive_compositions(total - first, parts - 1) {
            let mut composition = vec![u16::from(first)];
            composition.append(&mut rest);
            result.push(composition);
        }
    }
    result
}

pub(super) fn flatten_target_selections(selections: &[TargetSelection]) -> Vec<Target> {
    selections
        .iter()
        .flat_map(TargetSelection::targets)
        .copied()
        .collect()
}

pub(super) fn mode_id_selections(
    modes: &[ModeId],
    minimum: usize,
    maximum: usize,
    may_repeat: bool,
) -> Vec<Vec<ModeId>> {
    (minimum..=maximum)
        .flat_map(|count| {
            if may_repeat {
                repeated_mode_selections(modes, count)
            } else {
                mode_combinations(modes, count)
            }
        })
        .collect()
}

pub(super) fn mode_combinations(modes: &[ModeId], count: usize) -> Vec<Vec<ModeId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if modes.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, mode) in modes.iter().enumerate() {
        for mut tail in mode_combinations(&modes[index + 1..], count - 1) {
            let mut choice = vec![*mode];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

pub(super) fn repeated_mode_selections(modes: &[ModeId], count: usize) -> Vec<Vec<ModeId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    let mut result = Vec::new();
    for (index, mode) in modes.iter().enumerate() {
        for mut tail in repeated_mode_selections(&modes[index..], count - 1) {
            let mut choice = vec![*mode];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

/// "This spell costs {1} more to cast for each target beyond the first." The
/// first target is free, so a single-target cast pays nothing extra.
pub(super) fn extra_target_cost(definition: &CardDefinition, target_count: usize) -> u16 {
    let per_target = definition.rules.additional_generic_per_extra_target();
    if per_target == 0 {
        return 0;
    }
    u16::try_from(target_count.saturating_sub(1))
        .unwrap_or(u16::MAX)
        .saturating_mul(per_target)
}

pub(super) fn one_or_none(values: &[GameObjectId]) -> Vec<Vec<GameObjectId>> {
    std::iter::once(Vec::new())
        .chain(values.iter().map(|value| vec![*value]))
        .collect()
}

pub(super) fn combinations(values: &[GameObjectId], count: usize) -> Vec<Vec<GameObjectId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if values.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, value) in values.iter().enumerate() {
        for mut tail in combinations(&values[index + 1..], count - 1) {
            let mut choice = vec![*value];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

pub(super) fn target_combinations(values: &[Target], count: usize) -> Vec<Vec<Target>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if values.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, value) in values.iter().enumerate() {
        for mut tail in target_combinations(&values[index + 1..], count - 1) {
            let mut choice = vec![*value];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}
