use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::SET_MODULES;
use crate::card::CardCatalog;
use crate::{CardSet, Format};

const DECLARATION_PREFIX: &str = "pub(in crate::card::sets) static ";
const HEADER_PREFIX: &str = "// ";
const HEADER_SEPARATOR: &str = " — ";
const AUDIT_PREFIX: &str = "// Audit: ";
const ADDITIONAL_REGISTRY_PREFIX: &str =
    "pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceEntry {
    symbol: Option<String>,
    collector_number: String,
    audit: Option<SourceAudit>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AuditStatus {
    Partial,
    MetadataOnly,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SourceAudit {
    pub(super) set: CardSet,
    pub(super) name: String,
    pub(super) status: AuditStatus,
    pub(super) gap: String,
}

#[test]
fn printed_set_sources_follow_collector_number_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();
    assert!(
        !files.is_empty(),
        "at least one printed set source must exist"
    );

    let mut source_sets = HashSet::new();
    let mut source_definitions = 0;
    let mut source_additional_printings = 0;
    for path in files {
        let set_source = set_source_for_file(&path);
        assert!(
            source_sets.insert(set_source.set),
            "{:?} has more than one printed set source",
            set_source.set
        );
        let source = fs::read_to_string(&path).expect("a printed set source file is readable");
        let entries = source_entries(&source, set_source, &path);
        source_definitions += entries
            .iter()
            .filter(|entry| entry.symbol.is_some())
            .count();

        for cards in entries.windows(2) {
            assert_eq!(
                natural_collector_cmp(&cards[0].collector_number, &cards[1].collector_number),
                Ordering::Less,
                "{}: collector number {} is not before {}",
                path.display(),
                cards[0].collector_number,
                cards[1].collector_number
            );
        }

        let registry = registry_symbols(&source, &path);
        let declaration_symbols = entries
            .iter()
            .filter_map(|card| card.symbol.as_deref())
            .collect::<Vec<_>>();
        assert_eq!(
            registry,
            declaration_symbols,
            "{}: CARDS must exactly mirror declaration order",
            path.display()
        );

        let additional_printings = additional_printings(&source, &path);
        source_additional_printings += additional_printings.len();
        if !additional_printings.is_empty() {
            for printing in &additional_printings {
                assert_eq!(
                    printing.0,
                    set_source.code,
                    "{}: wrong set code on an ADDITIONAL_PRINTINGS entry",
                    path.display()
                );
            }
            for printings in additional_printings.windows(2) {
                assert_ne!(
                    natural_collector_cmp(printings[0].1, printings[1].1),
                    Ordering::Greater,
                    "{}: additional printing {} is after {}",
                    path.display(),
                    printings[0].1,
                    printings[1].1
                );
            }
        }
    }

    let registered_printed_modules = SET_MODULES
        .iter()
        .filter(|module| module.set != CardSet::Token)
        .collect::<Vec<_>>();
    assert_eq!(
        source_sets,
        registered_printed_modules
            .iter()
            .map(|module| module.set)
            .collect(),
        "printed source files and registered set modules must correspond",
    );
    assert_eq!(
        source_definitions,
        registered_printed_modules
            .iter()
            .map(|module| module.cards.len())
            .sum::<usize>(),
        "source declarations and registered definitions must correspond",
    );
    assert_eq!(
        source_additional_printings,
        registered_printed_modules
            .iter()
            .map(|module| module.additional_printings.len())
            .sum::<usize>(),
        "source and registered additional printings must correspond",
    );
}

fn printed_set_files(sets: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for year in fs::read_dir(sets).expect("card set directory exists") {
        let path = year.expect("year directory entry is readable").path();
        if !path.is_dir()
            || !path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with('y'))
        {
            continue;
        }
        for entry in fs::read_dir(path).expect("year directory is readable") {
            let path = entry.expect("set file entry is readable").path();
            if path.extension().is_some_and(|extension| extension == "rs")
                && !path.ends_with("mod.rs")
            {
                files.push(path);
            }
        }
    }
    files
}

