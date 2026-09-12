//! Ikoria: Lair of Behemoths cards cataloged for the Vintage Cube pool.

use crate::card::CounterKind;
use crate::card::ObjectRefDef;
use crate::card::SubtypeDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CompanionConditionDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DeckConstructionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// A triome is a tapped land with three basic land types and cycling, and
/// nothing else. Its printed mana ability is reminder text for what the
/// subtypes already grant, so it is not restated as a clause.
const TRIOME_ABILITIES: &[AbilityDef] = &[
    abilities::enters_tapped(CardType::Land),
    abilities::cycling!(
        "Cycling {3} ({3}, Discard this card: Draw a card.)",
        &[CostDef::Mana(mana_cost!("{3}"))],
    ),
];

const fn triome(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(TRIOME_ABILITIES)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "IKO",
    slug: "ikoria",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// IKO 11 — Drannith Magistrate
// Audit: unsupported — Needs a static cast prohibition based on a spell's origin zone being other than its controller's hand.
pub(in crate::card::sets) static DRANNITH_MAGISTRATE_11: CardRecord = CardRecord::new(
    "Drannith Magistrate",
    "98b0a4a8-9319-451b-9b79-b0bca7a41e91",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// IKO 39 — Aegis Turtle
pub(in crate::card::sets) static AEGIS_TURTLE: CardRecord = CardRecord::new(
    "Aegis Turtle",
    "e433e7f0-7417-4dfe-a7a4-3f222b0a835f",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{U}"), &["Turtle"], 0, 5),
);

// IKO 69 — Thieving Otter
pub(in crate::card::sets) static THIEVING_OTTER: CardRecord = CardRecord::new(
    "Thieving Otter",
    "07f84b0a-37d9-4b0f-8d75-1fab45a12d44",
    "Jakub Kasper",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Otter"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, draw a card.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::Source),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Opponent),
            }),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// IKO 70 — Voracious Greatshark
pub(in crate::card::sets) static VORACIOUS_GREATSHARK: CardRecord = CardRecord::new(
    "Voracious Greatshark",
    "1400155f-8911-45fd-aab2-998c8a28292c",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Shark"], 5, 4).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, counter target artifact or \
             creature spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// IKO 91 — Heartless Act
// Audit: unsupported — Needs one up-to-three counter-removal choice distributed among any counter kinds on the target; current removal effects specify one kind or remove every counter.
pub(in crate::card::sets) static HEARTLESS_ACT: CardRecord = CardRecord::new(
    "Heartless Act",
    "e4e6794a-feeb-4fc8-a2ee-38c75c18aaae",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// IKO 134 — Rumbling Rockslide
pub(in crate::card::sets) static RUMBLING_ROCKSLIDE: CardRecord = CardRecord::new(
    "Rumbling Rockslide",
    "96f9aaa7-11c7-4cd0-9803-9471c14ab846",
    "Adam Paquette",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Rumbling Rockslide deals damage to target creature equal to \
         the number of lands you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        ),
    )]),
);

// IKO 137 — Spelleater Wolverine
pub(in crate::card::sets) static SPELLEATER_WOLVERINE: CardRecord = CardRecord::new(
    "Spelleater Wolverine",
    "a5f03ffd-dcdb-441c-8dfc-4fe06a289b22",
    "Uriah Voth",
    // A vanilla 3/2 until the graveyard fills, then six damage a turn: the
    // threshold is what a spells deck is being paid for.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolverine"], 3, 2).with_ability(
        AbilityDef::static_ability(
            "This creature has double strike as long as there are three or more instant and/or \
             sorcery cards in your graveyard.",
            EffectDef::IfCondition {
                // One count over both types rather than two, since the
                // printed clause adds them together.
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            },
        ),
    ),
);

// IKO 148 — Colossification
pub(in crate::card::sets) static COLOSSIFICATION: CardRecord = CardRecord::new(
    "Colossification",
    "7b6e6f2a-5015-44c6-aa8d-85188494d1a6",
    "Johan Grenier",
    CardRules::new_enchantment(mana_cost!("{5}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +20/+20.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(20),
                        ValueDef::Constant(20),
                    ),
                },
            ),
        ]),
);

