use super::{
    AbilityDef, AbilityId, AbilityOperationDef, AbilityOrigin, AbilitySourceRef, AppliedEffectDef,
    AppliedRuleDef, AppliedStackEffect, CastSignature, CharacteristicOperationDef,
    ColorChoiceOperationDef, ColorSet, ComparisonDef, ContinuousEffectExpiration,
    ContinuousEffectTimestamp, ControlFlow, CounterKind, EffectRecipientDef, EffectRecipientSetDef,
    EffectResolutionContext, Game, GameObjectId, GrantId, ManaColor, NonbattlefieldAbilityGrant,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, Permanent, PlayerId,
    PlayerRefDef, PlayerSetDef, PowerToughnessOperationDef, QuantifierDef,
    ResolvedAbilityOperation, ResolvedAttackRestriction, ResolvedContinuousEffect,
    ResolvedContinuousEffectKind, ResolvedDamageRedirect, ResolvedEffectDurationDef,
    ResolvedPlayPermission, ResolvedPlayRestriction, ResolvedPlayerProtection, ResolvedPlayerRule,
    ResolvedPowerToughnessOperation, RetiredObject, ScopedEffect, StackObject, StackObjectKind,
    Target, TargetIndex, TargetSelection, TargetSlotId, TriggerConditionDef, TriggerContext,
    ZoneKind, abilities,
};
use crate::card::{PlayerAttachmentQueryDef, TargetChooserDef};

#[derive(Clone, Copy)]
struct ResolvedAppliedEffect<'a> {
    duration: ResolvedEffectDurationDef,
    timestamp: ContinuousEffectTimestamp,
    object: &'a StackObject,
    context: &'a EffectResolutionContext,
    scoped: ScopedEffect,
    component_order: u16,
}

mod queries;

impl Game {
    /// One protection ability per colour, so the chosen one has a static
    /// grant to point at. A granted ability is borrowed for the life of the
    /// game, which a colour picked at resolution cannot supply on its own.
    const PROTECTION_FROM_COLOR: [AbilityDef; 6] = [
        abilities::protection_from_color(ManaColor::White),
        abilities::protection_from_color(ManaColor::Blue),
        abilities::protection_from_color(ManaColor::Black),
        abilities::protection_from_color(ManaColor::Red),
        abilities::protection_from_color(ManaColor::Green),
        abilities::protection_from_color(ManaColor::Colorless),
    ];

    /// The single-colour sets, in the same order the choice offers them.
    const CHOSEN_COLOR_SETS: [ColorSet; 5] = [
        ColorSet::from_colors(&[ManaColor::White]),
        ColorSet::from_colors(&[ManaColor::Blue]),
        ColorSet::from_colors(&[ManaColor::Black]),
        ColorSet::from_colors(&[ManaColor::Red]),
        ColorSet::from_colors(&[ManaColor::Green]),
    ];

