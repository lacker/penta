//! Built-in card records, grouped by release year and set.
//!
//! Each canonical card is defined in one set module. Records default to a
//! complete implementation and explicitly carry a reason when they are partial
//! or metadata-only. Reprints and alternate-art variants point back to that
//! canonical record from their own set module.

mod y1993;
mod y1994;
mod y1995;
mod y1996;
mod y1997;
mod y1998;
mod y1999;
mod y2000;
mod y2001;
mod y2002;
mod y2003;
mod y2004;
mod y2005;
mod y2007;
mod y2008;
mod y2009;
mod y2010;
mod y2011;
mod y2012;
mod y2013;
mod y2014;
mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;
mod y2024;
mod y2025;

use super::record::{CardAbilityBinding, CardRecord, PrintingAnchor, PrintingRecord};
use crate::AbilityOrigin;
use crate::card::{AbilityDef, CardBehavior, CardDefinition, CardPrinting, CardRules, CardSet};
use crate::game::{PileChosen, PilesSeparated};

static UNSUPPORTED_RULES: CardRules = CardRules::unsupported();

struct SetModule {
    set: CardSet,
    cards: &'static [&'static CardRecord],
    additional_printings: &'static [PrintingRecord],
}

impl SetModule {
    const fn new(
        set: CardSet,
        cards: &'static [&'static CardRecord],
        additional_printings: &'static [PrintingRecord],
    ) -> Self {
        Self {
            set,
            cards,
            additional_printings,
        }
    }
}

