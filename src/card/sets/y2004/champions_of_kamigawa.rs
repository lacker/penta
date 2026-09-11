//! Champions of Kamigawa cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CHK",
    slug: "champions_of_kamigawa",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CHK 7 — Devoted Retainer
pub(in crate::card::sets) static DEVOTED_RETAINER: CardRecord = CardRecord::new(
    "Devoted Retainer",
    "fc41d6d6-d7e5-4874-b6e2-fa4c72454f15",
    "Greg Hildebrandt",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Samurai"], 1, 1)
        .with_ability(abilities::bushido(ValueDef::Constant(1))),
);

// CHK 10 — Ghostly Prison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTLY_PRISON_10: CardRecord = CardRecord::new(
    "Ghostly Prison",
    "82d7de2b-c909-48dc-9ab7-c4a8328e37bb",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// CHK 30 — Konda, Lord of Eiganjo
pub(in crate::card::sets) static KONDA_LORD_OF_EIGANJO: CardRecord = CardRecord::new(
    "Konda, Lord of Eiganjo",
    "5edab171-94b9-4e5e-ab61-bd8c6c8cfc38",
    "John Bolton",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Human", "Samurai"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::indestructible(),
            abilities::bushido(ValueDef::Constant(5)),
        ]),
);

// CHK 62 — Gifts Ungiven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIFTS_UNGIVEN_62: CardRecord = CardRecord::new(
    "Gifts Ungiven",
    "32b91eb5-ea53-4a21-a2e5-9c545a42fa30",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// CHK 97 — Time Stop
// Audit: unsupported — Needs an end-the-turn rules procedure that exiles the stack, ends resolving procedures, skips directly to cleanup, and performs the special cleanup rules.
pub(in crate::card::sets) static TIME_STOP: CardRecord = CardRecord::new(
    "Time Stop",
    "f968c5e9-12a8-4542-90b4-84e0238fa375",
    "Scott M. Fischer",
    CardRules::unsupported(),
);

