//! Wilds of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::PlayOptionDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityTargetDef, AbilityTargetPredicate,
    AlternateSpellKind, AppliedEffectDef, AppliedRuleDef, BlockRestrictionDef, CardArt,
    CardComposition, CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardSupertype,
    CardType, CharacteristicOperationDef, CounterKind, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, SpellForm, SpellResolutionDestinationDef, TokenCharacteristics,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::{CardPartId, PlayOptionId, mana_cost};

// WOE 62 — Mocking Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOCKING_SPRITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e595014d-4ff4-4561-b7f2-a9bd56300b01"),
    "Mocking Sprite",
    crate::card::CardArt::new("e595014d-4ff4-4561-b7f2-a9bd56300b01", "Ben Hill"),
    crate::card::CardSet::WildsOfEldraine,
    crate::card::CardRules::unsupported(),
);

// WOE 83 — Candy Grapple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANDY_GRAPPLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("190d97bc-dbef-496d-9bd1-b785bdf8a964"),
    "Candy Grapple",
    crate::card::CardArt::new("190d97bc-dbef-496d-9bd1-b785bdf8a964", "Konstantin Porubov"),
    crate::card::CardSet::WildsOfEldraine,
    crate::card::CardRules::unsupported(),
);

// WOE 116 — Voracious Vermin
pub(in crate::card::sets) static VORACIOUS_VERMIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8059be65-3c73-49bb-a3b6-c346ce2f9fa4"),
    "Voracious Vermin",
    CardArt::new("8059be65-3c73-49bb-a3b6-c346ce2f9fa4", "Milivoj Ćeran"),
    CardSet::WildsOfEldraine,
    // The Rat it brings is also the first thing to feed it: a sacrifice
    // outlet turns the token into a counter.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature token with \"This token can't block.\"",
            EffectDef::create_creature_token(&["Rat"], &[ManaColor::Black], 1, 1).with_abilities(
                &[AbilityDef::static_ability(
                    "This token can't block.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::CANNOT_BLOCK,
                        )),
                    },
                )],
            ),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 131 — Gnawing Crescendo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GNAWING_CRESCENDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("254fc64a-9734-44a6-8869-ab03512f1a99"),
    "Gnawing Crescendo",
    crate::card::CardArt::new("254fc64a-9734-44a6-8869-ab03512f1a99", "Alexey Kruglov"),
    crate::card::CardSet::WildsOfEldraine,
    crate::card::CardRules::unsupported(),
);

// WOE 142 — Monstrous Rage
pub(in crate::card::sets) static MONSTROUS_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eef5a0ae-5907-42c9-a097-3f973737e392"),
    "Monstrous Rage",
    CardArt::new("eef5a0ae-5907-42c9-a097-3f973737e392", "Borja Pindado"),
    CardSet::WildsOfEldraine,
    // One mana for three power and trample this turn, two of which stay
    // afterwards on the back of the Role.
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 until end of turn. Create a Monster Role token attached to \
         it. (If you control another Role on it, put that one into the graveyard. Enchanted \
         creature gets +1/+1 and has trample.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The pump is until end of turn and the Role is not: the +2/+0 lapses with
        // the turn and the +1/+1 stays for as long as the token does.
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::CreateAttachedToken {
                        // The Monster Role: an Aura token that is never cast, so it carries no
                        // enchant clause of its own -- what it attaches to is decided by the effect
                        // that creates it. Two Roles from one player on one creature is the older
                        // one's problem, which the Role rule settles.
                        token: TokenCharacteristics::enchantment(&["Aura", "Role"], &[])
                            // What a Role may be attached to. Held as a static because the token
                            // carries it by reference.
                            .enchanting(&ObjectPredicateDef::HasType(CardType::Creature))
                            .with_abilities(
                                &const {
                                    [AbilityDef::static_ability(
                                        "Enchanted creature gets +1/+1 and has trample.",
                                        EffectDef::StaticApply {
                                            recipient: EffectRecipientDef::AttachedPermanent,
                                            effect: AppliedEffectDef::Composite(
                                                &const {
                                                    [
                                                        AppliedEffectDef::modify_power_toughness(
                                                            ValueDef::Constant(1),
                                                            ValueDef::Constant(1),
                                                        ),
                                                        AppliedEffectDef::add_ability(
                                                            &const { abilities::trample() },
                                                        ),
                                                    ]
                                                },
                                            ),
                                        },
                                    )]
                                },
                            ),
                        host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    },
                ]
            },
        ),
    )),
);