/// Every cataloged set has one source module. `cards` contains definitions
/// introduced by that module; `additional_printings` contains reprints and
/// further variants of definitions introduced elsewhere.
const SET_MODULES: &[SetModule] = &[
    SetModule::new(
        CardSet::Alpha,
        y1993::alpha::CARDS,
        y1993::alpha::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Beta,
        y1993::beta::CARDS,
        y1993::beta::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Unlimited,
        y1993::unlimited::CARDS,
        y1993::unlimited::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::CollectorsEdition,
        y1993::collectors_edition::CARDS,
        y1993::collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InternationalCollectorsEdition,
        y1993::international_collectors_edition::CARDS,
        y1993::international_collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ArabianNights,
        y1993::arabian_nights::CARDS,
        y1993::arabian_nights::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Antiquities,
        y1994::antiquities::CARDS,
        y1994::antiquities::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Revised,
        y1994::revised::CARDS,
        y1994::revised::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Legends,
        y1994::legends::CARDS,
        y1994::legends::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TheDark,
        y1994::the_dark::CARDS,
        y1994::the_dark::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FallenEmpires,
        y1994::fallen_empires::CARDS,
        y1994::fallen_empires::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Promo1994,
        y1994::promo_1994::CARDS,
        y1994::promo_1994::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FourthEdition,
        y1995::fourth_edition::CARDS,
        y1995::fourth_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::IceAge,
        y1995::ice_age::CARDS,
        y1995::ice_age::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Chronicles,
        y1995::chronicles::CARDS,
        y1995::chronicles::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Alliances,
        y1996::alliances::CARDS,
        y1996::alliances::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Mirage,
        y1996::mirage::CARDS,
        y1996::mirage::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Visions,
        y1997::visions::CARDS,
        y1997::visions::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Weatherlight,
        y1997::weatherlight::CARDS,
        y1997::weatherlight::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Tempest,
        y1997::tempest::CARDS,
        y1997::tempest::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Stronghold,
        y1998::stronghold::CARDS,
        y1998::stronghold::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::PortalSecondAge,
        y1998::portal_second_age::CARDS,
        y1998::portal_second_age::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::UrzasSaga,
        y1998::urzas_saga::CARDS,
        y1998::urzas_saga::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::UrzasLegacy,
        y1999::urzas_legacy::CARDS,
        y1999::urzas_legacy::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::UrzasDestiny,
        y1999::urzas_destiny::CARDS,
        y1999::urzas_destiny::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::MercadianMasques,
        y1999::mercadian_masques::CARDS,
        y1999::mercadian_masques::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Nemesis,
        y2000::nemesis::CARDS,
        y2000::nemesis::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Invasion,
        y2000::invasion::CARDS,
        y2000::invasion::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Planeshift,
        y2001::planeshift::CARDS,
        y2001::planeshift::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::SeventhEdition,
        y2001::seventh_edition::CARDS,
        y2001::seventh_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Apocalypse,
        y2001::apocalypse::CARDS,
        y2001::apocalypse::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Odyssey,
        y2001::odyssey::CARDS,
        y2001::odyssey::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Torment,
        y2002::torment::CARDS,
        y2002::torment::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Judgment,
        y2002::judgment::CARDS,
        y2002::judgment::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Onslaught,
        y2002::onslaught::CARDS,
        y2002::onslaught::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Legions,
        y2003::legions::CARDS,
        y2003::legions::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Scourge,
        y2003::scourge::CARDS,
        y2003::scourge::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Mirrodin,
        y2003::mirrodin::CARDS,
        y2003::mirrodin::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Darksteel,
        y2004::darksteel::CARDS,
        y2004::darksteel::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FifthDawn,
        y2004::fifth_dawn::CARDS,
        y2004::fifth_dawn::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ChampionsOfKamigawa,
        y2004::champions_of_kamigawa::CARDS,
        y2004::champions_of_kamigawa::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::BetrayersOfKamigawa,
        y2005::betrayers_of_kamigawa::CARDS,
        y2005::betrayers_of_kamigawa::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::PlanarChaos,
        y2007::planar_chaos::CARDS,
        y2007::planar_chaos::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FutureSight,
        y2007::future_sight::CARDS,
        y2007::future_sight::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Lorwyn,
        y2007::lorwyn::CARDS,
        y2007::lorwyn::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::MirrodinBesieged,
        y2011::mirrodin_besieged::CARDS,
        y2011::mirrodin_besieged::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::NewPhyrexia,
        y2011::new_phyrexia::CARDS,
        y2011::new_phyrexia::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Conflux,
        y2009::conflux::CARDS,
        y2009::conflux::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Zendikar,
        y2009::zendikar::CARDS,
        y2009::zendikar::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Shadowmoor,
        y2008::shadowmoor::CARDS,
        y2008::shadowmoor::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Eventide,
        y2008::eventide::CARDS,
        y2008::eventide::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ShardsOfAlara,
        y2008::shards_of_alara::CARDS,
        y2008::shards_of_alara::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Worldwake,
        y2010::worldwake::CARDS,
        y2010::worldwake::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ScarsOfMirrodin,
        y2010::scars_of_mirrodin::CARDS,
        y2010::scars_of_mirrodin::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2011,
        y2010::magic_2011::CARDS,
        y2010::magic_2011::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::RiseOfTheEldrazi,
        y2010::rise_of_the_eldrazi::CARDS,
        y2010::rise_of_the_eldrazi::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2012,
        y2011::magic_2012::CARDS,
        y2011::magic_2012::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Innistrad,
        y2011::innistrad::CARDS,
        y2011::innistrad::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DarkAscension,
        y2012::dark_ascension::CARDS,
        y2012::dark_ascension::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AvacynRestored,
        y2012::avacyn_restored::CARDS,
        y2012::avacyn_restored::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2013,
        y2012::magic_2013::CARDS,
        y2012::magic_2013::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ReturnToRavnica,
        y2012::return_to_ravnica::CARDS,
        y2012::return_to_ravnica::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Gatecrash,
        y2013::gatecrash::CARDS,
        y2013::gatecrash::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DragonsMaze,
        y2013::dragons_maze::CARDS,
        y2013::dragons_maze::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2014,
        y2013::magic_2014::CARDS,
        y2013::magic_2014::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Theros,
        y2013::theros::CARDS,
        y2013::theros::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Commander2013,
        y2013::commander_2013::CARDS,
        y2013::commander_2013::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::JourneyIntoNyx,
        y2014::journey_into_nyx::CARDS,
        y2014::journey_into_nyx::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Conspiracy,
        y2014::conspiracy::CARDS,
        y2014::conspiracy::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2015,
        y2014::magic_2015::CARDS,
        y2014::magic_2015::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Commander2014,
        y2014::commander_2014::CARDS,
        y2014::commander_2014::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::KhansOfTarkir,
        y2014::khans_of_tarkir::CARDS,
        y2014::khans_of_tarkir::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DragonsOfTarkir,
        y2015::dragons_of_tarkir::CARDS,
        y2015::dragons_of_tarkir::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Commander2015,
        y2015::commander_2015::CARDS,
        y2015::commander_2015::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Ixalan,
        y2017::ixalan::CARDS,
        y2017::ixalan::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Battlebond,
        y2018::battlebond::CARDS,
        y2018::battlebond::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2020,
        y2019::magic_2020::CARDS,
        y2019::magic_2020::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ModernHorizons1,
        y2019::modern_horizons::CARDS,
        y2019::modern_horizons::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::WarOfTheSpark,
        y2019::war_of_the_spark::CARDS,
        y2019::war_of_the_spark::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ThroneOfEldraine,
        y2019::throne_of_eldraine::CARDS,
        y2019::throne_of_eldraine::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TherosBeyondDeath,
        y2020::theros_beyond_death::CARDS,
        y2020::theros_beyond_death::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ZendikarRising,
        y2020::zendikar_rising::CARDS,
        y2020::zendikar_rising::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Ikoria,
        y2020::ikoria::CARDS,
        y2020::ikoria::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Kaldheim,
        y2021::kaldheim::CARDS,
        y2021::kaldheim::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Commander2021,
        y2021::commander_2021::CARDS,
        y2021::commander_2021::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::StrixhavenSchoolOfMages,
        y2021::strixhaven_school_of_mages::CARDS,
        y2021::strixhaven_school_of_mages::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ModernHorizons2,
        y2021::modern_horizons_2::CARDS,
        y2021::modern_horizons_2::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AdventuresInTheForgottenRealms,
        y2021::adventures_in_the_forgotten_realms::CARDS,
        y2021::adventures_in_the_forgotten_realms::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InnistradMidnightHunt,
        y2021::innistrad_midnight_hunt::CARDS,
        y2021::innistrad_midnight_hunt::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InnistradCrimsonVowCommander,
        y2021::innistrad_crimson_vow_commander::CARDS,
        y2021::innistrad_crimson_vow_commander::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::KamigawaNeonDynasty,
        y2022::kamigawa_neon_dynasty::CARDS,
        y2022::kamigawa_neon_dynasty::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::StreetsOfNewCapenna,
        y2022::streets_of_new_capenna::CARDS,
        y2022::streets_of_new_capenna::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::CommanderLegendsBattleForBaldursGate,
        y2022::commander_legends_baldurs_gate::CARDS,
        y2022::commander_legends_baldurs_gate::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DominariaUnited,
        y2022::dominaria_united::CARDS,
        y2022::dominaria_united::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::EternalMasters,
        y2016::eternal_masters::CARDS,
        y2016::eternal_masters::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::EldritchMoon,
        y2016::eldritch_moon::CARDS,
        y2016::eldritch_moon::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ConspiracyTakeTheCrown,
        y2016::conspiracy_take_the_crown::CARDS,
        y2016::conspiracy_take_the_crown::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Kaladesh,
        y2016::kaladesh::CARDS,
        y2016::kaladesh::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AetherRevolt,
        y2017::aether_revolt::CARDS,
        y2017::aether_revolt::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::PhyrexiaAllWillBeOne,
        y2023::phyrexia_all_will_be_one::CARDS,
        y2023::phyrexia_all_will_be_one::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::MarchOfTheMachine,
        y2023::march_of_the_machine::CARDS,
        y2023::march_of_the_machine::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::LordOfTheRings,
        y2023::lord_of_the_rings::CARDS,
        y2023::lord_of_the_rings::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::WildsOfEldraine,
        y2023::wilds_of_eldraine::CARDS,
        y2023::wilds_of_eldraine::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::MurdersAtKarlovManor,
        y2024::murders_at_karlov_manor::CARDS,
        y2024::murders_at_karlov_manor::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::RavnicaClueEdition,
        y2024::ravnica_clue_edition::CARDS,
        y2024::ravnica_clue_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Fallout,
        y2024::fallout::CARDS,
        y2024::fallout::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::LostCavernsOfIxalan,
        y2023::lost_caverns_of_ixalan::CARDS,
        y2023::lost_caverns_of_ixalan::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ModernHorizons3,
        y2024::modern_horizons_3::CARDS,
        y2024::modern_horizons_3::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::OutlawsOfThunderJunction,
        y2024::outlaws_of_thunder_junction::CARDS,
        y2024::outlaws_of_thunder_junction::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TheBigScore,
        y2024::the_big_score::CARDS,
        y2024::the_big_score::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ModernHorizons3Commander,
        y2024::modern_horizons_3_commander::CARDS,
        y2024::modern_horizons_3_commander::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Bloomburrow,
        y2024::bloomburrow::CARDS,
        y2024::bloomburrow::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::BloomburrowCommander,
        y2024::bloomburrow_commander::CARDS,
        y2024::bloomburrow_commander::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DuskmournHouseOfHorror,
        y2024::duskmourn_house_of_horror::CARDS,
        y2024::duskmourn_house_of_horror::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DuskmournHouseOfHorrorCommander,
        y2024::duskmourn_house_of_horror_commander::CARDS,
        y2024::duskmourn_house_of_horror_commander::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FoundationsJumpstart,
        y2024::foundations_jumpstart::CARDS,
        y2024::foundations_jumpstart::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TarkirDragonstorm,
        y2025::tarkir_dragonstorm::CARDS,
        y2025::tarkir_dragonstorm::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Aetherdrift,
        y2025::aetherdrift::CARDS,
        y2025::aetherdrift::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FinalFantasy,
        y2025::final_fantasy::CARDS,
        y2025::final_fantasy::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FinalFantasyCommander,
        y2025::final_fantasy_commander::CARDS,
        y2025::final_fantasy_commander::ADDITIONAL_PRINTINGS,
    ),
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let capacity = SET_MODULES.iter().map(|module| module.cards.len()).sum();
    let mut definitions = Vec::with_capacity(capacity);
    for module in SET_MODULES {
        definitions.extend(module.cards.iter().map(|record| record.definition()));
    }
    definitions
}