// CHK 107 — Cursed Ronin
pub(in crate::card::sets) static CURSED_RONIN: CardRecord = CardRecord::new(
    "Cursed Ronin",
    "b8f24fe9-22c4-4e53-9d7a-3cbf5533ac9b",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Samurai"], 1, 1).with_abilities(&[
        abilities::bushido(ValueDef::Constant(1)),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// CHK 111 — Distress
pub(in crate::card::sets) static DISTRESS: CardRecord = CardRecord::new(
    "Distress",
    "8130a902-3a03-4473-a64f-84cf3590f4c6",
    "Michael Sutfin",
CardRules::new_sorcery(mana_cost!("{B}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target player reveals their hand. You choose a nonland card from it. That player discards that card.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                crate::card::PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )),
        ),
    ),
);

// CHK 126 — Myojin of Night's Reach
// Audit: unsupported — Needs a positive cast-from-hand entry condition; SourceNotCastFrom cannot distinguish an actual hand cast from a non-cast battlefield arrival when inverted by the current entry condition vocabulary.
pub(in crate::card::sets) static MYOJIN_OF_NIGHT_S_REACH: CardRecord = CardRecord::new(
    "Myojin of Night's Reach",
    "13a295b0-535e-4c2d-879d-62603d1f2f1b",
    "Kev Walker",
    CardRules::unsupported(),
);

// CHK 156 — Battle-Mad Ronin
pub(in crate::card::sets) static BATTLE_MAD_RONIN: CardRecord = CardRecord::new(
    "Battle-Mad Ronin",
    "a6e4394a-fa91-4cf9-99c1-dc0bc1011c5b",
    "Wayne England",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Samurai"], 1, 1).with_abilities(&[
        abilities::bushido(ValueDef::Constant(2)),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// CHK 160a — Brothers Yamazaki
// Audit: unsupported — Needs bushido plus a static legend-rule exemption gated on exactly two same-named permanents across the battlefield.
pub(in crate::card::sets) static BROTHERS_YAMAZAKI: CardRecord = CardRecord::new(
    "Brothers Yamazaki",
    "acef8c94-469b-4a76-b507-25b51f2501ab",
    "Ron Spears",
    CardRules::unsupported(),
);

// CHK 163 — Desperate Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_RITUAL_163: CardRecord = CardRecord::new(
    "Desperate Ritual",
    "c8bdb92a-7bdb-434b-8cc0-873969faf566",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// CHK 175 — Kiki-Jiki, Mirror Breaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIKI_JIKI_MIRROR_BREAKER_175: CardRecord = CardRecord::new(
    "Kiki-Jiki, Mirror Breaker",
    "162018eb-5483-4fa2-9c5a-abb639eecf91",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// CHK 193 — Through the Breach
pub(in crate::card::sets) static THROUGH_THE_BREACH: CardRecord = CardRecord::new(
    "Through the Breach",
    "6da09e6a-2965-4855-bd41-41b41ba188fb",
    "Hugh Jamieson",
CardRules::new_instant(mana_cost!("{4}{R}"))
        .with_subtypes(&["Arcane"])
        .with_abilities(&[
            AbilityDef::spell(
                "You may put a creature card from your hand onto the battlefield. That creature gains haste. Sacrifice that creature at the beginning of the next end step.",
                EffectDef::WithZoneMoveResult {
                    // A minimum of zero is the printed "you may": the offer may be answered
                    // with nothing, and with no creature in hand it is never made at all.
                    effect: &EffectDef::ChooseCards {
                        player: EffectRecipientDef::Controller,
                        sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        minimum: 0,
                        maximum: 1,
                        reveal: false,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    binding: ParentBinding,
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::binding_zone_change_successors(
                            ParentBinding,
                        ),
                        effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::haste()),
                                // The creature sacrifices itself rather than being named by a delayed
                                // trigger the spell installs: it is the object that arrived, and it carries
                                // the clause with it. Nothing else can name it -- the card was chosen only
                                // as this spell resolved, and what entered is a new object.
                                AppliedEffectDef::add_ability(&AbilityDef::triggered(
                                        "At the beginning of the next end step, sacrifice this creature.",
                                        TriggerEventDef::StepBegins {
                                            step: TurnStepDef::End,
                                            player: PlayerRelation::Any,
                                        },
                                        EffectDef::sacrifice(EffectRecipientDef::Source),
                                    )),
                            ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
            // Not a second spell ability and not a way to cast this card:
            // splice is a cast-time option on the card in hand, so the
            // clause exists to give the splice cost somewhere printed to
            // live, exactly as plot's does.
            abilities::splice_onto_arcane(
                &[CostDef::Mana(mana_cost!("{2}{R}{R}"))],
            ),
        ]),
);

// CHK 204 — Commune with Nature
pub(in crate::card::sets) static COMMUNE_WITH_NATURE: CardRecord = CardRecord::new(
    "Commune with Nature",
    "ce0b706e-017d-4f82-b280-cf9fdf75aef8",
    "Edward P. Beard, Jr.",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell(
        "Look at the top five cards of your library. You may reveal a \
creature card from among them and put it into your hand. Put \
the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_reveal_choice_to_hand_rest_bottom(
            ValueDef::Constant(5),
            ObjectPredicateDef::HasType(CardType::Creature),
            0,
            1,
        ),
    )]),
);

// CHK 205 — Dosan the Falling Leaf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOSAN_THE_FALLING_LEAF_205: CardRecord = CardRecord::new(
    "Dosan the Falling Leaf",
    "ffb190db-48fc-4c39-ae9f-5e304eabb4f4",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// CHK 225 — Kodama's Reach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KODAMA_S_REACH_225: CardRecord = CardRecord::new(
    "Kodama's Reach",
    "85d207ac-0680-47ef-85d9-4323c1321d6f",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// CHK 239 — Sakura-Tribe Elder
pub(in crate::card::sets) static SAKURA_TRIBE_ELDER: CardRecord = CardRecord::new(
    "Sakura-Tribe Elder",
    "91c7707a-bae0-4196-bf26-d276f57b7369",
    "Carl Critchlow",
    // The sacrifice is not part of a tap, which is the whole card: it blocks,
    // and then it ramps after damage is already on the stack.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake", "Shaman"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: Search your library for a basic land card, put that card \
             onto the battlefield tapped, then shuffle.",
            &[CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                // "Search ... for a basic land card" is a may: an empty
                // library, or a deck that wants to keep its basics, can find
                // nothing.
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// CHK 247 — Time of Need
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_OF_NEED_247: CardRecord = CardRecord::new(
    "Time of Need",
    "514577a5-c7ae-4cfc-872a-e3786d78a6c3",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// CHK 266 — Orochi Hatchery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OROCHI_HATCHERY_266: CardRecord = CardRecord::new(
    "Orochi Hatchery",
    "7e662e2e-f706-4d79-86ed-48b60787a5d0",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// CHK 268 — Sensei's Divining Top
pub(in crate::card::sets) static SENSEIS_DIVINING_TOP: CardRecord = CardRecord::new(
    "Sensei's Divining Top",
    "4a08ca06-58db-4ce6-b490-be4bea8956a1",
    "Michael Sutfin",
    // One mana that fixes every draw for the rest of the game: the tap
    // trades the card it just arranged for itself, and the {1} sets up the
    // next one.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{1}: Look at the top three cards of your library, then put them back in any order.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(3),
            ),
        ),
        AbilityDef::activated(
            "{T}: Draw a card, then put this artifact on top of its owner's library.",
            &[CostDef::TapSource],
            // The draw and the trip back to the library are one clause: the Top is on
            // the battlefield as the card is drawn and gone by the time anything could
            // answer it, which is why it is never really spent.
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
            ]),
        ),
    ]),
);

