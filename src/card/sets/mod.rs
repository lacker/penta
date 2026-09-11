//! Built-in card records, reprints, alternate art, and unsupported entries grouped by set.

mod y1993;
pub use y1993::*;
mod y1994;
pub use y1994::*;
mod y1995;
pub use y1995::*;
mod y1996;
pub use y1996::*;
mod y1997;
pub use y1997::*;
mod y1998;
pub use y1998::*;
mod y1999;
pub use y1999::*;
mod y2000;
pub use y2000::*;
mod y2001;
pub use y2001::*;
mod y2002;
pub use y2002::*;
mod y2003;
pub use y2003::*;
mod y2004;
pub use y2004::*;
mod y2005;
pub use y2005::*;
mod y2006;
pub use y2006::*;
mod y2007;
pub use y2007::*;
mod y2008;
pub use y2008::*;
mod y2009;
pub use y2009::*;
mod y2010;
pub use y2010::*;
mod y2011;
pub use y2011::*;
mod y2012;
pub use y2012::*;
mod y2013;
pub use y2013::*;
mod y2014;
pub use y2014::*;
mod y2015;
pub use y2015::*;
mod y2016;
pub use y2016::*;
mod y2017;
pub use y2017::*;
mod y2018;
pub use y2018::*;
mod y2019;
pub use y2019::*;
mod y2020;
pub use y2020::*;
mod y2021;
pub use y2021::*;
mod y2022;
pub use y2022::*;
mod y2023;
pub use y2023::*;
mod y2024;
pub use y2024::*;
mod y2025;
pub use y2025::*;
mod y2026;
pub use y2026::*;

use super::record::{CardRecord, PrintingRecord};
use crate::card::{CardDefinition, CardPrinting, CardSet};

#[cfg(test)]
pub(crate) use y1994::antiquities::TETRAVITE;

pub(super) struct SetDefinition {
    set: CardSet,
    cards: &'static [&'static CardRecord],
    additional_printings: &'static [PrintingRecord],
    #[cfg(test)]
    source_path: &'static str,
}

impl SetDefinition {
    const fn new(
        set: CardSet,
        cards: &'static [&'static CardRecord],
        additional_printings: &'static [PrintingRecord],
        source_path: &'static str,
    ) -> Self {
        #[cfg(not(test))]
        let _ = source_path;
        Self {
            set,
            cards,
            additional_printings,
            #[cfg(test)]
            source_path,
        }
    }
}

