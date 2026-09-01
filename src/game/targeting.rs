use super::{
    AppliedEffectDef, CardBehavior, CardDefinition, CardDefinitionId, CardSupertype, CardTypeSet,
    CharacteristicContext, CharacteristicOperationDef, Cow, DeclarativeAbilityDef, EffectDef,
    EffectRecipientDef, Game, GameObjectId, ManaCost, ModeId, ObjectCharacteristics,
    PlayRestriction, PlayerId, PowerToughnessOperationDef, RetiredObject, SetOperationDef,
    StackObject, StackObjectKind, Step, Target, TargetPredicate, TargetSelection,
    TriggerEventObject, ValueDef, ZoneKind, applicable_part_ids_ref,
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
        let definition = self
            .catalog
            .get(object.card.definition.card_definition()?)?;
        let signature = object.signature.as_ref()?;
        let option = definition.play_option(signature.play_option())?;
        Self::play_option_types(definition, option)
    }

    pub(super) fn stack_trigger_event_object(
        &self,
        object: &StackObject,
    ) -> Option<TriggerEventObject> {
        let signature = object.signature.as_ref()?;
        self.printed_trigger_event_object(
            object.id,
            object.card.definition.card_definition()?,
            object.controller,
            &CharacteristicContext::Stack {
                form: signature.form().clone(),
            },
        )
    }

    /// The same view, widened to the abilities waiting on the stack. An
    /// ability has no cast signature and no characteristics of its own, so it
    /// borrows its source's and keeps its own id and controller: what a
    /// predicate asks about one is which stack object it is and what it
    /// targets, not what it costs.
    pub(super) fn stack_object_event_object(
        &self,
        object: &StackObject,
    ) -> Option<TriggerEventObject> {
        self.stack_trigger_event_object(object).or_else(|| {
            let token = object.source.is_some_and(|source| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .map(|permanent| permanent.card.definition.is_token())
                    .or_else(|| match self.retired_objects.get(&source) {
                        Some(RetiredObject::Permanent { permanent, .. }) => {
                            Some(permanent.card.definition.is_token())
                        }
                        Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                    })
                    .unwrap_or(false)
            });
            let mut view = self.presentation_trigger_event_object(
                object.id,
                object.presentation(),
                object.controller,
                token,
            )?;
            view.controller = object.controller;
            Some(view)
        })
    }

    fn presentation_trigger_event_object(
        &self,
        id: GameObjectId,
        presentation: ObjectCharacteristics,
        controller: PlayerId,
        token: bool,
    ) -> Option<TriggerEventObject> {
        let rules = match presentation {
            ObjectCharacteristics::Card { definition, part } => {
                self.catalog.get(definition)?.part(part)?.rules
            }
            ObjectCharacteristics::Token { token, part } => token.part(part)?.rules,
            ObjectCharacteristics::Emblem { emblem } => emblem.rules_view(),
            ObjectCharacteristics::FaceDown { face_down } => face_down.rules(),
        };
        let mut keywords = 0_u64;
        for ability in rules.ability_clauses() {
            if ability.is_executable()
                && let DeclarativeAbilityDef::Keyword(keyword) = ability.definition
                && let Some(index) = keyword.simple_index()
            {
                keywords |= 1 << index;
            }
        }
        let stats = rules.creature_stats();
        let mut supertypes = [false; CardSupertype::COUNT];
        for supertype in CardSupertype::ALL {
            supertypes[supertype.index()] = rules.has_supertype(supertype);
        }
        Some(TriggerEventObject {
            id,
            token,
            types: rules.types(),
            controller,
            colors: rules.colors(),
            subtypes: Cow::Owned(rules.subtypes().to_vec()),
            attacking_or_blocking: false,
            keywords,
            mana_value: rules.mana_cost().map_or(0, ManaCost::mana_value),
            power: stats.map(|stats| stats.power),
            toughness: stats.map(|stats| stats.toughness),
            supertypes,
            attacking: false,
            tapped: false,
            attacked_during_controllers_last_turn: false,
            attacked_this_turn: false,
            saddled: false,
        })
    }

    /// The applied effects a card's own static clauses hand it while it is
    /// in `zone`. Only clauses that name the card itself, and only from the
    /// zones the clause says it works in -- which is what makes "as long as
    /// this isn't on the battlefield" a source-zone list rather than a
    /// condition.
    fn self_characteristics_in_zone(
        definition: &CardDefinition,
        zone: ZoneKind,
    ) -> Vec<AppliedEffectDef> {
        let mut applied = Vec::new();
        for ability in definition.rules.ability_clauses() {
            let (true, DeclarativeAbilityDef::Static(static_definition)) =
                (ability.is_executable(), ability.definition)
            else {
                continue;
            };
            if !static_definition.source_zones.contains(&zone) {
                continue;
            }
            let Some(EffectDef::StaticApply { recipient, effect }) = ability.declarative_effect()
            else {
                continue;
            };
            if recipient != EffectRecipientDef::Source {
                continue;
            }
            match effect {
                AppliedEffectDef::Composite(effects) => applied.extend(effects.iter().copied()),
                effect => applied.push(effect),
            }
        }
        applied
    }

    /// The body a card's own zone-scoped clause gives it, for a card whose
    /// corner prints none. A planeswalker card that is "a 1/1 Insect
    /// creature" anywhere but the battlefield has a power to read there and
    /// nothing printed to read it from.
    pub(super) fn card_zone_stats(
        definition: &CardDefinition,
        zone: ZoneKind,
    ) -> Option<crate::CreatureStats> {
        Self::self_characteristics_in_zone(definition, zone)
            .into_iter()
            .find_map(|effect| match effect {
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::SetBase {
                        power: ValueDef::Constant(power),
                        toughness: ValueDef::Constant(toughness),
                    },
                )) => Some(crate::CreatureStats {
                    power: i16::try_from(power).ok()?,
                    toughness: i16::try_from(toughness).ok()?,
                }),
                _ => None,
            })
    }

    pub(super) fn printed_trigger_event_object(
        &self,
        id: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        context: &CharacteristicContext,
    ) -> Option<TriggerEventObject> {
        let definition = self.catalog.get(definition)?;
        let parts = applicable_part_ids_ref(definition, context).ok()?;
        let mut types = CardTypeSet::empty();
        let mut colors = [false; 5];
        let mut subtypes = Vec::new();
        let mut mana_value = 0;
        let mut power = None;
        let mut toughness = None;
        let mut supertypes = [false; CardSupertype::COUNT];
        let mut keywords = 0_u64;
        for part in parts.iter().copied() {
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
                // A characteristic-defining ability answers here too: what a
                // predicate asks of a card in a graveyard is what that card
                // says it is there, not the number in its corner.
                let stats = self
                    .card_defined_stats(definition, id, controller)
                    .over(stats);
                power = Some(stats.power);
                toughness = Some(stats.toughness);
            }
            for supertype in CardSupertype::ALL {
                supertypes[supertype.index()] |= part.rules.has_supertype(supertype);
            }
        }
        // What the card says about itself while it is here. "As long as
        // Grist isn't on the battlefield, it's a 1/1 Insect creature in
        // addition to its other types" is a clause about a card rather than
        // about a permanent, so nothing in the battlefield layer walk would
        // ever read it.
        if let Some(zone) = context.zone() {
            for effect in Self::self_characteristics_in_zone(definition, zone) {
                match effect {
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                        SetOperationDef::Add(added),
                    )) => types = types.union(added),
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Add(added),
                    )) => {
                        for subtype in added {
                            if !subtypes.contains(subtype) {
                                subtypes.push(*subtype);
                            }
                        }
                    }
                    AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::PowerToughness(
                            PowerToughnessOperationDef::SetBase {
                                power: ValueDef::Constant(set_power),
                                toughness: ValueDef::Constant(set_toughness),
                            },
                        ),
                    ) => {
                        power = i16::try_from(set_power).ok();
                        toughness = i16::try_from(set_toughness).ok();
                    }
                    _ => {}
                }
            }
        }
        Some(TriggerEventObject {
            id,
            token: false,
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
            attacked_during_controllers_last_turn: false,
            attacked_this_turn: false,
            saddled: false,
        })
    }

    /// Whether the step this spell would be cast in satisfies its own timing
    /// restriction. "Before the combat damage step" means combat damage has
    /// not started; once it has, the window is gone for the rest of the turn
    /// even in a later step.
    pub(super) fn play_timing_allows(
        &self,
        player: PlayerId,
        restriction: PlayRestriction,
    ) -> bool {
        match restriction {
            PlayRestriction::Normal | PlayRestriction::FromHandOnly => true,
            // Their turn, their first step. Read from whoever is casting
            // rather than from the spell's controller-to-be, because they are
            // the same player and only one of them is in hand here.
            PlayRestriction::OpponentsUpkeep => {
                self.step == Step::Upkeep && self.active_player != player
            }
            PlayRestriction::DeclareAttackersStep => self.step == Step::DeclareAttackers,
            // Their turn, past the upkeep. Cleanup is excluded with it: no
            // player receives priority there unless something has to be
            // discarded, and the card is not meant to be held that long.
            PlayRestriction::OpponentsTurnAfterUpkeep => {
                self.active_player != player && !matches!(self.step, Step::Upkeep | Step::Cleanup)
            }
            PlayRestriction::BeforeCombatDamage => !matches!(
                self.step,
                Step::CombatDamage
                    | Step::EndOfCombat
                    | Step::PostcombatMain
                    | Step::End
                    | Step::Cleanup
            ),
            // Combat has started and the blockers are not committed yet.
            // There is no priority inside the blocker declaration itself --
            // the defending player is choosing blocks and nobody may cast --
            // so the window is exactly the two steps before it.
            PlayRestriction::BeforeBlockersDeclared => {
                matches!(self.step, Step::BeginningOfCombat | Step::DeclareAttackers)
            }
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
        source: GameObjectId,
    ) -> Vec<Vec<Target>> {
        self.printed_target_lists(behavior, exact_count)
            .into_iter()
            .filter(|choice| {
                choice.iter().all(|target| match target {
                    Target::Permanent(id) => {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == *id)
                            .is_none_or(|permanent| {
                                // Hexproof stops opponents only; you can always
                                // target your own. Protection stops everyone,
                                // including the permanent's own controller.
                                (permanent.controller == player || !self.has_hexproof(permanent))
                                    && !self.is_protected_from_object(permanent, source, true)
                            })
                    }
                    Target::Player(targeted) => {
                        self.player_can_be_targeted_by(*targeted, player, source, true)
                    }
                    Target::Card(_) | Target::Spell(_) => true,
                })
            })
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn printed_target_lists(
        &self,
        behavior: CardBehavior,
        exact_count: Option<usize>,
    ) -> Vec<Vec<Target>> {
        match behavior {
            CardBehavior::GoblinGrenade => self
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
