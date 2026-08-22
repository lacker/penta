//! Urza's Saga cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt,
    CardChoiceSourceDef, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef,
    ChooseDef, DiscardSelectionDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, SpellResolutionDestinationDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

// USG 21 — Monk Realist
pub(in crate::card::sets) static MONK_REALIST: CardRecord = CardRecord::new_with_legacy_id(
    274,
    "Monk Realist",
    CardArt::new("7a7fe9f1-f3c0-43e4-aa30-d0bdab4ae94d", "Daren Bader"),
    CardSet::UrzasSaga,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Monk", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, destroy target enchantment.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// USG 59 — Annul
pub(in crate::card::sets) static ANNUL: CardRecord = CardRecord::new_with_legacy_id(
    275,
    "Annul",
    CardArt::new("3f8c73ff-be92-41ca-93a7-76f9823adb38", "Greg Simanson"),
    CardSet::UrzasSaga,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target artifact or enchantment spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])),
    )),
);

/// "A Goblin permanent card": Gempalm Incinerator is a Goblin card that is
/// also a creature, and nothing in the pool is a Goblin instant, but the
/// clause names permanents rather than creatures and so does this.
static A_GOBLIN_PERMANENT_IN_HAND: [CardChoiceSourceDef; 1] =
    [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

/// A minimum of zero is the "you may": the offer may be answered with
/// nothing, and with no Goblin in hand it is never made at all.
static GOBLIN_LACKEY_TRIGGER: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &A_GOBLIN_PERMANENT_IN_HAND,
    object: ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Goblin"),
        ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Instant),
            ObjectPredicateDef::HasType(CardType::Sorcery),
        ])),
    ]),
    minimum: 0,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    arrival_effect: None,
};

// USG 61 — Attunement
pub(in crate::card::sets) static ATTUNEMENT: CardRecord = CardRecord::new_with_legacy_id(
    2079,
    "Attunement",
    CardArt::new("b752a0d5-61f8-4f16-9d61-341464c9b2a2", "Randy Gallegos"),
    CardSet::UrzasSaga,
    // A net card down every time, and that is the point: the deck wants the
    // graveyard, and the enchantment comes back to do it again.
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::activated(
        "Return this enchantment to its owner's hand: Draw three then discard four cards.",
        &[AbilityCostDef::ReturnSourceToHand],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

/// One effect rather than two control changes: both controllers are read
/// before either permanent moves, which is the only way "exchange" can mean
/// what it says.
static DRAKE_EXCHANGE: EffectDef = EffectDef::ExchangeControl {
    first: EffectRecipientDef::Source,
    second: EffectRecipientDef::Target(TargetIndex::PRIMARY),
};

/// "If you don't or can't make an exchange, sacrifice this creature." The two
/// halves are complementary conditions on the same fact rather than an
/// effect with two branches, so each reads the way its own clause does.
static DRAKE_ENTERS: EffectDef = EffectDef::Sequence(&[
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::TargetMatches {
            slot: TargetIndex::PRIMARY,
            object: ObjectPredicateDef::Any,
        },
        then: &DRAKE_EXCHANGE,
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::Not(&TriggerConditionDef::TargetMatches {
            slot: TargetIndex::PRIMARY,
            object: ObjectPredicateDef::Any,
        }),
        then: &EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    },
]);

static A_CREATURE_AN_OPPONENT_CONTROLS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
]);

// USG 76 — Gilded Drake
pub(in crate::card::sets) static GILDED_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    2083,
    "Gilded Drake",
    CardArt::new("9ada76ca-ae9d-40e8-a3ff-71e6fc581b79", "Bob Eggleton"),
    CardSet::UrzasSaga,
    // Two mana to take the best creature on the board and hand back a 3/3
    // flier. Against a board with nothing worth taking it simply dies.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, exchange control of this creature and up to one target creature an opponent controls. If you don't or can't make an exchange, sacrifice this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: A_CREATURE_AN_OPPONENT_CONTROLS,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
            DRAKE_ENTERS,
        ),
    ]),
);

/// Each player looks only at their own graveyard, and what they find arrives
/// under their own control: the choice is asked of each of them in turn
/// rather than made once by the caster.
static EXHUME_EACH_GRAVEYARD: [CardChoiceSourceDef; 1] =
    [CardChoiceSourceDef::Zone(ZoneKind::Graveyard)];

// USG 103 — Time Spiral
pub(in crate::card::sets) static TIME_SPIRAL: CardRecord = CardRecord::new_with_legacy_id(
    2290,
    "Time Spiral",
    CardArt::new("f3d62dbd-63db-4ac9-950f-9852627f23f2", "Michael Sutfin"),
    CardSet::UrzasSaga,
    // Six mana that gives back six, so the wheel is free and the seven new
    // cards arrive with the mana to cast them still up.
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_ability(
        AbilityDef::spell(
            "Exile Time Spiral. Each player shuffles their hand and graveyard into their \
             library, then draws seven cards. You untap up to six lands.",
            EffectDef::Sequence(&TIME_SPIRAL_EFFECT),
        )
        // "Exile Time Spiral" is the first thing printed and the last thing
        // that happens: the card is on the stack while the rest resolves, so
        // what the clause settles is where it goes afterwards.
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ),
);

