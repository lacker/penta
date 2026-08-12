use std::fmt::Write as _;

use crate::ids::{ModeId, TargetSlotId};

use super::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardEffectStatus, CardSupertype,
    CardType, DeclarativeAbilityDef, DividedTotal, ManaColor, ObjectPredicateDef, PlayerRelation,
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

fn object_predicate_implies(predicate: ObjectPredicateDef, expected: ObjectPredicateDef) -> bool {
    if predicate == expected {
        return true;
    }
    match predicate {
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .any(|predicate| object_predicate_implies(predicate, expected)),
        ObjectPredicateDef::AnyOf(predicates) => {
            !predicates.is_empty()
                && predicates
                    .iter()
                    .copied()
                    .all(|predicate| object_predicate_implies(predicate, expected))
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

fn predicate_color(predicate: ObjectPredicateDef) -> Option<ManaColor> {
    match predicate {
        ObjectPredicateDef::Color(color) => Some(color),
        ObjectPredicateDef::All(predicates) => predicates.iter().copied().find_map(predicate_color),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_color_count(predicate: ObjectPredicateDef) -> Option<u8> {
    match predicate {
        ObjectPredicateDef::ColorCount(count) => Some(count),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_color_count)
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Subtype(subtype) => Some(subtype),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_subtype)
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_negated_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Not(inner) => match *inner {
            ObjectPredicateDef::Subtype(subtype) => Some(subtype),
            _ => None,
        },
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_negated_subtype),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_power_at_least(predicate: ObjectPredicateDef) -> Option<i16> {
    match predicate {
        // An exact power is also a minimum, which is all this reports.
        ObjectPredicateDef::PowerAtLeast(power) | ObjectPredicateDef::PowerExactly(power) => {
            Some(power)
        }
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_power_at_least),
        ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_mana_value_at_most(predicate: ObjectPredicateDef) -> Option<u8> {
    match predicate {
        ObjectPredicateDef::ManaValueAtMost(value) => Some(value),
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_mana_value_at_most),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_controller(predicate: ObjectPredicateDef) -> Option<PlayerRelation> {
    match predicate {
        ObjectPredicateDef::ControlledBy(controller) => Some(controller),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_controller)
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
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
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
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
        CardType::Land => "land",
        CardType::Planeswalker => "planeswalker",
        CardType::Sorcery => "sorcery",
        CardType::Emblem => "emblem",
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
    if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
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
    if zones == [ZoneKind::Graveyard] {
        let subject = semantic_card_subject(object);
        let graveyard = match owner {
            Some(PlayerRelation::You) => "your graveyard",
            Some(PlayerRelation::Opponent) => "an opponent's graveyard",
            Some(PlayerRelation::NotYou) => "a graveyard other than yours",
            Some(PlayerRelation::ActivePlayer) => "the active player's graveyard",
            Some(PlayerRelation::NonactivePlayer) => "the nonactive player's graveyard",
            Some(PlayerRelation::EventPlayer) => "the event player's graveyard",
            Some(PlayerRelation::ChosenPlayer) => "the chosen player's graveyard",
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

const fn player_target_label(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "target player",
        PlayerRelation::You => "yourself",
        PlayerRelation::NotYou => "target player other than you",
        PlayerRelation::Opponent => "target opponent",
        PlayerRelation::ActivePlayer => "target active player",
        PlayerRelation::NonactivePlayer => "target nonactive player",
        PlayerRelation::EventPlayer => "target event player",
        PlayerRelation::ChosenPlayer => "the chosen player",
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
        PlayerRelation::ChosenPlayer => "the chosen player or planeswalker",
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
        PlayerRelation::ChosenPlayer => " the chosen player controls",
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
        PlayerRelation::ChosenPlayer => " the chosen player owns",
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
        AbilityTargetPredicate::Player(_) => Some(TargetPredicate::Player),
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
        AbilityTargetPredicate::Object { .. } => None,
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
            AbilityTargetPredicate::Player(relation) => player_target_label(relation).into(),
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                let predicate = presentation_target_predicate(self.predicate);
                let subject = predicate.map_or_else(
                    || semantic_object_target_subject(object, zones, owner),
                    |predicate| object_target_subject(object, predicate),
                );
                let mut label = format!("target {subject}");
                if predicate_negates(object, ObjectPredicateDef::Source) {
                    label.insert_str("target ".len(), "another ");
                }
                if predicate_negates(object, ObjectPredicateDef::SharesNameWithSource) {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeDef {
    pub id: ModeId,
    pub label: String,
    pub targets: Vec<TargetSlotDef>,
    pub effect_status: CardEffectStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeSetDef {
    pub minimum: u8,
    pub maximum: u8,
    /// Some cards explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
    pub modes: Vec<ModeDef>,
}

impl ModeSetDef {
    #[must_use]
    pub fn choose_one(modes: Vec<ModeDef>) -> Self {
        Self {
            minimum: 1,
            maximum: 1,
            may_repeat: false,
            modes,
        }
    }
}

impl AbilityDef {
    pub(super) fn mode_presentation(
        self,
        id: ModeId,
        outer_is_executable: bool,
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
            targets,
            effect_status: if outer_is_executable && self.is_executable() {
                CardEffectStatus::Implemented
            } else {
                CardEffectStatus::MetadataOnly
            },
        })
    }
}
