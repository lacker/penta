use std::fmt::Write as _;

use crate::ids::{ModeId, TargetSlotId};

use super::presentation_predicates::{
    object_predicate_implies, predicate_color, predicate_color_count, predicate_controller,
    predicate_mana_value_at_most, predicate_negated_subtype, predicate_power_at_least,
    predicate_subtype,
};
use super::{
    AbilityDef, AbilityKindDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    CardEffectStatus, CardSupertype, CardType, ConditionalModeMaximumDef, DeclarativeAbilityDef,
    DividedTotal, ManaColor, ManaCost, ObjectPredicateDef, ObjectRefDef, PlayerRelation,
    TargetPredicate, ZoneKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetSlotDef {
    pub id: TargetSlotId,
    pub label: String,
    pub predicate: TargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
    /// The total this slot divides among its targets, when the card says
    /// "divided as you choose". Every chosen target takes at least one, which
    /// is what makes the number of targets a consequence of the division.
    pub divided_total: Option<DividedTotal>,
}

impl TargetSlotDef {
    #[must_use]
    pub fn exactly_one(
        id: TargetSlotId,
        label: impl Into<String>,
        predicate: TargetPredicate,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            predicate,
            minimum: 1,
            maximum: 1,
            divided_total: None,
        }
    }

    /// "N damage divided as you choose among one, two, or three targets."
    #[must_use]
    pub fn divided(
        id: TargetSlotId,
        label: impl Into<String>,
        predicate: TargetPredicate,
        total: u8,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            predicate,
            minimum: 1,
            maximum: total,
            divided_total: Some(DividedTotal::Fixed(total)),
        }
    }
}

fn predicate_negates(predicate: ObjectPredicateDef, expected: ObjectPredicateDef) -> bool {
    match predicate {
        // Stay deliberately conservative: `not (red land)` does not imply
        // "nonland," even though the inner conjunction implies `land`.
        ObjectPredicateDef::Not(inner) => *inner == expected,
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .any(|predicate| predicate_negates(predicate, expected)),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::GenericManaCostAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::PowerGreaterThan(_)
        | ObjectPredicateDef::PowerLessThan(_)
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::ToughnessGreaterThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasName(_)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::HasAnyCounter
        | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

const fn color_name(color: ManaColor) -> &'static str {
    match color {
        ManaColor::White => "white",
        ManaColor::Blue => "blue",
        ManaColor::Black => "black",
        ManaColor::Red => "red",
        ManaColor::Green => "green",
        ManaColor::Colorless => "colorless",
    }
}

const fn card_type_name(card_type: CardType) -> &'static str {
    match card_type {
        CardType::Artifact => "artifact",
        CardType::Creature => "creature",
        CardType::Enchantment => "enchantment",
        CardType::Instant => "instant",
        CardType::Kindred => "kindred",
        CardType::Land => "land",
        CardType::Planeswalker => "planeswalker",
        CardType::Sorcery => "sorcery",
    }
}