// IKO 162 — Kogla, the Titan Ape
pub(in crate::card::sets) static KOGLA_THE_TITAN_APE_162: CardRecord = CardRecord::new(
    "Kogla, the Titan Ape",
    "3c35ca79-eb72-427a-a8ed-404b2214389a",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Ape"], 7, 6).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::enters_trigger_with_targets("When Kogla enters, it fights up to one target creature you don't control.", &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::NotYou), owner: None }, 1)], EffectDef::Fight { first: ObjectRefDef::Source, second: ObjectRefDef::Target(TargetIndex::PRIMARY), excess: None }),
AbilityDef::triggered_with_targets("Whenever Kogla attacks, destroy target artifact or enchantment defending player controls.", TriggerEventDef::attacks(ObjectPredicateDef::Source), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Enchantment)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::DefendingPlayer), owner: None })], EffectDef::destroy_target(TargetIndex::PRIMARY)),
AbilityDef::activated_with_targets("{1}{G}: Return target Human you control to its owner's hand. Kogla gains indestructible until end of turn.", &[CostDef::Mana(mana_cost!("{1}{G}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Human")), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top), EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::add_ability(&abilities::indestructible()), duration: ResolvedEffectDurationDef::UntilEndOfTurn }]))
]),
);

// IKO 170 — Ram Through
// Audit: unsupported — Needs excess-damage routing on a one-sided damage effect. DealDamage carries no excess routing option and FightExcessDef attaches only to Fight, so "if the creature you control has trample, excess damage is dealt to that creature's controller instead" cannot be said without dropping the trample clause.
pub(in crate::card::sets) static RAM_THROUGH: CardRecord = CardRecord::new(
    "Ram Through",
    "ac0b24e7-14e7-45ee-b5d8-bdb8674b669c",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// IKO 174 — Titanoth Rex
pub(in crate::card::sets) static TITANOTH_REX_174: CardRecord = CardRecord::new(
    "Titanoth Rex",
    "9d02e1e8-b85b-4e26-8ab8-ca2f49d05b88",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{7}{G}{G}"), &["Dinosaur", "Beast"], 11, 11)
        .with_abilities(&[
            abilities::trample(),
            abilities::cycling!(
                "Cycling {1}{G} ({1}{G}, Discard this card: Draw a card.)",
                &[CostDef::Mana(mana_cost!("{1}{G}"))]
            ),
            AbilityDef::triggered_with_targets(
                "When you cycle this card, put a trample counter on target creature you control.",
                TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Trample,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// IKO 216 — Winota, Joiner of Forces
// Audit: unsupported — Needs a frozen top-library choice that puts a selected Human card onto the battlefield tapped and attacking, then randomizes the remainder.
pub(in crate::card::sets) static WINOTA_JOINER_OF_FORCES_216: CardRecord = CardRecord::new(
    "Winota, Joiner of Forces",
    "5dd13a6c-23d3-44ce-a628-cb1c19d777c4",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// IKO 222 — Jegantha, the Wellspring
// Audit: unsupported — Needs companion deck validation over repeated mana symbols and a mana restriction forbidding generic-cost payment.
pub(in crate::card::sets) static JEGANTHA_THE_WELLSPRING_222: CardRecord = CardRecord::new(
    "Jegantha, the Wellspring",
    "1d52e527-3835-4350-8c01-0f2d5d623b9c",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// IKO 226 — Lurrus of the Dream-Den
pub(in crate::card::sets) static LURRUS_OF_THE_DREAM_DEN: CardRecord = CardRecord::new(
    "Lurrus of the Dream-Den",
    "5ad36fb2-c44e-4085-ba0d-54277841ad3a",
    "Slawomir Maniak",
// Three mana for a lifelinking body that turns every cheap permanent in
    // the graveyard back into a card, one a turn -- which is why the decks
    // that play him keep their curve at two.
    CardRules::new_creature(mana_cost!("{1}{W/B}{W/B}"), &["Cat", "Nightmare"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::deck_construction(
                "Companion — Each permanent card in your starting deck has mana value 2 or less. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckConstructionDef::Companion(CompanionConditionDef::PermanentManaValueAtMost(2)),
                "Both halves are here: the deck-construction condition the deck layer checks, and the \
                 special action that takes it from outside the game for {3}.",
            ),
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Once during each of your turns, you may cast a permanent spell with mana value 2 \
                 or less from your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        // "A permanent spell with mana value 2 or less." The action is a cast, so a
                        // land card in the graveyard is not among them: lands are played rather
                        // than cast, which is what keeps this from being a Crucible.
                        GraveyardPlayPermissionDef::once_each_of_your_turns(
                            PlayRestrictionDef::new(
                                PlayActionMatcherDef::CastSpell,
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    ]),
                                    ObjectPredicateDef::ManaValueAtMost(2),
                                ]),
                            ),
                        ),
                    )),
                },
            ),
        ]),
);

