use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, ActivationChoices, ActivationTimingDef,
    BattlefieldExitCompletion, CardBehavior, CardInstance, CharacteristicContext,
    CommittedTriggerEvent, CounterKind, DeclarativeAbilityDef, FrozenActivatedAbility, Game,
    GameEvent, GameObjectId, ManaCost, ManaPaymentPurpose, ManaPlanOptions, ObjectCharacteristics,
    ObjectInstance, PendingActivation, PendingActivationTargeting, PlayRestriction, PlayerId,
    SacrificeQuota, Step, Target, TargetSelection, ZoneKind, ZoneMoveCause, ZonePlacement,
    remove_card,
};

use crate::ManaPaymentChoice;

mod nonbattlefield;

/// What one activation announced about paying its cost: the value of X, the
/// objects its costs spend, what the mana is being raised for, and how any
/// flexible symbol in it is being paid.
#[derive(Clone, Copy)]
struct AnnouncedActivationCost<'a> {
    cost_objects: &'a [GameObjectId],
    x: u16,
    payment_purpose: &'a ManaPaymentPurpose,
    mana_payment: Option<&'a ManaPaymentChoice>,
}

impl Game {
    /// Moves the counters a loyalty cost names and records that this
    /// planeswalker has used its one ability for the turn (CR 606.3). Shared
    /// by the two activation paths, because a loyalty ability that makes
    /// mana is paid for exactly like one that does not.
    pub(super) fn pay_loyalty_cost(&mut self, source: GameObjectId, change: i8) {
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        if change >= 0 {
            permanent.add_counters(CounterKind::Loyalty, u16::from(change.unsigned_abs()));
        } else {
            permanent.remove_counters(CounterKind::Loyalty, u16::from(change.unsigned_abs()));
        }
        permanent.activated_loyalty_this_turn = true;
    }

    /// Whether a printed "Activate only ..." window is currently open for
    /// this player. The restriction narrows when an ability may be activated;
    /// it says nothing about priority, so an ability still needs its
    /// controller to have it.
    pub(super) fn activation_timing_allows(
        &self,
        player: PlayerId,
        timing: ActivationTimingDef,
    ) -> bool {
        match timing {
            ActivationTimingDef::Any => true,
            ActivationTimingDef::YourTurn => self.active_player == player,
            ActivationTimingDef::YourUpkeep => {
                self.active_player == player && self.step == Step::Upkeep
            }
            ActivationTimingDef::AnyUpkeep => self.step == Step::Upkeep,
            ActivationTimingDef::DuringCombat => self.step.is_combat(),
            ActivationTimingDef::EndOfCombat => self.step == Step::EndOfCombat,
            ActivationTimingDef::SorcerySpeed => {
                self.active_player == player && self.step.is_main() && self.stack.is_empty()
            }
            // The same window Berserk uses, and read the same way: once
            // combat damage has started it is gone for the rest of the turn,
            // even in a later step.
            // The priority round the declaration opens, which is the only
            // window between the two declarations: once the step advances,
            // blockers are being declared and nothing else has priority.
            ActivationTimingDef::AfterAttackersDeclared => {
                self.step == Step::DeclareAttackers && self.attackers_declared
            }
            ActivationTimingDef::BeforeCombatDamage => {
                self.play_timing_allows(player, PlayRestriction::BeforeCombatDamage)
            }
        }
    }

    /// Activates an ability printed to work from its owner's graveyard. The
    /// card is not a permanent and never becomes one, so the only cost it can
    /// pay with itself is exile, and the ability on the stack outlives it.
    /// Scavenge's cost, and the only one a card pays out of its own
    /// graveyard. Retiring the old identity here is what later lets the
    /// resolving ability read the card's power: by then it is in exile under
    /// a new one.
    pub(super) fn exile_graveyard_source(&mut self, player: PlayerId, source: GameObjectId) {
        self.exile_graveyard_cards(player, &[source]);
    }

