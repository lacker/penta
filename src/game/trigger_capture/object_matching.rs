impl Game {
    fn definition_has_ability(
        &self,
        definition: CardDefinitionId,
        context: &CharacteristicContext,
        predicate: AbilityPredicateDef,
    ) -> bool {
        let Some(definition) = self.catalog.get(definition) else {
            return false;
        };
        let Ok(parts) = crate::card::applicable_part_ids(definition, context) else {
            return false;
        };
        parts.into_iter().any(|part| {
            definition.part(part).is_some_and(|part| {
                part.rules
                    .ability_clauses()
                    .iter()
                    .any(|ability| predicate.matches(ability))
            })
        })
    }

    pub(super) fn object_has_ability(
        &self,
        object: GameObjectId,
        predicate: AbilityPredicateDef,
    ) -> bool {
        // A face-down exiled card has no abilities, even when its owner can
        // look at it and knows that its front face has a matching ability.
        if self.exiled_card_is_face_down(object) {
            return false;
        }

        if self
            .nonbattlefield_ability_grants
            .iter()
            .any(|grant| grant.object == object && predicate.matches(&grant.ability))
        {
            return true;
        }

        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return self
                .effective_abilities(permanent)
                .into_iter()
                .any(|ability| predicate.matches(&ability.ability));
        }

        if let Some(stack) = self.stack.iter().find(|stack| stack.id == object) {
            let Some(signature) = &stack.signature else {
                return false;
            };
            let Some(definition) = stack.presentation().card_definition() else {
                return false;
            };
            return self.definition_has_ability(
                definition,
                &CharacteristicContext::Stack {
                    form: signature.form().clone(),
                },
                predicate,
            );
        }

        let Some((zone, card)) = self.card_in_nonbattlefield_zone(object) else {
            return false;
        };
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return false,
        };
        self.definition_has_ability(card.definition, &context, predicate)
    }

    fn trigger_event_object_reference(
        &self,
        reference: ObjectRefDef,
        ability_source: GameObjectId,
        event: &CommittedTriggerEvent,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => Some(ability_source),
            ObjectRefDef::AttachedToSource => {
                self.current_or_last_known_attached_host(ability_source)
            }
            ObjectRefDef::TriggeringObject => event.context().object,
            ObjectRefDef::DamagedObject => event.context().damaged_object,
            ObjectRefDef::CreatingSource => self.creating_source_of(ability_source),
            ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::ResolvingObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_) => None,
        }
    }

    /// Whether a card's owner stands in this relation to the asker. Owning is
    /// not controlling: a stolen permanent goes back to the hand of whoever
    /// it came from, so the owner is looked up wherever the card presently is
    /// rather than read off whoever has it.
    fn object_owner_matches(
        &self,
        object: GameObjectId,
        relation: PlayerRelation,
        controller: Option<PlayerId>,
        source: GameObjectId,
    ) -> bool {
        let owner = self.current_or_last_known_owner(object);
        owner.zip(controller).is_some_and(|(owner, controller)| {
            self.player_relation_matches_for_source(
                owner,
                relation,
                controller,
                source,
                TriggerContext::empty(),
            )
        })
    }

    /// Whether `object` satisfies `predicate`. `source` is the ability's own
    /// object, which is what a controller relation is measured against.
    /// The predicates comparing a stat against a value read off the ability's
    /// own source. They share a shape, so they share a body.
    /// The stat comparisons whose limit is a printed number rather than a
    /// value read off the ability's source. Every one of them reads the
    /// object live, so anything that pumps a creature moves it in or out of
    /// range as it goes.
    fn printed_stat_matches(predicate: ObjectPredicateDef, object: &TriggerEventObject) -> bool {
        match predicate {
            ObjectPredicateDef::PowerAtLeast(minimum) => {
                object.power.is_some_and(|power| power >= minimum)
            }
            ObjectPredicateDef::PowerExactly(exact) => object.power == Some(exact),
            ObjectPredicateDef::ToughnessExactly(exact) => object.toughness == Some(exact),
            ObjectPredicateDef::TotalPowerAndToughnessAtMost(limit) => object
                .power
                .zip(object.toughness)
                .is_some_and(|(power, toughness)| power.saturating_add(toughness) <= limit),
            _ => false,
        }
    }

    fn computed_stat_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
    ) -> bool {
        let (value, stat, greater) = match predicate {
            ObjectPredicateDef::ToughnessLessThan(value) => (value, object.toughness, false),
            ObjectPredicateDef::PowerGreaterThan(value) => (value, object.power, true),
            ObjectPredicateDef::PowerLessThan(value) => (value, object.power, false),
            ObjectPredicateDef::ToughnessGreaterThan(value) => (value, object.toughness, true),
            _ => return false,
        };
        self.value_from_source(value, source)
            .zip(stat)
            .is_some_and(|(limit, stat)| {
                if greater {
                    i32::from(stat) > limit
                } else {
                    i32::from(stat) < limit
                }
            })
    }

    /// The same fact summoning sickness reads: the permanent has not been
    /// under its controller since their turn began. Read from the battlefield
    /// rather than the event object, because it is about how long the
    /// permanent has been sitting there.
    fn came_under_control_this_turn(&self, object: GameObjectId) -> bool {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .is_some_and(|permanent| {
                self.turns_started[permanent.controller.index()]
                    <= permanent.entered_controller_turn
            })
    }

    /// The predicates answered by looking at the battlefield rather than at
    /// the object's own recorded characteristics.
    fn battlefield_relationship_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::HasNonManaActivatedAbility => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| self.has_nonmana_activated_ability(permanent)),
            // A fact about the permanent's turn rather than a characteristic
            // frozen with the event, so it is read live off the battlefield.
            ObjectPredicateDef::WasDealtDamageThisTurn => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| permanent.was_dealt_damage_this_turn),
            ObjectPredicateDef::DealtDamageThisTurn => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| permanent.dealt_damage_this_turn),
            ObjectPredicateDef::Unpaired => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| permanent.paired_with.is_none()),
            // Symmetric, so it reads the same from either side.
            ObjectPredicateDef::PairedWithSource => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .is_some_and(|permanent| permanent.paired_with == Some(object.id)),
            ObjectPredicateDef::AttachedToSource => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .and_then(|permanent| permanent.attached_to)
                .is_some_and(|host| host == object.id),
            // Read from the source: the Wall knows what it blocked, and the
            // attacker's own record does not name its blockers.
            // Last-known, because a creature that died in combat still knows
            // what it had blocked and its death trigger has to read it.
            ObjectPredicateDef::BlockedBySource => self
                .current_or_last_known_blocking(source)
                .is_some_and(|attacker| attacker == object.id),
            // The other direction, read from the candidate: a blocker records
            // what it blocked, so this one needs no lookup on the source.
            ObjectPredicateDef::BlockingSource => self
                .battlefield
                .iter()
                .find(|candidate| candidate.card.id == object.id)
                .is_some_and(|candidate| candidate.is_blocking(source)),
            // Band membership is symmetric, so it reads the same from either
            // side; excluding the source is what makes "banded with it" mean
            // the others rather than the whole band.
            ObjectPredicateDef::BandedWithSource => {
                object.id != source && self.share_a_band(source, object.id)
            }
            ObjectPredicateDef::Enchanted => self.battlefield.iter().any(|candidate| {
                candidate.attached_to == Some(object.id) && self.is_aura_permanent(candidate)
            }),
            // The Aura's own side of the question: what is it on?
            ObjectPredicateDef::AttachedTo(predicate) => self
                .battlefield
                .iter()
                .find(|candidate| candidate.card.id == object.id)
                .and_then(|candidate| candidate.attached_to)
                .and_then(|host| {
                    self.battlefield
                        .iter()
                        .find(|candidate| candidate.card.id == host)
                })
                .is_some_and(|host| {
                    self.trigger_object_matches_for_controller(
                        *predicate,
                        &self.trigger_event_object(host),
                        source,
                        false,
                        controller,
                    )
                }),
            _ => unreachable!("only the battlefield-reading predicates arrive here"),
        }
    }

    pub(super) fn trigger_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        is_spell: bool,
    ) -> bool {
        self.trigger_object_matches_for_controller(
            predicate,
            object,
            source,
            is_spell,
            self.controller_of_object(source),
        )
    }

    /// The three predicates built from other predicates, split out so the
    /// flat dispatch below stays one screen of enum arms.
    fn composite_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        is_spell: bool,
        controller: Option<PlayerId>,
    ) -> bool {
        let mut matches = |predicate: &ObjectPredicateDef| {
            self.trigger_object_matches_for_controller(
                *predicate, object, source, is_spell, controller,
            )
        };
        match predicate {
            ObjectPredicateDef::All(predicates) => predicates.iter().all(&mut matches),
            ObjectPredicateDef::AnyOf(predicates) => predicates.iter().any(&mut matches),
            ObjectPredicateDef::Not(predicate) => !matches(predicate),
            _ => unreachable!("only the composite predicates arrive here"),
        }
    }

    /// The predicates that compare an object against a name, a chosen scalar,
    /// or the targets something else already has.
    ///
    /// A source with no choice recorded matches nothing rather than
    /// everything: Meddling Mage's lock and Engineered Plague's shrink both
    /// key on a name or type that a permanent which never made its entry
    /// choice simply does not have.
    fn indirect_predicate_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Named(name) => self
                .object_card_name(object.id)
                .is_some_and(|actual| actual == name),
            // The chosen name lives in the resolution that chose it, which
            // nothing here can see, so this matches nothing rather than
            // everything. The one effect that reads it does so itself.
            ObjectPredicateDef::HasChosenName => false,
            ObjectPredicateDef::TargetsObjectMatching(predicate) => {
                self.stack_object_targets_match(object.id, *predicate, source, controller)
            }
            ObjectPredicateDef::HasSourcesChosenScalar(destination) => {
                self.matches_chosen_scalar(destination, object, source)
            }
            _ => unreachable!("only the three indirect predicates arrive here"),
        }
    }

    /// Whether a spell or ability on the stack already targets something
    /// matching. Read off the targets it chose, not the ones it could have
    /// taken: "that targets a land you control" is about the object as it
    /// sits on the stack.
    fn stack_object_targets_match(
        &self,
        object: GameObjectId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        self.stack
            .iter()
            .find(|candidate| candidate.id == object)
            .is_some_and(|stack_object| {
                stack_object.iter_targets().any(|target| {
                    let Target::Permanent(id) = target else {
                        return false;
                    };
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                        .is_some_and(|permanent| {
                            // Carried through rather than re-derived from the
                            // source: the source of a spell being targeted is
                            // still a card in hand, and "a land you control"
                            // is measured from whoever is doing the asking.
                            self.trigger_object_matches_for_controller(
                                predicate,
                                &self.trigger_event_object(permanent),
                                source,
                                false,
                                controller,
                            )
                        })
                })
            })
    }

    /// Whether `object` answers to the scalar `source` chose as it entered.
    ///
    /// A source that never made its choice matches nothing: Meddling Mage's
    /// lock and Engineered Plague's shrink both key on a specific answer, and
    /// a permanent with none of its own would otherwise match everything.
    fn matches_chosen_scalar(
        &self,
        destination: BattlefieldEntryChoiceDestinationDef,
        object: &TriggerEventObject,
        source: GameObjectId,
    ) -> bool {
        let chooser = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source);
        match destination {
            // Player relations and land types have dedicated predicates;
            // neither is matched as a scalar against an object here.
            BattlefieldEntryChoiceDestinationDef::Player
            | BattlefieldEntryChoiceDestinationDef::BasicLandType
            | BattlefieldEntryChoiceDestinationDef::Color => false,
            BattlefieldEntryChoiceDestinationDef::CardName => chooser
                .and_then(|permanent| permanent.chosen_card_name.as_deref())
                .is_some_and(|chosen| {
                    self.object_card_name(object.id)
                        .is_some_and(|actual| actual == chosen)
                }),
            BattlefieldEntryChoiceDestinationDef::CreatureType => chooser
                .and_then(|permanent| permanent.chosen_creature_type.as_deref())
                .is_some_and(|chosen| {
                    object
                        .subtypes
                        .iter()
                        .any(|subtype| subtype.eq_ignore_ascii_case(chosen))
                }),
        }
    }

    // Long because the predicate vocabulary is wide, not because the
    // function does several things: every arm reads one property of one
    // object. It is a table, and a table only grows.
    #[allow(clippy::too_many_lines)]
    pub(in crate::game) fn trigger_object_matches_for_controller(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        is_spell: bool,
        controller: Option<PlayerId>,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Any => true,
            ObjectPredicateDef::Source => object.id == source,
            ObjectPredicateDef::Token => object.token,
            ObjectPredicateDef::Saddled => object.saddled,
            ObjectPredicateDef::HasType(card_type) => object.types.contains(card_type),
            ObjectPredicateDef::HasAnyBasicLandType(land_types) => {
                object.types.contains(CardType::Land)
                    && land_types
                        .iter()
                        .any(|land_type| object.subtypes.contains(&land_type.subtype()))
            }
            ObjectPredicateDef::Spell => is_spell,
            ObjectPredicateDef::NoncreatureSpell => {
                is_spell && !object.types.contains(CardType::Creature)
            }
            ObjectPredicateDef::Color(color) => color
                .color_index()
                .is_some_and(|index| object.colors[index]),
            ObjectPredicateDef::ColorCount(count) => {
                object.colors.iter().filter(|present| **present).count() == usize::from(count)
            }
            ObjectPredicateDef::Subtype(subtype) => object.subtypes.contains(&subtype),
            ObjectPredicateDef::ManaValueAtMost(limit) => object.mana_value <= u16::from(limit),
            ObjectPredicateDef::ManaValueEqualTo(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| value == i32::from(object.mana_value)),
            ObjectPredicateDef::ManaValueAtMostValue(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| i32::from(object.mana_value) <= value),
            ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
            | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_) => {
                Self::printed_stat_matches(predicate, object)
            }
            ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::PowerGreaterThan(_)
            | ObjectPredicateDef::PowerLessThan(_)
            | ObjectPredicateDef::ToughnessGreaterThan(_) => {
                self.computed_stat_matches(predicate, object, source)
            }
            // One object compared with itself: a 0/3 qualifies and a 3/3
            // does not, whatever anything else on the board says.
            ObjectPredicateDef::ToughnessGreaterThanItsPower => object
                .power
                .zip(object.toughness)
                .is_some_and(|(power, toughness)| toughness > power),
            ObjectPredicateDef::Supertype(supertype) => object.supertypes[supertype.index()],
            // Read from the definition rather than the object: what matters
            // is where the card was first printed, not what it has become.
            ObjectPredicateDef::DebutSet(set) => self
                .object_debut_set(object.id)
                .is_some_and(|debut| debut == set),
            ObjectPredicateDef::AttackingOrBlocking => object.attacking_or_blocking,
            ObjectPredicateDef::HasName(ObjectRefDef::Source) => {
                let name = self.object_card_name(object.id);
                name.is_some() && name == self.object_card_name(source)
            }
            // The four that read a value from somewhere other than the
            // object's own characteristics: a printed name, a name chosen in
            // a resolution, a scalar the source chose on entry, or the
            // targets something else has.
            ObjectPredicateDef::Named(_)
            | ObjectPredicateDef::HasChosenName
            | ObjectPredicateDef::TargetsObjectMatching(_)
            | ObjectPredicateDef::HasSourcesChosenScalar(_) => {
                self.indirect_predicate_matches(predicate, object, source, controller)
            }
            ObjectPredicateDef::HasKeyword(keyword) => keyword
                .simple_index()
                .is_some_and(|index| object.keywords & (1 << index) != 0),
            ObjectPredicateDef::HasAbility(ability) => {
                self.object_has_ability(object.id, ability)
            }
            // Counters are permanent state rather than a characteristic, so
            // reading them live cannot feed back into the layer being
            // computed the way a keyword or a stat could.
            ObjectPredicateDef::HasCounter(kind) => {
                self.current_or_last_known_counters(object.id, kind) > 0
            }
            ObjectPredicateDef::HasAnyCounter => {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == object.id)
                    .is_some_and(|permanent| !permanent.counters.is_empty())
                    || self
                        .card_in_nonbattlefield_zone(object.id)
                        .is_some_and(|(_, card)| !card.counters.is_empty())
            }
            // The same reading with a bound on it, which is what a level
            // band asks: at least this many, or fewer than that many.
            ObjectPredicateDef::CounterCount {
                kind,
                comparison,
                amount,
            } => crate::game::effect_support::compare(
                &self.current_or_last_known_counters(object.id, kind),
                comparison,
                &u16::from(amount),
            ),
            ObjectPredicateDef::OwnedBy(relation) => {
                self.object_owner_matches(object.id, relation, controller, source)
            }
            ObjectPredicateDef::ControlledBy(relation) => controller.is_some_and(|controller| {
                self.player_relation_matches_for_source(
                    object.controller,
                    relation,
                    controller,
                    source,
                    TriggerContext::empty(),
                )
            }),
            ObjectPredicateDef::Attacking
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::AttackedDuringControllersLastTurn
            | ObjectPredicateDef::Blocking => combat_state_matches(predicate, object),
            ObjectPredicateDef::CameUnderControlThisTurn => {
                self.came_under_control_this_turn(object.id)
            }
            ObjectPredicateDef::EnteredThisTurn => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| permanent.entered_turn == self.turn),
            ObjectPredicateDef::AttachedToSource => self
                .current_or_last_known_attached_host(source)
                .is_some_and(|host| host == object.id),
            ObjectPredicateDef::HasNonManaActivatedAbility
            | ObjectPredicateDef::BlockedBySource
            | ObjectPredicateDef::BlockingSource
            | ObjectPredicateDef::BandedWithSource
            | ObjectPredicateDef::Enchanted
            | ObjectPredicateDef::WasDealtDamageThisTurn
            | ObjectPredicateDef::DealtDamageThisTurn
            | ObjectPredicateDef::Unpaired
            | ObjectPredicateDef::PairedWithSource
            | ObjectPredicateDef::AttachedTo(_) => {
                self.battlefield_relationship_matches(predicate, object, source, controller)
            }
            ObjectPredicateDef::Tapped => object.tapped,
            ObjectPredicateDef::All(_)
            | ObjectPredicateDef::AnyOf(_)
            | ObjectPredicateDef::Not(_) => {
                self.composite_matches(predicate, object, source, is_spell, controller)
            }
            ObjectPredicateDef::HasName(_) | ObjectPredicateDef::Special(_) => false,
        }
    }

    pub(super) fn player_relation_matches(
        &self,
        player: PlayerId,
        relation: PlayerRelation,
        controller: PlayerId,
        context: TriggerContext,
    ) -> bool {
        match relation {
            PlayerRelation::Any => true,
            PlayerRelation::You => player == controller,
            PlayerRelation::NotYou => player != controller,
            PlayerRelation::Opponent => player == controller.opponent(),
            PlayerRelation::ActivePlayer => player == self.active_player,
            PlayerRelation::NonactivePlayer => player == self.active_player.opponent(),
            PlayerRelation::EventPlayer => context.event_player == Some(player),
            PlayerRelation::NotEventPlayer => {
                context.event_player.is_some_and(|event| event != player)
            }
            // Both of these live on the ability's source, which this does not
            // have. The triggers that name them resolve the relation where
            // the source is known.
            PlayerRelation::ChosenPlayer
            | PlayerRelation::DefendingPlayer
            | PlayerRelation::ControllerOfAttachedPermanent
            | PlayerRelation::EnchantedPlayer => false,
        }
    }

    /// Relations whose answer is recorded on an ability source rather than
    /// supplied by an event. Ordinary relations still share the central
    /// matcher; this wrapper supplies the two source-local cases to object
    /// predicates such as protection from the chosen player.
    pub(super) fn player_relation_matches_for_source(
        &self,
        player: PlayerId,
        relation: PlayerRelation,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> bool {
        match relation {
            PlayerRelation::ChosenPlayer => self.chosen_player_of(source) == Some(player),
            PlayerRelation::DefendingPlayer => self.defending_player_of(source) == Some(player),
            PlayerRelation::ControllerOfAttachedPermanent => {
                self.attached_host_controller_of(source) == Some(player)
            }
            PlayerRelation::EnchantedPlayer => {
                self.current_or_last_known_enchanted_player(source) == Some(player)
            }
            _ => self.player_relation_matches(
                player,
                relation,
                controller,
                context,
            ),
        }
    }

    /// Whoever controls what this permanent is attached to. An Aura that has
    /// come loose is attached to nothing and so matches nobody.
    pub(super) fn attached_host_controller_of(&self, source: GameObjectId) -> Option<PlayerId> {
        self.current_or_last_known_attached_host(source)
            .and_then(|host| self.current_or_last_known_controller(host))
    }

    /// The player a permanent chose as it entered.
    pub(super) fn chosen_player_of(&self, source: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_player)
    }
}

/// Combat facts about a creature, all read straight off the snapshot.
///
/// They are grouped because they answer four versions of one question and
/// each is worthless for anything that is not a creature: an artifact is
/// never "attacking", whatever else is true of it.
fn combat_state_matches(predicate: ObjectPredicateDef, object: &TriggerEventObject) -> bool {
    object.types.contains(CardType::Creature)
        && match predicate {
            ObjectPredicateDef::Attacking => object.attacking,
            ObjectPredicateDef::Saddled => object.saddled,
            // Still attacking is not the question: this asks whether the
            // creature attacked at any point this turn, which is what an
            // end-step check has to read once combat is over.
            ObjectPredicateDef::AttackedThisTurn => object.attacked_this_turn,
            ObjectPredicateDef::AttackedDuringControllersLastTurn => {
                object.attacked_during_controllers_last_turn
            }
            ObjectPredicateDef::Blocking => object.attacking_or_blocking && !object.attacking,
            _ => false,
        }
}