// WOE 242 — Agatha's Soul Cauldron
pub(in crate::card::sets) static AGATHAS_SOUL_CAULDRON: CardRecord = CardRecord::new_with_legacy_id(
    2251,
    "Agatha's Soul Cauldron",
    CardArt::new("019b51b0-e5c6-4208-922b-7736686dddcd", "Jason A. Engle"),
    CardSet::WildsOfEldraine,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may spend mana as though it were mana of any color to activate abilities of \
                 creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities,
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Creatures you control with +1/+1 counters on them have all activated abilities of all \
                 creature cards exiled with Agatha's Soul Cauldron.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        // The Cauldron hands its abilities to creatures that are carrying a counter,
                        // whoever put it there. Read every time the layer is walked, so a creature
                        // that loses its last counter loses the abilities with it.
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                        AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ),
                    )),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Exile target card from a graveyard. When a creature card is exiled this way, put a \
                 +1/+1 counter on target creature you control.",
                &[AbilityCostDef::TapSource],
                // "Target card from a graveyard" reaches every graveyard, not only its
                // controller's.
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    }),
                    // The counter's target belongs to a reflexive trigger, which this engine
                    // declares up front alongside the activation's own target. "Up to one"
                    // rather than "one" is what keeps the activation legal for a player who
                    // controls no creature, which the printed card allows: the reflexive
                    // trigger simply never gets a target.
                    AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                        1,
                    ),
                ],
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    EffectDef::IfCondition {
                        // "When a creature card is exiled this way": asked of the card the
                        // activation named, which by then has already moved to exile.
                        condition: &TriggerConditionDef::TargetMatches {
                            slot: TargetIndex::PRIMARY,
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                        },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex(1)),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
        ]),
);

// WOE 243 — Candy Trail
pub(in crate::card::sets) static CANDY_TRAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a860925-d912-49e5-9ddc-41ab26916bb3"),
    "Candy Trail",
    CardArt::new("1a860925-d912-49e5-9ddc-41ab26916bb3", "Alix Branwyn"),
    CardSet::WildsOfEldraine,
    // A one-mana artifact that smooths the draw now and replaces itself
    // later, which is what makes it a fine card in a deck that just wants
    // its land drops.
    CardRules::new_artifact(mana_cost!("{1}"))
        // Food and Clue are printed types here rather than granted rules:
        // the sacrifice ability this card wants is its own, not either
        // token's.
        .with_subtypes(&["Food", "Clue"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this artifact enters, scry 2.",
                abilities::scry(ValueDef::Constant(2)),
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life and draw a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// WOE 277 — Virtue of Loyalty
/// "Those creatures" is the same set the clause just counted: nothing joins
/// or leaves the battlefield while one effect resolves, so asking twice and
/// binding the first answer come to the same thing.
static YOUR_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

const fn virtue_of_loyalty_rules() -> CardRules {
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your end step, put a +1/+1 counter on each creature you control. \
         Untap those creatures.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::AddCounters {
                        object: YOUR_CREATURES,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Untap {
                        object: YOUR_CREATURES,
                    },
                ]
            },
        ),
    ))
}

fn virtue_of_loyalty_composition() -> CardComposition {
    let virtue = virtue_of_loyalty_rules();
    let fealty = const {
        CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Knight creature token with vigilance.",
                    EffectDef::create_creature_token(
                        &const { ["Knight"] },
                        &const { [ManaColor::White] },
                        2,
                        2,
                    )
                    .with_abilities(&const { [abilities::vigilance()] }),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Virtue of Loyalty", virtue),
            CardPart::new(CardPartId(1), "Ardenvale Fealty", fealty),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Virtue of Loyalty",
                SpellForm::Part(CardPartId::PRIMARY),
                virtue
                    .mana_cost()
                    .expect("the enchantment has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Ardenvale Fealty",
                SpellForm::Part(CardPartId(1)),
                fealty
                    .mana_cost()
                    .expect("the Adventure has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static VIRTUE_OF_LOYALTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9622e597-dc7c-4198-9ce5-4df53bb0c96c"),
    "Virtue of Loyalty",
    CardArt::new("9622e597-dc7c-4198-9ce5-4df53bb0c96c", "Keith Garletts"),
    CardSet::WildsOfEldraine,
    virtue_of_loyalty_rules(),
)
.with_composition(virtue_of_loyalty_composition);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOCKING_SPRITE,
    &CANDY_GRAPPLE,
    &VORACIOUS_VERMIN,
    &GNAWING_CRESCENDO,
    &MONSTROUS_RAGE,
    &AGATHAS_SOUL_CAULDRON,
    &CANDY_TRAIL,
    &VIRTUE_OF_LOYALTY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
