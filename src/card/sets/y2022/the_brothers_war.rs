//! The Brothers' War cards cataloged for the Vintage Cube pool.

use crate::card::AbilityKindDef;
use crate::card::AbilityPredicateDef;
use crate::card::CopyStackObjectDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BRO",
    slug: "the-brothers-war",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BRO 5 — Deadly Riposte
pub(in crate::card::sets) static DEADLY_RIPOSTE: CardRecord = CardRecord::new(
    "Deadly Riposte",
    "38eca0ae-d400-4afb-9a45-7100f4cd7149",
    "Olena Richards",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Deadly Riposte deals 3 damage to target tapped creature and \
         you gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Tapped,
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// BRO 12 — Loran of the Third Path
pub(in crate::card::sets) static LORAN_OF_THE_THIRD_PATH: CardRecord = CardRecord::new(
    "Loran of the Third Path",
    "59faa45d-868b-4bc7-934c-0e077642e129",
    "Steven Belledin",
    // Three mana for an answer to an artifact, a body that blocks, and a
    // symmetrical draw nobody else gets to use as often as you do.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Artificer"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy up to one target artifact or enchantment.",
                // "Up to one target artifact or enchantment": an Loran with nothing worth
                // answering still arrives as a 2/1 that draws.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::activated_with_targets(
                "{T}: You and target opponent each draw a card.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                // "You and target opponent each draw a card." Two draws rather than one
                // instruction naming both, because only one of them is targeted: the
                // opponent has to be a legal target and you never are.
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// BRO 22 — Recommission
// Audit: unsupported — The target and graveyard return are representable, but the entry wrapper
// accepts unconditional battlefield modifications only. This card's +1/+1 counter applies only
// when the selected artifact-or-creature card enters as a creature; there is no prospective-entry
// predicate to make the arrival counter conditional without also countering returned artifacts.
pub(in crate::card::sets) static RECOMMISSION_22: CardRecord = CardRecord::new(
    "Recommission",
    "2a64e330-1257-4ec3-9a75-889cdcac3ade",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// BRO 23 — Recruitment Officer
pub(in crate::card::sets) static RECRUITMENT_OFFICER_23: CardRecord = CardRecord::new(
    "Recruitment Officer",
    "c226656b-68d5-4df2-b313-a323a728c520",
    "Johan Grenier",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::activated("{3}{W}: Look at the top four cards of your library. You may reveal a creature card with mana value 3 or less from among them and put it into your hand. Put the rest on the bottom of your library in a random order.", &[CostDef::Mana(mana_cost!("{3}{W}"))], abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(ValueDef::Constant(4), ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMost(3)]), 0, 1)),
    ),
);

// BRO 26 — Soul Partition
pub(in crate::card::sets) static SOUL_PARTITION_26: CardRecord = CardRecord::new(
    "Soul Partition",
    "28bb8ec0-9729-4aa1-8ce4-a3a5598b0d70",
    "Kekai Kotaki",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
AbilityDef::spell_with_targets("Exile target nonland permanent. For as long as that card remains exiled, its owner may play it. A spell cast by an opponent this way costs {2} more to cast.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)))], EffectDef::IfElseCondition { condition: &TriggerConditionDef::TargetMatches { slot: TargetIndex::PRIMARY, object: ObjectPredicateDef::OwnedBy(PlayerRelation::You) }, then: &EffectDef::ExileGrantingOwnerPlay { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), surcharge: mana_cost!("{0}") }, otherwise: &EffectDef::ExileGrantingOwnerPlay { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), surcharge: mana_cost!("{2}") } })
]),
);

// BRO 72 — Weakstone's Subjugation
static AN_ARTIFACT_OR_CREATURE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

