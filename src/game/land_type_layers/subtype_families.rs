//! Card-type requirements for permanent subtypes (CR 205.1b, 205.3g-j,m).
//! Noncreature vocabularies follow the cached CR effective August 7, 2026.

use super::{CardType, Game, Permanent};
use crate::card::{CardTypeSet, LAND_SUBTYPES};

pub(super) fn subtype_has_card_type(subtype: &str, types: CardTypeSet) -> bool {
    if crate::card::creature_type_name(subtype).is_some() {
        return types.contains(CardType::Creature) || types.contains(CardType::Kindred);
    }
    if LAND_SUBTYPES.contains(&subtype) {
        return types.contains(CardType::Land);
    }
    match subtype {
        "Attraction" | "Blood" | "Bobblehead" | "Book" | "Clue" | "Contraption" | "Equipment"
        | "Food" | "Fortification" | "Gold" | "Incubator" | "Infinity" | "Junk" | "Lander"
        | "Map" | "Mutagen" | "Powerstone" | "Spacecraft" | "Stone" | "Treasure" | "Vehicle"
        | "Vibranium" => types.contains(CardType::Artifact),
        "Aura" | "Background" | "Cartouche" | "Case" | "Class" | "Curse" | "Plan" | "Role"
        | "Room" | "Rune" | "Saga" | "Shard" | "Shrine" => types.contains(CardType::Enchantment),
        "Ajani" | "Aminatou" | "Angrath" | "Arlinn" | "Ashiok" | "Bahamut" | "Basri" | "Bolas"
        | "Calix" | "Chandra" | "Comet" | "Dack" | "Dakkon" | "Daretti" | "Davriel" | "Dellian"
        | "Dihada" | "Domri" | "Dovin" | "Ellywick" | "Elminster" | "Elspeth" | "Estrid"
        | "Freyalise" | "Garruk" | "Gideon" | "Grist" | "Guff" | "Huatli" | "Jace" | "Jared"
        | "Jaya" | "Jeska" | "Kaito" | "Karn" | "Kasmina" | "Kaya" | "Kiora" | "Koth"
        | "Liliana" | "Lolth" | "Lukka" | "Minsc" | "Mordenkainen" | "Nahiri" | "Narset"
        | "Niko" | "Nissa" | "Nixilis" | "Oko" | "Quintorius" | "Ral" | "Rowan" | "Saheeli"
        | "Samut" | "Sarkhan" | "Serra" | "Sivitri" | "Sorin" | "Szat" | "Tamiyo" | "Tasha"
        | "Teferi" | "Teyo" | "Tezzeret" | "Tibalt" | "Tyvar" | "Ugin" | "Urza" | "Venser"
        | "Vivien" | "Vraska" | "Vronos" | "Will" | "Windgrace" | "Wrenn" | "Xenagos"
        | "Yanggu" | "Yanling" | "Zariel" => types.contains(CardType::Planeswalker),
        // Other card-type families are outside the modeled battlefield types.
        _ => true,
    }
}

impl Game {
    pub(super) fn drop_subtypes_without_their_card_type(
        &self,
        permanent: &Permanent,
        subtypes: &mut Vec<&'static str>,
    ) {
        if let Some(types) = self.permanent_types(permanent) {
            subtypes.retain(|subtype| subtype_has_card_type(subtype, types));
        }
    }

    pub(super) fn has_subtypes_without_their_card_type(
        &self,
        permanent: &Permanent,
        subtypes: &[&'static str],
    ) -> bool {
        self.permanent_types(permanent).is_some_and(|types| {
            subtypes
                .iter()
                .any(|subtype| !subtype_has_card_type(subtype, types))
        })
    }
}