#[derive(Clone, Copy)]
struct SetSource {
    set: CardSet,
    code: &'static str,
}

// Long because it is a table: one line per set module, and the list only
// ever grows. Splitting it would put half the sets somewhere else for no
// reason a reader would thank us for.
#[allow(clippy::too_many_lines)]
fn set_source_for_file(path: &Path) -> SetSource {
    let source = |set, code| SetSource { set, code };
    match path.file_name().and_then(|name| name.to_str()) {
        Some("alpha.rs") => source(CardSet::Alpha, "LEA"),
        Some("arabian_nights.rs") => source(CardSet::ArabianNights, "ARN"),
        Some("beta.rs") => source(CardSet::Beta, "LEB"),
        Some("unlimited.rs") => source(CardSet::Unlimited, "2ED"),
        Some("collectors_edition.rs") => source(CardSet::CollectorsEdition, "CED"),
        Some("international_collectors_edition.rs") => {
            source(CardSet::InternationalCollectorsEdition, "CEI")
        }
        Some("antiquities.rs") => source(CardSet::Antiquities, "ATQ"),
        Some("revised.rs") => source(CardSet::Revised, "3ED"),
        Some("fallen_empires.rs") => source(CardSet::FallenEmpires, "FEM"),
        Some("legends.rs") => source(CardSet::Legends, "LEG"),
        Some("promo_1994.rs") => source(CardSet::Promo1994, "P94"),
        Some("the_dark.rs") => source(CardSet::TheDark, "DRK"),
        Some("chronicles.rs") => source(CardSet::Chronicles, "CHR"),
        Some("fourth_edition.rs") => source(CardSet::FourthEdition, "4ED"),
        Some("ice_age.rs") => source(CardSet::IceAge, "ICE"),
        Some("alliances.rs") => source(CardSet::Alliances, "ALL"),
        Some("mirage.rs") => source(CardSet::Mirage, "MIR"),
        Some("visions.rs") => source(CardSet::Visions, "VIS"),
        Some("tempest.rs") => source(CardSet::Tempest, "TMP"),
        Some("weatherlight.rs") => source(CardSet::Weatherlight, "WTH"),
        Some("stronghold.rs") => source(CardSet::Stronghold, "STH"),
        Some("portal_second_age.rs") => source(CardSet::PortalSecondAge, "P02"),
        Some("urzas_saga.rs") => source(CardSet::UrzasSaga, "USG"),
        Some("urzas_legacy.rs") => source(CardSet::UrzasLegacy, "ULG"),
        Some("urzas_destiny.rs") => source(CardSet::UrzasDestiny, "UDS"),
        Some("mercadian_masques.rs") => source(CardSet::MercadianMasques, "MMQ"),
        Some("nemesis.rs") => source(CardSet::Nemesis, "NEM"),
        Some("invasion.rs") => source(CardSet::Invasion, "INV"),
        Some("planeshift.rs") => source(CardSet::Planeshift, "PLS"),
        Some("seventh_edition.rs") => source(CardSet::SeventhEdition, "7ED"),
        Some("apocalypse.rs") => source(CardSet::Apocalypse, "APC"),
        Some("odyssey.rs") => source(CardSet::Odyssey, "ODY"),
        Some("judgment.rs") => source(CardSet::Judgment, "JUD"),
        Some("torment.rs") => source(CardSet::Torment, "TOR"),
        Some("onslaught.rs") => source(CardSet::Onslaught, "ONS"),
        Some("legions.rs") => source(CardSet::Legions, "LGN"),
        Some("scourge.rs") => source(CardSet::Scourge, "SCG"),
        Some("mirrodin.rs") => source(CardSet::Mirrodin, "MRD"),
        Some("darksteel.rs") => source(CardSet::Darksteel, "DST"),
        Some("fifth_dawn.rs") => source(CardSet::FifthDawn, "5DN"),
        Some("champions_of_kamigawa.rs") => source(CardSet::ChampionsOfKamigawa, "CHK"),
        Some("betrayers_of_kamigawa.rs") => source(CardSet::BetrayersOfKamigawa, "BOK"),
        Some("mirrodin_besieged.rs") => source(CardSet::MirrodinBesieged, "MBS"),
        Some("new_phyrexia.rs") => source(CardSet::NewPhyrexia, "NPH"),
        Some("future_sight.rs") => source(CardSet::FutureSight, "FUT"),
        Some("lorwyn.rs") => source(CardSet::Lorwyn, "LRW"),
        Some("planar_chaos.rs") => source(CardSet::PlanarChaos, "PLC"),
        Some("conflux.rs") => source(CardSet::Conflux, "CON"),
        Some("zendikar.rs") => source(CardSet::Zendikar, "ZEN"),
        Some("worldwake.rs") => source(CardSet::Worldwake, "WWK"),
        Some("shadowmoor.rs") => source(CardSet::Shadowmoor, "SHM"),
        Some("eventide.rs") => source(CardSet::Eventide, "EVE"),
        Some("shards_of_alara.rs") => source(CardSet::ShardsOfAlara, "ALA"),
        Some("ixalan.rs") => source(CardSet::Ixalan, "XLN"),
        Some("battlebond.rs") => source(CardSet::Battlebond, "BBD"),
        Some("scars_of_mirrodin.rs") => source(CardSet::ScarsOfMirrodin, "SOM"),
        Some("magic_2011.rs") => source(CardSet::Magic2011, "M11"),
        Some("rise_of_the_eldrazi.rs") => source(CardSet::RiseOfTheEldrazi, "ROE"),
        Some("innistrad.rs") => source(CardSet::Innistrad, "ISD"),
        Some("avacyn_restored.rs") => source(CardSet::AvacynRestored, "AVR"),
        Some("dark_ascension.rs") => source(CardSet::DarkAscension, "DKA"),
        Some("magic_2012.rs") => source(CardSet::Magic2012, "M12"),
        Some("magic_2013.rs") => source(CardSet::Magic2013, "M13"),
        Some("return_to_ravnica.rs") => source(CardSet::ReturnToRavnica, "RTR"),
        Some("dragons_maze.rs") => source(CardSet::DragonsMaze, "DGM"),
        Some("gatecrash.rs") => source(CardSet::Gatecrash, "GTC"),
        Some("magic_2014.rs") => source(CardSet::Magic2014, "M14"),
        Some("magic_2020.rs") => source(CardSet::Magic2020, "M20"),
        Some("theros.rs") => source(CardSet::Theros, "THS"),
        Some("planechase_2012.rs") => source(CardSet::Planechase2012, "PC2"),
        Some("commander_2013.rs") => source(CardSet::Commander2013, "C13"),
        Some("journey_into_nyx.rs") => source(CardSet::JourneyIntoNyx, "JOU"),
        Some("conspiracy.rs") => source(CardSet::Conspiracy, "CNS"),
        Some("magic_2015.rs") => source(CardSet::Magic2015, "M15"),
        Some("commander_2014.rs") => source(CardSet::Commander2014, "C14"),
        Some("commander_2015.rs") => source(CardSet::Commander2015, "C15"),
        Some("khans_of_tarkir.rs") => source(CardSet::KhansOfTarkir, "KTK"),
        Some("dragons_of_tarkir.rs") => source(CardSet::DragonsOfTarkir, "DTK"),
        Some("modern_horizons.rs") => source(CardSet::ModernHorizons1, "MH1"),
        Some("war_of_the_spark.rs") => source(CardSet::WarOfTheSpark, "WAR"),
        Some("throne_of_eldraine.rs") => source(CardSet::ThroneOfEldraine, "ELD"),
        Some("theros_beyond_death.rs") => source(CardSet::TherosBeyondDeath, "THB"),
        Some("zendikar_rising.rs") => source(CardSet::ZendikarRising, "ZNR"),
        Some("kaldheim.rs") => source(CardSet::Kaldheim, "KHM"),
        Some("commander_2021.rs") => source(CardSet::Commander2021, "C21"),
        Some("strixhaven_school_of_mages.rs") => source(CardSet::StrixhavenSchoolOfMages, "STX"),
        Some("modern_horizons_2.rs") => source(CardSet::ModernHorizons2, "MH2"),
        Some("adventures_in_the_forgotten_realms.rs") => {
            source(CardSet::AdventuresInTheForgottenRealms, "AFR")
        }
        Some("innistrad_midnight_hunt.rs") => source(CardSet::InnistradMidnightHunt, "MID"),
        Some("innistrad_crimson_vow.rs") => source(CardSet::InnistradCrimsonVow, "VOW"),
        Some("innistrad_crimson_vow_commander.rs") => {
            source(CardSet::InnistradCrimsonVowCommander, "VOC")
        }
        Some("ikoria.rs") => source(CardSet::Ikoria, "IKO"),
        Some("kamigawa_neon_dynasty.rs") => source(CardSet::KamigawaNeonDynasty, "NEO"),
        Some("kamigawa_neon_dynasty_commander.rs") => {
            source(CardSet::KamigawaNeonDynastyCommander, "NEC")
        }
        Some("streets_of_new_capenna.rs") => source(CardSet::StreetsOfNewCapenna, "SNC"),
        Some("commander_legends_baldurs_gate.rs") => {
            source(CardSet::CommanderLegendsBattleForBaldursGate, "CLB")
        }
        Some("dominaria_united.rs") => source(CardSet::DominariaUnited, "DMU"),
        Some("the_brothers_war.rs") => source(CardSet::TheBrothersWar, "BRO"),
        Some("eternal_masters.rs") => source(CardSet::EternalMasters, "EMA"),
        Some("eldritch_moon.rs") => source(CardSet::EldritchMoon, "EMN"),
        Some("conspiracy_take_the_crown.rs") => source(CardSet::ConspiracyTakeTheCrown, "CN2"),
        Some("kaladesh.rs") => source(CardSet::Kaladesh, "KLD"),
        Some("aether_revolt.rs") => source(CardSet::AetherRevolt, "AER"),
        Some("amonkhet.rs") => source(CardSet::Amonkhet, "AKH"),
        Some("phyrexia_all_will_be_one.rs") => source(CardSet::PhyrexiaAllWillBeOne, "ONE"),
        Some("phyrexia_all_will_be_one_commander.rs") => {
            source(CardSet::PhyrexiaAllWillBeOneCommander, "ONC")
        }
        Some("march_of_the_machine.rs") => source(CardSet::MarchOfTheMachine, "MOM"),
        Some("lord_of_the_rings.rs") => source(CardSet::LordOfTheRings, "LTR"),
        Some("lord_of_the_rings_commander.rs") => source(CardSet::LordOfTheRingsCommander, "LTC"),
        Some("wilds_of_eldraine.rs") => source(CardSet::WildsOfEldraine, "WOE"),
        Some("lost_caverns_of_ixalan.rs") => source(CardSet::LostCavernsOfIxalan, "LCI"),
        Some("murders_at_karlov_manor.rs") => source(CardSet::MurdersAtKarlovManor, "MKM"),
        Some("ravnica_clue_edition.rs") => source(CardSet::RavnicaClueEdition, "CLU"),
        Some("fallout.rs") => source(CardSet::Fallout, "PIP"),
        Some("modern_horizons_3.rs") => source(CardSet::ModernHorizons3, "MH3"),
        Some("outlaws_of_thunder_junction.rs") => source(CardSet::OutlawsOfThunderJunction, "OTJ"),
        Some("the_big_score.rs") => source(CardSet::TheBigScore, "BIG"),
        Some("modern_horizons_3_commander.rs") => source(CardSet::ModernHorizons3Commander, "M3C"),
        Some("bloomburrow.rs") => source(CardSet::Bloomburrow, "BLB"),
        Some("bloomburrow_commander.rs") => source(CardSet::BloomburrowCommander, "BLC"),
        Some("duskmourn_house_of_horror.rs") => source(CardSet::DuskmournHouseOfHorror, "DSK"),
        Some("duskmourn_house_of_horror_commander.rs") => {
            source(CardSet::DuskmournHouseOfHorrorCommander, "DSC")
        }
        Some("foundations_jumpstart.rs") => source(CardSet::FoundationsJumpstart, "J25"),
        Some("tarkir_dragonstorm.rs") => source(CardSet::TarkirDragonstorm, "TDM"),
        Some("aetherdrift.rs") => source(CardSet::Aetherdrift, "DFT"),
        Some("final_fantasy.rs") => source(CardSet::FinalFantasy, "FIN"),
        Some("final_fantasy_commander.rs") => source(CardSet::FinalFantasyCommander, "FIC"),
        Some("through_the_omenpaths.rs") => source(CardSet::ThroughTheOmenpaths, "OM1"),
        Some("homelands.rs") => source(CardSet::Homelands, "HML"),
        Some("fifth_edition.rs") => source(CardSet::FifthEdition, "5ED"),
        Some("exodus.rs") => source(CardSet::Exodus, "EXO"),
        Some("classic_sixth_edition.rs") => source(CardSet::ClassicSixthEdition, "6ED"),
        Some("prophecy.rs") => source(CardSet::Prophecy, "PCY"),
        Some("saviors_of_kamigawa.rs") => source(CardSet::SaviorsOfKamigawa, "SOK"),
        Some("guildpact.rs") => source(CardSet::Guildpact, "GPT"),
        Some("dissension.rs") => source(CardSet::Dissension, "DIS"),
        Some("time_spiral.rs") => source(CardSet::TimeSpiral, "TSP"),
        Some("alara_reborn.rs") => source(CardSet::AlaraReborn, "ARB"),
        Some("fate_reforged.rs") => source(CardSet::FateReforged, "FRF"),
        Some("battle_for_zendikar.rs") => source(CardSet::BattleForZendikar, "BFZ"),
        Some("magic_origins.rs") => source(CardSet::MagicOrigins, "ORI"),
        Some("shadows_over_innistrad.rs") => source(CardSet::ShadowsOverInnistrad, "SOI"),
        Some("hour_of_devastation.rs") => source(CardSet::HourOfDevastation, "HOU"),
        Some("core_set_2019.rs") => source(CardSet::CoreSet2019, "M19"),
        Some("ravnica_allegiance.rs") => source(CardSet::RavnicaAllegiance, "RNA"),
        Some("commander_2020.rs") => source(CardSet::Commander2020, "C20"),
        Some("magic_foundations.rs") => source(CardSet::MagicFoundations, "FDN"),
        Some("avatar_the_last_airbender.rs") => source(CardSet::AvatarTheLastAirbender, "TLA"),
        Some("edge_of_eternities.rs") => source(CardSet::EdgeOfEternities, "EOE"),
        Some("lorwyn_eclipsed.rs") => source(CardSet::LorwynEclipsed, "ECL"),
        Some("secrets_of_strixhaven.rs") => source(CardSet::SecretsOfStrixhaven, "SOS"),
        Some("teenage_mutant_ninja_turtles.rs") => {
            source(CardSet::TeenageMutantNinjaTurtles, "TLE")
        }
        Some(name) => panic!(
            "{}: add {name} to the official set-code map",
            path.display()
        ),
        None => panic!("{}: set source has no UTF-8 file name", path.display()),
    }
}