// CHK 273 — Boseiju, Who Shelters All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOSEIJU_WHO_SHELTERS_ALL_273: CardRecord = CardRecord::new(
    "Boseiju, Who Shelters All",
    "0180d9a8-992c-4d55-8ac4-33a587786993",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// CHK 276 — Forbidden Orchard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_ORCHARD_276: CardRecord = CardRecord::new(
    "Forbidden Orchard",
    "88d78261-c8c9-4e0e-b157-f70ed46c3a25",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// CHK 277 — Hall of the Bandit Lord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALL_OF_THE_BANDIT_LORD_277: CardRecord = CardRecord::new(
    "Hall of the Bandit Lord",
    "59fa5bab-8626-4b45-a3a3-621f6d9509ab",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// CHK 279 — Minamo, School at Water's Edge
pub(in crate::card::sets) static MINAMO_SCHOOL_AT_WATERS_EDGE: CardRecord = CardRecord::new(
    "Minamo, School at Water's Edge",
    "7536292c-da25-41c8-ba28-1e35758a7f3d",
    "Jeremy Jarvis",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Blue),
            AbilityDef::activated_with_targets(
                "{U}, {T}: Untap target legendary permanent.",
                &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// CHK 282 — Shinka, the Bloodsoaked Keep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHINKA_THE_BLOODSOAKED_KEEP_282: CardRecord = CardRecord::new(
    "Shinka, the Bloodsoaked Keep",
    "d2d5f30e-cc3a-46c1-82a9-2cd73705b2f5",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// CHK 283 — Shizo, Death's Storehouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIZO_DEATH_S_STOREHOUSE_283: CardRecord = CardRecord::new(
    "Shizo, Death's Storehouse",
    "2de9a046-3db1-436e-b666-ba96e2e8c5db",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEVOTED_RETAINER,
    &GHOSTLY_PRISON_10,
    &KONDA_LORD_OF_EIGANJO,
    &GIFTS_UNGIVEN_62,
    &TIME_STOP,
    &CURSED_RONIN,
    &DISTRESS,
    &MYOJIN_OF_NIGHT_S_REACH,
    &BATTLE_MAD_RONIN,
    &BROTHERS_YAMAZAKI,
    &DESPERATE_RITUAL_163,
    &KIKI_JIKI_MIRROR_BREAKER_175,
    &THROUGH_THE_BREACH,
    &COMMUNE_WITH_NATURE,
    &DOSAN_THE_FALLING_LEAF_205,
    &KODAMA_S_REACH_225,
    &SAKURA_TRIBE_ELDER,
    &TIME_OF_NEED_247,
    &OROCHI_HATCHERY_266,
    &SENSEIS_DIVINING_TOP,
    &BOSEIJU_WHO_SHELTERS_ALL_273,
    &FORBIDDEN_ORCHARD_276,
    &HALL_OF_THE_BANDIT_LORD_277,
    &MINAMO_SCHOOL_AT_WATERS_EDGE,
    &SHINKA_THE_BLOODSOAKED_KEEP_282,
    &SHIZO_DEATH_S_STOREHOUSE_283,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
