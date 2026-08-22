use std::borrow::Cow;

use super::{
    BattlefieldExitSnapshot, CardPartId, CardRules, CardStructure, CardSupertype, CardType,
    CardTypeSet, CopiableCharacteristics, CounterKind, DeclarativeAbilityDef,
    DoubleFacedCopiableCharacteristics, Game, ObjectCharacteristics, ObjectKind, Permanent,
    PermanentLastKnownInformation, TriggerEventObject,
};

impl Game {
    pub(super) fn presentation_name(
        &self,
        presentation: ObjectCharacteristics,
    ) -> Option<Cow<'_, str>> {
        match presentation {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)?
                .part(part)
                .map(|part| Cow::Borrowed(part.name.as_str())),
            ObjectCharacteristics::Token { token, part } => {
                token.part(part).map(crate::card::TokenPart::name)
            }
            ObjectCharacteristics::Emblem { emblem } => Some(Cow::Borrowed(emblem.name())),
            ObjectCharacteristics::FaceDown { face_down } => {
                Some(Cow::Borrowed(face_down.display_name()))
            }
        }
    }

    pub(super) fn is_artifact_permanent(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Artifact))
    }

    pub(super) fn initialize_battlefield_entry(&self, permanent: &mut Permanent) {
        let starting_loyalty = self.effective_rules(permanent).and_then(|rules| {
            rules
                .has_type(CardType::Planeswalker)
                .then(|| rules.starting_loyalty())
                .flatten()
        });
        if let Some(loyalty) = starting_loyalty {
            permanent.set_counters(CounterKind::Loyalty, loyalty);
        }
    }

    pub(super) fn effective_permanent_name<'a>(
        &'a self,
        permanent: &Permanent,
    ) -> Option<Cow<'a, str>> {
        // A face-down permanent has no name at all, so nothing that reads
        // one -- naming a card, matching another object's name -- ever finds
        // it. The catalog entry behind its body is legible for a client, not
        // a name the rules can see.
        if permanent.face_down.is_some() {
            return None;
        }
        match Self::effective_rules_source(permanent) {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)?
                .part(part)
                .map(|part| Cow::Borrowed(part.name.as_str())),
            ObjectCharacteristics::Token { token, part } => {
                token.part(part).map(crate::card::TokenPart::name)
            }
            ObjectCharacteristics::Emblem { emblem } => Some(Cow::Borrowed(emblem.name())),
            ObjectCharacteristics::FaceDown { face_down } => {
                Some(Cow::Borrowed(face_down.display_name()))
            }
        }
    }

    /// Resolves the printed rules currently supplying baseline permanent
    /// characteristics. A copy's copiable rules take precedence over the
    /// physical card's presented part.
    pub(super) fn effective_rules(&self, permanent: &Permanent) -> Option<CardRules> {
        match Self::effective_rules_source(permanent) {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)?
                .part(part)
                .map(|part| part.rules),
            ObjectCharacteristics::Token { token, part } => token.part(part).map(|part| part.rules),
            ObjectCharacteristics::Emblem { emblem } => Some(emblem.rules_view()),
            ObjectCharacteristics::FaceDown { face_down } => Some(face_down.rules()),
        }
    }

    pub(super) fn effective_rules_source(permanent: &Permanent) -> ObjectCharacteristics {
        // A face-down permanent presents the shared face-down body whatever
        // the card under it says, and whatever it was copying: turning a
        // permanent face down is not a copy effect, it hides one (CR 708.2).
        if let Some(face_down) = permanent.face_down {
            return ObjectCharacteristics::face_down(face_down);
        }
        Self::unmasked_rules_source(permanent)
    }

    /// Copiable source before a face-down presentation masks it. A permanent's
    /// controller may inspect the object underneath their face-down body, and
    /// checkpoint/copy machinery must retain those values as well.
    pub(super) fn unmasked_rules_source(permanent: &Permanent) -> ObjectCharacteristics {
        if let Some(copy) = permanent.active_copy_values() {
            return copy.base;
        }
        match permanent.card.definition {
            ObjectKind::Card(definition) => {
                ObjectCharacteristics::card(definition, permanent.presented)
            }
            ObjectKind::Token => ObjectCharacteristics::token(
                permanent
                    .token_characteristics
                    .expect("a noncopy token has authored characteristics"),
                permanent.presented,
            ),
            ObjectKind::Emblem => match permanent.card.characteristics {
                super::CharacteristicSource::Emblem(emblem) => {
                    ObjectCharacteristics::emblem(emblem)
                }
                _ => unreachable!("an emblem has emblem characteristics"),
            },
            ObjectKind::Ability => unreachable!("a stack ability cannot be a permanent"),
        }
    }

    pub(super) fn copiable_characteristics(permanent: &Permanent) -> CopiableCharacteristics {
        permanent
            .active_copy_values()
            .cloned()
            .unwrap_or_else(|| CopiableCharacteristics {
                base: Self::effective_rules_source(permanent),
                added_types: CardTypeSet::empty(),
                added_abilities: Vec::new(),
                retain_printed_subtypes: false,
            })
    }

    /// The other physical face this permanent can present. Copy effects do
    /// not participate: a single-faced copier stays single-faced, while a
    /// double-faced permanent stays able to transform through a copy effect.
    pub(super) fn physical_other_face(&self, permanent: &Permanent) -> Option<CardPartId> {
        match permanent.card.definition {
            ObjectKind::Card(definition) => {
                let definition = self.catalog.get(definition)?;
                let CardStructure::DoubleFaced {
                    kind: crate::card::DoubleFacedKind::Transforming,
                    ..
                } = definition.structure
                else {
                    return None;
                };
                let other = definition.other_face(permanent.presented)?;
                let rules = definition.part(other)?.rules;
                (!rules.has_type(CardType::Instant) && !rules.has_type(CardType::Sorcery))
                    .then_some(other)
            }
            ObjectKind::Token => {
                if let Some(faces) = &permanent.double_faced_token_copy {
                    if faces.kind != crate::card::DoubleFacedKind::Transforming {
                        return None;
                    }
                    let other = faces.other_face(permanent.presented)?;
                    let copy = faces.face(other)?;
                    return self.copiable_face_can_be_up(copy).then_some(other);
                }
                let token = permanent.token_characteristics?;
                let other = token.other_face(permanent.presented)?;
                let rules = token.part(other)?.rules;
                (!rules.has_type(CardType::Instant) && !rules.has_type(CardType::Sorcery))
                    .then_some(other)
            }
            ObjectKind::Emblem | ObjectKind::Ability => None,
        }
    }

    fn copiable_face_can_be_up(&self, copy: &CopiableCharacteristics) -> bool {
        let Some(rules) = (match copy.base {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)
                .and_then(|definition| definition.part(part))
                .map(|part| part.rules),
            ObjectCharacteristics::Token { token, part } => token.part(part).map(|part| part.rules),
            ObjectCharacteristics::Emblem { .. } => None,
            ObjectCharacteristics::FaceDown { face_down } => Some(face_down.rules()),
        }) else {
            return false;
        };
        let types = rules.types().union(copy.added_types);
        !types.contains(CardType::Instant) && !types.contains(CardType::Sorcery)
    }

    /// Freezes both faces when an effect creates a token copy of a physical
    /// double-faced permanent (CR 707.8a). A copy effect applying to the
    /// source supplies both faces, but the source's physical topology remains
    /// the topology of the resulting token.
    pub(super) fn double_faced_copiable_characteristics(
        &self,
        permanent: &Permanent,
    ) -> Option<DoubleFacedCopiableCharacteristics> {
        let unmodified = |base| CopiableCharacteristics {
            base,
            added_types: CardTypeSet::empty(),
            added_abilities: Vec::new(),
            retain_printed_subtypes: false,
        };
        let mut faces = match permanent.card.definition {
            ObjectKind::Card(definition) => {
                let definition_record = self.catalog.get(definition)?;
                let CardStructure::DoubleFaced { front, back, kind } = definition_record.structure
                else {
                    return None;
                };
                DoubleFacedCopiableCharacteristics {
                    kind,
                    front_part: front,
                    back_part: back,
                    front: unmodified(ObjectCharacteristics::card(definition, front)),
                    back: unmodified(ObjectCharacteristics::card(definition, back)),
                }
            }
            ObjectKind::Token => {
                if let Some(faces) = &permanent.double_faced_token_copy {
                    faces.clone()
                } else {
                    let token = permanent.token_characteristics?;
                    let front_part = token.primary_part_id();
                    let back_part = token.other_face(front_part)?;
                    DoubleFacedCopiableCharacteristics {
                        kind: crate::card::DoubleFacedKind::Transforming,
                        front_part,
                        back_part,
                        front: unmodified(ObjectCharacteristics::token(token, front_part)),
                        back: unmodified(ObjectCharacteristics::token(token, back_part)),
                    }
                }
            }
            ObjectKind::Emblem | ObjectKind::Ability => return None,
        };
        if let Some(copy) = &permanent.copy_effect {
            faces.front.clone_from(copy);
            faces.back.clone_from(copy);
        }
        Some(faces)
    }

    pub(super) fn trigger_event_object(&self, permanent: &Permanent) -> TriggerEventObject {
        let rules = self
            .effective_rules(permanent)
            .expect("a battlefield object has effective rules");
        TriggerEventObject {
            id: permanent.card.id,
            token: permanent.card.definition.is_token(),
            types: self
                .permanent_types(permanent)
                .expect("a battlefield object has effective types"),
            controller: permanent.controller,
            attacking_or_blocking: permanent.attacking || permanent.is_blocking_this_combat(),
            colors: self.effective_colors(permanent, &rules),
            subtypes: self.effective_subtypes(permanent),
            mana_value: self.permanent_mana_value(permanent),
            power: self.power_ignoring_static_effects(permanent),
            toughness: self.toughness_ignoring_static_effects(permanent),
            keywords: self.keyword_mask(permanent, None),
            supertypes: {
                let mut supertypes = [false; CardSupertype::COUNT];
                for supertype in CardSupertype::ALL {
                    supertypes[supertype.index()] = rules.has_supertype(supertype);
                }
                supertypes
            },
            attacking: permanent.attacking,
            tapped: permanent.tapped,
            attacked_this_turn: permanent.attacked_this_turn,
            attacked_during_controllers_last_turn: self
                .attacked_during_controllers_last_turn(permanent),
        }
    }

    /// The same characteristics with continuous static power and toughness
    /// included.
    ///
    /// [`Self::trigger_event_object`] deliberately leaves those out: it is
    /// used while static effects are being resolved, and asking for a value
    /// that depends on them there would re-enter the computation. Target
    /// legality is asked from outside that resolution, so it can and must see
    /// the real numbers -- a creature a Crusade has made 2/2 is not a legal
    /// target for "target 1/1 creature".
    ///
    /// Keywords need no widening here. `Game::keyword_mask` stratifies its
    /// walk instead of truncating it, so every caller already reads the
    /// complete set; only power and toughness still need the second view.
    pub(super) fn targeting_event_object(&self, permanent: &Permanent) -> TriggerEventObject {
        TriggerEventObject {
            power: self.power(permanent),
            toughness: self.toughness(permanent),
            ..self.trigger_event_object(permanent)
        }
    }

    pub(super) fn trigger_event_object_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> TriggerEventObject {
        let rules = self
            .effective_rules(permanent)
            .expect("a battlefield object has effective rules");
        TriggerEventObject {
            id: permanent.card.id,
            token: permanent.card.definition.is_token(),
            types: self
                .permanent_types(permanent)
                .expect("a battlefield object has effective types"),
            controller: permanent.controller,
            attacking_or_blocking: permanent.attacking || permanent.is_blocking_this_combat(),
            colors: self.effective_colors(permanent, &rules),
            subtypes: self.effective_subtypes_with_prospective(permanent, prospective),
            mana_value: self.permanent_mana_value(permanent),
            power: self.power_ignoring_static_effects(permanent),
            toughness: self.toughness_ignoring_static_effects(permanent),
            keywords: self.keyword_mask(permanent, Some(prospective)),
            supertypes: {
                let mut supertypes = [false; CardSupertype::COUNT];
                for supertype in CardSupertype::ALL {
                    supertypes[supertype.index()] = rules.has_supertype(supertype);
                }
                supertypes
            },
            attacking: permanent.attacking,
            tapped: permanent.tapped,
            attacked_this_turn: permanent.attacked_this_turn,
            attacked_during_controllers_last_turn: self
                .attacked_during_controllers_last_turn(permanent),
        }
    }

    /// Whether this permanent attacked during its controller's previous turn.
    ///
    /// The recorded turn count belongs to whoever controlled it at the time,
    /// so a permanent that has changed hands since is not answering about the
    /// turn its current controller just took.
    fn attacked_during_controllers_last_turn(&self, permanent: &Permanent) -> bool {
        permanent.last_attacked_turn.is_some_and(|(player, turn)| {
            player == permanent.controller
                && turn + 1 == self.turns_started[permanent.controller.index()]
        })
    }

    pub(super) fn battlefield_exit_snapshot(
        &self,
        permanent: &Permanent,
    ) -> BattlefieldExitSnapshot {
        let abilities = self.effective_abilities(permanent);
        let mut keywords = Vec::new();
        for effective in &abilities {
            if effective.ability.is_executable()
                && let DeclarativeAbilityDef::Keyword(ability) = effective.ability.definition
                && !keywords.contains(&ability)
            {
                keywords.push(ability);
            }
        }
        BattlefieldExitSnapshot {
            object: self.trigger_event_object(permanent),
            abilities,
            last_known: PermanentLastKnownInformation {
                power: self.power(permanent),
                toughness: self.toughness(permanent),
                mana_value: self.permanent_mana_value(permanent),
                keywords,
            },
        }
    }
}
