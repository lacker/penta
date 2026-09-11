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

// CHK 97 — Time Stop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_STOP: CardRecord = CardRecord::new(
    "Time Stop",
    "f968c5e9-12a8-4542-90b4-84e0238fa375",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYOJIN_OF_NIGHT_S_REACH: CardRecord = CardRecord::new(
    "Myojin of Night's Reach",
    "13a295b0-535e-4c2d-879d-62603d1f2f1b",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMUNE_WITH_NATURE: CardRecord = CardRecord::new(
    "Commune with Nature",
    "ce0b706e-017d-4f82-b280-cf9fdf75aef8",
    "Edward P. Beard, Jr.",
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEVOTED_RETAINER,
    &KONDA_LORD_OF_EIGANJO,
    &TIME_STOP,
    &CURSED_RONIN,
    &DISTRESS,
    &MYOJIN_OF_NIGHT_S_REACH,
    &BATTLE_MAD_RONIN,
    &BROTHERS_YAMAZAKI,
    &THROUGH_THE_BREACH,
    &COMMUNE_WITH_NATURE,
    &SAKURA_TRIBE_ELDER,
    &SENSEIS_DIVINING_TOP,
    &MINAMO_SCHOOL_AT_WATERS_EDGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
