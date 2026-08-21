use serde_json::{Value, json};

use crate::card::{
    AlternateSpellKind, BasicLandType, CardArt, CardRules, DoubleFacedKind, HybridPair,
    ImplementationStatus, SpellForm, TargetPredicate, TokenStructure,
};
use crate::casting::{CastChoices, CastSignature};
use crate::{
    AbilityOrigin, AttackDefender, CardCatalog, DecisionVisibility, DecisionZone, GameObjectId,
    ManaColor, ObjectCharacteristics, PlayerId, Step, Target,
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

fn card_art_json(art: CardArt) -> Value {
    json!({
        "scryfallId": art.scryfall_id,
        "artist": art.artist,
    })
}

fn mana_cost_json(cost: crate::ManaCost) -> Value {
    json!({
        "generic": cost.generic,
        "white": cost.white,
        "blue": cost.blue,
        "black": cost.black,
        "red": cost.red,
        "green": cost.green,
        "colorless": cost.colorless,
        "hybrid": HybridPair::ALL
            .into_iter()
            .filter(|pair| cost.hybrid[pair.index()] > 0)
            .map(|pair| json!({ "symbol": pair.symbol(), "count": cost.hybrid[pair.index()] }))
            .collect::<Vec<_>>(),
        "variableX": cost.variable_x,
        "xMultiplier": cost.x_multiplier,
    })
}

const fn implementation_status_name(status: ImplementationStatus) -> &'static str {
    match status {
        ImplementationStatus::Complete => "complete",
        ImplementationStatus::Partial => "partial",
        ImplementationStatus::MetadataOnly => "metadataOnly",
    }
}

fn rules_presentation_json(rules: &CardRules) -> Value {
    let stats = rules.creature_stats();
    json!({
        "kind": rules.kind_name(),
        "typeLine": rules.type_line(),
        "manaCost": rules.mana_cost().map(mana_cost_json),
        "power": stats.map(|stats| stats.power),
        "toughness": stats.map(|stats| stats.toughness),
        "rulesText": rules.rules_text(),
        "implementationStatus": implementation_status_name(rules.implementation_status()),
        "colors": rules.colors(),
        "isLand": rules.has_type(crate::card::CardType::Land),
    })
}

fn token_structure_json(structure: TokenStructure) -> Value {
    match structure {
        TokenStructure::Single => json!({
            "kind": "single",
            "mainPartId": crate::CardPartId::PRIMARY.0,
        }),
        TokenStructure::TransformingDoubleFaced { back } => json!({
            "kind": "transformingDoubleFaced",
            "frontPartId": crate::CardPartId::PRIMARY.0,
            "backPartId": back.id.0,
        }),
    }
}

pub(super) fn object_characteristics_name(
    catalog: &CardCatalog,
    characteristics: ObjectCharacteristics,
) -> Option<String> {
    match characteristics {
        ObjectCharacteristics::Card { definition, part } => catalog.get(definition).map(|card| {
            card.part(part)
                .map_or_else(|| card.name.clone(), |part| part.name.clone())
        }),
        ObjectCharacteristics::Token { token, part } => Some(
            token
                .part(part)
                .unwrap_or_else(|| token.primary_part())
                .name()
                .into_owned(),
        ),
        ObjectCharacteristics::Emblem { emblem } => Some(emblem.name().to_owned()),
        ObjectCharacteristics::FaceDown { face_down } => Some(face_down.display_name().to_owned()),
    }
}

/// Stable public presentation identity for either a catalog-backed card or
/// creator-owned virtual characteristics. Printed objects deliberately stay
/// compact: their rules and art join through the catalog. Tokens and emblems
/// have no catalog identity, so their complete display data travels inline.
pub(super) fn object_characteristics_json(characteristics: ObjectCharacteristics) -> Value {
    match characteristics {
        ObjectCharacteristics::Card { definition, part } => json!({
            "kind": "printed",
            "definition": definition.0,
            "partId": part.0,
        }),
        ObjectCharacteristics::Token { token, part } => {
            let current = token.part(part).unwrap_or_else(|| token.primary_part());
            let rules = current.rules();
            let mut value = json!({
                "kind": "token",
                "partId": current.id.0,
                "name": current.name(),
                "structure": token_structure_json(token.structure),
                "presentation": rules_presentation_json(&rules),
            });
            if let Some(art) = token.art {
                value["art"] = card_art_json(art);
            }
            value
        }
        ObjectCharacteristics::Emblem { emblem } => {
            json!({
                "kind": "emblem",
                "name": emblem.name(),
                "presentation": emblem_presentation_json(emblem),
            })
        }
        ObjectCharacteristics::FaceDown { face_down } => json!({
            "kind": "faceDown",
            "name": face_down.display_name(),
            "presentation": rules_presentation_json(&face_down.rules()),
        }),
    }
}

fn emblem_presentation_json(emblem: crate::EmblemCharacteristics) -> Value {
    json!({
        "kind": "Emblem",
        "typeLine": "Emblem",
        "manaCost": null,
        "power": null,
        "toughness": null,
        "rulesText": emblem.rules_text(),
        "implementationStatus": implementation_status_name(emblem.implementation_status()),
        "colors": [],
        "isLand": false,
    })
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
        AbilityOrigin::Token { part, ability } => json!({
            "kind": "token",
            "partId": part.0,
            "abilityId": ability.0,
        }),
        AbilityOrigin::Emblem { ability } => json!({
            "kind": "emblem",
            "abilityId": ability.0,
        }),
        AbilityOrigin::FaceDown { ability } => json!({
            "kind": "faceDown",
            "abilityId": ability.0,
        }),
        AbilityOrigin::IntrinsicBasicLand(land_type) => json!({
            "kind": "intrinsicBasicLand",
            "landType": basic_land_type_name(land_type),
        }),
        AbilityOrigin::IntrinsicCounter(kind) => json!({
            "kind": "intrinsicCounter",
            "counter": kind.name(),
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
        AbilityOrigin::TokenGranted {
            source,
            source_part,
            source_ability,
            grant,
        } => json!({
            "kind": "tokenGranted",
            "source": source.0,
            "sourcePartId": source_part.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
        AbilityOrigin::EmblemGranted {
            source,
            source_ability,
            grant,
        } => json!({
            "kind": "emblemGranted",
            "source": source.0,
            "sourceAbilityId": source_ability.0,
            "grantId": grant.0,
        }),
        AbilityOrigin::FaceDownGranted {
            source,
            source_ability,
            grant,
        } => json!({
            "kind": "faceDownGranted",
            "source": source.0,
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