pub(super) fn all_source_audits(root: &Path) -> Vec<SourceAudit> {
    let mut files = printed_set_files(&root.join("src/card/sets"));
    files.sort();
    files
        .into_iter()
        .flat_map(|path| {
            let set_source = set_source_for_file(&path);
            let source = fs::read_to_string(&path).expect("a printed set source file is readable");
            source_entries(&source, set_source, &path)
                .into_iter()
                .filter_map(|entry| entry.audit)
        })
        .collect()
}

pub(super) fn source_audits_for_format(
    root: &Path,
    catalog: &CardCatalog,
    format: Format,
) -> Vec<SourceAudit> {
    all_source_audits(root)
        .into_iter()
        .filter(|audit| {
            format.allows_set(audit.set)
                || catalog
                    .find_by_name(&audit.name)
                    .is_some_and(|id| catalog.is_allowed_in(id, format))
        })
        .collect()
}

fn source_entries(source: &str, set_source: SetSource, path: &Path) -> Vec<SourceEntry> {
    let lines = source.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with(AUDIT_PREFIX) {
            assert!(
                parse_audit(line).is_some(),
                "{}:{}: expected exact `// Audit: blocked|partial|metadata-only — GAP` comment",
                path.display(),
                index + 1
            );
            assert!(
                index > 0 && parse_header(lines[index - 1]).is_some(),
                "{}:{}: an Audit comment must immediately follow a card header",
                path.display(),
                index + 1
            );
        }
        if let Some(symbol) = declaration_symbol(line) {
            let directly_headered = index > 0 && parse_header(lines[index - 1]).is_some();
            let audited_header = index > 1
                && parse_audit(lines[index - 1])
                    .is_some_and(|(status, _)| status != AuditStatus::Blocked)
                && parse_header(lines[index - 2]).is_some();
            assert!(
                directly_headered || audited_header,
                "{}:{}: expected a card header, optionally followed by a partial or metadata-only Audit comment, immediately before {symbol}",
                path.display(),
                index + 1
            );
        }
    }

    let mut entries = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(header) = parse_header(line) else {
            continue;
        };
        assert_eq!(
            header.0,
            set_source.code,
            "{}:{}: wrong set code in card header",
            path.display(),
            index + 1
        );

        let (symbol, audit) = match lines.get(index + 1).copied() {
            Some(next) if declaration_symbol(next).is_some() => {
                let symbol = declaration_symbol(next).expect("the declaration was recognized");
                validate_declaration(&lines, index + 1, symbol, header.2, path);
                (Some(symbol.to_string()), None)
            }
            Some(next) if parse_audit(next).is_some() => {
                let (status, gap) = parse_audit(next).expect("the Audit comment was recognized");
                let declaration = lines
                    .get(index + 2)
                    .and_then(|line| declaration_symbol(line));
                match status {
                    AuditStatus::Blocked => assert!(
                        declaration.is_none(),
                        "{}:{}: a blocked Audit entry cannot have a CardRecord declaration",
                        path.display(),
                        index + 1
                    ),
                    AuditStatus::Partial | AuditStatus::MetadataOnly => assert!(
                        declaration.is_some(),
                        "{}:{}: a partial or metadata-only Audit entry must immediately precede a CardRecord declaration",
                        path.display(),
                        index + 1
                    ),
                }
                if let Some(symbol) = declaration {
                    validate_declaration(&lines, index + 2, symbol, header.2, path);
                }
                (
                    declaration.map(str::to_string),
                    Some(SourceAudit {
                        set: set_source.set,
                        name: header.2.to_string(),
                        status,
                        gap: gap.to_string(),
                    }),
                )
            }
            _ => {
                panic!(
                    "{}:{}: a card header must immediately precede either a CardRecord declaration or an Audit comment",
                    path.display(),
                    index + 1
                )
            }
        };
        entries.push(SourceEntry {
            symbol,
            collector_number: header.1.to_string(),
            audit,
        });
    }
    entries
}