// USG 134 — Exhume
pub(in crate::card::sets) static EXHUME: CardRecord = CardRecord::new_with_legacy_id(
    2267,
    "Exhume",
    CardArt::new("a88b23ce-ce19-47da-b9f2-055a4d6bdc79", "Carl Critchlow"),
    CardSet::UrzasSaga,
    // Two mana for the biggest thing anybody has discarded, and the reason
    // the deck playing it discarded something bigger than the other one has.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Each player puts a creature card from their graveyard onto the battlefield.",
        EffectDef::ChooseCards {
            player: EffectRecipientDef::players(PlayerSetDef::All),
            sources: &EXHUME_EACH_GRAVEYARD,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            // Not a "may": a player with a creature card down there has to
            // put one back, and one with none is never asked.
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            arrival_effect: None,
        },
    )),
);

// USG 190 — Goblin Lackey
pub(in crate::card::sets) static GOBLIN_LACKEY: CardRecord = CardRecord::new_with_legacy_id(
    2017,
    "Goblin Lackey",
    CardArt::new("9b848caa-aad8-4060-8f86-304a8556de2d", "Jerry Tiritilli"),
    CardSet::UrzasSaga,
    // One connection puts a Siege-Gang Commander down for free, which is the
    // whole reason a 1/1 for one is a format staple.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage to a player, you may put a Goblin permanent card from your hand onto the battlefield.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            GOBLIN_LACKEY_TRIGGER,
        ),
    ),
);

// USG 191 — Goblin Matron
pub(in crate::card::sets) static GOBLIN_MATRON: CardRecord = CardRecord::new_with_legacy_id(
    2018,
    "Goblin Matron",
    CardArt::new("9e9e2e5d-ad06-4378-9afb-ffb174e6a5b4", "DiTerlizzi"),
    CardSet::UrzasSaga,
    // Any Goblin card, so it fetches the answer rather than the biggest
    // body: Tinkerer against artifacts, Ringleader for more cards.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, you may search your library for a Goblin card, reveal that card, put it into your hand, then shuffle.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Subtype("Goblin"),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                binding: None,
                then: None,
            },
        ),
    ),
);

static UNTAP_THE_CHOSEN_LANDS: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
};

/// "Up to six", and not your own: the lands are chosen as the spell resolves
/// rather than targeted, and nothing in the clause says who controls them.
/// A minimum of none is what "up to" means.
static UNTAP_UP_TO_SIX_LANDS: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    )),
    exclude: None,
    minimum: 0,
    maximum: 6,
    visibility: ChoiceVisibilityDef::Public,
    then: &UNTAP_THE_CHOSEN_LANDS,
});

static TIME_SPIRAL_EFFECT: [EffectDef; 2] = [
    abilities::shuffle_back_and_draw_seven(),
    UNTAP_UP_TO_SIX_LANDS,
];

// USG 193 — Goblin Patrol
pub(in crate::card::sets) static GOBLIN_PATROL: CardRecord = CardRecord::new_with_legacy_id(
    2034,
    "Goblin Patrol",
    CardArt::new("d0fcd8d3-f159-49a1-8dd9-582ae4a0adc3", "Greg Staples"),
    CardSet::UrzasSaga,
    // A 2/1 for one, rented rather than bought: the echo comes due on your
    // next upkeep and once only.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 2, 1).with_ability(abilities::echo(
        "Echo {R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
        mana_cost!("{R}"),
    )),
);

// USG 290 — Claws of Gix
pub(in crate::card::sets) static CLAWS_OF_GIX: CardRecord = CardRecord::new_with_legacy_id(
    288,
    "Claws of Gix",
    CardArt::new(
        "78372366-8c4c-46ac-bd7c-a735c2b24b5d",
        "Henry G. Higginbotham",
    ),
    CardSet::UrzasSaga,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice a permanent: You gain 1 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Any,
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// USG 321 — Gaea's Cradle
pub(in crate::card::sets) static GAEAS_CRADLE: CardRecord = CardRecord::new_with_legacy_id(
    2111,
    "Gaea's Cradle",
    CardArt::new("25b0b816-0583-44aa-9dc5-f3ff48993a51", "Mark Zug"),
    CardSet::UrzasSaga,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {G} for each creature you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL),
            },
        )),
);

static ARTIFACTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// USG 330 — Tolarian Academy
pub(in crate::card::sets) static TOLARIAN_ACADEMY: CardRecord = CardRecord::new_with_legacy_id(
    2112,
    "Tolarian Academy",
    CardArt::new("ad7ac9a5-340f-4509-826c-7b9416d47887", "Stephen Daniele"),
    CardSet::UrzasSaga,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {U} for each artifact you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Blue,
                amount: ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
            },
        )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MONK_REALIST,
    &ANNUL,
    &ATTUNEMENT,
    &GILDED_DRAKE,
    &TIME_SPIRAL,
    &EXHUME,
    &GOBLIN_LACKEY,
    &GOBLIN_MATRON,
    &GOBLIN_PATROL,
    &CLAWS_OF_GIX,
    &GAEAS_CRADLE,
    &TOLARIAN_ACADEMY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y1994::legends::PRESENCE_OF_THE_MASTER), // USG 32
];
