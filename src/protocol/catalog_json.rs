use serde_json::{Value, json};

use super::json_common::{
    alternate_spell_kind_name, double_faced_kind_name, spell_form_json, target_predicate_name,
};
use super::{ENGINE_VERSION, PROTOCOL_CAPABILITIES, PROTOCOL_VERSION, SIMULATION_FINGERPRINT};
use crate::card::{
    CardDefinition, CardRules, CardSet, CardStructure, HybridPair, ImplementationStatus, ManaCost,
    ModeDef, PlayActionKind, PlayOptionDef, PlayRestriction, TargetSlotDef,
};
use crate::{CardCatalog, CardPart, Format};

fn mana_cost_json(cost: &ManaCost) -> Value {
    json!({
        "generic": cost.generic,
        "white": cost.white,
        "blue": cost.blue,
        "black": cost.black,
        "red": cost.red,
        "green": cost.green,
        // One entry per pair the cost actually carries, so a client renders
        // the printed symbols without knowing every pair in the game.
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

const fn card_set_slug(set: CardSet) -> &'static str {
    match set {
        CardSet::Alpha => "alpha",
        CardSet::Beta => "beta",
        CardSet::Unlimited => "unlimited",
        CardSet::CollectorsEdition => "collectors-edition",
        CardSet::InternationalCollectorsEdition => "international-collectors-edition",
        CardSet::ArabianNights => "arabian-nights",
        CardSet::Antiquities => "antiquities",
        CardSet::Revised => "revised",
        CardSet::Legends => "legends",
        CardSet::TheDark => "the-dark",
        CardSet::FallenEmpires => "fallen-empires",
        CardSet::Promo1994 => "promo-1994",
        CardSet::IceAge => "ice-age",
        CardSet::Mirage => "mirage",
        CardSet::Visions => "visions",
        CardSet::Tempest => "tempest",
        CardSet::Stronghold => "stronghold",
        CardSet::PortalSecondAge => "portal-second-age",
        CardSet::UrzasSaga => "urzas-saga",
        CardSet::MercadianMasques => "mercadian-masques",
        CardSet::Nemesis => "nemesis",
        CardSet::Invasion => "invasion",
        CardSet::Planeshift => "planeshift",
        CardSet::Apocalypse => "apocalypse",
        CardSet::Odyssey => "odyssey",
        CardSet::Judgment => "judgment",
        CardSet::Onslaught => "onslaught",
        CardSet::Darksteel => "darksteel",
        CardSet::PlanarChaos => "planar-chaos",
        CardSet::FutureSight => "future-sight",
        CardSet::Innistrad => "innistrad",
        CardSet::DarkAscension => "dark-ascension",
        CardSet::AvacynRestored => "avacyn-restored",
        CardSet::Magic2013 => "magic-2013",
        CardSet::ReturnToRavnica => "return-to-ravnica",
        CardSet::Gatecrash => "gatecrash",
        CardSet::DragonsMaze => "dragons-maze",
        CardSet::Magic2014 => "magic-2014",
        CardSet::Theros => "theros",
        CardSet::ModernHorizons2 => "modern-horizons-2",
        CardSet::Token => "token",
    }
}

fn rules_json(rules: &CardRules, mana_cost: Option<&ManaCost>) -> Value {
    let stats = rules.creature_stats();
    json!({
        "kind": rules.kind_name(),
        "typeLine": rules.type_line(),
        "manaCost": mana_cost.map(mana_cost_json),
        "power": stats.map(|stats| stats.power),
        "toughness": stats.map(|stats| stats.toughness),
        "rulesText": rules.rules_text(),
        "implementationStatus": implementation_status_name(rules.implementation_status()),
        "colors": rules.colors(),
    })
}

fn structure_json(structure: &CardStructure) -> Value {
    match structure {
        CardStructure::Single { main } => json!({
            "kind": "single",
            "mainPartId": main.0,
        }),
        CardStructure::Split { parts, fused } => json!({
            "kind": "split",
            "partIds": parts.iter().map(|part| part.0).collect::<Vec<_>>(),
            "fusedPlayOptionId": fused.map(|option| option.0),
        }),
        CardStructure::Flip { normal, flipped } => json!({
            "kind": "flip",
            "normalPartId": normal.0,
            "flippedPartId": flipped.0,
        }),
        CardStructure::DoubleFaced { front, back, kind } => json!({
            "kind": "doubleFaced",
            "frontPartId": front.0,
            "backPartId": back.0,
            "doubleFacedKind": double_faced_kind_name(*kind),
        }),
        CardStructure::AlternateSpell {
            main,
            alternate,
            kind,
        } => json!({
            "kind": "alternateSpell",
            "mainPartId": main.0,
            "alternatePartId": alternate.0,
            "alternateSpellKind": alternate_spell_kind_name(*kind),
        }),
        CardStructure::MeldPart { front, recipe } => json!({
            "kind": "meldPart",
            "frontPartId": front.0,
            "meldRecipeId": recipe.0,
        }),
    }
}

fn target_slot_json(slot: &TargetSlotDef) -> Value {
    json!({
        "id": slot.id.0,
        "label": slot.label,
        "predicate": target_predicate_name(slot.predicate),
        "minimum": slot.minimum,
        "maximum": slot.maximum,
    })
}

fn mode_json(mode: &ModeDef) -> Value {
    json!({
        "id": mode.id.0,
        "label": mode.label,
        "targets": mode.targets.iter().map(target_slot_json).collect::<Vec<_>>(),
    })
}

fn play_option_json(option: &PlayOptionDef) -> Value {
    json!({
        "id": option.id.0,
        "label": option.label,
        "action": match option.action {
            PlayActionKind::CastSpell => "CastSpell",
            PlayActionKind::PlayLand => "PlayLand",
        },
        "form": spell_form_json(&option.form),
        "manaCost": option.mana_cost.as_ref().map(mana_cost_json),
        "restriction": match option.restriction {
            PlayRestriction::Normal => "normal",
            PlayRestriction::FromHandOnly => "fromHandOnly",
            PlayRestriction::BeforeCombatDamage => "beforeCombatDamage",
        },
        "modes": option.modes.as_ref().map(|modes| json!({
            "minimum": modes.minimum,
            "maximum": modes.maximum,
            "mayRepeat": modes.may_repeat,
            "choices": modes.modes.iter().map(mode_json).collect::<Vec<_>>(),
        })),
        "targets": option.targets.iter().map(target_slot_json).collect::<Vec<_>>(),
        "alternativeCosts": option.alternative_costs.iter().map(|cost| json!({
            "id": cost.id.0,
            "label": cost.label,
            "manaCost": mana_cost_json(&cost.mana_cost),
        })).collect::<Vec<_>>(),
        "additionalCosts": option.additional_costs.iter().map(|cost| json!({
            "id": cost.id.0,
            "label": cost.label,
            "manaCost": cost.mana_cost.as_ref().map(mana_cost_json),
        })).collect::<Vec<_>>(),
    })
}

fn definition_json(catalog: &CardCatalog, format: Format, card: &CardDefinition) -> Value {
    let rules = &card.rules;
    let stats = rules.creature_stats();
    let mana_cost = card.primary_part().and_then(CardPart::mana_cost);
    let allowed = catalog.is_allowed_in(card.id, format);
    let banned = catalog.is_banned_in(card.id, format);
    let restricted = catalog.is_restricted_in(card.id, format);
    json!({
        // Compatibility fields retained from protocol v1.
        "definition": card.id.0,
        "name": card.name,
        "kind": rules.kind_name(),
        "isBasicLand": card.is_basic_land(),
        "manaCost": mana_cost.as_ref().map(mana_cost_json),
        "power": stats.map(|stats| stats.power),
        "toughness": stats.map(|stats| stats.toughness),
        "rulesText": rules.rules_text(),
        "banned": banned,
        "restricted": restricted,
        // Protocol v2 structured and format-aware metadata.
        "allowed": allowed,
        "legal": allowed && !banned,
        "debutSet": card_set_slug(card.debut_set),
        "implementationStatus": implementation_status_name(card.implementation_status()),
        "structure": structure_json(&card.structure),
        "parts": card.parts.iter().map(|part| {
            let mana_cost = part.mana_cost();
            let mut value = rules_json(&part.rules, mana_cost.as_ref());
            let Value::Object(fields) = &mut value else {
                unreachable!("rules JSON is always an object");
            };
            fields.insert("id".into(), Value::from(part.id.0));
            fields.insert("name".into(), Value::from(part.name.clone()));
            value
        }).collect::<Vec<_>>(),
        "playOptions": card.play_options.iter().map(play_option_json).collect::<Vec<_>>(),
        "printings": card.printings.iter().map(|printing| json!({
            "set": card_set_slug(printing.id.set),
            "variant": printing.id.variant,
        })).collect::<Vec<_>>(),
    })
}

/// Serializes every card definition for the default Old School format.
#[must_use]
pub fn catalog_json(catalog: &CardCatalog) -> Value {
    catalog_json_for_format(catalog, Format::OldSchool9394)
}

/// Serializes every canonical definition, its structured play metadata, and
/// legality in `format`. Printings remain metadata and never duplicate a card
/// definition in the returned list.
#[must_use]
pub fn catalog_json_for_format(catalog: &CardCatalog, format: Format) -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "protocolCapabilities": PROTOCOL_CAPABILITIES,
        "engineVersion": ENGINE_VERSION,
        "simulationFingerprint": SIMULATION_FINGERPRINT,
        "format": format.slug(),
        "formatName": format.display_name(),
        "cards": catalog
            .definitions()
            .into_iter()
            .map(|card| definition_json(catalog, format, card))
            .collect::<Vec<_>>(),
    })
}