fn declaration_symbol(line: &str) -> Option<&str> {
    let declaration = line.strip_prefix(DECLARATION_PREFIX)?;
    declaration
        .split_once(": CardRecord")
        .map(|(symbol, _)| symbol)
}

fn validate_declaration(
    lines: &[&str],
    index: usize,
    _symbol: &str,
    header_name: &str,
    path: &Path,
) {
    let initializer_index = (index..lines.len().min(index + 3))
        .find(|candidate| lines[*candidate].trim().ends_with('('))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: CardRecord declaration is missing its constructor",
                path.display(),
                index + 1
            )
        });
    let identity = lines
        .get(initializer_index + 1)
        .map_or("", |line| line.trim());
    assert!(
        identity
            .strip_suffix(',')
            .unwrap_or(identity)
            .replace('_', "")
            .parse::<u64>()
            .is_ok()
            || identity.starts_with("PrintingAnchor::scryfall("),
        "{}:{}: expected a legacy ID or immutable printing anchor",
        path.display(),
        initializer_index + 2
    );
    let name = lines
        .get(initializer_index + 2)
        .and_then(|line| line.trim().strip_prefix('"'))
        .and_then(|line| line.strip_suffix("\","))
        .unwrap_or_else(|| {
            panic!(
                "{}:{}: expected a one-line canonical card name",
                path.display(),
                initializer_index + 3
            )
        });
    assert_eq!(
        header_name,
        name,
        "{}:{}: header name must match CardRecord name",
        path.display(),
        index + 1
    );
}

