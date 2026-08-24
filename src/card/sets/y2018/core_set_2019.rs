//! M19 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AddManaEffectDef, AppliedEffectDef, CardRules,
    CardType, CharacteristicOperationDef, EffectDef, EffectRecipientDef, LAND_SUBTYPES,
    ObjectPredicateDef, PlayerRelation, SetOperationDef, ZoneKind, abilities,
};
use crate::mana_cost;

// M19 29 — Militia Bugler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MILITIA_BUGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc"),
    "Militia Bugler",
    crate::card::CardArt::new("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc", "David Gaillet"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

// M19 125 — Vampire Sovereign
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_SOVEREIGN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee338221-ead9-4b89-8b0c-12745c4ca13d"),
    "Vampire Sovereign",
    crate::card::CardArt::new("ee338221-ead9-4b89-8b0c-12745c4ca13d", "Volkan Baǵa"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

static ALPINE_MOON_MANA: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::any_color()),
);

static ALPINE_MOON_CHANGES: [AppliedEffectDef; 3] = [
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
        SetOperationDef::Remove(LAND_SUBTYPES),
    )),
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
    AppliedEffectDef::add_ability(&ALPINE_MOON_MANA),
];

// M19 128 — Alpine Moon
pub(in crate::card::sets) static ALPINE_MOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2435c810-2baf-4e3b-80ce-542b94694901"),
    "Alpine Moon",
    crate::card::CardArt::new("2435c810-2baf-4e3b-80ce-542b94694901", "Alayna Danner"),
    crate::card::CardSet::CoreSet2019,
    CardRules::new_enchantment(mana_cost!("{R}")).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this enchantment enters, choose a nonbasic land card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::NONBASIC_LAND_CARD_NAME,
        ),
        AbilityDef::static_ability(
            "Lands your opponents control with the chosen name lose all land types and abilities, and they gain \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        abilities::SOURCES_CHOSEN_CARD_NAME,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::Composite(&ALPINE_MOON_CHANGES),
            },
        ),
    ]),
);

// M19 134 — Dark-Dweller Oracle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_DWELLER_ORACLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69a57bfc-1de2-4b3a-84bc-19ec41087f0d"),
    "Dark-Dweller Oracle",
    crate::card::CardArt::new(
        "69a57bfc-1de2-4b3a-84bc-19ec41087f0d",
        "Deruchenko Alexander",
    ),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

// M19 143 — Goblin Motivator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MOTIVATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2"),
    "Goblin Motivator",
    crate::card::CardArt::new("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2", "Johann Bodin"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MILITIA_BUGLER,
    &VAMPIRE_SOVEREIGN,
    &ALPINE_MOON,
    &DARK_DWELLER_ORACLE,
    &GOBLIN_MOTIVATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