/// Every cataloged set has one source module for definitions and additional printings.
const SET_MODULES: &[SetDefinition] = &[
    y1993::alpha::DEFINITION,
    y1993::beta::DEFINITION,
    y1993::unlimited::DEFINITION,
    y1993::collectors_edition::DEFINITION,
    y1993::international_collectors_edition::DEFINITION,
    y1993::arabian_nights::DEFINITION,
    y1994::antiquities::DEFINITION,
    y1994::revised::DEFINITION,
    y1994::legends::DEFINITION,
    y1994::the_dark::DEFINITION,
    y1994::fallen_empires::DEFINITION,
    y1994::harper_prism_book_promos::DEFINITION,
    y1994::dragon_con::DEFINITION,
    y1995::fourth_edition::DEFINITION,
    y1995::ice_age::DEFINITION,
    y1995::chronicles::DEFINITION,
    y1996::alliances::DEFINITION,
    y1996::mirage::DEFINITION,
    y1997::portal::DEFINITION,
    y1997::visions::DEFINITION,
    y1997::weatherlight::DEFINITION,
    y1997::tempest::DEFINITION,
    y1998::stronghold::DEFINITION,
    y1998::portal_second_age::DEFINITION,
    y1998::urzas_saga::DEFINITION,
    y1999::urzas_legacy::DEFINITION,
    y1999::urzas_destiny::DEFINITION,
    y1999::mercadian_masques::DEFINITION,
    y1999::starter_1999::DEFINITION,
    y2000::nemesis::DEFINITION,
    y2000::invasion::DEFINITION,
    y2001::planeshift::DEFINITION,
    y2001::seventh_edition::DEFINITION,
    y2001::apocalypse::DEFINITION,
    y2001::odyssey::DEFINITION,
    y2002::torment::DEFINITION,
    y2002::judgment::DEFINITION,
    y2002::onslaught::DEFINITION,
    y2003::legions::DEFINITION,
    y2003::scourge::DEFINITION,
    y2003::mirrodin::DEFINITION,
    y2004::darksteel::DEFINITION,
    y2004::fifth_dawn::DEFINITION,
    y2004::champions_of_kamigawa::DEFINITION,
    y2005::betrayers_of_kamigawa::DEFINITION,
    y2007::planar_chaos::DEFINITION,
    y2007::future_sight::DEFINITION,
    y2007::lorwyn::DEFINITION,
    y2008::morningtide::DEFINITION,
    y2011::mirrodin_besieged::DEFINITION,
    y2011::new_phyrexia::DEFINITION,
    y2009::conflux::DEFINITION,
    y2009::zendikar::DEFINITION,
    y2008::shadowmoor::DEFINITION,
    y2008::eventide::DEFINITION,
    y2008::shards_of_alara::DEFINITION,
    y2010::worldwake::DEFINITION,
    y2010::scars_of_mirrodin::DEFINITION,
    y2009::magic_2010::DEFINITION,
    y2010::magic_2011::DEFINITION,
    y2010::archenemy::DEFINITION,
    y2010::rise_of_the_eldrazi::DEFINITION,
    y2011::magic_2012::DEFINITION,
    y2011::commander_2011::DEFINITION,
    y2011::innistrad::DEFINITION,
    y2012::dark_ascension::DEFINITION,
    y2012::avacyn_restored::DEFINITION,
    y2012::magic_2013::DEFINITION,
    y2012::return_to_ravnica::DEFINITION,
    y2013::gatecrash::DEFINITION,
    y2013::dragons_maze::DEFINITION,
    y2013::magic_2014::DEFINITION,
    y2013::theros::DEFINITION,
    y2012::planechase_2012::DEFINITION,
    y2013::commander_2013::DEFINITION,
    y2014::journey_into_nyx::DEFINITION,
    y2014::conspiracy::DEFINITION,
    y2014::magic_2015::DEFINITION,
    y2014::commander_2014::DEFINITION,
    y2014::khans_of_tarkir::DEFINITION,
    y2015::dragons_of_tarkir::DEFINITION,
    y2015::commander_2015::DEFINITION,
    y2017::ixalan::DEFINITION,
    y2018::battlebond::DEFINITION,
    y2019::magic_2020::DEFINITION,
    y2019::modern_horizons::DEFINITION,
    y2019::war_of_the_spark::DEFINITION,
    y2019::throne_of_eldraine::DEFINITION,
    y2020::theros_beyond_death::DEFINITION,
    y2020::zendikar_rising::DEFINITION,
    y2020::ikoria::DEFINITION,
    y2021::kaldheim::DEFINITION,
    y2021::commander_2021::DEFINITION,
    y2021::strixhaven_school_of_mages::DEFINITION,
    y2021::modern_horizons_2::DEFINITION,
    y2021::adventures_in_the_forgotten_realms::DEFINITION,
    y2021::innistrad_midnight_hunt::DEFINITION,
    y2021::innistrad_crimson_vow::DEFINITION,
    y2021::innistrad_crimson_vow_commander::DEFINITION,
    y2022::kamigawa_neon_dynasty::DEFINITION,
    y2022::kamigawa_neon_dynasty_commander::DEFINITION,
    y2022::streets_of_new_capenna::DEFINITION,
    y2022::streets_of_new_capenna_commander::DEFINITION,
    y2022::commander_legends_baldurs_gate::DEFINITION,
    y2022::dominaria_united::DEFINITION,
    y2022::the_brothers_war::DEFINITION,
    y2016::eternal_masters::DEFINITION,
    y2016::eldritch_moon::DEFINITION,
    y2016::conspiracy_take_the_crown::DEFINITION,
    y2016::kaladesh::DEFINITION,
    y2017::aether_revolt::DEFINITION,
    y2017::amonkhet::DEFINITION,
    y2023::phyrexia_all_will_be_one::DEFINITION,
    y2023::phyrexia_all_will_be_one_commander::DEFINITION,
    y2023::march_of_the_machine::DEFINITION,
    y2023::lord_of_the_rings::DEFINITION,
    y2023::lord_of_the_rings_commander::DEFINITION,
    y2023::wilds_of_eldraine::DEFINITION,
    y2024::murders_at_karlov_manor::DEFINITION,
    y2024::ravnica_clue_edition::DEFINITION,
    y2024::fallout::DEFINITION,
    y2023::lost_caverns_of_ixalan::DEFINITION,
    y2024::modern_horizons_3::DEFINITION,
    y2024::outlaws_of_thunder_junction::DEFINITION,
    y2024::the_big_score::DEFINITION,
    y2024::modern_horizons_3_commander::DEFINITION,
    y2024::bloomburrow::DEFINITION,
    y2024::bloomburrow_commander::DEFINITION,
    y2024::duskmourn_house_of_horror::DEFINITION,
    y2024::duskmourn_house_of_horror_commander::DEFINITION,
    y2024::foundations_jumpstart::DEFINITION,
    y2025::tarkir_dragonstorm::DEFINITION,
    y2025::aetherdrift::DEFINITION,
    y2025::final_fantasy::DEFINITION,
    y2025::final_fantasy_commander::DEFINITION,
    y2025::through_the_omenpaths::DEFINITION,
    y1995::homelands::DEFINITION,
    y1997::fifth_edition::DEFINITION,
    y1998::exodus::DEFINITION,
    y1999::classic_sixth_edition::DEFINITION,
    y2000::prophecy::DEFINITION,
    y2005::saviors_of_kamigawa::DEFINITION,
    y2005::ravnica_city_of_guilds::DEFINITION,
    y2006::guildpact::DEFINITION,
    y2006::dissension::DEFINITION,
    y2006::time_spiral::DEFINITION,
    y2009::alara_reborn::DEFINITION,
    y2015::fate_reforged::DEFINITION,
    y2015::battle_for_zendikar::DEFINITION,
    y2015::magic_origins::DEFINITION,
    y2016::shadows_over_innistrad::DEFINITION,
    y2016::oath_of_the_gatewatch::DEFINITION,
    y2017::hour_of_devastation::DEFINITION,
    y2018::core_set_2019::DEFINITION,
    y2018::rivals_of_ixalan::DEFINITION,
    y2018::commander_2018::DEFINITION,
    y2019::ravnica_allegiance::DEFINITION,
    y2020::commander_2020::DEFINITION,
    y2020::core_set_2021::DEFINITION,
    y2024::magic_foundations::DEFINITION,
    y2025::marvels_spider_man::DEFINITION,
    y2025::avatar_the_last_airbender::DEFINITION,
    y2025::edge_of_eternities::DEFINITION,
    y2025::edge_of_eternities_commander::DEFINITION,
    y2026::lorwyn_eclipsed::DEFINITION,
    y2026::secrets_of_strixhaven::DEFINITION,
    y2026::teenage_mutant_ninja_turtles::DEFINITION,
    y2026::marvel_super_heroes::DEFINITION,
    y1999::portal_three_kingdoms::DEFINITION,
    y2006::coldsnap::DEFINITION,
    y2014::born_of_the_gods::DEFINITION,
    y2017::commander_2017::DEFINITION,
    y2018::dominaria::DEFINITION,
    y2020::commander_legends::DEFINITION,
    y2022::dominaria_united_commander::DEFINITION,
    y2023::march_of_the_machine_commander::DEFINITION,
    y2023::lost_caverns_of_ixalan_commander::DEFINITION,
    y2018::guilds_of_ravnica::DEFINITION,
    y2023::commander_masters::DEFINITION,
    y2023::doctor_who::DEFINITION,
    y2026::marvel_super_heroes_commander::DEFINITION,
    y2026::the_hobbit::DEFINITION,
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let capacity = SET_MODULES.iter().map(|module| module.cards.len()).sum();
    let mut definitions = Vec::with_capacity(capacity);
    for module in SET_MODULES {
        definitions.extend(
            module
                .cards
                .iter()
                .map(|record| record.definition(module.set)),
        );
    }
    definitions
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

#[cfg(test)]
mod tests;
