use serde_json::{Value, json};

use super::json_common::{
    alternate_spell_kind_name, double_faced_kind_name, spell_form_json, target_predicate_name,
};
use super::{ENGINE_VERSION, PROTOCOL_CAPABILITIES, PROTOCOL_VERSION, SIMULATION_FINGERPRINT};
use crate::card::{
    CardDefinition, CardRules, CardSet, CardStructure, FlexibleManaSymbol, ImplementationStatus,
    ManaCost, ModeDef, PlayActionKind, PlayOptionDef, PlayRestriction, TargetSlotDef,
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
        "colorless": cost.colorless,
        // One entry per flexible symbol the cost actually carries, so a
        // client need not know the engine's complete symbol vocabulary.
        "hybrid": FlexibleManaSymbol::ALL
            .into_iter()
            .filter(|symbol| cost.flexible_count(*symbol) > 0)
            .map(|symbol| json!({
                "symbol": symbol.symbol(),
                "count": cost.flexible_count(symbol),
            }))
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

// Long because it is a table: one line per set, and the list only ever
// grows. The source-organization map it mirrors is marked the same way.
#[allow(clippy::too_many_lines)]
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
        CardSet::FourthEdition => "fourth-edition",
        CardSet::IceAge => "ice-age",
        CardSet::Chronicles => "chronicles",
        CardSet::Homelands => "homelands",
        CardSet::Alliances => "alliances",
        CardSet::Mirage => "mirage",
        CardSet::Visions => "visions",
        CardSet::FifthEdition => "fifth-edition",
        CardSet::Weatherlight => "weatherlight",
        CardSet::Tempest => "tempest",
        CardSet::Stronghold => "stronghold",
        CardSet::Exodus => "exodus",
        CardSet::PortalSecondAge => "portal-second-age",
        CardSet::UrzasSaga => "urzas-saga",
        CardSet::UrzasLegacy => "urzas-legacy",
        CardSet::ClassicSixthEdition => "classic-sixth-edition",
        CardSet::UrzasDestiny => "urzas-destiny",
        CardSet::MercadianMasques => "mercadian-masques",
        CardSet::Nemesis => "nemesis",
        CardSet::Prophecy => "prophecy",
        CardSet::Invasion => "invasion",
        CardSet::Planeshift => "planeshift",
        CardSet::SeventhEdition => "seventh-edition",
        CardSet::Apocalypse => "apocalypse",
        CardSet::Odyssey => "odyssey",
        CardSet::Torment => "torment",
        CardSet::Judgment => "judgment",
        CardSet::Onslaught => "onslaught",
        CardSet::Legions => "legions",
        CardSet::Scourge => "scourge",
        CardSet::Mirrodin => "mirrodin",
        CardSet::Darksteel => "darksteel",
        CardSet::FifthDawn => "fifth-dawn",
        CardSet::ChampionsOfKamigawa => "champions_of_kamigawa",
        CardSet::BetrayersOfKamigawa => "betrayers_of_kamigawa",
        CardSet::MirrodinBesieged => "mirrodin-besieged",
        CardSet::NewPhyrexia => "new-phyrexia",
        CardSet::PlanarChaos => "planar-chaos",
        CardSet::FutureSight => "future-sight",
        CardSet::Lorwyn => "lorwyn",
        CardSet::Conflux => "conflux",
        CardSet::Zendikar => "zendikar",
        CardSet::Worldwake => "worldwake",
        CardSet::WarOfTheSpark => "war-of-the-spark",
        CardSet::ThroneOfEldraine => "throne-of-eldraine",
        CardSet::TherosBeyondDeath => "theros-beyond-death",
        CardSet::ZendikarRising => "zendikar-rising",
        CardSet::Shadowmoor => "shadowmoor",
        CardSet::Eventide => "eventide",
        CardSet::ShardsOfAlara => "shards-of-alara",
        CardSet::Ixalan => "ixalan",
        CardSet::Battlebond => "battlebond",
        CardSet::ScarsOfMirrodin => "scars-of-mirrodin",
        CardSet::Magic2011 => "magic-2011",
        CardSet::RiseOfTheEldrazi => "rise-of-the-eldrazi",
        CardSet::Innistrad => "innistrad",
        CardSet::DarkAscension => "dark-ascension",
        CardSet::AvacynRestored => "avacyn-restored",
        CardSet::Magic2012 => "magic-2012",
        CardSet::Magic2013 => "magic-2013",
        CardSet::ReturnToRavnica => "return-to-ravnica",
        CardSet::Gatecrash => "gatecrash",
        CardSet::DragonsMaze => "dragons-maze",
        CardSet::Magic2014 => "magic-2014",
        CardSet::Magic2020 => "magic-2020",
        CardSet::Theros => "theros",
        CardSet::Planechase2012 => "planechase-2012",
        CardSet::Commander2013 => "commander-2013",
        CardSet::JourneyIntoNyx => "journey-into-nyx",
        CardSet::Conspiracy => "conspiracy",
        CardSet::Magic2015 => "magic-2015",
        CardSet::Commander2014 => "commander-2014",
        CardSet::Commander2015 => "commander-2015",
        CardSet::KhansOfTarkir => "khans-of-tarkir",
        CardSet::DragonsOfTarkir => "dragons-of-tarkir",
        CardSet::ModernHorizons1 => "modern-horizons-1",
        CardSet::Kaldheim => "kaldheim",
        CardSet::Commander2018 => "commander-2018",
        CardSet::Commander2021 => "commander-2021",
        CardSet::StrixhavenSchoolOfMages => "strixhaven-school-of-mages",
        CardSet::AdventuresInTheForgottenRealms => "adventures-in-the-forgotten-realms",
        CardSet::ModernHorizons2 => "modern-horizons-2",
        CardSet::InnistradMidnightHunt => "innistrad-midnight-hunt",
        CardSet::InnistradCrimsonVow => "innistrad-crimson-vow",
        CardSet::InnistradCrimsonVowCommander => "innistrad-crimson-vow-commander",
        CardSet::Ikoria => "ikoria",
        CardSet::KamigawaNeonDynasty => "kamigawa-neon-dynasty",
        CardSet::KamigawaNeonDynastyCommander => "kamigawa-neon-dynasty-commander",
        CardSet::StreetsOfNewCapenna => "streets-of-new-capenna",
        CardSet::StreetsOfNewCapennaCommander => "streets-of-new-capenna-commander",
        CardSet::CommanderLegendsBattleForBaldursGate => "commander-legends-baldurs-gate",
        CardSet::Dominaria => "dominaria",
        CardSet::DominariaUnited => "dominaria-united",
        CardSet::TheBrothersWar => "the-brothers-war",
        CardSet::EternalMasters => "eternal-masters",
        CardSet::EldritchMoon => "eldritch-moon",
        CardSet::ConspiracyTakeTheCrown => "conspiracy-take-the-crown",
        CardSet::Kaladesh => "kaladesh",
        CardSet::AetherRevolt => "aether-revolt",
        CardSet::Amonkhet => "amonkhet",
        CardSet::PhyrexiaAllWillBeOne => "phyrexia-all-will-be-one",
        CardSet::PhyrexiaAllWillBeOneCommander => "phyrexia-all-will-be-one-commander",
        CardSet::MarchOfTheMachine => "march-of-the-machine",
        CardSet::LordOfTheRings => "lord-of-the-rings",
        CardSet::LordOfTheRingsCommander => "lord-of-the-rings-commander",
        CardSet::WildsOfEldraine => "wilds-of-eldraine",
        CardSet::LostCavernsOfIxalan => "lost-caverns-of-ixalan",
        CardSet::MurdersAtKarlovManor => "murders-at-karlov-manor",
        CardSet::RavnicaClueEdition => "ravnica-clue-edition",
        CardSet::Fallout => "fallout",
        CardSet::ModernHorizons3 => "modern-horizons-3",
        CardSet::OutlawsOfThunderJunction => "outlaws-of-thunder-junction",
        CardSet::TheBigScore => "the-big-score",
        CardSet::ModernHorizons3Commander => "modern-horizons-3-commander",
        CardSet::Bloomburrow => "bloomburrow",
        CardSet::BloomburrowCommander => "bloomburrow-commander",
        CardSet::DuskmournHouseOfHorror => "duskmourn-house-of-horror",
        CardSet::DuskmournHouseOfHorrorCommander => "duskmourn-house-of-horror-commander",
        CardSet::FoundationsJumpstart => "foundations-jumpstart",
        CardSet::TarkirDragonstorm => "tarkir-dragonstorm",
        CardSet::Aetherdrift => "aetherdrift",
        CardSet::FinalFantasy => "final-fantasy",
        CardSet::FinalFantasyCommander => "final-fantasy-commander",
        CardSet::ThroughTheOmenpaths => "through-the-omenpaths",
        CardSet::SaviorsOfKamigawa => "saviors-of-kamigawa",
        CardSet::RavnicaCityOfGuilds => "ravnica-city-of-guilds",
        CardSet::Guildpact => "guildpact",
        CardSet::Dissension => "dissension",
        CardSet::TimeSpiral => "time-spiral",
        CardSet::AlaraReborn => "alara-reborn",
        CardSet::FateReforged => "fate-reforged",
        CardSet::BattleForZendikar => "battle-for-zendikar",
        CardSet::MagicOrigins => "magic-origins",
        CardSet::ShadowsOverInnistrad => "shadows-over-innistrad",
        CardSet::HourOfDevastation => "hour-of-devastation",
        CardSet::CoreSet2019 => "core-set-2019",
        CardSet::RavnicaAllegiance => "ravnica-allegiance",
        CardSet::Commander2020 => "commander-2020",
        CardSet::MagicFoundations => "magic-foundations",
        CardSet::MarvelsSpiderMan => "marvels-spider-man",
        CardSet::AvatarTheLastAirbender => "avatar-the-last-airbender",
        CardSet::EdgeOfEternities => "edge-of-eternities",
        CardSet::EdgeOfEternitiesCommander => "edge-of-eternities-commander",
        CardSet::LorwynEclipsed => "lorwyn-eclipsed",
        CardSet::SecretsOfStrixhaven => "secrets-of-strixhaven",
        CardSet::TeenageMutantNinjaTurtles => "teenage-mutant-ninja-turtles",
        CardSet::MarvelSuperHeroes => "marvel-super-heroes",
        CardSet::PortalThreeKingdoms => "portal-three-kingdoms",
        CardSet::Coldsnap => "coldsnap",
        CardSet::BornOfTheGods => "born-of-the-gods",
        CardSet::Commander2017 => "commander-2017",
        CardSet::CommanderLegends => "commander-legends",
        CardSet::DominariaUnitedCommander => "dominaria-united-commander",
        CardSet::MarchOfTheMachineCommander => "march-of-the-machine-commander",
        CardSet::LostCavernsOfIxalanCommander => "lost-caverns-of-ixalan-commander",
        CardSet::GuildsOfRavnica => "guilds-of-ravnica",
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
        CardStructure::Room {
            doors,
            combined,
            locked,
        } => json!({
            "kind": "room",
            "doors": doors.iter().map(|part| part.0).collect::<Vec<_>>(),
            "combined": combined.0,
            "locked": locked.0,
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
    let mut value = json!({
        "id": mode.id.0,
        "label": mode.label,
        "targets": mode.targets.iter().map(target_slot_json).collect::<Vec<_>>(),
    });
    if let Some(cost) = mode.additional_mana_cost {
        value["additionalManaCost"] = mana_cost_json(&cost);
    }
    value
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
            PlayRestriction::BeforeBlockersDeclared => "beforeBlockersDeclared",
            PlayRestriction::OpponentsUpkeep => "opponentsUpkeep",
            PlayRestriction::DeclareAttackersStep => "declareAttackersStep",
            PlayRestriction::OpponentsTurnAfterUpkeep => "opponentsTurnAfterUpkeep",
        },
        "modes": option.modes.as_ref().map(|modes| {
            let mut value = json!({
                "minimum": modes.minimum,
                "maximum": modes.maximum,
                "mayRepeat": modes.may_repeat,
                "choices": modes.modes.iter().map(mode_json).collect::<Vec<_>>(),
            });
            // Optional, and present only for a spell whose printed maximum
            // rises under a condition -- "if you control a Wizard as you cast
            // this spell, you may choose two instead". Every other modal
            // spell's shape is unchanged, and the legal actions already show
            // which selections are available right now.
            if let Some(conditional) = modes.conditional_maximum {
                value["conditionalMaximum"] = json!(conditional.maximum);
            }
            value
        }),
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
        "definition": card.id.get(),
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