pub(in crate::card::sets) static WEAKSTONE_S_SUBJUGATION: CardRecord = CardRecord::new(
    "Weakstone's Subjugation",
    "ef93ac79-8575-40f8-a222-63c2ffb30f60",
    "Igor Kieryluk",
    // One mana to hold a permanent down permanently; the {3} is only for
    // catching one that is already untapped.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .enchanting(AN_ARTIFACT_OR_CREATURE)
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    AN_ARTIFACT_OR_CREATURE,
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, you may pay {3}. If you do, tap enchanted permanent.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::GenericMana(ValueDef::Constant(3))],
                    &EffectDef::Tap {
                        object: EffectRecipientDef::AttachedPermanent,
                    },
                )),
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// BRO 77 — Combat Courier
// Audit: unsupported — The sacrifice-to-draw activation is expressible, but the card also has
// unearth. The current engine has no unearth program for returning the source from its graveyard
// with the delayed exile and replacement if it would leave the battlefield.
pub(in crate::card::sets) static COMBAT_COURIER_77: CardRecord = CardRecord::new(
    "Combat Courier",
    "171edf80-ffc1-4894-9be5-c3e93a96f734",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// BRO 98 — Gixian Infiltrator
pub(in crate::card::sets) static GIXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    "Gixian Infiltrator",
    "c94a3317-7d1f-4f29-8353-180f1ab48d18",
    "Peter Polach",
    // Any permanent, not just a creature, which is what makes it a payoff
    // for the artifact deck this set is built around.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Phyrexian", "Human"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you sacrifice another permanent, put a +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                // "Another" excludes this creature, so sacrificing it to its
                // own outlet never grows a body that has already left.
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// BRO 127 — Bitter Reunion
pub(in crate::card::sets) static BITTER_REUNION_127: CardRecord = CardRecord::new(
    "Bitter Reunion",
    "345a1c80-41d6-43b1-83ab-1aa56dd06b1b",
    "Jake Murray",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, you may discard a card. If you do, draw two cards.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::WithCosts {
                    costs: &[CostDef::DiscardCards(1)],
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                },
            },
        ),
        AbilityDef::activated(
            "{1}, Sacrifice this enchantment: Creatures you control gain haste until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// BRO 131 — Dwarven Forge-Chanter
pub(in crate::card::sets) static DWARVEN_FORGE_CHANTER_131: CardRecord = CardRecord::new(
    "Dwarven Forge-Chanter",
    "bbd6a95a-11b9-43aa-b293-20a3102bae71",
    "Bartłomiej Gaweł",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Wizard"], 1, 3).with_abilities(&[
        abilities::ward(&[CostDef::PayLife(2)], "Ward—Pay 2 life. (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays 2 life.)"),
        abilities::prowess(),
    ]),
);

// BRO 136 — Giant Cindermaw
pub(in crate::card::sets) static GIANT_CINDERMAW: CardRecord = CardRecord::new(
    "Giant Cindermaw",
    "1349465f-d29f-4d4b-a653-f4388574c336",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dinosaur", "Beast"], 4, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Players can't gain life.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
            },
        ),
    ]),
);

// BRO 145 — Obliterating Bolt
pub(in crate::card::sets) static OBLITERATING_BOLT: CardRecord = CardRecord::new(
    "Obliterating Bolt",
    "7f886411-8216-4fb7-9172-a408c39043ee",
    "Campbell White",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Obliterating Bolt deals 4 damage to target creature or \
         planeswalker. If that creature or planeswalker would die this \
         turn, exile it instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// BRO 164 — Scrapwork Mutt
// Audit: unsupported — Needs unearth; see First-Sphere Gargantua. The optional discard-then-draw entry is expressible.
pub(in crate::card::sets) static SCRAPWORK_MUTT: CardRecord = CardRecord::new(
    "Scrapwork Mutt",
    "4742800a-4872-4c2d-b884-01e0ba16950c",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// BRO 174 — Bushwhack
pub(in crate::card::sets) static BUSHWHACK: CardRecord = CardRecord::new(
    "Bushwhack",
    "712a0640-d9c8-46fc-b38b-bf20a40fa902",
    "Artur Nakhodkin",
    // One mana that is never dead: it fixes a land drop early and answers a
    // creature late, which is what the modal split is buying.
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Search your library for a basic land card, reveal it, put it into your hand, \
                 then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature you don't control. (Each \
                 deals damage equal to its power to the other.)",
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    }),
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::NotYou),
                        owner: None,
                    }),
                ],
                // Fighting is one event rather than two damage clauses, so
                // neither creature dies before dealing its own damage.
                EffectDef::Fight {
                    first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    second: ObjectRefDef::Target(TargetIndex(1)),
                    excess: None,
                },
            ),
        ],
    )),
);