pub(crate) fn ability_binding(
    origin: AbilityOrigin,
    actual: &AbilityDef,
) -> Option<&'static CardAbilityBinding> {
    let AbilityOrigin::Printed {
        definition,
        part,
        ability,
    } = origin
    else {
        return None;
    };
    SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
        .find(|record| record.id() == definition)?
        .ability_bindings
        .iter()
        .find(|binding| {
            binding.part == part && binding.ability == ability && binding.expected == *actual
        })
}

pub(super) fn additional_printings() -> Vec<CardPrinting> {
    SET_MODULES
        .iter()
        .flat_map(|module| {
            module
                .additional_printings
                .iter()
                .map(|record| record.printing(module.set))
        })
        .collect()
}

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
pub(super) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::AugurOfBolas => &y2012::magic_2013::AUGUR_OF_BOLAS.rules,
        CardBehavior::BloodBaronOfVizkopa => &y2013::dragons_maze::BLOOD_BARON_OF_VIZKOPA.rules,
        CardBehavior::ChainLightning => &y1994::legends::CHAIN_LIGHTNING.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::Fork => &y1993::alpha::FORK.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::ReflectingPool => &y1997::tempest::REFLECTING_POOL.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::Channel => &y1993::alpha::CHANNEL.rules,
        CardBehavior::EssenceScatter => &y2012::magic_2013::ESSENCE_SCATTER.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::SylvanLibrary => &y1994::legends::SYLVAN_LIBRARY.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::GrislySalvage => &y2012::return_to_ravnica::GRISLY_SALVAGE.rules,
        CardBehavior::KirdApe => &y1993::arabian_nights::KIRD_APE.rules,
        CardBehavior::Moat => &y1994::legends::MOAT.rules,
        CardBehavior::Mulch => &y2011::innistrad::MULCH.rules,
        CardBehavior::Negate => &y2012::magic_2013::NEGATE.rules,
        CardBehavior::PillarOfFlame => &y2012::avacyn_restored::PILLAR_OF_FLAME.rules,
        CardBehavior::SedgeTroll => &y1993::alpha::SEDGE_TROLL.rules,
        CardBehavior::SphinxsRevelation => &y2012::return_to_ravnica::SPHINXS_REVELATION.rules,
        CardBehavior::TetravusDetach | CardBehavior::TetravusAssemble => {
            &y1994::antiquities::TETRAVUS.rules
        }
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

pub(crate) fn piles_separated_resolver(key: &str) -> Option<PilesSeparated> {
    match key {
        "lilianaOfTheVeil.pilesSeparated" => Some(y2011::innistrad::LILIANA_PILES_SEPARATED),
        _ => None,
    }
}

pub(crate) fn pile_chosen_resolver(key: &str) -> Option<PileChosen> {
    match key {
        "lilianaOfTheVeil.pileChosen" => Some(y2011::innistrad::LILIANA_PILE_CHOSEN),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
