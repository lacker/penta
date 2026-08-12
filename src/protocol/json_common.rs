use serde_json::{Value, json};

use crate::card::{AlternateSpellKind, BasicLandType, DoubleFacedKind, SpellForm, TargetPredicate};
use crate::casting::{CastChoices, CastSignature};
use crate::{
    AbilityOrigin, AttackDefender, DecisionVisibility, DecisionZone, GameObjectId, ManaColor,
    PlayerId, Step, Target,
};

pub(super) fn seat_name(player: PlayerId) -> &'static str {
    match player {
        PlayerId::One => "p1",
        PlayerId::Two => "p2",
    }
}

pub(super) fn seat_by_name(name: &str) -> Option<PlayerId> {
    match name {
        "p1" => Some(PlayerId::One),
        "p2" => Some(PlayerId::Two),
        _ => None,
    }
}

pub(super) const fn step_name(step: Step) -> &'static str {
    match step {
        Step::Upkeep => "Upkeep",
        Step::Draw => "Draw",
        Step::PrecombatMain => "PrecombatMain",
        Step::BeginningOfCombat => "BeginningOfCombat",
        Step::DeclareAttackers => "DeclareAttackers",
        Step::DeclareBlockers => "DeclareBlockers",
        Step::CombatDamage => "CombatDamage",
        Step::EndOfCombat => "EndOfCombat",
        Step::PostcombatMain => "PostcombatMain",
        Step::End => "End",
        Step::Cleanup => "Cleanup",
    }
}

pub(super) const fn decision_visibility_name(visibility: DecisionVisibility) -> &'static str {
    match visibility {
        DecisionVisibility::Public => "Public",
        DecisionVisibility::Private => "Private",
    }
}

pub(super) const fn decision_zone_name(zone: DecisionZone) -> &'static str {
    match zone {
        DecisionZone::Hand => "Hand",
        DecisionZone::Graveyard => "Graveyard",
        DecisionZone::Battlefield => "Battlefield",
        DecisionZone::Stack => "Stack",
        DecisionZone::Library => "Library",
        DecisionZone::Exile => "Exile",
        DecisionZone::OutsideGame => "OutsideGame",
        DecisionZone::Command => "Command",
        DecisionZone::DrawnThisStep => "DrawnThisStep",
        DecisionZone::None => "None",
    }
}

pub(super) const fn double_faced_kind_name(kind: DoubleFacedKind) -> &'static str {
    match kind {
        DoubleFacedKind::Transforming => "Transforming",
        DoubleFacedKind::Modal => "Modal",
    }
}

pub(super) const fn alternate_spell_kind_name(kind: AlternateSpellKind) -> &'static str {
    match kind {
        AlternateSpellKind::Adventure => "Adventure",
        AlternateSpellKind::Omen => "Omen",
    }
}

pub(super) const fn target_predicate_name(predicate: TargetPredicate) -> &'static str {
    match predicate {
        TargetPredicate::AnyTarget => "AnyTarget",
        TargetPredicate::Player => "Player",
        TargetPredicate::Permanent => "Permanent",
        TargetPredicate::CreaturePermanent => "CreaturePermanent",
        TargetPredicate::Spell => "Spell",
        TargetPredicate::NoncreatureSpell => "NoncreatureSpell",
    }
}

/// Serializes the unredacted zone view. `objectId` matches the identifier the
/// observation already uses, so a caller can join the two without a lookup.
pub(super) fn zone_cards_json(cards: &[crate::ZoneCard]) -> Value {
    Value::from(
        cards
            .iter()
            .map(|card| {
                json!({
                    "objectId": card.object.0,
                    "definition": card.definition.0,
                })
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn target_json(target: Target) -> Value {
    match target {
        Target::Player(player) => json!({ "type": "player", "seat": seat_name(player) }),
        Target::Card(id) => json!({
            "type": "card",
            "objectId": id.0,
            "instance": id.0,
        }),
        Target::Permanent(id) => json!({
            "type": "permanent",
            "objectId": id.0,
            "instance": id.0,
        }),
        Target::Spell(id) => json!({
            "type": "spell",
            "objectId": id.0,
            "stackId": id.0,
        }),
    }
}

pub(super) fn defender_json(defender: AttackDefender) -> Value {
    match defender {
        AttackDefender::Player(player) => {
            json!({ "type": "player", "seat": seat_name(player) })
        }
        AttackDefender::Planeswalker(permanent) => {
            json!({ "type": "planeswalker", "objectId": permanent.0 })
        }
    }
}

pub(super) fn instances_json(cards: &[GameObjectId]) -> Value {
    Value::from(cards.iter().map(|card| card.0).collect::<Vec<_>>())
}

pub(super) fn spell_form_json(form: &SpellForm) -> Value {
    match form {
        SpellForm::Part(part) => json!({
            "kind": "part",
            "partId": part.0,
        }),
        SpellForm::Combined(parts) => json!({
            "kind": "combined",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
        }),
    }
}

pub(super) fn target_selections_json(selections: &[crate::TargetSelection]) -> Vec<Value> {
    selections
        .iter()
        .map(|selection| {
            json!({
                "slotId": selection.slot().0,
                "targets": selection
                    .targets()
                    .iter()
                    .copied()
                    .map(target_json)
                    .collect::<Vec<_>>(),
                // Present only for a slot the card divides; each entry is the
                // share of the target at the same position.
                "amounts": selection.amounts(),
            })
        })
        .collect()
}

pub(super) fn cast_choices_json(choices: &CastChoices) -> Value {
    json!({
        "playOptionId": choices.play_option().0,
        "modeIds": choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": choices.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": choices
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": choices.x(),
        "targetSelections": target_selections_json(choices.targets()),
    })
}

pub(super) fn cast_signature_json(signature: &CastSignature) -> Value {
    json!({
        "playOptionId": signature.play_option().0,
        "form": spell_form_json(signature.form()),
        "modeIds": signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
        "alternativeCostId": signature.costs().alternative().map(|cost| cost.0),
        "additionalCostIds": signature
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect::<Vec<_>>(),
        "x": signature.x(),
        "targetSelections": target_selections_json(signature.targets()),
    })
}

pub(super) const fn mana_color_name(color: ManaColor) -> &'static str {
    match color {
        ManaColor::White => "white",
        ManaColor::Blue => "blue",
        ManaColor::Black => "black",
        ManaColor::Red => "red",
        ManaColor::Green => "green",
        ManaColor::Colorless => "colorless",
    }
}

pub(super) fn ability_origin_json(origin: AbilityOrigin) -> Value {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } => json!({
            "kind": "printed",
            "definition": definition.0,
            "partId": part.0,
            "abilityId": ability.0,
        }),
        AbilityOrigin::IntrinsicBasicLand(land_type) => json!({
            "kind": "intrinsicBasicLand",
            "landType": basic_land_type_name(land_type),
        }),
        AbilityOrigin::Granted {
            source,
            source_definition,
            source_part,
            source_ability,
            grant,
        } => json!({
            "kind": "granted",
            "source": source.0,
            "sourceDefinition": source_definition.0,
            "sourcePartId": source_part.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
    }
}

const fn basic_land_type_name(land_type: BasicLandType) -> &'static str {
    match land_type {
        BasicLandType::Plains => "plains",
        BasicLandType::Island => "island",
        BasicLandType::Swamp => "swamp",
        BasicLandType::Mountain => "mountain",
        BasicLandType::Forest => "forest",
    }
}