// BRO 175 — Citanul Stalwart
// Audit: unsupported — The mana-ability planner rejects a selected artifact-or-creature tap cost in addition to tapping this source.
pub(in crate::card::sets) static CITANUL_STALWART_175: CardRecord = CardRecord::new(
    "Citanul Stalwart",
    "a842a945-21d9-432c-b970-6da65b16f309",
    "Alexandr Leskinen",
    crate::card::CardRules::unsupported(),
);

// BRO 185 — Gwenna, Eyes of Gaea
// Audit: unsupported — Mana restrictions are conjunctive, so no restriction permits either creature spell casting or creature-source activation while excluding all other payments.
pub(in crate::card::sets) static GWENNA_EYES_OF_GAEA_185: CardRecord = CardRecord::new(
    "Gwenna, Eyes of Gaea",
    "7ee387b7-18e4-41b7-aefe-f2b5954e3051",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// BRO 199 — Haywire Mite
pub(in crate::card::sets) static HAYWIRE_MITE: CardRecord = CardRecord::new(
    "Haywire Mite",
    "847a175e-ead1-4596-baf3-5f7f57859e0b",
    "Izzy",
    // One mana for a body that is never dead: it answers whichever artifact
    // or enchantment the format is afraid of this week, and every deck can
    // cast it whether or not it can pay the green.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::activated_with_targets(
            "{G}, Sacrifice this creature: Exile target noncreature artifact or noncreature \
             enchantment.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::SacrificeSource],
            // "Noncreature artifact or noncreature enchantment." The two types are
            // alternatives and the exclusion applies to both, so it sits outside the
            // choice rather than inside it -- which is what leaves a creature that
            // happens to be an artifact alone.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// BRO 223 — Third Path Iconoclast
pub(in crate::card::sets) static THIRD_PATH_ICONOCLAST: CardRecord = CardRecord::new(
    "Third Path Iconoclast",
    "f1a21287-e244-4960-84fb-c4f6e5c346d9",
    "Manuel Castañón",
    // Two mana for a body that turns every cantrip into an artifact
    // creature, which is what the deck around it is counting.
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Monk"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, create a 1/1 colorless Soldier artifact \
             creature token.",
            // A noncreature spell of your own. What it does is no part of the trigger:
            // the Soldier arrives whether the spell resolves, is countered, or is
            // answered on the stack.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ])),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Soldier"], &[], 1, 1),
            ))),
        ),
    ),
);

