//! Built-in card records, grouped by release year and set.
//!
//! Each canonical card is defined in one set module. Records default to a
//! complete implementation and explicitly carry a reason when they are partial
//! or metadata-only. Reprints and alternate-art variants point back to that
//! canonical record from their own set module.

mod tokens;
mod y1993;
mod y1994;
mod y2004;
mod y2007;
mod y2011;
mod y2012;
mod y2013;
mod y2021;

use super::record::{CardAbilityBinding, CardRecord, PrintingRecord};
use crate::AbilityOrigin;
use crate::card::{AbilityDef, CardBehavior, CardDefinition, CardPrinting, CardRules, CardSet};

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
    SetModule::new(CardSet::Token, tokens::CARDS, tokens::ADDITIONAL_PRINTINGS),
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
        CardSet::Darksteel,
        y2004::darksteel::CARDS,
        y2004::darksteel::ADDITIONAL_PRINTINGS,
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
        CardSet::ModernHorizons2,
        y2021::modern_horizons_2::CARDS,
        y2021::modern_horizons_2::ADDITIONAL_PRINTINGS,
    ),
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let mut definitions = Vec::with_capacity(263);
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
        .find(|record| record.id == definition)?
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
        CardBehavior::Smoke => &y1993::alpha::SMOKE.rules,
        CardBehavior::WinterOrb => &y1993::alpha::WINTER_ORB.rules,
        CardBehavior::ChaosOrb => &y1993::alpha::CHAOS_ORB.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::IronclawOrcs => &y1993::alpha::IRONCLAW_ORCS.rules,
        CardBehavior::WheelOfFortune => &y1993::alpha::WHEEL_OF_FORTUNE.rules,
        CardBehavior::ManaVault | CardBehavior::ManaVaultUntap | CardBehavior::ManaVaultDamage => {
            &y1993::alpha::MANA_VAULT.rules
        }
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::TimeWalk => &y1993::alpha::TIME_WALK.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::Channel => &y1993::alpha::CHANNEL.rules,
        CardBehavior::Crusade => &y1993::alpha::CRUSADE.rules,
        CardBehavior::DemonicTutor => &y1993::alpha::DEMONIC_TUTOR.rules,
        CardBehavior::Duress => &y2012::magic_2013::DURESS.rules,
        CardBehavior::Earthquake => &y1993::alpha::EARTHQUAKE.rules,
        CardBehavior::EssenceScatter => &y2012::magic_2013::ESSENCE_SCATTER.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::LifebaneZombie => &y2013::magic_2014::LIFEBANE_ZOMBIE.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::SylvanLibrary => &y1994::legends::SYLVAN_LIBRARY.rules,
        CardBehavior::TimeVault => &y1993::alpha::TIME_VAULT.rules,
        CardBehavior::Timetwister => &y1993::alpha::TIMETWISTER.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::GrislySalvage => &y2012::return_to_ravnica::GRISLY_SALVAGE.rules,
        CardBehavior::KirdApe => &y1993::arabian_nights::KIRD_APE.rules,
        CardBehavior::Moat => &y1994::legends::MOAT.rules,
        CardBehavior::Mulch => &y2011::innistrad::MULCH.rules,
        CardBehavior::Negate => &y2012::magic_2013::NEGATE.rules,
        CardBehavior::PillarOfFlame => &y2012::avacyn_restored::PILLAR_OF_FLAME.rules,
        CardBehavior::SedgeTroll => &y1993::alpha::SEDGE_TROLL.rules,
        CardBehavior::SinCollector => &y2013::dragons_maze::SIN_COLLECTOR.rules,
        CardBehavior::SphinxsRevelation => &y2012::return_to_ravnica::SPHINXS_REVELATION.rules,
        CardBehavior::TetravusDetach | CardBehavior::TetravusAssemble => {
            &y1994::antiquities::TETRAVUS.rules
        }
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

#[cfg(test)]
mod tests;