fn simple_disjunction_subject(predicate: ObjectPredicateDef) -> Option<String> {
    let ObjectPredicateDef::AnyOf(predicates) = predicate else {
        return None;
    };
    let subjects = predicates
        .iter()
        .copied()
        .map(|predicate| match predicate {
            ObjectPredicateDef::HasType(card_type) => Some(card_type_name(card_type)),
            ObjectPredicateDef::Subtype(subtype) => Some(subtype),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    (!subjects.is_empty()).then(|| subjects.join(" or "))
}

fn object_target_subject(object: ObjectPredicateDef, predicate: TargetPredicate) -> String {
    if let ObjectPredicateDef::Special(description) = object {
        return description.into();
    }
    if object_predicate_implies(object, ObjectPredicateDef::Attacking) {
        return "attacking creature".into();
    }
    match predicate {
        TargetPredicate::AnyTarget => "target".into(),
        TargetPredicate::Player => "player".into(),
        TargetPredicate::NoncreatureSpell => predicate_color(object).map_or_else(
            || "noncreature spell".into(),
            |color| format!("{} noncreature spell", color_name(color)),
        ),
        TargetPredicate::Spell => predicate_color(object).map_or_else(
            || "spell".into(),
            |color| format!("{} spell", color_name(color)),
        ),
        TargetPredicate::CreaturePermanent => {
            if object_predicate_implies(object, ObjectPredicateDef::AttackingOrBlocking) {
                "attacking or blocking creature".into()
            } else if object_predicate_implies(object, ObjectPredicateDef::Attacking) {
                "attacking creature".into()
            } else if let Some(subtype) = predicate_negated_subtype(object) {
                format!("non-{subtype} creature")
            } else if let Some(subtype) = predicate_subtype(object) {
                format!("{subtype} creature")
            } else if let Some(count) = predicate_color_count(object) {
                match count {
                    0 => "colorless creature".into(),
                    1 => "monocolored creature".into(),
                    _ => format!("creature with exactly {count} colors"),
                }
            } else if let Some(color) = predicate_color(object) {
                format!("{} creature", color_name(color))
            } else if let Some(power) = predicate_power_at_least(object) {
                format!("creature with power {power} or greater")
            } else {
                "creature".into()
            }
        }
        TargetPredicate::Permanent => {
            if let Some(subject) = simple_disjunction_subject(object) {
                subject
            } else if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Land))
                && predicate_negates(object, ObjectPredicateDef::Supertype(CardSupertype::Basic))
            {
                "nonbasic land".into()
            } else if predicate_negates(object, ObjectPredicateDef::HasType(CardType::Land)) {
                let mut subject = "nonland permanent".to_string();
                if let Some(value) = predicate_mana_value_at_most(object) {
                    let _ = write!(subject, " with mana value {value} or less");
                }
                subject
            } else if let Some(card_type) = CardType::DISPLAY_ORDER.into_iter().find(|card_type| {
                object_predicate_implies(object, ObjectPredicateDef::HasType(*card_type))
            }) {
                card_type_name(card_type).into()
            } else if let Some(subtype) = predicate_subtype(object) {
                subtype.into()
            } else if let Some(count) = predicate_color_count(object) {
                match count {
                    0 => "colorless permanent".into(),
                    1 => "monocolored permanent".into(),
                    _ => format!("permanent with exactly {count} colors"),
                }
            } else if let Some(color) = predicate_color(object) {
                format!("{} permanent", color_name(color))
            } else {
                "permanent".into()
            }
        }
    }
}

fn semantic_card_subject(object: ObjectPredicateDef) -> String {
    if let Some(subject) = simple_disjunction_subject(object) {
        return format!("{subject} card");
    }
    if object_predicate_implies(
        object,
        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Is(AbilityKindDef::Flashback)),
    ) {
        "card with flashback".into()
    } else if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
        "creature card".into()
    } else if let Some(subtype) = predicate_subtype(object) {
        format!("{subtype} card")
    } else if let ObjectPredicateDef::Special(description) = object {
        description.into()
    } else {
        "card".into()
    }
}

fn semantic_object_target_subject(
    object: ObjectPredicateDef,
    zones: &'static [ZoneKind],
    owner: Option<PlayerRelation>,
) -> String {
    if zones == [ZoneKind::Stack] {
        return semantic_stack_object_subject(object);
    }
    if zones == [ZoneKind::Graveyard] {
        let subject = semantic_card_subject(object);
        let graveyard = match owner {
            Some(PlayerRelation::You) => "your graveyard",
            Some(PlayerRelation::Opponent) => "an opponent's graveyard",
            Some(PlayerRelation::NotYou) => "a graveyard other than yours",
            Some(PlayerRelation::ActivePlayer) => "the active player's graveyard",
            Some(PlayerRelation::NonactivePlayer) => "the nonactive player's graveyard",
            Some(PlayerRelation::EventPlayer) => "the event player's graveyard",
            Some(PlayerRelation::NotEventPlayer) => "a graveyard other than the event player's",
            Some(PlayerRelation::ControllerOfAttachedPermanent) => {
                "the enchanted permanent's controller's graveyard"
            }
            Some(PlayerRelation::EnchantedPlayer) => "the enchanted player's graveyard",
            Some(PlayerRelation::ChosenPlayer) => "the chosen player's graveyard",
            Some(PlayerRelation::DefendingPlayer) => "the defending player's graveyard",
            Some(PlayerRelation::Any) | None => "a graveyard",
        };
        return format!("{subject} in {graveyard}");
    }
    if zones == [ZoneKind::Battlefield, ZoneKind::Graveyard]
        && object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature))
    {
        return "creature on the battlefield or creature card in a graveyard".into();
    }
    let subject = semantic_card_subject(object);
    match zones {
        [ZoneKind::Hand] => format!("{subject} in a hand"),
        [ZoneKind::Library] => format!("{subject} in a library"),
        [ZoneKind::Exile] => format!("{subject} in exile"),
        _ => subject,
    }
}

