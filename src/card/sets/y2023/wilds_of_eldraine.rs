//! Wilds of Eldraine cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::CardPartId;
use crate::PlayOptionId;
use crate::card::AbilityDef;
use crate::card::AbilityOperationDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AdditionalCostValueDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::OptionalAdditionalCostAbilityDef;
use crate::card::OptionalAdditionalCostKindDef;
use crate::card::PlayOptionDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellForm;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WOE",
    slug: "wilds-of-eldraine",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const DEFENSELESS_RAT_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Rat"], &[ManaColor::Black], 1, 1)
        .with_abilities(&[AbilityDef::static_ability(
            "This token can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::CANNOT_BLOCK,
                )),
            },
        )])
        .with_art(CardArt::new(
            "1e0205f2-25c1-403b-b408-56e3f2d63b4d",
            "Kim Sokol",
        ));

// WOE 62 — Mocking Sprite
pub(in crate::card::sets) static MOCKING_SPRITE: CardRecord = CardRecord::new(
    "Mocking Sprite",
    "e595014d-4ff4-4561-b7f2-a9bd56300b01",
    "Ben Hill",
    // The discount is read off the battlefield, so an evasive body that
    // survives is what makes it pay -- and flying is why it does.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ]),
);

// WOE 83 — Candy Grapple
pub(in crate::card::sets) static CANDY_GRAPPLE: CardRecord = CardRecord::new(
    "Candy Grapple",
    "190d97bc-dbef-496d-9bd1-b785bdf8a964",
    "Konstantin Porubov",
    // Two mana kills most of what a limited deck plays, and the Food this
    // set hands out is what turns the rest into targets too.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::optional_additional_cost(
            "Bargain (You may sacrifice an artifact, enchantment, or token as you cast this \
             spell.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Bargain,
                label: OptionalAdditionalCostKindDef::Bargain.label(),
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
                costs: &[CostDef::Sacrifice {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Token,
                    ]),
                    quantity: CostQuantityDef::Fixed(1),
                }],
            },
        ),
        AbilityDef::spell_with_targets(
            "Target creature gets -3/-3 until end of turn. If this spell was bargained, that \
             creature gets -5/-5 until end of turn instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // One effect reading the payment back, not two: "instead"
                // means the bargained spell never applies the smaller number.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 116 — Voracious Vermin
pub(in crate::card::sets) static VORACIOUS_VERMIN: CardRecord = CardRecord::new(
    "Voracious Vermin",
    "8059be65-3c73-49bb-a3b6-c346ce2f9fa4",
    "Milivoj Ćeran",
// The Rat it brings is also the first thing to feed it: a sacrifice
    // outlet turns the token into a counter.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature token with \"This token can't block.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))),
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
pub(in crate::card::sets) static GNAWING_CRESCENDO: CardRecord = CardRecord::new(
    "Gnawing Crescendo",
    "254fc64a-9734-44a6-8869-ab03512f1a99",
    "Alexey Kruglov",
    // The pump is what wins the combat; the watcher is what stops the
    // opponent from blocking profitably to answer it.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control get +2/+0 until end of turn. Whenever a nontoken creature you \
         control dies this turn, create a 1/1 black Rat creature token with \"This token can't \
         block.\"",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // A watcher installed for the rest of the turn rather than a
            // one-shot: every nontoken creature that dies makes its own Rat,
            // and the Rats it makes are excluded from feeding it.
            EffectDef::InstallTrigger(InstalledTriggerDef::this_turn(&AbilityDef::triggered(
                "Whenever a nontoken creature you control dies this turn, create a 1/1 black \
                 Rat creature token with \"This token can't block.\"",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
            ))),
        ]),
    )),
);

// WOE 142 — Monstrous Rage
pub(in crate::card::sets) static MONSTROUS_RAGE: CardRecord = CardRecord::new(
    "Monstrous Rage",
    "eef5a0ae-5907-42c9-a097-3f973737e392",
    "Borja Pindado",
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
        EffectDef::Sequence(&[
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
                    .with_abilities(&[AbilityDef::static_ability(
                        "Enchanted creature gets +1/+1 and has trample.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::AttachedPermanent,
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                                AppliedEffectDef::add_ability(&abilities::trample()),
                            ]),
                        },
                    )]),
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )),
);

// WOE 242 — Agatha's Soul Cauldron
pub(in crate::card::sets) static AGATHAS_SOUL_CAULDRON: CardRecord = CardRecord::new(
    "Agatha's Soul Cauldron",
    "019b51b0-e5c6-4208-922b-7736686dddcd",
    "Jason A. Engle",
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
                &[CostDef::TapSource],
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
    "Candy Trail",
    "1a860925-d912-49e5-9ddc-41ab26916bb3",
    "Alix Branwyn",
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
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
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
            .with_subtypes(&["Adventure"])
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Knight creature token with vigilance.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Knight"], &[ManaColor::White], 2, 2)
                            .with_abilities(&const { [abilities::vigilance()] }),
                    ))),
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
    "Virtue of Loyalty",
    "9622e597-dc7c-4198-9ce5-4df53bb0c96c",
    "Keith Garletts",
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