fn parse_header(line: &str) -> Option<(&str, &str, &str)> {
    let body = line.strip_prefix(HEADER_PREFIX)?;
    let (identity, name) = body.split_once(HEADER_SEPARATOR)?;
    let (set_code, collector_number) = identity.split_once(' ')?;
    if set_code.is_empty()
        || !set_code
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        || collector_number.is_empty()
        || collector_number.chars().any(char::is_whitespace)
        || name.is_empty()
    {
        return None;
    }
    Some((set_code, collector_number, name))
}

fn parse_audit(line: &str) -> Option<(AuditStatus, &str)> {
    let body = line.strip_prefix(AUDIT_PREFIX)?;
    let (status, gap) = body.split_once(HEADER_SEPARATOR)?;
    if gap.is_empty() {
        return None;
    }
    let status = match status {
        "blocked" => AuditStatus::Blocked,
        "partial" => AuditStatus::Partial,
        "metadata-only" => AuditStatus::MetadataOnly,
        _ => return None,
    };
    Some((status, gap))
}

fn registry_symbols<'a>(source: &'a str, path: &Path) -> Vec<&'a str> {
    const REGISTRY_DECLARATION: &str = "pub(in crate::card::sets) static CARDS: &[&CardRecord] =";

    let start = source
        .find(REGISTRY_DECLARATION)
        .unwrap_or_else(|| panic!("{}: CARDS registry is missing", path.display()));
    let body = source[start + REGISTRY_DECLARATION.len()..]
        .trim_start()
        .strip_prefix("&[")
        .unwrap_or_else(|| panic!("{}: CARDS registry is malformed", path.display()));
    let body = body.split_once("];").map_or_else(
        || panic!("{}: CARDS registry is malformed", path.display()),
        |(body, _)| body,
    );
    body.split(',')
        .filter_map(|entry| {
            let entry = entry.trim();
            (!entry.is_empty()).then_some(entry)
        })
        .map(|entry| {
            entry
                .strip_prefix('&')
                .unwrap_or_else(|| panic!("{}: malformed CARDS entry {entry:?}", path.display()))
        })
        .collect()
}

