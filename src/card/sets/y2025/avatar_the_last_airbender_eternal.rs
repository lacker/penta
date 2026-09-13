//! TLE card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostModificationDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TLE",
    slug: "avatar-the-last-airbender-eternal",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TLE 64 — Fire Nation Turret
// Audit: unsupported — Firebending requires attack-generated mana retained through the combat phase. Mana expiry cannot currently be authored for the end of combat.
pub(in crate::card::sets) static FIRE_NATION_TURRET_64: CardRecord = CardRecord::new(
    "Fire Nation Turret",
    "f25cc190-05e2-4aba-b214-46f687c07a10",
    "Fahmi Fauzi",
    crate::card::CardRules::unsupported(),
);

// TLE 76 — The Duke, Rebel Sentry
// Audit: unsupported — Activated counter-removal costs require a fixed counter kind. Removing
// exactly one counter of the controller's choice of any kind is not enumerated or payable.
pub(in crate::card::sets) static THE_DUKE_REBEL_SENTRY_76: CardRecord = CardRecord::new(
    "The Duke, Rebel Sentry",
    "cd9d91a8-7315-4355-af99-941f3cf7398c",
    "Logan Feliciano",
    CardRules::unsupported(),
);

// TLE 105 — Fire Nation Occupation
// Audit: unsupported — Firebending requires attack-generated mana that lasts until end of combat;
// mana expiry has no end-of-combat duration.
pub(in crate::card::sets) static FIRE_NATION_OCCUPATION_105: CardRecord = CardRecord::new(
    "Fire Nation Occupation",
    "3455d55e-aef5-4eb1-bcd9-1ff9d1ab3698",
    "Arthur Yuan",
    CardRules::unsupported(),
);

// TLE 120 — Longshot, Rebel Bowman
pub(in crate::card::sets) static LONGSHOT_REBEL_BOWMAN_120: CardRecord = CardRecord::new(
    "Longshot, Rebel Bowman",
    "b36efbe2-3798-43e5-8640-f003c77440a1",
    "Morry Hollowell",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Rebel", "Ally"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::static_ability(
                "Noncreature spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            ),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, Longshot deals 2 damage to each opponent.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::NoncreatureSpell,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
            ),
        ]),
);

// TLE 134 — The Cabbage Merchant
// Audit: unsupported — Activated mana payment cannot tap two chosen Food permanents; the supported tap-source mana cost cannot represent this selection.
pub(in crate::card::sets) static THE_CABBAGE_MERCHANT_134: CardRecord = CardRecord::new(
    "The Cabbage Merchant",
    "2fea0356-6684-4730-9eb4-0262856bc1f9",
    "Patrick Gañas",
    crate::card::CardRules::unsupported(),
);

// TLE 198 — Smellerbee, Rebel Fighter
pub(in crate::card::sets) static SMELLERBEE_REBEL_FIGHTER_198: CardRecord = CardRecord::new(
    "Smellerbee, Rebel Fighter",
    "2f1cae39-6120-4630-83c6-9ededc96308c",
    "Enishi",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Rebel", "Ally"], 3, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::first_strike(),
AbilityDef::static_ability("Other creatures you control have haste.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::haste()) }),
AbilityDef::triggered("Whenever Smellerbee attacks, you may discard your hand. If you do, draw cards equal to the number of attacking creatures.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Sequence(&[EffectDef::discard_cards(EffectRecipientDef::matching_objects(ObjectPredicateDef::Any, &[ZoneKind::Hand], PlayerRelation::You)), abilities::draw_cards(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Attacking]), &[ZoneKind::Battlefield], PlayerRelation::Any)))]) })
]),
);

// TLE 276 — Wolf Cove Villager
pub(in crate::card::sets) static WOLF_COVE_VILLAGER: CardRecord = CardRecord::new(
    "Wolf Cove Villager",
    "993652d5-b44b-4142-a081-427edb480dcf",
    "Gemi",
    // A 2/2 for one, paid for entirely by arriving tapped: it blocks the
    // turn after it lands and never the turn it does.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Peasant"], 2, 2)
        .with_ability(abilities::enters_tapped(CardType::Creature)),
);

// TLE 285 — Warship Scout
pub(in crate::card::sets) static WARSHIP_SCOUT: CardRecord = CardRecord::new(
    "Warship Scout",
    "f47fc407-5b7d-4c9d-90b4-3eb234f9f18b",
    "Brandon L. Hunt",
    // A vanilla 2/1 for one: nothing is missing from the definition, the
    // card simply prints no rules text.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Scout"], 2, 1),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FIRE_NATION_TURRET_64,
    &THE_DUKE_REBEL_SENTRY_76,
    &FIRE_NATION_OCCUPATION_105,
    &LONGSHOT_REBEL_BOWMAN_120,
    &THE_CABBAGE_MERCHANT_134,
    &SMELLERBEE_REBEL_FIGHTER_198,
    &WOLF_COVE_VILLAGER,
    &WARSHIP_SCOUT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