fn semantic_stack_object_subject(object: ObjectPredicateDef) -> String {
    if object_predicate_implies(object, ObjectPredicateDef::ActivatedAbility) {
        "activated ability".into()
    } else if object_predicate_implies(object, ObjectPredicateDef::TriggeredAbility) {
        "triggered ability".into()
    } else if object_predicate_implies(object, ObjectPredicateDef::Ability) {
        "activated or triggered ability".into()
    } else if object_predicate_implies(object, ObjectPredicateDef::Spell) {
        object_target_subject(object, TargetPredicate::Spell)
    } else {
        "spell or ability".into()
    }
}

const fn player_target_label(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "target player",
        PlayerRelation::You => "yourself",
        PlayerRelation::NotYou => "target player other than you",
        PlayerRelation::Opponent => "target opponent",
        PlayerRelation::ActivePlayer => "target active player",
        PlayerRelation::NonactivePlayer => "target nonactive player",
        PlayerRelation::EventPlayer => "target event player",
        PlayerRelation::NotEventPlayer => "target player other than the event player",
        PlayerRelation::ChosenPlayer => "the chosen player",
        PlayerRelation::DefendingPlayer => "the defending player",
        PlayerRelation::ControllerOfAttachedPermanent => "the enchanted permanent's controller",
        PlayerRelation::EnchantedPlayer => "the enchanted player",
    }
}

const fn player_or_planeswalker_target_label(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "target player or planeswalker",
        PlayerRelation::You => "yourself or target planeswalker",
        PlayerRelation::NotYou => "target player other than you or planeswalker",
        PlayerRelation::Opponent => "target opponent or planeswalker",
        PlayerRelation::ActivePlayer => "target active player or planeswalker",
        PlayerRelation::NonactivePlayer => "target nonactive player or planeswalker",
        PlayerRelation::EventPlayer => "target event player or planeswalker",
        PlayerRelation::NotEventPlayer => {
            "target player other than the event player or planeswalker"
        }
        PlayerRelation::ChosenPlayer => "the chosen player or planeswalker",
        PlayerRelation::DefendingPlayer => "the defending player or their planeswalker",
        PlayerRelation::ControllerOfAttachedPermanent => {
            "the enchanted permanent's controller or planeswalker"
        }
        PlayerRelation::EnchantedPlayer => "the enchanted player or their planeswalker",
    }
}

const fn controller_suffix(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "",
        PlayerRelation::You => " you control",
        PlayerRelation::NotYou => " you don't control",
        PlayerRelation::Opponent => " an opponent controls",
        PlayerRelation::ActivePlayer => " the active player controls",
        PlayerRelation::NonactivePlayer => " the nonactive player controls",
        PlayerRelation::EventPlayer => " the event player controls",
        PlayerRelation::NotEventPlayer => " the event player does not control",
        PlayerRelation::ChosenPlayer => " the chosen player controls",
        PlayerRelation::DefendingPlayer => " the defending player controls",
        PlayerRelation::ControllerOfAttachedPermanent => {
            " the enchanted permanent's controller controls"
        }
        PlayerRelation::EnchantedPlayer => " the enchanted player controls",
    }
}

const fn owner_suffix(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "",
        PlayerRelation::You => " you own",
        PlayerRelation::NotYou => " you don't own",
        PlayerRelation::Opponent => " an opponent owns",
        PlayerRelation::ActivePlayer => " the active player owns",
        PlayerRelation::NonactivePlayer => " the nonactive player owns",
        PlayerRelation::EventPlayer => " the event player owns",
        PlayerRelation::NotEventPlayer => " the event player does not own",
        PlayerRelation::ChosenPlayer => " the chosen player owns",
        PlayerRelation::DefendingPlayer => " the defending player owns",
        PlayerRelation::ControllerOfAttachedPermanent => {
            " the enchanted permanent's controller owns"
        }
        PlayerRelation::EnchantedPlayer => " the enchanted player owns",
    }
}

