//! Adventures in the Forgotten Realms cards cataloged for the Vintage Cube
//! pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "AFR",
    slug: "adventures-in-the-forgotten-realms",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// AFR 33 — Portable Hole
pub(in crate::card::sets) static PORTABLE_HOLE: CardRecord = CardRecord::new(
    "Portable Hole",
    "80fca8c0-ae3e-439e-b202-228b9f360e9a",
    "John Stanko",
    // One white mana answers most of what a fast deck opens on, and it
    // answers it at instant speed on the other player's turn only because
    // somebody flashed it in -- otherwise the Hole is simply the cheapest
    // unconditional removal a white deck gets.
    CardRules::new_artifact(mana_cost!("{W}")).with_ability(
        abilities::enters_trigger_with_targets(
            "When this artifact enters, exile target nonland permanent an opponent controls with \
         mana value 2 or less until this artifact leaves the battlefield.",
            // A cheap nonland permanent across the table. Mana value is read off the
            // card, so a token is a zero and qualifies.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(2),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            abilities::exile_until_source_leaves(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        ),
    ),
);

// AFR 42 — You Hear Something on Watch
pub(in crate::card::sets) static YOU_HEAR_SOMETHING_ON_WATCH: CardRecord = CardRecord::new(
    "You Hear Something on Watch",
    "76e939ab-9d0c-4685-805c-c8bc4e6af163",
    "Zezhou Chen",
    // A combat trick or a removal spell for the same two mana, chosen after
    // blockers, which is what makes holding it up rarely wrong.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Rouse the Party — Creatures you control get +1/+1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Set Off Traps — This spell deals 5 damage to target attacking creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(5),
                ),
            ),
        ],
    )),
);

// AFR 119 — Shambling Ghast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMBLING_GHAST_119: CardRecord = CardRecord::new(
    "Shambling Ghast",
    "d96198a7-dd19-4940-bf8f-23135011fc84",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// AFR 123 — Vampire Spawn
pub(in crate::card::sets) static VAMPIRE_SPAWN: CardRecord = CardRecord::new(
    "Vampire Spawn",
    "b8975c72-b2ec-4c5f-86a4-4e1e3bb41c15",
    "Alex Brock",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent loses 2 life and you \
             gain 2 life.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// AFR 132 — Battle Cry Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_CRY_GOBLIN_132: CardRecord = CardRecord::new(
    "Battle Cry Goblin",
    "9766a427-2bb3-4028-a502-d1194cdc93aa",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// AFR 147 — Hobgoblin Bandit Lord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOBGOBLIN_BANDIT_LORD_147: CardRecord = CardRecord::new(
    "Hobgoblin Bandit Lord",
    "09e9dc36-f2d8-4384-98cb-e44c00b02433",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// AFR 158 — Plundering Barbarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLUNDERING_BARBARIAN_158: CardRecord = CardRecord::new(
    "Plundering Barbarian",
    "d875881c-c285-47f1-8a37-e4a0239d47ec",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// AFR 164 — Unexpected Windfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNEXPECTED_WINDFALL_164: CardRecord = CardRecord::new(
    "Unexpected Windfall",
    "bae6a5fb-39f5-4cf8-85f7-661cb4570507",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// AFR 176 — Circle of Dreams Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_DREAMS_DRUID_176: CardRecord = CardRecord::new(
    "Circle of Dreams Druid",
    "be6fdec0-a2c4-4da2-ae14-961185eaee66",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// AFR 198 — Owlbear
pub(in crate::card::sets) static OWLBEAR: CardRecord = CardRecord::new(
    "Owlbear",
    "30e8a00f-8131-470d-8072-4c23b812281a",
    "Ilse Gort",
    // "Keen Senses" is an ability word: flavour on the front of the clause
    // that changes nothing about how it works.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Bird", "Bear"], 4, 4).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "Keen Senses — When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AFR 207 — The Tarrasque
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_TARRASQUE_207: CardRecord = CardRecord::new(
    "The Tarrasque",
    "8a26fa15-d81f-4152-ae33-e91aa276b3fc",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// AFR 215 — You Meet in a Tavern
pub(in crate::card::sets) static YOU_MEET_IN_A_TAVERN: CardRecord = CardRecord::new(
    "You Meet in a Tavern",
    "593aa59a-4025-4df8-9f27-188fc7712fde",
    "Zoltan Boros",
    // Refuel or finish, chosen on the turn it is cast, which is what four
    // mana buys in a deck that is sometimes ahead and sometimes empty.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Form a Party — Look at the top five cards of your library. You may reveal any \
                 number of creature cards from among them and put them into your hand. Put the \
                 rest on the bottom of your library in a random order.",
                // "Any number" is nought through five, so a whiff takes
                // nothing and still buries the five.
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::Constant(5),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    0,
                    5,
                ),
            ),
            AbilityDef::spell(
                "Start a Brawl — Creatures you control get +2/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// AFR 296 — Old Gnawbone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OLD_GNAWBONE_296: CardRecord = CardRecord::new(
    "Old Gnawbone",
    "16ead969-8ba4-4587-a68d-47730943605e",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// AFR 304 — Oswald Fiddlebender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSWALD_FIDDLEBENDER_304: CardRecord = CardRecord::new(
    "Oswald Fiddlebender",
    "853c9db7-504e-4dfb-8067-abd2a36f6a1a",
    "Phil Stone",
    crate::card::CardRules::unsupported(),
);

// AFR 317 — Delina, Wild Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELINA_WILD_MAGE_317: CardRecord = CardRecord::new(
    "Delina, Wild Mage",
    "c4f1dc68-868b-4f3f-b10a-62842d88edc4",
    "Justine Mara Andersen",
    crate::card::CardRules::unsupported(),
);

// AFR 322 — Xorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static XORN_322: CardRecord = CardRecord::new(
    "Xorn",
    "713a6502-5239-449a-b04a-c82d525f9916",
    "Justine Jones",
    crate::card::CardRules::unsupported(),
);