// BRO 235 — Goblin Firebomb
pub(in crate::card::sets) static GOBLIN_FIREBOMB: CardRecord = CardRecord::new(
    "Goblin Firebomb",
    "0ba00d0f-0ea5-417c-a792-06b3b9d1c8f1",
    "Noah Thatcher",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{7}, {T}, Sacrifice this artifact: Destroy target permanent.",
            &[
                CostDef::Mana(mana_cost!("{7}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// BRO 238 — The Mightstone and Weakstone
pub(in crate::card::sets) static THE_MIGHTSTONE_AND_WEAKSTONE: CardRecord = CardRecord::new(
    "The Mightstone and Weakstone",
    "02aea379-b444-46a3-82f4-3038f698d4f4",
    "Ryan Pancoast",
    // Five mana for two cards or a dead creature, and two mana a turn
    // afterwards. The meld is Urza's ability rather than this card's: the
    // parenthesis here only says which card it pairs with.
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Powerstone"])
        .with_abilities(&[
            AbilityDef::modal_triggered(
                "When this artifact enters, choose one —\n• Draw two cards.\n• Target creature \
                 gets -5/-5 until end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[
                    AbilityDef::spell(
                        "Draw two cards.",
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target creature gets -5/-5 until end of turn.",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(-5),
                                ValueDef::Constant(-5),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ],
            ),
            AbilityDef::activated_mana(
                "{T}: Add {C}{C}. This mana can't be spent to cast nonartifact spells.",
                &[CostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Colorless)
                        .with_amount(2)
                        // A Powerstone's restriction is a prohibition rather than a permission:
                        // this mana activates abilities and pays for artifact spells, and the one
                        // thing it cannot do is cast a spell that is not an artifact.
                        .with_restrictions(&[ManaRestrictionDef::CannotCastSpell(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Artifact,
                            )),
                        )]),
                ),
            ),
        ]),
);

// BRO 240 — Portal to Phyrexia
pub(in crate::card::sets) static PORTAL_TO_PHYREXIA: CardRecord = CardRecord::new(
    "Portal to Phyrexia",
    "5f608efc-0dbc-4cc3-aadd-ed473bfc29ab",
    "Svetlin Velinov",
    // Nine mana, and the game is over: three of their creatures die on the
    // way in and one comes back for you every upkeep afterwards.
    CardRules::new_artifact(mana_cost!("{9}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, each opponent sacrifices three creatures of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(3),
                then: None,
                amount: crate::card::SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, put target creature card from a graveyard onto the \
             battlefield under your control. It's a Phyrexian in addition to its other types.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // Any graveyard, not only yours: the Portal is as happy to take back what
            // it made an opponent sacrifice as anything of your own.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: crate::card::BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
                binding: crate::ParentBinding,
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successors(
                        crate::ParentBinding,
                    ),
                    // "It's a Phyrexian in addition to its other types." Added rather than set:
                    // what comes back through the Portal keeps whatever it already was, and is
                    // a Phyrexian as well.
                    effect: AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::CreatureTypes(SetOperationDef::Add(
                            CreatureTypeSetDef::named(&["Phyrexian"]),
                        )),
                    ),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ]),
);

// BRO 260 — Demolition Field
pub(in crate::card::sets) static DEMOLITION_FIELD: CardRecord = CardRecord::new(
    "Demolition Field",
    "d9c88546-13c9-4d7e-a618-cb2ccd1dbc0f",
    "Kamila Szutenberg",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Sacrifice this land: Destroy target nonbasic land \
             an opponent controls. That land's controller may search their \
             library for a basic land card, put it onto the battlefield, \
             then shuffle. You may search your library for a basic land \
             card, put it onto the battlefield, then shuffle.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::May {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
            ]),
        ),
    ]),
);

// BRO 266 — Tocasia's Dig Site
pub(in crate::card::sets) static TOCASIA_S_DIG_SITE: CardRecord = CardRecord::new(
    "Tocasia's Dig Site",
    "23d4b90c-95b1-4828-bc08-7067da0d5364",
    "Nadia Hurianova",
    // Colourless and untapped, so the sink costs nothing to include: it is
    // a Wastes that does something on the turns nothing else does.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// BRO 305 — Myrel, Shield of Argive
pub(in crate::card::sets) static MYREL_SHIELD_OF_ARGIVE_305: CardRecord = CardRecord::new(
    "Myrel, Shield of Argive",
    "977da60c-073a-42d1-b9f5-789a2b7071b8",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 3, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("During your turn, your opponents can't cast spells or activate abilities of artifacts, creatures, or enchantments.", EffectDef::IfCondition { condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You), then: &EffectDef::Sequence(&[EffectDef::StaticApply { recipient: EffectRecipientDef::Opponent, effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any))) }, EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Enchantment)]), &[ZoneKind::Battlefield], PlayerRelation::Opponent), effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities(AbilityPredicateDef::Is(AbilityKindDef::Activated))) }]) }),
AbilityDef::triggered("Whenever Myrel attacks, create X 1/1 colorless Soldier artifact creature tokens, where X is the number of Soldiers you control.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::create_artifact_creature_token(&["Soldier"], &[], 1, 1).with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")), &[ZoneKind::Battlefield], PlayerRelation::You))))
]),
);

