//! Kaldheim cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "KHM",
    slug: "kaldheim",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "4ae9f454-4f8c-4123-9886-674bc439dfe7",
    "Olena Richards",
));

// KHM 46 — Behold the Multiverse
pub(in crate::card::sets) static BEHOLD_THE_MULTIVERSE: CardRecord = CardRecord::new(
    "Behold the Multiverse",
    "27855a38-a682-4f97-ad22-ac625e86faec",
    "Magali Villeneuve",
    // Foretell splits the four mana across two turns, which is what lets a
    // deck hold up interaction and still draw two at instant speed.
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Scry 2, then draw two cards.",
            EffectDef::Sequence(&[
                abilities::scry(ValueDef::Constant(2)),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        abilities::foretell(&[CostDef::Mana(mana_cost!("{1}{U}"))]),
    ]),
);

// KHM 117 — Village Rites (reprint)
const VILLAGE_RITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::core_set_2021::VILLAGE_RITES,
    "0fab9ee8-776a-48e5-b309-bcd381e67bf7",
    "Igor Kieryluk",
);

// KHM 121 — Axgard Cavalry
pub(in crate::card::sets) static AXGARD_CAVALRY: CardRecord = CardRecord::new(
    "Axgard Cavalry",
    "2411c341-a470-4484-9248-7c1d3ca12978",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains haste until end of turn. (It can \
             attack and {T} this turn.)",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// KHM 139 — Goldspan Dragon
pub(in crate::card::sets) static GOLDSPAN_DRAGON: CardRecord = CardRecord::new(
    "Goldspan Dragon",
    "9d914868-9000-4df2-a818-0ef8a7f636ae",
    "Andrew Mar",
    // Five mana for a hasty 4/4 flier that attacks for four and pays for
    // itself: every attack and every removal spell aimed at him is two mana
    // back, which is why he so often lands and casts something the same turn.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature attacks or becomes the target of a spell, create a Treasure \
                 token.",
            // He pays for anything he is answered with: the Treasure lands whether the
            // spell that named him resolves or not, since the trigger is the targeting
            // rather than what it does.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                TriggerEventDef::becomes_targeted(ObjectPredicateDef::Spell),
            ]),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))),
        ),
        AbilityDef::static_ability(
            "Treasures you control have \"{T}, Sacrifice this artifact: Add two mana of any one \
                 color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}, Sacrifice this artifact: Add two mana of any one color.",
                    // The granted ability sits beside the Treasure's own rather than replacing
                    // it, so a Treasure under him may still be cashed for one mana of any
                    // colour -- there is simply no reason to.
                    &[CostDef::TapSource, CostDef::SacrificeSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(2)),
                )),
            },
        ),
    ]),
);

// KHM 142 — Magda, Brazen Outlaw
pub(in crate::card::sets) static MAGDA_BRAZEN_OUTLAW: CardRecord = CardRecord::new(
    "Magda, Brazen Outlaw",
    "079e6263-e54c-4899-a336-5315909b9322",
    "Slawomir Maniak",
// Two mana that turns every tap into a Treasure, and five Treasures into
    // whatever artifact the deck is built around.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Dwarves you control get +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        // "Other Dwarves you control": Magda pumps the rest of the Dwarves and not
                        // herself, which is the whole reason she is a 2/1 rather than a 3/1.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Dwarf you control becomes tapped, create a Treasure token.",
                // Any Dwarf you control becoming tapped, not just an attack: tapping one
                // for mana or to pay a cost makes a Treasure just the same.
                TriggerEventDef::tapped(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TREASURE_TOKEN,
                ))),
            ),
            AbilityDef::activated(
                "Sacrifice five Treasures: Search your library for an artifact or Dragon card, put that \
                 card onto the battlefield, then shuffle.",
                &[CostDef::SacrificePermanents {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                    controller: PlayerRelation::You,
                    count: 5,
                }],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// KHM 149 — Seize the Spoils
pub(in crate::card::sets) static SEIZE_THE_SPOILS: CardRecord = CardRecord::new(
    "Seize the Spoils",
    "b3b7a69c-75d2-49a6-ab56-ef608d0b0208",
    "Jesper Ejsing",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell(
        "As an additional cost to cast this spell, discard a \
         card.\nDraw two cards and create a Treasure token. (It's an \
         artifact with \"{T}, Sacrifice this token: Add one mana of \
         any color.\")",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )
    .with_spell_additional_cost(&CostDef::discard(ObjectPredicateDef::Any))]),
);

// KHM 157 — Tuskeri Firewalker
pub(in crate::card::sets) static TUSKERI_FIREWALKER: CardRecord = CardRecord::new(
    "Tuskeri Firewalker",
    "a54d0170-a375-4e65-b98d-3e94a3aeef90",
    "Victor Adame Minguez",
    // A 3/2 that turns each connected attack into a card. Boast is what
    // rations it: the mana is trivial, so the real cost is having to attack
    // with a 3/2 first.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Berserker"], 3, 2).with_ability(
        abilities::boast(AbilityDef::activated(
            "Boast — {1}: Exile the top card of your library. You may play that card this turn. \
             (Activate only if this creature attacked this turn and only once each turn.)",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::ExileTopOfLibraryToPlay {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                // "You may play that card this turn", not "without paying its
                // mana cost": the card is still bought at full price, and the
                // permission dies with the turn.
                free: false,
                face_down: false,
                duration: ExilePlayDurationDef::ThisTurn,
                spend_any_color: false,
                play_condition: None,
                cast_only: false,
            },
        )),
    ),
);