// AFR 351 — Den of the Bugbear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEN_OF_THE_BUGBEAR_351: CardRecord = CardRecord::new(
    "Den of the Bugbear",
    "565be37a-4c14-420b-a07c-18e21f7fd731",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// AFR 358 — Treasure Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_VAULT_358: CardRecord = CardRecord::new(
    "Treasure Vault",
    "8d75f53d-d105-4973-ab2e-58cdf3a41eed",
    "Erol Otus",
    crate::card::CardRules::unsupported(),
);

// AFR 363 — Loyal Warhound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOYAL_WARHOUND_363: CardRecord = CardRecord::new(
    "Loyal Warhound",
    "bce58575-7607-49a4-a7c9-b23b715c3bf5",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// AFR 373 — Asmodeus the Archfiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASMODEUS_THE_ARCHFIEND_373: CardRecord = CardRecord::new(
    "Asmodeus the Archfiend",
    "1a0bbab6-b9ad-456d-ab4a-485dd8d89b35",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PORTABLE_HOLE,
    &YOU_HEAR_SOMETHING_ON_WATCH,
    &SHAMBLING_GHAST_119,
    &VAMPIRE_SPAWN,
    &BATTLE_CRY_GOBLIN_132,
    &HOBGOBLIN_BANDIT_LORD_147,
    &PLUNDERING_BARBARIAN_158,
    &UNEXPECTED_WINDFALL_164,
    &CIRCLE_OF_DREAMS_DRUID_176,
    &OWLBEAR,
    &THE_TARRASQUE_207,
    &YOU_MEET_IN_A_TAVERN,
    &OLD_GNAWBONE_296,
    &OSWALD_FIDDLEBENDER_304,
    &DELINA_WILD_MAGE_317,
    &XORN_322,
    &DEN_OF_THE_BUGBEAR_351,
    &TREASURE_VAULT_358,
    &LOYAL_WARHOUND_363,
    &ASMODEUS_THE_ARCHFIEND_373,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