    /// Apply a named colour to everything the choice was resolved against.
    /// `index` is a position in [`Self::CHOOSABLE_COLORS`], which is what the
    /// decision offered.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_chosen_color(
        &mut self,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        targets: &[Target],
        operation: ColorChoiceOperationDef,
        duration: ResolvedEffectDurationDef,
        index: usize,
    ) {
        let effect = match operation {
            ColorChoiceOperationDef::ProtectionFromChosenColor
            | ColorChoiceOperationDef::ProtectionFromChosenColorOrColorless => {
                AppliedEffectDef::add_ability(&Self::PROTECTION_FROM_COLOR[index])
            }
            ColorChoiceOperationDef::BecomesChosenColor => {
                AppliedEffectDef::set_colors(Self::CHOSEN_COLOR_SETS[index])
            }
        };
        let timestamp = self.allocate_continuous_effect_timestamp();
        for target in targets {
            self.apply_applied_effect_component(
                *target,
                effect,
                ResolvedAppliedEffect {
                    duration,
                    timestamp,
                    object,
                    context,
                    scoped,
                    component_order: 0,
                },
            );
        }
    }

    pub(super) fn resolve_applied_effect(
        &mut self,
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let timestamp = self.allocate_continuous_effect_timestamp();
        let base_resolution = ResolvedAppliedEffect {
            duration,
            timestamp,
            object,
            context,
            scoped,
            component_order: 0,
        };
        let mut components = Vec::new();
        Self::flatten_applied_effect(effect, &mut components);
        for target in self.effect_recipients(recipient, object, context, scoped) {
            Self::apply_components_to(self, target, &components, base_resolution);
        }
    }

    /// Prepared execution for the common resolving self-grant. This commits
    /// the same single layer-6 component as [`Self::resolve_applied_effect`]
    /// after the compiler has proven its recipient, operation, and duration.
    pub(super) fn grant_source_ability_until_end_of_turn(
        &mut self,
        source: Option<GameObjectId>,
        origin: AbilityOrigin,
        ability: &'static AbilityDef,
    ) {
        // Reference resolution allocates the application timestamp before it
        // discovers whether the source is still a battlefield permanent.
        let timestamp = self.allocate_continuous_effect_timestamp();
        let Some(source) = source else {
            return;
        };
        let Some(permanent_index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == source)
        else {
            self.apply_nonbattlefield_granted_ability(
                Target::Card(source),
                ability,
                ContinuousEffectExpiration::EndOfTurn,
                Some(origin),
            );
            return;
        };
        let permanent = &mut self.battlefield[permanent_index];
        let grant = Self::next_resolved_ability_grant(permanent);
        let definition = AppliedEffectDef::add_ability(ability);
        permanent
            .resolved_continuous_effects
            .push(ResolvedContinuousEffect {
                definition,
                source: AbilitySourceRef {
                    object: source,
                    ability: origin,
                },
                timestamp,
                component_order: 0,
                expiration: ContinuousEffectExpiration::EndOfTurn,
                kind: ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    ability: *ability,
                    grant,
                }),
            });
    }

    /// The same application, for recipients an effect has already resolved to
    /// concrete targets. Damage riders need this: which objects they apply to
    /// is decided by what the damage did, not by a recipient the runtime can
    /// evaluate a second time.
    pub(super) fn apply_effect_to_targets(
        &mut self,
        targets: &[Target],
        effect: AppliedEffectDef,
        duration: ResolvedEffectDurationDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let timestamp = self.allocate_continuous_effect_timestamp();
        let base_resolution = ResolvedAppliedEffect {
            duration,
            timestamp,
            object,
            context,
            scoped,
            component_order: 0,
        };
        let mut components = Vec::new();
        Self::flatten_applied_effect(effect, &mut components);
        for target in targets.iter().copied() {
            Self::apply_components_to(self, target, &components, base_resolution);
        }
    }

    fn apply_components_to(
        game: &mut Self,
        target: Target,
        components: &[AppliedEffectDef],
        base_resolution: ResolvedAppliedEffect<'_>,
    ) {
        for (index, component) in components.iter().copied().enumerate() {
            let component_order = u16::try_from(index)
                .expect("one applied effect contains at most 65,536 components");
            game.apply_applied_effect_component(
                target,
                component,
                ResolvedAppliedEffect {
                    component_order,
                    ..base_resolution
                },
            );
        }
    }

    fn flatten_applied_effect(effect: AppliedEffectDef, components: &mut Vec<AppliedEffectDef>) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::flatten_applied_effect(*effect, components);
                }
            }
            leaf => components.push(leaf),
        }
    }

    /// Where a granted ability lands: the supported nonbattlefield flashback
    /// case keeps its cleanup-bounded card grant, while a permanent records an
    /// ordered, duration-aware layer operation for every ability category.
    fn apply_nonbattlefield_granted_ability(
        &mut self,
        target: Target,
        ability: &'static AbilityDef,
        expiration: ContinuousEffectExpiration,
        source: Option<AbilityOrigin>,
    ) {
        let Target::Card(target) = target else {
            return;
        };
        let grant = NonbattlefieldAbilityGrant {
            object: target,
            ability: *ability,
            expiration,
            source,
        };
        if self.card_in_nonbattlefield_zone(target).is_some()
            && !self.nonbattlefield_ability_grants.contains(&grant)
        {
            self.nonbattlefield_ability_grants.push(grant);
        }
    }

    /// Gives an entering permanent an ability it gained from the way it was
    /// played: "if you do, it gains ...". The grant is written onto the
    /// permanent rather than left with whatever allowed the play, because
    /// the printed clause outlives its source -- a Serra Paragon that dies
    /// does not take the exile clause back off what it returned.
    ///
    /// The permanent is its own source here for the same reason a granted
    /// keyword is: nothing is left to point at, and nothing about the grant
    /// depends on what allowed it.
    pub(super) fn grant_resolved_ability_to_entering_permanent(
        &mut self,
        permanent: &mut Permanent,
        source: AbilitySourceRef,
        effect: AppliedEffectDef,
    ) {
        let AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) = effect
        else {
            return;
        };
        let timestamp = self.allocate_continuous_effect_timestamp();
        let used = permanent
            .resolved_continuous_effects
            .iter()
            .filter_map(|resolved| match resolved.kind {
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    grant,
                    ..
                }) => Some(grant.index()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let Some(grant) = (0..=u8::MAX as usize)
            .find(|index| !used.contains(index))
            .and_then(GrantId::from_index)
        else {
            return;
        };
        permanent
            .resolved_continuous_effects
            .push(ResolvedContinuousEffect {
                definition: effect,
                source,
                timestamp,
                component_order: 0,
                expiration: ContinuousEffectExpiration::Never,
                kind: ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    ability: *ability,
                    grant,
                }),
            });
    }

    pub(super) fn continuous_effect_expiration(
        duration: ResolvedEffectDurationDef,
        controller: PlayerId,
        turns_started: u32,
    ) -> ContinuousEffectExpiration {
        ContinuousEffectExpiration::any_of(
            [
                duration
                    .contains(ResolvedEffectDurationDef::UntilEndOfTurn)
                    .then_some(ContinuousEffectExpiration::EndOfTurn),
                duration
                    .contains(ResolvedEffectDurationDef::UntilNextMatchingCast)
                    .then_some(ContinuousEffectExpiration::NextMatchingCast),
                duration
                    .contains(ResolvedEffectDurationDef::UntilEndOfCombat)
                    .then_some(ContinuousEffectExpiration::EndOfCombat),
                duration
                    .contains(ResolvedEffectDurationDef::UntilYourNextUpkeep)
                    .then_some(ContinuousEffectExpiration::UpkeepOf(controller)),
                duration
                    .contains(ResolvedEffectDurationDef::UntilYourNextTurn)
                    .then_some(ContinuousEffectExpiration::TurnOf {
                        player: controller,
                        turn: turns_started.saturating_add(1),
                    }),
                duration
                    .contains(ResolvedEffectDurationDef::WhileSourceTapped)
                    .then_some(ContinuousEffectExpiration::WhileSourceTapped),
                duration
                    .contains(ResolvedEffectDurationDef::WhileSourceRemains)
                    .then_some(ContinuousEffectExpiration::WhileSourceRemains),
            ]
            .into_iter()
            .flatten(),
        )
    }

    fn apply_applied_effect_component(
        &mut self,
        target: Target,
        effect: AppliedEffectDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) {
        match effect {
            AppliedEffectDef::Composite(_) => {
                unreachable!("applied effects are flattened before dispatch")
            }
            AppliedEffectDef::Characteristic(operation) => {
                self.apply_characteristic_component(target, effect, operation, resolution);
            }
            AppliedEffectDef::Rule(rule) => {
                self.apply_rule_component(target, effect, rule, resolution);
            }
        }
    }

    fn apply_rule_component(
        &mut self,
        target: Target,
        definition: AppliedEffectDef,
        rule: AppliedRuleDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) {
        // `CannotBeCountered` is meaningful on a stack object, whose lifetime
        // is already represented by `AppliedStackEffect`. A resolving Apply
        // program cannot honestly give it one of the permanent durations.
        debug_assert_ne!(rule, AppliedRuleDef::CannotBeCountered);
        if rule == AppliedRuleDef::CannotBeCountered {
            return;
        }
        let expiration = Self::continuous_effect_expiration(
            resolution.duration,
            resolution.object.controller,
            self.turns_started[resolution.object.controller.index()],
        );
        if let AppliedRuleDef::RedirectDamageFromTo {
            source,
            destination,
        } = rule
        {
            let Target::Player(player) = target else {
                return;
            };
            let Some(source) = self.effect_object_reference_id(
                source,
                resolution.object,
                resolution.context,
                resolution.scoped,
            ) else {
                return;
            };
            let Some(destination) = self.effect_object_reference_id(
                destination,
                resolution.object,
                resolution.context,
                resolution.scoped,
            ) else {
                return;
            };
            self.damage_redirects.push(ResolvedDamageRedirect {
                player,
                source,
                destination,
                expiration,
            });
            return;
        }
        let source = AbilitySourceRef {
            object: resolution.object.source.unwrap_or(resolution.object.id),
            ability: resolution.object.ability_origin().unwrap_or_else(|| {
                Self::authored_ability_origin(resolution.object.presentation(), AbilityId::PRIMARY)
            }),
        };
        if self.apply_player_play_rule(target, definition, rule, &resolution, source, expiration) {
            return;
        }
        let Target::Permanent(target) = target else {
            return;
        };
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == target)
        {
            permanent
                .resolved_continuous_effects
                .push(ResolvedContinuousEffect {
                    definition,
                    source,
                    timestamp: resolution.timestamp,
                    component_order: resolution.component_order,
                    expiration,
                    kind: ResolvedContinuousEffectKind::Rule(rule),
                });
        }
    }

    /// The rules whose subject is a player rather than a permanent: what they
    /// may not play, and what they may play that the ordinary rules would
    /// refuse. Returns whether the rule was one of them.
    fn apply_player_play_rule(
        &mut self,
        target: Target,
        definition: AppliedEffectDef,
        rule: AppliedRuleDef,
        resolution: &ResolvedAppliedEffect<'_>,
        source: AbilitySourceRef,
        expiration: ContinuousEffectExpiration,
    ) -> bool {
        match rule {
            AppliedRuleDef::MayPlayFromGraveyard(_)
            | AppliedRuleDef::MayPlayFromTopOfLibrary { .. }
            | AppliedRuleDef::MayCastAsThoughItHadFlash(_)
            | AppliedRuleDef::GrantsAlternativeCastFromGraveyard { .. } => {
                if let Target::Player(affected_player) = target {
                    self.resolved_play_permissions.push(ResolvedPlayPermission {
                        definition,
                        source,
                        affected_player,
                        expiration,
                        rule,
                    });
                }
                true
            }
            // "For the rest of the game" is the only duration this is
            // printed with, so what it leaves behind is a flag on the player
            // rather than an entry in a list that has to expire.
            AppliedRuleDef::CannotGainLife => {
                if let Target::Player(affected_player) = target {
                    self.cannot_gain_life[affected_player.index()] = true;
                }
                true
            }
            AppliedRuleDef::PlayerProtectionFrom(quality) => {
                if let Target::Player(affected_player) = target {
                    self.resolved_player_protections
                        .push(ResolvedPlayerProtection {
                            definition,
                            source,
                            affected_player,
                            expiration,
                            quality,
                        });
                }
                true
            }
            AppliedRuleDef::PlayerRule(rule) => {
                if let Target::Player(affected_player) = target {
                    self.resolved_player_rules.push(ResolvedPlayerRule {
                        definition,
                        source,
                        affected_player,
                        expiration,
                        rule,
                    });
                }
                true
            }
            AppliedRuleDef::CannotPlay(restriction) => {
                if let Target::Player(affected_player) = target {
                    self.resolved_play_restrictions
                        .push(ResolvedPlayRestriction {
                            definition,
                            source,
                            affected_player,
                            timestamp: resolution.timestamp,
                            component_order: resolution.component_order,
                            expiration,
                            restriction,
                        });
                }
                true
            }
            AppliedRuleDef::AttackRestriction(restriction)
                if restriction.defender != crate::card::AttackDefenderScopeDef::Any =>
            {
                if let Target::Player(affected_player) = target {
                    self.resolved_attack_restrictions
                        .push(ResolvedAttackRestriction {
                            definition,
                            source,
                            affected_player,
                            expiration,
                            restriction,
                        });
                }
                true
            }
            _ => false,
        }
    }

    fn apply_characteristic_component(
        &mut self,
        target: Target,
        definition: AppliedEffectDef,
        operation: CharacteristicOperationDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) {
        if let (
            Target::Spell(target),
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Add(_)),
        ) = (target, operation)
        {
            // A resolved grant to a creature spell is carried onto the
            // permanent it becomes. Bloodlord of Vaasgoth uses this for a
            // whole replacement ability rather than a keyword-only mana
            // rider. No printed spell grant in this lane has a temporary
            // duration; declining one prevents an expiration from silently
            // becoming permanent on entry.
            if resolution.duration != ResolvedEffectDurationDef::Permanent {
                return;
            }
            let granting = AbilitySourceRef {
                object: resolution.object.source.unwrap_or(resolution.object.id),
                ability: resolution.object.ability_origin().unwrap_or_else(|| {
                    Self::authored_ability_origin(
                        resolution.object.presentation(),
                        AbilityId::PRIMARY,
                    )
                }),
            };
            if let Some(spell) = self.stack.iter_mut().find(|spell| spell.id == target) {
                spell.applied_effects.push(AppliedStackEffect {
                    source: None,
                    granting: Some(granting),
                    effect: definition,
                });
            }
            return;
        }
        if let CharacteristicOperationDef::Abilities(AbilityOperationDef::Add(ability)) = operation
            && matches!(target, Target::Card(_))
        {
            let expiration = Self::continuous_effect_expiration(
                resolution.duration,
                resolution.object.controller,
                self.turns_started[resolution.object.controller.index()],
            );
            self.apply_nonbattlefield_granted_ability(
                target,
                ability,
                expiration,
                resolution
                    .object
                    .ability
                    .as_ref()
                    .map(|ability| ability.origin),
            );
            return;
        }
        if let (Target::Spell(target), CharacteristicOperationDef::Colors(operation)) =
            (target, operation)
        {
            let current = ManaColor::COLORS
                .into_iter()
                .zip(self.object_colors(target))
                .filter_map(|(color, present)| present.then_some(color))
                .fold(ColorSet::empty(), ColorSet::with);
            let colors = Self::apply_color_operation(current, operation);
            if let Some(spell) = self.stack.iter_mut().find(|spell| spell.id == target) {
                spell.colors = Some(colors);
            }
            return;
        }
        let Target::Permanent(target) = target else {
            return;
        };

        let Some(kind) = self.resolve_characteristic_kind(target, operation, resolution) else {
            return;
        };
        let source = AbilitySourceRef {
            object: resolution.object.source.unwrap_or(resolution.object.id),
            ability: resolution.object.ability_origin().unwrap_or_else(|| {
                Self::authored_ability_origin(resolution.object.presentation(), AbilityId::PRIMARY)
            }),
        };
        let expiration = Self::continuous_effect_expiration(
            resolution.duration,
            resolution.object.controller,
            self.turns_started[resolution.object.controller.index()],
        );
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == target)
        {
            permanent
                .resolved_continuous_effects
                .push(ResolvedContinuousEffect {
                    definition,
                    source,
                    timestamp: resolution.timestamp,
                    component_order: resolution.component_order,
                    expiration,
                    kind,
                });
        }
    }

    fn resolve_characteristic_kind(
        &self,
        target: GameObjectId,
        operation: CharacteristicOperationDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) -> Option<ResolvedContinuousEffectKind> {
        Some(match operation {
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Add(ability)) => {
                let permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == target)?;
                let grant = Self::next_resolved_ability_grant(permanent);
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    ability: *ability,
                    grant,
                })
            }
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Remove(predicate)) => {
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Remove(predicate))
            }
            // A grant read off the exile pile is answered by the layer walk
            // every time it is asked, because the pile it reads keeps
            // changing. Freezing one into a resolved effect would fix it at
            // whatever the pile held the moment it resolved, so no resolving
            // effect produces this shape.
            // Neither has a resolved form: one is a static shape, and the
            // chosen land type is read live off the permanent that made the
            // choice.
            CharacteristicOperationDef::Abilities(
                AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(_),
            )
            | CharacteristicOperationDef::ChosenBasicLandType
            | CharacteristicOperationDef::AddChosenCreatureType
            | CharacteristicOperationDef::SetChosenCreatureType
            | CharacteristicOperationDef::Color(_)
            | CharacteristicOperationDef::Supertypes(_) => return None,
            CharacteristicOperationDef::BasicLandTypes(operation) => {
                ResolvedContinuousEffectKind::BasicLandTypes(operation)
            }

            CharacteristicOperationDef::CardTypes(operation) => {
                ResolvedContinuousEffectKind::CardTypes(operation)
            }
            CharacteristicOperationDef::Colors(operation) => {
                ResolvedContinuousEffectKind::Colors(operation)
            }
            CharacteristicOperationDef::CreatureTypes(operation) => {
                ResolvedContinuousEffectKind::CreatureTypes(operation)
            }
            CharacteristicOperationDef::Subtypes(operation) => {
                ResolvedContinuousEffectKind::Subtypes(operation)
            }
            CharacteristicOperationDef::PowerToughness(operation) => {
                let freeze = |value| {
                    i16::try_from(
                        self.effect_value(
                            value,
                            resolution.object,
                            resolution.context,
                            resolution.scoped,
                        )
                        .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                    )
                    .expect("the effect value was clamped to i16")
                };
                ResolvedContinuousEffectKind::PowerToughness(match operation {
                    PowerToughnessOperationDef::SetBase { power, toughness } => {
                        ResolvedPowerToughnessOperation::SetBase {
                            power: freeze(power),
                            toughness: freeze(toughness),
                        }
                    }
                    PowerToughnessOperationDef::SetBasePower(power) => {
                        ResolvedPowerToughnessOperation::SetBasePower {
                            power: freeze(power),
                        }
                    }
                    PowerToughnessOperationDef::SetBaseToughness(toughness) => {
                        ResolvedPowerToughnessOperation::SetBaseToughness {
                            toughness: freeze(toughness),
                        }
                    }
                    PowerToughnessOperationDef::Modify { power, toughness } => {
                        ResolvedPowerToughnessOperation::Modify {
                            power: freeze(power),
                            toughness: freeze(toughness),
                        }
                    }
                    // Nothing to freeze: the switch names no value.
                    PowerToughnessOperationDef::Switch => ResolvedPowerToughnessOperation::Switch,
                    // A characteristic-defining ability is printed on a card
                    // and read live wherever that card is; no resolving
                    // effect creates one, and freezing one would fix a
                    // number that is supposed to keep answering.
                    PowerToughnessOperationDef::Define { .. } => return None,
                })
            }
        })
    }

    fn next_resolved_ability_grant(permanent: &Permanent) -> GrantId {
        let mut used_grants = [false; 256];
        for grant in permanent
            .resolved_continuous_effects
            .iter()
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    grant,
                    ..
                }) => Some(grant),
                _ => None,
            })
        {
            used_grants[grant.index()] = true;
        }
        used_grants
            .iter()
            .position(|used| !used)
            .and_then(GrantId::from_index)
            .expect("one permanent has at most 256 active resolved grants")
    }

    pub(super) fn live_object_target(&self, object: GameObjectId) -> Option<Target> {
        if self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == object)
        {
            return Some(Target::Permanent(object));
        }
        if self.stack.iter().any(|candidate| candidate.id == object) {
            return Some(Target::Spell(object));
        }
        if self.card_in_nonbattlefield_zone(object).is_some() {
            return Some(Target::Card(object));
        }
        None
    }

    /// The exact object incarnation a reference names, including a retired
    /// representation when last-known information still remembers it.
    pub(super) fn object_target_with_lki(&self, object: GameObjectId) -> Option<Target> {
        self.live_object_target(object)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Card(_)) => Some(Target::Card(object)),
                Some(RetiredObject::Permanent { .. }) => Some(Target::Permanent(object)),
                Some(RetiredObject::Stack(_)) => Some(Target::Spell(object)),
                None => None,
            })
    }

    /// The live object created by exactly one zone change of `object`.
    /// Following one edge, rather than the whole physical-card chain, is the
    /// identity rule behind printed "return it" exceptions: a second move
    /// makes another new object and breaks the reference again.
    pub(super) fn zone_change_successor_target(&self, object: GameObjectId) -> Option<Target> {
        let successor = *self.successors.get(&object)?;
        self.live_object_target(successor)
    }

    /// How much of a divided total one target takes, read off the selection
    /// frozen when the object was put on the stack.
    pub(super) fn divided_share(
        object: &StackObject,
        slot: TargetSlotId,
        target: Target,
    ) -> Option<u16> {
        object
            .signature
            .as_ref()
            .map(CastSignature::targets)
            .or_else(|| {
                object
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
            })?
            .iter()
            .find(|selection| selection.slot() == slot)?
            .amount_for(target)
    }

    /// The targets frozen into one slot when the object was put on the stack,
    /// before any legality check.
    pub(super) fn chosen_targets(
        object: &StackObject,
        slot: TargetSlotId,
    ) -> impl Iterator<Item = Target> {
        object
            .signature
            .as_ref()
            .map(CastSignature::targets)
            .or_else(|| {
                object
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
            })
            .and_then(|selections| selections.iter().find(|selection| selection.slot() == slot))
            .into_iter()
            .flat_map(TargetSelection::targets)
            .copied()
    }

    pub(super) fn stack_ability_target_is_legal(
        &self,
        object: &StackObject,
        slot: TargetSlotId,
        target: Target,
    ) -> bool {
        let source = object.source.unwrap_or(object.id);
        let Some(ability) = &object.ability else {
            return true;
        };
        let Some(definition) = ability.target_defs.get(slot.index()) else {
            // An installed trigger retains its installer's selections as
            // lexical references while declaring no fresh targets of its own.
            // They do not become targets again and therefore are not checked
            // for legality as the delayed ability resolves.
            return ability.target_defs.is_empty();
        };
        // Measured against whoever chose, which for almost every ability is
        // its controller. A slot the clause handed to somebody else is still
        // theirs on resolution: legality asks the same question it asked
        // when the target was named.
        let chooser = match definition.chooser {
            TargetChooserDef::Controller => object.controller,
            TargetChooserDef::EventPlayer => ability
                .context
                .trigger
                .event_player
                .unwrap_or(object.controller),
            TargetChooserDef::Opponent => object.controller.opponent(),
        };
        Self::without_excluded_source(
            definition,
            source,
            self.ability_targets_matching_with_selections_for_chooser(
                definition.predicate,
                &ability.targets,
                chooser,
                object.controller,
                source,
                ability.context.trigger,
            ),
        )
        .contains(&target)
    }
}

include!("effect_support/conditions.rs");
include!("effect_support/references.rs");

/// One comparison, so a condition reads the same however it is counted.
pub(super) fn compare<T: Ord>(left: &T, comparison: ComparisonDef, right: &T) -> bool {
    match comparison {
        ComparisonDef::Less => left < right,
        ComparisonDef::LessOrEqual => left <= right,
        ComparisonDef::Equal => left == right,
        ComparisonDef::GreaterOrEqual => left >= right,
        ComparisonDef::Greater => left > right,
    }
}

#[cfg(test)]
mod tests {
    use super::compare;
    use crate::ComparisonDef;

    #[test]
    fn comparisons_follow_their_ordering_semantics() {
        assert!(compare(&1, ComparisonDef::Less, &2));
        assert!(compare(&2, ComparisonDef::LessOrEqual, &2));
        assert!(compare(&2, ComparisonDef::Equal, &2));
        assert!(compare(&2, ComparisonDef::GreaterOrEqual, &2));
        assert!(compare(&3, ComparisonDef::Greater, &2));
    }
}