fn additional_printings<'a>(source: &'a str, path: &Path) -> Vec<(&'a str, &'a str)> {
    let start = source.find(ADDITIONAL_REGISTRY_PREFIX).unwrap_or_else(|| {
        panic!(
            "{}: ADDITIONAL_PRINTINGS registry is missing",
            path.display()
        )
    });
    let body = &source[start + ADDITIONAL_REGISTRY_PREFIX.len()..];
    let body = body.split_once("];").map_or_else(
        || {
            panic!(
                "{}: ADDITIONAL_PRINTINGS registry is malformed",
                path.display()
            )
        },
        |(body, _)| body,
    );

    body.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let entry = line.trim();
            let (expression, comment) = entry.split_once("// ").unwrap_or_else(|| {
                panic!(
                    "{}: ADDITIONAL_PRINTINGS entry needs an EOL `// SET NUMBER` comment: {entry:?}",
                    path.display()
                )
            });
            assert!(
                !comment.contains("// "),
                "{}: malformed ADDITIONAL_PRINTINGS comment {comment:?}",
                path.display()
            );
            let expression = expression.trim_end().strip_suffix(',').unwrap_or_else(|| {
                panic!(
                    "{}: ADDITIONAL_PRINTINGS expression must end in a comma: {entry:?}",
                    path.display()
                )
            });
            assert!(
                (expression.starts_with("PrintingRecord::reprint(")
                    || expression.starts_with("PrintingRecord::alternate("))
                    && expression.ends_with(')'),
                "{}: malformed ADDITIONAL_PRINTINGS expression {expression:?}",
                path.display()
            );

            let (set_code, collector_number) = comment.split_once(' ').unwrap_or_else(|| {
                panic!(
                    "{}: expected exact `// SET NUMBER` comment, got {comment:?}",
                    path.display()
                )
            });
            assert!(
                !set_code.is_empty()
                    && set_code
                        .bytes()
                        .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
                    && !collector_number.is_empty()
                    && !collector_number.chars().any(char::is_whitespace),
                "{}: expected exact `// SET NUMBER` comment, got {comment:?}",
                path.display()
            );
            (set_code, collector_number)
        })
        .collect()
}