// KHM 170 — Fynn, the Fangbearer
pub(in crate::card::sets) static FYNN_THE_FANGBEARER: CardRecord = CardRecord::new(
    "Fynn, the Fangbearer",
    "7d7a8a90-13c1-4b0c-ab2e-fc8d91ccefd9",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Warrior"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever a creature you control with deathtouch deals combat \
                 damage to a player, that player gets two poison counters. (A \
                 player with ten or more poison counters loses the game.)",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Deathtouch),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddPlayerCounters {
                    recipient: EffectRecipientDef::EventPlayer,
                    kind: CounterKind::Poison,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// KHM 192 — Sarulf's Packmate
pub(in crate::card::sets) static SARULF_S_PACKMATE: CardRecord = CardRecord::new(
    "Sarulf's Packmate",
    "6061113e-7dd8-4739-b4dd-55bb7f9e39a2",
    "Ilse Gort",
    // Foretelling costs the same four mana in total but splits it across two
    // turns, which is what lets a green deck spend an otherwise dead turn.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Wolf"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::foretell(&[CostDef::Mana(mana_cost!("{1}{G}"))]),
    ]),
);

// KHM 194 — Snakeskin Veil
pub(in crate::card::sets) static SNAKESKIN_VEIL: CardRecord = CardRecord::new(
    "Snakeskin Veil",
    "e692c208-c171-4964-9207-43c2cbc62845",
    "Matt Stewart",
    // One mana that answers removal and leaves the creature bigger, so it is
    // never a blank the way a pure protection spell is.
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature you control. It gains hexproof until end of \
         turn. (It can't be the target of spells or abilities your opponents control.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            // The hexproof arrives too late to stop what this was cast in
            // response to, which is why the counter is what saves the
            // creature and the hexproof only stops the next spell.
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// KHM 214 — Immersturm Predator
pub(in crate::card::sets) static IMMERSTURM_PREDATOR: CardRecord = CardRecord::new(
    "Immersturm Predator",
    "0d83d2d9-b9d0-47f5-989b-f2c726401ade",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{2}{B}{R}"), &["Vampire", "Dragon"], 3, 3).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature becomes tapped, exile up to one target \
                 card from a graveyard and put a +1/+1 counter on this \
                 creature.",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated(
                "Sacrifice another creature: This creature gains \
                 indestructible until end of turn. Tap it.",
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Tap {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            ),
        ],
    ),
);

// KHM 239 — Goldvein Pick
pub(in crate::card::sets) static GOLDVEIN_PICK: CardRecord = CardRecord::new(
    "Goldvein Pick",
    "9bf5e4ad-a6e9-4b7c-a1ec-8246d3a3b6ca",
    "Dan Murayama Scott",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage to a player, \
                 create a Treasure token. (It's an artifact with \"{T}, \
                 Sacrifice this token: Add one mana of any color.\")",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::AttachedToSource),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// KHM 315 — Esika's Chariot
pub(in crate::card::sets) static ESIKA_S_CHARIOT: CardRecord = CardRecord::new(
    "Esika's Chariot",
    "57a7d7e5-428d-4f42-8f13-9908fc65dcb4",
    "WolfSkullJack",
// Four mana for four power of Cats, which then crew the Chariot they
    // came with -- and every attack after that is another one of them.
    CardRules::new_vehicle(mana_cost!("{3}{G}"), 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Esika's Chariot enters, create two 2/2 green Cat creature tokens.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Cat"], &[ManaColor::Green], 2, 2).with_art(CardArt::new(
                            "2e07758f-0d1c-47d9-ba5a-43bc2a7423cd",
                            "Raoul Vitale",
                        )),
                    ))
                    .with_count(ValueDef::Constant(2)),
                ),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Esika's Chariot attacks, create a token that's a copy of target token you \
                 control.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // A token you control, which the Chariot itself is not: what it copies is
                // one of the Cats it brought, or anything else a token-making deck has
                // lying around.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                )],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                    &crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        exceptions: CopyExceptionsDef::NONE,
                    },
                ))),
            ),
            abilities::crew(
                "Crew 4 (Tap any number of creatures you control with total power 4 or more: This \
                 Vehicle becomes an artifact creature until end of turn.)",
                4,
            ),
        ]),
);

// KHM 382 — Youthful Valkyrie
pub(in crate::card::sets) static YOUTHFUL_VALKYRIE: CardRecord = CardRecord::new(
    "Youthful Valkyrie",
    "ffe93b27-f8ae-4abf-8ade-90f503f132c2",
    "Anna Steinbauer",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Angel"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another Angel you control enters, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Angel")),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BEHOLD_THE_MULTIVERSE,
    &AXGARD_CAVALRY,
    &GOLDSPAN_DRAGON,
    &MAGDA_BRAZEN_OUTLAW,
    &SEIZE_THE_SPOILS,
    &TUSKERI_FIREWALKER,
    &FYNN_THE_FANGBEARER,
    &SARULF_S_PACKMATE,
    &SNAKESKIN_VEIL,
    &IMMERSTURM_PREDATOR,
    &GOLDVEIN_PICK,
    &ESIKA_S_CHARIOT,
    &YOUTHFUL_VALKYRIE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[VILLAGE_RITES_REPRINT];