fn append_relation_suffix(label: &mut String, suffix: &'static str) {
    if suffix.is_empty() {
        return;
    }
    // Keep the relation next to its noun: "creature you control with ...",
    // rather than making it appear to modify a later characteristic.
    let position = label.find(" with ").unwrap_or(label.len());
    label.insert_str(position, suffix);
}

fn presentation_target_predicate(predicate: AbilityTargetPredicate) -> Option<TargetPredicate> {
    match predicate {
        AbilityTargetPredicate::IfAdditionalCostPaid {
            if_paid, otherwise, ..
        } => {
            let paid = presentation_target_predicate(*if_paid)?;
            (presentation_target_predicate(*otherwise) == Some(paid)).then_some(paid)
        }
        AbilityTargetPredicate::AnyOf(predicates) => {
            let mut predicates = predicates.iter().copied();
            let first = presentation_target_predicate(predicates.next()?)?;
            predicates
                .all(|predicate| presentation_target_predicate(predicate) == Some(first))
                .then_some(first)
        }
        // A client has no slot kind narrower than every damage target, which
        // is closer than presenting only the player half of this predicate.
        AbilityTargetPredicate::AnyTarget | AbilityTargetPredicate::PlayerOrPlaneswalker(_) => {
            Some(TargetPredicate::AnyTarget)
        }
        AbilityTargetPredicate::ControlledByTargetOf { object, .. } => {
            if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
                Some(TargetPredicate::CreaturePermanent)
            } else {
                Some(TargetPredicate::Permanent)
            }
        }
        AbilityTargetPredicate::Player(_)
        | AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser { .. } => {
            Some(TargetPredicate::Player)
        }
        AbilityTargetPredicate::Object { object, zones, .. } if zones == [ZoneKind::Stack] => {
            if object_predicate_implies(object, ObjectPredicateDef::NoncreatureSpell) {
                Some(TargetPredicate::NoncreatureSpell)
            } else {
                Some(TargetPredicate::Spell)
            }
        }
        AbilityTargetPredicate::Object { object, zones, .. }
            if zones == [ZoneKind::Battlefield] =>
        {
            if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
                Some(TargetPredicate::CreaturePermanent)
            } else {
                Some(TargetPredicate::Permanent)
            }
        }
        AbilityTargetPredicate::OwnedByTargetPlayer { .. }
        | AbilityTargetPredicate::Object { .. } => None,
    }
}

impl AbilityTargetDef {
    /// Derives concise presentation text from the authoritative predicate.
    ///
    /// This is only a label: compound restrictions may be summarized, while
    /// target enumeration and legality always use [`Self::predicate`]. The
    /// renderer prefers a broader accurate noun phrase over guessing at
    /// English for an unfamiliar predicate combination.
    pub(crate) fn label(self) -> String {
        match self.predicate {
            AbilityTargetPredicate::IfAdditionalCostPaid {
                if_paid, otherwise, ..
            } => conditional_target_label(self, *if_paid, *otherwise),
            AbilityTargetPredicate::AnyOf(predicates) => {
                let alternatives = predicates
                    .iter()
                    .map(|predicate| {
                        let label = Self {
                            predicate: *predicate,
                            ..self
                        }
                        .label();
                        label
                            .strip_prefix("target ")
                            .unwrap_or(label.as_str())
                            .to_string()
                    })
                    .collect::<Vec<_>>();
                format!("target {}", alternatives.join(" or "))
            }
            AbilityTargetPredicate::AnyTarget => "any target".into(),
            AbilityTargetPredicate::PlayerOrPlaneswalker(relation) => {
                player_or_planeswalker_target_label(relation).into()
            }
            AbilityTargetPredicate::ControlledByTargetOf { object, .. } => {
                let predicate = presentation_target_predicate(self.predicate)
                    .expect("dependent targets always project to a permanent target");
                let subject = object_target_subject(object, predicate);
                format!("target {subject} that player or that planeswalker's controller controls")
            }
            AbilityTargetPredicate::OwnedByTargetPlayer { object, zones, .. } => {
                let subject = semantic_card_subject(object);
                match zones {
                    [ZoneKind::Graveyard] => {
                        format!("target {subject} in that player's graveyard")
                    }
                    [ZoneKind::Hand] => format!("target {subject} in that player's hand"),
                    [ZoneKind::Library] => {
                        format!("target {subject} in that player's library")
                    }
                    [ZoneKind::Exile] => format!("target {subject} that player owns in exile"),
                    _ => format!("target {subject} that player owns"),
                }
            }
            AbilityTargetPredicate::Player(relation) => player_target_label(relation).into(),
            // The comparison is what the card says out loud, so the label
            // says it too rather than settling for "target player".
            AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser { object, .. } => {
                let subject = object_target_subject(object, TargetPredicate::Permanent);
                format!("target player who controls more {subject}s than they do")
            }
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                let predicate = presentation_target_predicate(self.predicate);
                let subject = if zones == [ZoneKind::Stack] {
                    semantic_stack_object_subject(object)
                } else {
                    predicate.map_or_else(
                        || semantic_object_target_subject(object, zones, owner),
                        |predicate| object_target_subject(object, predicate),
                    )
                };
                let mut label = format!("target {subject}");
                if predicate_negates(object, ObjectPredicateDef::Source) {
                    label.insert_str("target ".len(), "another ");
                }
                if predicate_negates(object, ObjectPredicateDef::HasName(ObjectRefDef::Source)) {
                    label.push_str(" with a different name from this source");
                }
                let relation = controller.or_else(|| predicate_controller(object));
                if let Some(relation) = relation {
                    append_relation_suffix(&mut label, controller_suffix(relation));
                } else if predicate.is_some()
                    && let Some(relation) = owner
                {
                    append_relation_suffix(&mut label, owner_suffix(relation));
                }
                label
            }
        }
    }

    pub(in crate::card) fn presentation(self, id: TargetSlotId) -> Option<TargetSlotDef> {
        let predicate = presentation_target_predicate(self.predicate)?;
        Some(TargetSlotDef {
            id,
            label: self.label(),
            predicate,
            minimum: self.minimum,
            maximum: self.maximum,
            divided_total: self.divided_total,
        })
    }
}

