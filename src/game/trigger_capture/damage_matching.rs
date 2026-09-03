// Whether a committed damage event answers a printed damage matcher.
//
// Split from the capture logic because the question is self-contained: the
// source, the recipient, and the kind of damage are each matched on their
// own terms, and none of it depends on how the event was collected.

impl Game {
    fn damage_trigger_matches(
        &self,
        matcher: DamageEventMatcherDef,
        event: &CommittedTriggerEvent,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        let CommittedTriggerEvent::DamageDealt {
            source,
            source_is_spell,
            recipient,
            recipient_object,
            combat,
            ..
        } = event
        else {
            return false;
        };
        (matcher.kind == DamageKindDef::Any || *combat)
            && self.damage_trigger_source_matches(
                matcher.source,
                source.as_ref(),
                *source_is_spell,
                ability_source,
                controller,
                event,
            )
            && self.damage_trigger_recipient_matches(
                matcher.recipient,
                *recipient,
                recipient_object.as_ref(),
                ability_source,
                controller,
                event,
            )
    }

    fn damage_trigger_source_matches(
        &self,
        matcher: DamageSourceMatcherDef,
        damage_source: Option<&TriggerEventObject>,
        source_is_spell: bool,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match matcher {
            DamageSourceMatcherDef::Any => true,
            DamageSourceMatcherDef::AffectedObject => {
                damage_source.is_some_and(|object| object.id == ability_source)
            }
            DamageSourceMatcherDef::Object(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .is_some_and(|expected| damage_source.is_some_and(|object| object.id == expected)),
            DamageSourceMatcherDef::Except(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .is_some_and(|excluded| damage_source.is_none_or(|object| object.id != excluded)),
            DamageSourceMatcherDef::Matching(predicate) => damage_source.is_some_and(|object| {
                self.trigger_object_matches_for_controller(
                    predicate,
                    object,
                    ability_source,
                    source_is_spell,
                    controller,
                )
            }),
            DamageSourceMatcherDef::Group(group) => damage_source.is_some_and(|object| {
                let flying = KeywordAbility::Flying
                    .simple_index()
                    .is_some_and(|index| object.keywords & (1 << index) != 0);
                match group {
                    DamageSourceGroupDef::CreaturesWithFlying => {
                        object.types.contains(CardType::Creature) && flying
                    }
                    DamageSourceGroupDef::AttackingCreaturesWithoutFlying => {
                        object.types.contains(CardType::Creature) && object.attacking && !flying
                    }
                    DamageSourceGroupDef::Artifacts => object.types.contains(CardType::Artifact),
                    DamageSourceGroupDef::UnblockedCreatures => {
                        object.types.contains(CardType::Creature)
                            && object.attacking
                            && !self
                                .battlefield
                                .iter()
                                .any(|blocker| blocker.is_blocking(object.id))
                    }
                }
            }),
        }
    }

    fn damage_trigger_recipient_matches(
        &self,
        matcher: DamageRecipientMatcherDef,
        recipient: Target,
        recipient_object: Option<&TriggerEventObject>,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match matcher {
            DamageRecipientMatcherDef::Any => true,
            DamageRecipientMatcherDef::AffectedObject => {
                recipient == Target::Permanent(ability_source)
            }
            // Read from the snapshot rather than the battlefield: a creature
            // that took lethal damage is often gone by the time the trigger
            // is placed, and it is still what the clause is about.
            DamageRecipientMatcherDef::MatchingObject(predicate) => {
                recipient_object.is_some_and(|object| {
                    self.trigger_object_matches_for_controller(
                        predicate,
                        object,
                        ability_source,
                        false,
                        controller,
                    )
                })
            }
            DamageRecipientMatcherDef::Recipients(recipients) => match recipients.0 {
                EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)) => self
                    .trigger_event_object_reference(reference, ability_source, event)
                    .is_some_and(|expected| match recipient {
                        Target::Card(object)
                        | Target::Permanent(object)
                        | Target::Spell(object) => object == expected,
                        Target::Player(_) => false,
                    }),
                // A clause that names both kinds at once is not something a
                // trigger points back at: nothing about a committed damage
                // event names a pair, and nothing about it names a combat.
                EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
                | EffectRecipientSetDef::DefenderOf(_)
                | EffectRecipientSetDef::LegalTargets(_)
                | EffectRecipientSetDef::Objects(
                    ObjectSetDef::Binding(_)

                    | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
                    | ObjectSetDef::MatchingBinding { .. }
                    | ObjectSetDef::Matching { .. }
                    | ObjectSetDef::LegalTargets(_)
                    | ObjectSetDef::Query(_)
                    | ObjectSetDef::PermanentsTargetedBy(_)
                    | ObjectSetDef::PlayerAttachments(_)
                    | ObjectSetDef::LegalAttachmentHosts(_)
                    | ObjectSetDef::LinkedExiles
                    | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                    | ObjectSetDef::PermanentsControlledBy(_)
                    | ObjectSetDef::TokensCreatedBy(_)
                    | ObjectSetDef::BottomOfGraveyard(_)
                    | ObjectSetDef::ExceptObject { .. }
                    | ObjectSetDef::TopOfGraveyardMatching { .. },
                ) => false,
                EffectRecipientSetDef::Players(players) => {
                    let Target::Player(recipient) = recipient else {
                        return false;
                    };
                    self.damage_trigger_player_set_matches(
                        players,
                        recipient,
                        ability_source,
                        controller,
                        event,
                    )
                }
            },
            // A player or a planeswalker: one event goes to one of the two,
            // and the printed clause does not care which.
            DamageRecipientMatcherDef::PlayerOrPlaneswalker => match recipient {
                Target::Player(_) => true,
                Target::Permanent(_) => recipient_object
                    .is_some_and(|object| object.types.contains(CardType::Planeswalker)),
                Target::Card(_) | Target::Spell(_) => false,
            },
            DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
                let Some(player) =
                    self.trigger_event_player_reference(player, ability_source, controller, event)
                else {
                    return false;
                };
                match recipient {
                    Target::Player(recipient) => recipient == player,
                    Target::Permanent(object) => recipient_object.is_some_and(|recipient| {
                        recipient.id == object
                            && recipient.controller == player
                            && recipient.types.contains(CardType::Creature)
                    }),
                    Target::Card(_) | Target::Spell(_) => false,
                }
            }
        }
    }

    fn damage_trigger_player_set_matches(
        &self,
        players: PlayerSetDef,
        recipient: PlayerId,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match players {
            PlayerSetDef::All => true,
            PlayerSetDef::LegalTargets(_) => false,
            PlayerSetDef::One(reference) => {
                self.trigger_event_player_reference(reference, ability_source, controller, event)
                    == Some(recipient)
            }
            PlayerSetDef::Related(PlayerRelation::ChosenPlayer) => {
                self.chosen_player_of(ability_source) == Some(recipient)
            }
            PlayerSetDef::Related(PlayerRelation::EnchantedPlayer) => {
                self.current_or_last_known_enchanted_player(ability_source) == Some(recipient)
            }
            PlayerSetDef::Related(relation) => controller.is_some_and(|controller| {
                self.player_relation_matches(recipient, relation, controller, event.context())
            }),
        }
    }
}