// BRO 313 — Drafna, Founder of Lat-Nam
pub(in crate::card::sets) static DRAFNA_FOUNDER_OF_LAT_NAM_313: CardRecord = CardRecord::new(
    "Drafna, Founder of Lat-Nam",
    "c3f9fd87-5c9b-4732-b8a5-f6be360a5fa5",
    "Lie Setiawan",
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Human", "Artificer", "Advisor"],
        2,
        1,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{U}: Return target artifact you control to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Copy target artifact spell you control. (The copy becomes a token.)",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Spell,
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: false,
                colors: None,
            }),
        ),
    ]),
);

// BRO 363 — Cityscape Leveler
// Audit: unsupported — Unearth requires its distinct graveyard activation, delayed end-step exile, and replacement of every later battlefield departure. There is no complete shared unearth procedure.
pub(in crate::card::sets) static CITYSCAPE_LEVELER_363: CardRecord = CardRecord::new(
    "Cityscape Leveler",
    "35d2bcd1-3ed3-4b99-9bb6-d0fa0a9f2ea1",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// BRO 364 — Liberator, Urza's Battlethopter
// Audit: unsupported — The cast-as-though-flash permission is representable, but the triggered
// clause needs the total amount of mana spent to cast its triggering spell. The engine records
// ColorsOfManaSpent, not a spent-mana amount, so it cannot compare that amount to this creature's
// current power without omitting a printed trigger condition.
pub(in crate::card::sets) static LIBERATOR_URZA_S_BATTLETHOPTER_364: CardRecord = CardRecord::new(
    "Liberator, Urza's Battlethopter",
    "04acd5af-bd55-4c16-9b6d-10822d564c14",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// BRO 368 — Thran Spider
pub(in crate::card::sets) static THRAN_SPIDER_368: CardRecord = CardRecord::new(
    "Thran Spider",
    "42c400de-25cb-4865-ad1b-9a8a8da3da55",
    "Joshua Cairos",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Spider"], 2, 4).with_abilities(&[
abilities::reach(),
AbilityDef::triggered_with_targets("When this creature enters, you and target opponent each create a tapped Powerstone token.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Sequence(&[EffectDef::create_artifact_token(&["Powerstone"], &[]).with_abilities(&[AbilityDef::activated_mana("{T}: Add {C}. This mana can't be spent to cast nonartifact spells.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_restrictions(&[ManaRestrictionDef::CannotCastSpell(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)))])))]).entering_tapped(), EffectDef::create_artifact_token(&["Powerstone"], &[]).with_abilities(&[AbilityDef::activated_mana("{T}: Add {C}. This mana can't be spent to cast nonartifact spells.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_restrictions(&[ManaRestrictionDef::CannotCastSpell(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)))])))]).entering_tapped().with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY))])),
AbilityDef::activated("{7}: Look at the top four cards of your library. You may reveal an artifact card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.", &[CostDef::Mana(mana_cost!("{7}"))], abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(ValueDef::Constant(4), ObjectPredicateDef::HasType(CardType::Artifact), 0, 1))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEADLY_RIPOSTE,
    &LORAN_OF_THE_THIRD_PATH,
    &RECOMMISSION_22,
    &RECRUITMENT_OFFICER_23,
    &SOUL_PARTITION_26,
    &WEAKSTONE_S_SUBJUGATION,
    &COMBAT_COURIER_77,
    &GIXIAN_INFILTRATOR,
    &BITTER_REUNION_127,
    &DWARVEN_FORGE_CHANTER_131,
    &GIANT_CINDERMAW,
    &OBLITERATING_BOLT,
    &SCRAPWORK_MUTT,
    &BUSHWHACK,
    &CITANUL_STALWART_175,
    &GWENNA_EYES_OF_GAEA_185,
    &HAYWIRE_MITE,
    &THIRD_PATH_ICONOCLAST,
    &GOBLIN_FIREBOMB,
    &THE_MIGHTSTONE_AND_WEAKSTONE,
    &PORTAL_TO_PHYREXIA,
    &DEMOLITION_FIELD,
    &TOCASIA_S_DIG_SITE,
    &MYREL_SHIELD_OF_ARGIVE_305,
    &DRAFNA_FOUNDER_OF_LAT_NAM_313,
    &CITYSCAPE_LEVELER_363,
    &LIBERATOR_URZA_S_BATTLETHOPTER_364,
    &THRAN_SPIDER_368,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
