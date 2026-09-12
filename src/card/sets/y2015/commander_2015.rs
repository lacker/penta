//! Commander 2015 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PlayerRelation;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y2005::betrayers_of_kamigawa as catalog_bok;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C15",
    slug: "commander-2015",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C15 14 — Mystic Confluence
pub(in crate::card::sets) static MYSTIC_CONFLUENCE: CardRecord = CardRecord::new(
    "Mystic Confluence",
    "81bbffc2-6f58-4baa-8f95-168eab106b15",
    "Kieran Yanner",
    // Five mana that is never dead: three cards when nothing is happening, a
    // hard counter plus a card when something is.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(
        AbilityDef::modal_spell(
            "Choose three. You may choose the same mode more than once.",
            // Each mode declares its own target slot, and a mode chosen twice gets two
            // of them -- which is what "you may choose the same mode more than once"
            // means for a clause that targets.
            &[
                AbilityDef::spell_with_targets(
                    "Counter target spell unless its controller pays {3}.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Spell,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )],
                    abilities::counter_target_unless_paid(&[crate::CostDef::GenericMana(
                        ValueDef::Constant(3),
                    )]),
                ),
                AbilityDef::spell_with_targets(
                    "Return target creature to its owner's hand.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::spell(
                    "Draw a card.",
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ],
        )
        .with_mode_selection(3, 3, true),
    ),
);

// C15 20 — Dread Summons
pub(in crate::card::sets) static DREAD_SUMMONS: CardRecord = CardRecord::new(
    "Dread Summons",
    "b2c20cb1-3e3d-4fea-b617-bd6d796c8d10",
    "Izzy",
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "Each player mills X cards. For each creature card put into a \
         graveyard this way, you create a tapped 2/2 black Zombie \
         creature token. (To mill a card, a player puts the top card \
         of their library into their graveyard.)",
        EffectDef::Sequence(&[
            EffectDef::BindOutput {
                binding: crate::Binding!("milled"),
                effect: &EffectDef::Mill {
                    player: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::ChosenX,
                },
            },
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
                .with_count(ValueDef::CountObjects(&ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                }))
                .entering_tapped(),
        ]),
    )]),
);

// C15 26 — Fiery Confluence
pub(in crate::card::sets) static FIERY_CONFLUENCE: CardRecord = CardRecord::new(
    "Fiery Confluence",
    "7b61c9bc-16e8-417f-99e7-8bd83d4666c5",
    "Kieran Yanner",
    // Four mana that is a burn spell, a sweeper, or artifact removal, and
    // usually two of the three at once.
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::modal_spell(
            "Choose three. You may choose the same mode more than once.",
            // Three modes chosen three times between them: six damage to the other
            // player, three damage to every creature, three artifacts destroyed, or any
            // mixture of the three.
            &[
                AbilityDef::spell(
                    "Fiery Confluence deals 1 damage to each creature.",
                    // "Deals 1 damage to each creature": everything on the battlefield, yours
                    // included, which is what makes the sweeper half a cost as well as an
                    // answer.
                    EffectDef::damage(
                        EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        ValueDef::Constant(1),
                    ),
                ),
                AbilityDef::spell(
                    "Fiery Confluence deals 2 damage to each opponent.",
                    EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
                ),
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
            ],
        )
        .with_mode_selection(3, 3, true),
    ),
);

// C15 34 — Caller of the Pack
pub(in crate::card::sets) static CALLER_OF_THE_PACK: CardRecord = CardRecord::new(
    "Caller of the Pack",
    "1286208b-896b-4f41-a837-1c8a2b199a0f",
    "Ryan Yee",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Beast"], 8, 6)
        .with_abilities(&[abilities::trample(), abilities::myriad()]),
);

// C15 69 — Faith's Fetters (reprint)
const FAITH_S_FETTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::FAITH_S_FETTERS,
    "fe653236-c5c1-4dcd-95cd-3c53f1e256ef",
    "Brian Despain",
);

// C15 99 — Ninja of the Deep Hours (reprint)
const NINJA_OF_THE_DEEP_HOURS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bok::NINJA_OF_THE_DEEP_HOURS,
    "26184ff2-3b8c-419a-9b28-95d6e4e996bb",
    "Dan Murayama Scott",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MYSTIC_CONFLUENCE,
    &DREAD_SUMMONS,
    &FIERY_CONFLUENCE,
    &CALLER_OF_THE_PACK,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[FAITH_S_FETTERS_REPRINT, NINJA_OF_THE_DEEP_HOURS_REPRINT];