// IKO 233 — Zirda, the Dawnwaker
pub(in crate::card::sets) static ZIRDA_THE_DAWNWAKER: CardRecord = CardRecord::new(
    "Zirda, the Dawnwaker",
    "1bd8e61c-2ee8-4243-a848-7008810db8a0",
    "Jesper Ejsing",
// Three mana for a 3/3 that makes every activated ability on the board
    // two cheaper, which is what a deck full of equipment and pingers is
    // waiting for.
    CardRules::new_creature(mana_cost!("{1}{R/W}{R/W}"), &["Elemental", "Fox"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::deck_construction(
                "Companion — Each permanent card in your starting deck has an activated ability. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckConstructionDef::Companion(
                    CompanionConditionDef::EveryPermanentHasAnActivatedAbility,
                ),
                "Both halves are here: the deck-construction condition the deck layer checks, and the \
                 special action that takes it from outside the game for {3}.",
            ),
            AbilityDef::static_ability(
                "Abilities you activate that aren't mana abilities cost {2} less to activate. \
                 This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    // "Abilities you activate", which is wider than the permanents you
                    // control: cycling and the rest of what a card in a hand or a
                    // graveyard prints is an ability you activate too, and the shared
                    // vocabulary reaches those objects with the same predicate. Mana
                    // abilities are outside it, and are excluded structurally rather
                    // than here: their activation never reaches this reduction.
                    permanent: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    amount: ValueDef::Constant(2),
                    minimum: 1,
                }),
            ),
            AbilityDef::activated_with_targets(
                "{1}, {T}: Target creature can't block this turn.",
                &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// IKO 248 — Indatha Triome
pub(in crate::card::sets) static INDATHA_TRIOME: CardRecord = CardRecord::new(
    "Indatha Triome",
    "2b74bb81-fb9a-40e5-a941-e517430b52f5",
    "Noah Bradley",
    triome(&["Plains", "Swamp", "Forest"]),
);

// IKO 250 — Ketria Triome
pub(in crate::card::sets) static KETRIA_TRIOME: CardRecord = CardRecord::new(
    "Ketria Triome",
    "a249b1f4-2b22-4b67-a207-e0c4ae95d2e1",
    "Sam Burley",
    triome(&["Forest", "Island", "Mountain"]),
);

// IKO 251 — Raugrin Triome
pub(in crate::card::sets) static RAUGRIN_TRIOME: CardRecord = CardRecord::new(
    "Raugrin Triome",
    "02138fbb-3962-4348-8d31-faaefba0b8b2",
    "Jonas De Ro",
    triome(&["Island", "Mountain", "Plains"]),
);

// IKO 253 — Savai Triome
pub(in crate::card::sets) static SAVAI_TRIOME: CardRecord = CardRecord::new(
    "Savai Triome",
    "748e6a61-9c1f-4225-9f04-e54002f63ac3",
    "Titus Lunter",
    triome(&["Mountain", "Plains", "Swamp"]),
);

// IKO 259 — Zagoth Triome
pub(in crate::card::sets) static ZAGOTH_TRIOME: CardRecord = CardRecord::new(
    "Zagoth Triome",
    "cc520518-2063-4b57-a0d4-10cf62a7175e",
    "Eytan Zana",
    triome(&["Swamp", "Forest", "Island"]),
);

// IKO 275 — Zilortha, Strength Incarnate
// Audit: unsupported — Needs a combat lethal-damage rule that substitutes controlled creatures' power for toughness.
pub(in crate::card::sets) static ZILORTHA_STRENGTH_INCARNATE_275: CardRecord = CardRecord::new(
    "Zilortha, Strength Incarnate",
    "9a0639a0-c898-4a07-975c-a02bdd53175b",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// IKO 338 — Kinnan, Bonder Prodigy
// Audit: unsupported — Needs a mana-tap trigger that duplicates the actual type produced and a top-five creature placement choice.
pub(in crate::card::sets) static KINNAN_BONDER_PRODIGY_338: CardRecord = CardRecord::new(
    "Kinnan, Bonder Prodigy",
    "532746e2-f822-4920-ab31-94e0c8baaa84",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// IKO 341 — Quartzwood Crasher
// Audit: unsupported — Needs combat-damage aggregation across one or more controlled trampling creatures for a dynamic token size.
pub(in crate::card::sets) static QUARTZWOOD_CRASHER_341: CardRecord = CardRecord::new(
    "Quartzwood Crasher",
    "39e1effa-92a6-4e8c-9cd6-fc57ae7b3cbf",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// IKO 355 — Lurrus of the Dream-Den (alternate printing)
const LURRUS_OF_THE_DREAM_DEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LURRUS_OF_THE_DREAM_DEN,
    1,
    "2c89beb2-3467-4be0-a066-919f54331942",
    "Slawomir Maniak",
);

// IKO 356 — Lutri, the Spellchaser
pub(in crate::card::sets) static LUTRI_THE_SPELLCHASER: CardRecord = CardRecord::new(
    "Lutri, the Spellchaser",
    "12c01a00-2128-4b6c-874f-a206eca3a756",
    "Lie Setiawan",
// Three mana at instant speed for a body and a copy of whatever you were
    // already casting -- and in a singleton cube the companion clause costs
    // the deck nothing it was not already paying.
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Elemental", "Otter"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::deck_construction(
                "Companion — Each nonland card in your starting deck has a different name. (If this card \
                 is your chosen companion, you may put it into your hand from outside the game for {3} \
                 as a sorcery.)",
                DeckConstructionDef::Companion(CompanionConditionDef::NonlandNamesAreDistinct),
                "Both halves are here: the deck-construction condition the deck layer checks, and the \
                 special action that takes it from outside the game for {3}.",
            ),
            abilities::flash(),
            AbilityDef::triggered_if_with_targets(
                "When Lutri enters, if you cast it, copy target instant or sorcery spell you control. \
                 You may choose new targets for the copy.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // "If you cast it": a Lutri put onto the battlefield some other way is a
                // 3/2 and nothing else, which is what keeps the trigger honest about being
                // half of a spell rather than half of a creature.
                &TriggerConditionDef::SourceWasCast,
                // Yours rather than anybody's: Lutri copies what you are casting, not what
                // is being cast at you.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            ),
        ]),
);

// IKO 375 — Yidaro, Wandering Monster
// Audit: unsupported — Needs per-card cycling history and a cycling replacement that moves the card from its graveyard to either library or battlefield before drawing.
pub(in crate::card::sets) static YIDARO_WANDERING_MONSTER_375: CardRecord = CardRecord::new(
    "Yidaro, Wandering Monster",
    "8bb6b4c7-4f18-4bea-b927-916c7bb987ee",
    "Yigit Koroglu",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DRANNITH_MAGISTRATE_11,
    &AEGIS_TURTLE,
    &THIEVING_OTTER,
    &VORACIOUS_GREATSHARK,
    &HEARTLESS_ACT,
    &RUMBLING_ROCKSLIDE,
    &SPELLEATER_WOLVERINE,
    &COLOSSIFICATION,
    &KOGLA_THE_TITAN_APE_162,
    &RAM_THROUGH,
    &TITANOTH_REX_174,
    &WINOTA_JOINER_OF_FORCES_216,
    &JEGANTHA_THE_WELLSPRING_222,
    &LURRUS_OF_THE_DREAM_DEN,
    &ZIRDA_THE_DAWNWAKER,
    &INDATHA_TRIOME,
    &KETRIA_TRIOME,
    &RAUGRIN_TRIOME,
    &SAVAI_TRIOME,
    &ZAGOTH_TRIOME,
    &ZILORTHA_STRENGTH_INCARNATE_275,
    &KINNAN_BONDER_PRODIGY_338,
    &QUARTZWOOD_CRASHER_341,
    &LUTRI_THE_SPELLCHASER,
    &YIDARO_WANDERING_MONSTER_375,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[LURRUS_OF_THE_DREAM_DEN_ALTERNATE_1];