fn conditional_target_label(
    target: AbilityTargetDef,
    if_paid: AbilityTargetPredicate,
    otherwise: AbilityTargetPredicate,
) -> String {
    let ordinary = AbilityTargetDef {
        predicate: otherwise,
        ..target
    }
    .label();
    let paid = AbilityTargetDef {
        predicate: if_paid,
        ..target
    }
    .label();
    if ordinary == paid {
        ordinary
    } else {
        format!(
            "{ordinary} or, if the additional cost was paid, {}",
            paid.strip_prefix("target ").unwrap_or(paid.as_str())
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeDef {
    pub id: ModeId,
    pub label: String,
    /// The additional mana required when this particular mode is chosen.
    /// Present for Spree modes and absent for ordinary modal instructions.
    pub additional_mana_cost: Option<ManaCost>,
    pub targets: Vec<TargetSlotDef>,
    pub effect_status: CardEffectStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeSetDef {
    pub minimum: u8,
    pub maximum: u8,
    /// Some cards explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
    /// The larger maximum a printed "you may choose two instead" allows, and
    /// what has to be true where the spell is offered for it to apply.
    pub conditional_maximum: Option<ConditionalModeMaximumDef>,
    pub modes: Vec<ModeDef>,
}

impl ModeSetDef {
    #[must_use]
    pub fn choose_one(modes: Vec<ModeDef>) -> Self {
        Self {
            minimum: 1,
            maximum: 1,
            may_repeat: false,
            conditional_maximum: None,
            modes,
        }
    }
}

impl AbilityDef {
    pub(super) fn mode_presentation(
        self,
        id: ModeId,
        outer_is_executable: bool,
        additional_mana_cost: Option<ManaCost>,
    ) -> Option<ModeDef> {
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            return None;
        };
        if spell.modal().is_some() {
            return None;
        }
        let mut targets = Vec::with_capacity(spell.targets().len());
        for (index, target) in spell.targets().iter().copied().enumerate() {
            let id = TargetSlotId::from_index(index)?;
            let Some(target) = target.presentation(id) else {
                // The semantic target vocabulary is richer than the legacy
                // presentation predicate. An empty projection keeps runtime
                // targeting authoritative without publishing an approximation.
                targets.clear();
                break;
            };
            targets.push(target);
        }
        Some(ModeDef {
            id,
            label: self.text.into(),
            additional_mana_cost,
            targets,
            effect_status: if outer_is_executable {
                CardEffectStatus::Implemented
            } else {
                CardEffectStatus::Unsupported
            },
        })
    }
}