    pub(super) fn exile_graveyard_cards(&mut self, player: PlayerId, sources: &[GameObjectId]) {
        let mut exiled = Vec::new();
        for source in sources {
            let card = remove_card(&mut self.players[player.index()].graveyard, *source)
                .expect("a legal graveyard payment still has its card");
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].exile.push(card.clone());
            exiled.push(card);
        }
        if !exiled.is_empty() {
            self.capture_cards_exiled(&exiled, crate::card::ZoneKind::Graveyard);
            self.note_card_left_graveyard(player);
        }
    }

    fn activate_graveyard_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        choices: ActivationChoices<'_>,
        source_card: &CardInstance,
    ) {
        let ActivationChoices {
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
        } = choices;
        let Some(effective) = self.find_printed_card_ability(
            source_card,
            &CharacteristicContext::Graveyard,
            |effective| effective.origin == ability,
        ) else {
            return;
        };
        let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
            return;
        };
        if !effective.ability.is_executable()
            || definition.procedure != AbilityProcedureDef::Shared
            || !definition.source_zones.contains(&ZoneKind::Graveyard)
        {
            return;
        }
        let Some(plan) = Self::selected_activated_plan(&definition, modes) else {
            return;
        };
        let frozen = FrozenActivatedAbility {
            origin: effective.origin,
            definition: Some(Box::new(effective.ability)),
            presentation: Self::ability_presentation(
                effective.origin,
                ObjectCharacteristics::card(source_card.definition, crate::CardPartId::PRIMARY),
            ),
            text: Some(effective.ability.text),
            target_defs: plan.target_defs,
            resolver: Self::ability_resolver(effective.origin, &effective.ability),
            mode_effects: plan.mode_effects,
            x,
            sacrificed_mana_value: 0,
        };
        let payment_purpose = ManaPaymentPurpose::Ability {
            source,
            taps_source: false,
            leaves_source: true,
        };
        self.pay_graveyard_activation_costs(
            player,
            source,
            &definition,
            AnnouncedActivationCost {
                cost_objects,
                x,
                payment_purpose: &payment_purpose,
                mana_payment,
            },
        );
        let chosen_permanents = targets
            .iter()
            .flat_map(TargetSelection::targets)
            .filter_map(|target| match target {
                Target::Permanent(permanent) => Some(*permanent),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
            })
            .collect();
        self.push_activated_ability(
            source,
            &source_card.clone().into(),
            player,
            frozen,
            targets,
            chosen_permanents,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }

    /// Spends what an announced flexible-symbol payment costs in life and
    /// returns the mana still owed (CR 118.4, CR 602.2b). An activation that
    /// announced nothing owes its cost exactly as printed, which is nearly
    /// every activation there is.
    fn announced_activation_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        payment: Option<&ManaPaymentChoice>,
    ) -> ManaCost {
        let Some((locked, life)) = payment.and_then(|payment| {
            // An activation is not a cast, so no cast-time "spend as though
            // any colour" permission reaches it.
            let locked = Self::locked_mana_payment(cost, payment, false)?;
            Some(locked)
        }) else {
            return cost;
        };
        self.lose_life(player, life);
        locked
    }

    /// Pays what a graveyard activation owes. The permanent it taps goes
    /// first, so automatic mana payment cannot tap it out from under the
    /// cost it is paying.
    fn pay_graveyard_activation_costs(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        definition: &crate::card::ActivatedAbilityDef,
        announced: AnnouncedActivationCost<'_>,
    ) {
        let AnnouncedActivationCost {
            cost_objects,
            x,
            payment_purpose,
            mana_payment,
        } = announced;
        let priced_mana_cost = self.priced_ability_mana_cost(source, definition);
        if definition
            .costs
            .iter()
            .any(|cost| matches!(cost, AbilityCostDef::TapPermanent { .. }))
            && let Some(chosen) = cost_objects.first()
        {
            let _ = self.tap_permanent(*chosen);
        }
        for cost in definition.costs.as_slice() {
            match cost {
                AbilityCostDef::Mana(printed) => {
                    let cost = priced_mana_cost.unwrap_or(*printed);
                    let cost = self.announced_activation_cost(player, cost, mana_payment);
                    self.activate_mana_for_cost_avoiding_for(
                        player,
                        cost,
                        x,
                        None,
                        payment_purpose,
                    );
                    let _ = self.pay_player_cost_for(player, cost, x, payment_purpose);
                }
                // Paid above, before anything else could tap it.
                AbilityCostDef::TapPermanent { .. } => {}
                AbilityCostDef::ExileSource => self.exile_graveyard_source(player, source),
                _ => unreachable!("unsupported graveyard-zone costs are not offered"),
            }
        }
    }

    /// Activates the ability carried by a resolved ongoing effect. The effect
    /// is command-zone-resident only as an engine source-zone approximation;
    /// it has no permanent state and therefore supports only the source-free
    /// mana-cost shape enforced by catalog validation.
    fn activate_ongoing_effect_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
    ) -> bool {
        let Some(ongoing) = self
            .ongoing_effects
            .iter()
            .find(|ongoing| {
                ongoing.source.object == source
                    && ongoing.source.ability == ability
                    && ongoing.controller == player
            })
            .cloned()
        else {
            return false;
        };
        let DeclarativeAbilityDef::Activated(definition) = ongoing.ability.definition else {
            return false;
        };
        if !ongoing.ability.is_executable()
            || definition.procedure != AbilityProcedureDef::Shared
            || definition.source_zones != [ZoneKind::Command]
        {
            return false;
        }
        let Some(cost) = Self::activated_ability_mana_cost(&definition) else {
            return false;
        };
        let purpose = ManaPaymentPurpose::Ability {
            source,
            taps_source: false,
            leaves_source: false,
        };
        self.activate_mana_for_cost_avoiding_for(player, cost, 0, None, &purpose);
        let _ = self.pay_player_cost_for(player, cost, 0, &purpose);
        let frozen = FrozenActivatedAbility {
            origin: ongoing.source.ability,
            definition: Some(Box::new(ongoing.ability)),
            presentation: ongoing.presentation,
            text: Some(ongoing.ability.text),
            target_defs: Vec::new(),
            resolver: Self::ability_resolver(ongoing.source.ability, &ongoing.ability),
            mode_effects: Vec::new(),
            x: 0,
            sacrificed_mana_value: 0,
        };
        self.push_activated_ability_with_context(
            source,
            ongoing.owner,
            player,
            frozen,
            Vec::new(),
            Vec::new(),
            ongoing.context,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
        true
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn activate_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        choices: ActivationChoices<'_>,
    ) {
        let ActivationChoices {
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
        } = choices;
        if self.activate_ongoing_effect_ability(player, source, ability) {
            return;
        }
        if let Some(source_card) = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            .cloned()
        {
            let Some(effective) = self.find_printed_card_ability(
                &source_card,
                &CharacteristicContext::Hand,
                |effective| effective.origin == ability,
            ) else {
                return;
            };
            let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
                return;
            };
            if !effective.ability.is_executable()
                || definition.procedure != AbilityProcedureDef::Shared
                || !definition.source_zones.contains(&ZoneKind::Hand)
            {
                return;
            }
            let Some(plan) = Self::selected_activated_plan(&definition, modes) else {
                return;
            };
            let frozen = FrozenActivatedAbility {
                origin: effective.origin,
                definition: Some(Box::new(effective.ability)),
                presentation: Self::ability_presentation(
                    effective.origin,
                    ObjectCharacteristics::card(source_card.definition, crate::CardPartId::PRIMARY),
                ),
                text: Some(effective.ability.text),
                target_defs: plan.target_defs,
                resolver: Self::ability_resolver(effective.origin, &effective.ability),
                mode_effects: plan.mode_effects,
                x,
                sacrificed_mana_value: 0,
            };
            let payment_purpose = ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
                leaves_source: false,
            };
            let priced_mana_cost = self.priced_ability_mana_cost(source, &definition);
            let is_cycling = definition.cycling;
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(cost) => {
                        let cost = priced_mana_cost.unwrap_or(*cost);
                        let cost = self.announced_activation_cost(player, cost, mana_payment);
                        self.activate_mana_for_cost_avoiding_for(
                            player,
                            cost,
                            x,
                            None,
                            &payment_purpose,
                        );
                        let _ = self.pay_player_cost_for(player, cost, x, &payment_purpose);
                    }
                    AbilityCostDef::ManaCostOf(_) => {
                        unreachable!("hand abilities cannot price another chosen card")
                    }
                    AbilityCostDef::DiscardSource => {
                        let discarded = remove_card(&mut self.players[player.index()].hand, source)
                            .expect("a legal hand activation still has its source");
                        let definition = discarded.definition;
                        let (discarded, _zone_change) = self.zone_change_card(discarded);
                        let discarded_id = discarded.id;
                        self.put_card_into_graveyard(player, discarded);
                        self.events.push(GameEvent::CardsDiscarded {
                            player,
                            cards: vec![(discarded_id, definition)],
                        });
                        // A discard paid as a cost is still a discard, so
                        // what watches for one sees this too.
                        let card = self.printed_trigger_event_object(
                            discarded_id,
                            definition,
                            player,
                            &CharacteristicContext::Graveyard,
                        );
                        self.capture_battlefield_triggers(&CommittedTriggerEvent::Discarded {
                            player,
                            card,
                        });
                        self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsDiscarded {
                            player,
                        });
                        // CR 702.29b fires cycling's trigger on activation
                        // rather than on resolution, so it belongs here
                        // beside the cost rather than at the draw. Only
                        // cycling raises it: channel pays the very same way
                        // and is a different keyword.
                        if is_cycling {
                            self.capture_cycling_triggers(discarded_id, player);
                        }
                    }
                    // The attacker named by the action, returned before the
                    // ability goes on the stack: it is a cost.
                    AbilityCostDef::ReturnUnblockedAttackerToHand => {
                        if let Some(returned) = cost_objects.first() {
                            self.ninjutsu_returned_defender = self.attack_defender_of(*returned);
                            self.move_target_to_zone(
                                Target::Permanent(*returned),
                                ZoneKind::Hand,
                                ZoneMoveCause::Effect { controller: player },
                                None,
                                ZonePlacement::Top,
                            );
                        }
                    }
                    AbilityCostDef::TapSource
                    | AbilityCostDef::ExertSource
                    | AbilityCostDef::DiscardHand
                    | AbilityCostDef::UntapSource
                    | AbilityCostDef::SacrificeSource
                    | AbilityCostDef::SacrificeObject(_)
                    | AbilityCostDef::ReturnSourceToHand
                    | AbilityCostDef::RemoveCountersFromSource { .. }
                    | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                    | AbilityCostDef::PayLife(_)
                    | AbilityCostDef::MillCards(_)
                    | AbilityCostDef::DiscardCards(_)
                    | AbilityCostDef::DiscardCardMatching(_)
                    | AbilityCostDef::ExileCardFromHand(_)
                    | AbilityCostDef::DiscardCardsAtRandom(_)
                    | AbilityCostDef::SacrificePermanent { .. }
                    | AbilityCostDef::SacrificePermanents { .. }
                    | AbilityCostDef::TapPermanent { .. }
                    | AbilityCostDef::TapCreaturesWithTotalPower { .. }
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::Loyalty(_)
                    | AbilityCostDef::MoveToZone(_)
                    | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported hand-zone costs are not offered")
                    }
                }
            }
            let chosen_permanents = targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect();
            self.push_activated_ability(
                source,
                &source_card.clone().into(),
                player,
                frozen,
                targets,
                chosen_permanents,
            );
            self.consecutive_passes = 0;
            self.check_state_based_actions();
            return;
        }
        if self.activate_exile_ability(
            player,
            source,
            ability,
            targets.clone(),
            cost_objects,
            x,
            modes,
        ) {
            return;
        }
        if let Some(source_card) = self.players[player.index()]
            .graveyard
            .iter()
            .find(|card| card.id == source)
            .cloned()
        {
            let choices = ActivationChoices {
                targets,
                cost_objects,
                x,
                modes,
                mana_payment,
            };
            self.activate_graveyard_ability(player, source, ability, choices, &source_card);
            return;
        }
        let Some(source_permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        let source_card = source_permanent.card.clone();
        let mut frozen_ability = self.freeze_activated_ability(source_permanent, ability);
        frozen_ability.x = x;
        // A modal ability's slots are its own followed by each chosen
        // mode's, and the chosen modes' effects resolve after its own.
        if let Some(DeclarativeAbilityDef::Activated(definition)) = self
            .find_effective_ability(source_permanent, |effective| effective.origin == ability)
            .map(|effective| effective.ability.definition)
        {
            let Some(plan) = Self::selected_activated_plan(&definition, modes) else {
                return;
            };
            frozen_ability.target_defs = plan.target_defs;
            frozen_ability.mode_effects = plan.mode_effects;
        }
        if targets.len() < frozen_ability.target_defs.len() {
            self.begin_deferred_activation_targeting(PendingActivationTargeting {
                controller: player,
                source,
                ability,
                target_defs: frozen_ability.target_defs.clone(),
                targets,
                cost_objects: cost_objects.to_vec(),
                x,
                modes: modes.to_vec(),
                mana_payment: mana_payment.cloned().map(Box::new),
            });
            self.consecutive_passes = 0;
            return;
        }
        // `apply` validated these exact ordered slot selections against a
        // generated legal action. Freeze both their slot identity and values
        // before any activation cost can move or change the source.
        let frozen_targets = targets;
        let selected_ability = self
            .find_effective_ability(source_permanent, |effective| effective.origin == ability)
            .map(|effective| effective.ability);
        let declarative = selected_ability.filter(|ability| {
            ability.is_executable()
                && matches!(
                    ability.definition,
                    DeclarativeAbilityDef::Activated(definition)
                        if definition.procedure == AbilityProcedureDef::Shared
                )
        });
        let behavior = selected_ability.and_then(|ability| match ability.definition {
            DeclarativeAbilityDef::Activated(definition)
                if definition.procedure == AbilityProcedureDef::Legacy =>
            {
                ability.custom_behavior()
            }
            DeclarativeAbilityDef::Legacy => ability.custom_behavior(),
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::ActivatedMana(_)
            | DeclarativeAbilityDef::TriggeredMana(_)
            | DeclarativeAbilityDef::Activated(_)
            | DeclarativeAbilityDef::Triggered(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::AlternativeCast(_)
            | DeclarativeAbilityDef::OptionalAdditionalCost(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Pregame(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::DeckConstruction(_) => None,
        });
        if let Some(ability_def) = declarative {
            let DeclarativeAbilityDef::Activated(definition) = ability_def.definition else {
                unreachable!("the declarative activation filter checked its category")
            };
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            let fixed_sacrifices = definition
                .costs
                .iter()
                .filter_map(|cost| {
                    let AbilityCostDef::SacrificeObject(reference) = cost else {
                        return None;
                    };
                    Self::activation_object_reference(*reference, source, frozen_ability.origin)
                })
                .collect::<Vec<_>>();
            let leaves_source = definition.costs.iter().any(|cost| {
                matches!(
                    cost,
                    AbilityCostDef::SacrificeSource | AbilityCostDef::ExileSource
                )
            }) || fixed_sacrifices.contains(&source);
            let animates_source = Self::effect_animates_source(ability_def.declarative_effect());
            let has_generic_sacrifice = definition
                .costs
                .iter()
                .any(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }));
            let sacrifice_choice_is_source =
                has_generic_sacrifice && cost_objects.contains(&source);
            let tap_cost_payer = if definition
                .costs
                .iter()
                .any(|cost| matches!(cost, AbilityCostDef::TapPermanent { .. }))
            {
                cost_objects.first().copied()
            } else {
                None
            };
            if definition.costs.iter().any(|cost| {
                matches!(
                    cost,
                    AbilityCostDef::ReturnUnblockedAttackerToHand
                        | AbilityCostDef::TapPermanent { .. }
                )
            }) {
                // Ahead of the loop, so automatic mana payment cannot tap the
                // chosen permanent out from under the cost it is paying.
                let chosen = *cost_objects
                    .first()
                    .expect("a legal activation chose the one to tap");
                let _ = self.tap_permanent(chosen);
            }
            let has_linked_card_mana = definition
                .costs
                .iter()
                .any(|cost| matches!(cost, AbilityCostDef::ManaCostOf(_)));
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(_) if has_linked_card_mana => {}
                    AbilityCostDef::Mana(cost) => {
                        // Read through any increase on the battlefield and
                        // any discount, printed or granted, so what is paid
                        // is what the offer was priced at.
                        let cost = self.activation_mana_cost(&definition, source, *cost);
                        let cost = self.announced_activation_cost(player, cost, mana_payment);
                        let payment_purpose = ManaPaymentPurpose::Ability {
                            source,
                            taps_source,
                            leaves_source,
                        };
                        self.activate_mana_for_cost_with_options_for(
                            player,
                            cost,
                            x,
                            ManaPlanOptions {
                                // Tapping the source to pay would hand back a
                                // tapped creature, so auto-payment leaves it
                                // alone even though the tap itself is legal.
                                avoid: (taps_source || animates_source).then_some(source),
                                tap_cost_payer,
                            },
                            &payment_purpose,
                        );
                        // The same purpose the mana was raised under. Paying
                        // under a different one would price the cost
                        // differently from the offer it came from.
                        let _ = self.pay_player_cost_for(player, cost, x, &payment_purpose);
                    }
                    AbilityCostDef::ManaCostOf(_) => {
                        let cost = self
                            .activated_ability_mana_cost_for(&definition, cost_objects)
                            .map(|cost| self.activation_mana_cost(&definition, source, cost))
                            .expect("a legal linked card-mana activation has its chosen card");
                        let payment_purpose = ManaPaymentPurpose::Ability {
                            source,
                            taps_source,
                            leaves_source,
                        };
                        self.activate_mana_for_cost_with_options_for(
                            player,
                            cost,
                            x,
                            ManaPlanOptions {
                                avoid: (taps_source || animates_source).then_some(source),
                                tap_cost_payer,
                            },
                            &payment_purpose,
                        );
                        let _ = self.pay_player_cost_for(player, cost, x, &payment_purpose);
                    }
                    AbilityCostDef::TapSource => {
                        let _ = self.tap_permanent(source);
                    }
                    // Exerting costs nothing now: what it spends is one
                    // untap step, owed and paid whenever that step next
                    // comes around.
                    AbilityCostDef::ExertSource => {
                        if let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                        {
                            permanent.exerted = true;
                            permanent.skipped_untap_steps =
                                permanent.skipped_untap_steps.saturating_add(1);
                        }
                    }
                    AbilityCostDef::UntapSource => {
                        self.battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                            .expect("a legal activation has its source")
                            .untap();
                    }
                    // The open-ended removal never reaches payment: mana
                    // enumeration replaced it with a sized one.
                    AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                    | AbilityCostDef::ReturnUnblockedAttackerToHand
                    | AbilityCostDef::TapPermanent { .. }
                    // Paid by decision after everything else, the way a
                    // multiple sacrifice is.
                    | AbilityCostDef::TapCreaturesWithTotalPower { .. }
                    | AbilityCostDef::SacrificeSource
                    | AbilityCostDef::SacrificeObject(_)
                    | AbilityCostDef::ReturnSourceToHand
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::SacrificePermanent { .. }
                    | AbilityCostDef::SacrificePermanents { .. } => {
                        // A tap of a chosen permanent was paid above, ahead of
                        // mana. The rest are deferred until mana and
                        // source-dependent costs have been paid: a chosen
                        // permanent may itself produce mana first, and the
                        // source may still owe a tap or counter-removal cost
                        // before it leaves.
                    }
                    AbilityCostDef::RemoveCountersFromSource { kind, amount } => {
                        self.battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                            .expect("a legal activation has its source")
                            .remove_counters(*kind, *amount);
                    }
                    AbilityCostDef::DiscardSource => {
                        unreachable!("a battlefield source cannot discard itself")
                    }
                    AbilityCostDef::PayLife(amount) => {
                        self.lose_life(player, *amount);
                    }
                    AbilityCostDef::DiscardCardMatching(_) => {
                        self.discard_cards(player, cost_objects);
                    }
                    AbilityCostDef::ExileCardFromHand(_) => {
                        for chosen in cost_objects {
                            if let Some(card) =
                                remove_card(&mut self.players[player.index()].hand, *chosen)
                            {
                                let (card, _zone_change) = self.zone_change_card(card);
                                self.players[player.index()].exile.push(card.clone());
                                self.capture_cards_exiled(
                                    std::slice::from_ref(&card),
                                    crate::card::ZoneKind::Hand,
                                );
                            }
                        }
                    }
                    // The cost names as many cards as it prints, and the
                    // activation carried every one of them.
                    AbilityCostDef::MoveToZone(movement) => {
                        self.pay_nonbattlefield_move_cost(player, *movement, cost_objects);
                    }
                    AbilityCostDef::Loyalty(change) => {
                        self.pay_loyalty_cost(source, *change);
                    }
                    AbilityCostDef::DiscardCardsAtRandom(amount) => {
                        self.discard_at_random(player, usize::from(*amount));
                    }
                    AbilityCostDef::DiscardHand => {
                        let hand = self.players[player.index()]
                            .hand
                            .iter()
                            .map(|card| card.id)
                            .collect::<Vec<_>>();
                        self.discard_cards(player, &hand);
                    }
                    AbilityCostDef::MillCards(amount) => {
                        let milled = self.take_top_of_library(player, usize::from(*amount));
                        assert_eq!(
                            milled.len(),
                            usize::from(*amount),
                            "a legal activation can still pay its mill cost",
                        );
                        self.bury_cards(player, milled);
                    }
                    AbilityCostDef::DiscardCards(_) | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported costs are not offered as legal actions")
                    }
                }
            }
            let mut remaining_sacrifices = Vec::new();
            if has_generic_sacrifice {
                remaining_sacrifices.extend(
                    cost_objects
                        .iter()
                        .copied()
                        .filter(|chosen| *chosen != source),
                );
            }
            for sacrificed in fixed_sacrifices {
                if !remaining_sacrifices.contains(&sacrificed) {
                    remaining_sacrifices.push(sacrificed);
                }
            }
            // Read while they are all still on the battlefield, which is the
            // last moment anything can: the ability resolves long after they
            // have gone, and what it owes is measured by what paid for it.
            frozen_ability.sacrificed_mana_value = remaining_sacrifices
                .iter()
                .filter_map(|sacrificed| self.current_or_last_known_mana_value(*sacrificed))
                .fold(0u16, u16::saturating_add);
            if definition.costs.contains(&AbilityCostDef::ExileSource) {
                self.exile_permanent(source);
            } else if definition
                .costs
                .contains(&AbilityCostDef::ReturnSourceToHand)
            {
                // The source leaves the battlefield to pay, the way a
                // sacrifice does, but it goes somewhere it can be cast from
                // again.
                self.return_permanent_to_hand(source);
            } else if definition.costs.contains(&AbilityCostDef::SacrificeSource)
                || sacrifice_choice_is_source
            {
                remaining_sacrifices.push(source);
            }
            let mut chosen_permanents = frozen_targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect::<Vec<_>>();
            for chosen in cost_objects {
                if !chosen_permanents.contains(chosen) {
                    chosen_permanents.push(*chosen);
                }
            }
            // Crew and saddle name the creatures that pay by decision too,
            // and the total they owe is what the offer counts down.
            if let Some(AbilityCostDef::TapCreaturesWithTotalPower { minimum }) = definition
                .costs
                .iter()
                .find(|cost| matches!(cost, AbilityCostDef::TapCreaturesWithTotalPower { .. }))
            {
                self.queue_activation_saddle(
                    player,
                    i32::from(*minimum),
                    PendingActivation {
                        source,
                        source_card,
                        controller: player,
                        frozen: frozen_ability,
                        targets: frozen_targets,
                        chosen_permanents,
                        remaining_sacrifices,
                    },
                    Vec::new(),
                );
                self.consecutive_passes = 0;
                return;
            }
            // A cost that takes a printed number of permanents names them
            // by decision, so the activation waits here with everything it
            // has already chosen and paid.
            if let Some(AbilityCostDef::SacrificePermanents {
                object,
                controller,
                count,
            }) = definition
                .costs
                .iter()
                .find(|cost| matches!(cost, AbilityCostDef::SacrificePermanents { .. }))
            {
                self.queue_activation_sacrifice(
                    player,
                    SacrificeQuota {
                        remaining: *count,
                        object: *object,
                        controller: *controller,
                    },
                    PendingActivation {
                        source,
                        source_card,
                        controller: player,
                        frozen: frozen_ability,
                        targets: frozen_targets,
                        chosen_permanents,
                        remaining_sacrifices,
                    },
                    Vec::new(),
                );
                self.consecutive_passes = 0;
                return;
            }
            self.continue_activated_ability_costs(
                source,
                source_card,
                player,
                frozen_ability,
                frozen_targets,
                chosen_permanents,
                remaining_sacrifices,
            );
            return;
        }
        if let Some(CardBehavior::LibraryOfAlexandria) = behavior {
            let card = self
                .tap_permanent(source)
                .expect("legal activation has a source");
            self.push_activated_ability(
                source,
                &card,
                player,
                frozen_ability,
                frozen_targets,
                Vec::new(),
            );
        }
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn continue_activated_ability_costs(
        &mut self,
        source: GameObjectId,
        source_card: ObjectInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
        mut remaining_sacrifices: Vec<GameObjectId>,
    ) {
        if !remaining_sacrifices.is_empty() {
            let sacrificed = remaining_sacrifices.remove(0);
            self.capture_sacrifices(&[sacrificed]);
            self.move_permanents_to_graveyard_then(
                &[sacrificed],
                Some(BattlefieldExitCompletion::CompleteActivatedAbility {
                    source,
                    source_card,
                    controller,
                    frozen,
                    targets,
                    chosen_permanents,
                    remaining_sacrifices,
                }),
            );
            return;
        }

        self.push_activated_ability(
            source,
            &source_card,
            controller,
            frozen,
            targets,
            chosen_permanents,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }
}