fn natural_collector_cmp(left: &str, right: &str) -> Ordering {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left.len() && right_index < right.len() {
        let left_is_digit = left[left_index].is_ascii_digit();
        let right_is_digit = right[right_index].is_ascii_digit();
        let left_end = run_end(left, left_index, left_is_digit);
        let right_end = run_end(right, right_index, right_is_digit);
        let left_run = &left[left_index..left_end];
        let right_run = &right[right_index..right_end];

        let order = if left_is_digit && right_is_digit {
            compare_digit_runs(left_run, right_run)
        } else {
            left_run.cmp(right_run)
        };
        if order != Ordering::Equal {
            return order;
        }
        left_index = left_end;
        right_index = right_end;
    }

    match (left_index == left.len(), right_index == right.len()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => unreachable!("the comparison loop stops only at the end of a value"),
    }
}

fn run_end(value: &[u8], start: usize, is_digit: bool) -> usize {
    value[start..]
        .iter()
        .position(|byte| byte.is_ascii_digit() != is_digit)
        .map_or(value.len(), |offset| start + offset)
}

fn compare_digit_runs(left: &[u8], right: &[u8]) -> Ordering {
    let left_significant = significant_digits(left);
    let right_significant = significant_digits(right);
    left_significant
        .len()
        .cmp(&right_significant.len())
        .then_with(|| left_significant.cmp(right_significant))
        .then_with(|| left.len().cmp(&right.len()))
        .then_with(|| left.cmp(right))
}

fn significant_digits(value: &[u8]) -> &[u8] {
    let first_nonzero = value
        .iter()
        .position(|byte| *byte != b'0')
        .unwrap_or(value.len() - 1);
    &value[first_nonzero..]
}
