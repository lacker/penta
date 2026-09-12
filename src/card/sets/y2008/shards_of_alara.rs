//! Shards of Alara cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
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
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ALA",
    slug: "shards-of-alara",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ALA 3 — Angelic Benediction
pub(in crate::card::sets) static ANGELIC_BENEDICTION: CardRecord = CardRecord::new(
    "Angelic Benediction",
    "dd1b9071-7dde-4128-8b18-1d7b07904638",
    "Michael Komarck",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_abilities(&[
        abilities::exalted(),
        AbilityDef::triggered_with_targets(
            "Whenever a creature you control attacks alone, you may tap target creature.",
            TriggerEventDef::attacks_in_declaration(
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                1,
                Some(1),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// ALA 9 — Elspeth, Knight-Errant
pub(in crate::card::sets) static ELSPETH_KNIGHT_ERRANT: CardRecord = CardRecord::new(
    "Elspeth, Knight-Errant",
    "44c52e52-2b1c-4ca8-ab6d-20d97a342704",
    "Volkan Baǵa",
// Four mana, two plus abilities, and neither of them is the safe one:
    // she makes a blocker or she makes an attacker, and the ultimate ends
    // the game against anything that answers permanents.
    CardRules::new_planeswalker(mana_cost!("{2}{W}{W}"), &["Elspeth"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Create a 1/1 white Soldier creature token.",
                &[CostDef::Loyalty(1)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
                ))),
            ),
            // The second plus is what makes her a threat rather than a hedge: any
            // creature, so the token she made last turn is a 4/4 flier this one.
            AbilityDef::activated_with_targets(
                "+1: Target creature gets +3/+3 and gains flying until end of turn.",
                &[CostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "\u{2212}8: You get an emblem with \"Artifacts, creatures, enchantments, and lands you \
                 control have indestructible.\"",
                &[CostDef::Loyalty(-8)],
                EffectDef::create_emblem("Elspeth, Knight-Errant emblem", &[AbilityDef::static_ability(
                    "Artifacts, creatures, enchantments, and lands you control have indestructible.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            // The four types the emblem names, which between them are every permanent
                            // a white deck is likely to control. Written as one alternation because the
                            // emblem grants one thing to all of them.
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    },
                )]),
            ),
        ]),
);

// ALA 10 — Ethersworn Canonist
// Audit: unsupported — The spell-history cast limit counts all spells; it cannot count only nonartifact spells while still allowing further artifact spells.
pub(in crate::card::sets) static ETHERSWORN_CANONIST_10: CardRecord = CardRecord::new(
    "Ethersworn Canonist",
    "2aebe7a8-b982-4be4-83ca-3594e8f606b4",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// ALA 12 — Guardians of Akrasa
pub(in crate::card::sets) static GUARDIANS_OF_AKRASA: CardRecord = CardRecord::new(
    "Guardians of Akrasa",
    "4f718030-cc41-4e0d-a1ca-a33f577dc1fb",
    "Alan Pollack",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 0, 4)
        .with_abilities(&[abilities::defender(), abilities::exalted()]),
);

// ALA 21 — Ranger of Eos
pub(in crate::card::sets) static RANGER_OF_EOS_21: CardRecord = CardRecord::new(
    "Ranger of Eos",
    "1a30ee26-5f78-4ac2-9105-1baa9ece8a21",
    "Volkan Baǵa",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier", "Ranger"], 3, 2).with_abilities(&[
abilities::enters_trigger("When this creature enters, you may search your library for up to two creature cards with mana value 1 or less, reveal them, put them into your hand, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMost(1)]), minimum: 0, maximum: ValueDef::Constant(2), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// ALA 42 — Etherium Sculptor
pub(in crate::card::sets) static ETHERIUM_SCULPTOR_42: CardRecord = CardRecord::new(
    "Etherium Sculptor",
    "0d050f2d-bd65-4ab9-9ea6-9deba91b2792",
    "Steven Belledin",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Vedalken", "Artificer"], 1, 2)
        .with_abilities(&[AbilityDef::static_ability(
            "Artifact spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::HasType(CardType::Artifact),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        )]),
);

// ALA 44 — Filigree Sages
pub(in crate::card::sets) static FILIGREE_SAGES_44: CardRecord = CardRecord::new(
    "Filigree Sages",
    "08790aaf-0142-4b20-89cf-cdaffeea4582",
    "Dan Murayama Scott",
    CardRules::new_artifact_creature(mana_cost!("{3}{U}"), &["Vedalken", "Wizard"], 2, 3)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{2}{U}: Untap target artifact.",
            &[CostDef::Mana(mana_cost!("{2}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )]),
);

// ALA 60 — Tezzeret the Seeker
// Audit: unsupported — Loyalty activation costs are fixed signed integers; the activated-cost planner cannot choose and pay a variable −X loyalty cost.
pub(in crate::card::sets) static TEZZERET_THE_SEEKER_60: CardRecord = CardRecord::new(
    "Tezzeret the Seeker",
    "3b214b6f-4734-4200-8467-92d7e3469b5d",
    "Anthony Francisco",
    crate::card::CardRules::unsupported(),
);

// ALA 63 — Ad Nauseam
// Audit: unsupported — The effect graph has no player-controlled repeat loop that reveals, moves to hand, and loses the revealed card’s mana value before offering another iteration.
pub(in crate::card::sets) static AD_NAUSEAM_63: CardRecord = CardRecord::new(
    "Ad Nauseam",
    "0a4ce4a1-65e3-4b40-be35-8fc55a968ec8",
    "Jeremy Jarvis",
    crate::card::CardRules::unsupported(),
);

// ALA 67 — Bone Splinters
pub(in crate::card::sets) static BONE_SPLINTERS: CardRecord = CardRecord::new(
    "Bone Splinters",
    "d4a4b3a3-b7ae-4210-8037-098fdf5808d0",
    "Cole Eastburn",
    // The sacrifice is paid on the way to the stack, so the creature it eats
    // is gone before the target is destroyed -- and the spell can eat the
    // very creature it is aimed at only if something else is left to target.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, sacrifice a creature.\nDestroy target \
             creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        CostDef::sacrifice(
            ObjectPredicateDef::HasType(CardType::Creature),
            CostQuantityDef::Fixed(1),
        ),
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// ALA 70 — Death Baron
pub(in crate::card::sets) static DEATH_BARON: CardRecord = CardRecord::new(
    "Death Baron",
    "4d59b5e5-fc16-4f1a-9f17-f42908473531",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Zombie", "Wizard"], 2, 2).with_abilities(
        &[AbilityDef::static_ability(
            "Skeletons you control and other Zombies you control get +1/+1 \
             and have deathtouch. (Any amount of damage they deal to a \
             creature is enough to destroy it.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Skeleton")),
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                            ]),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
            },
        )],
    ),
);

// ALA 97 — Dragon Fodder
pub(in crate::card::sets) static DRAGON_FODDER: CardRecord = CardRecord::new(
    "Dragon Fodder",
    "9eab4120-e7d8-4132-a304-30b88e3175e2",
    "Jaime Jones",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell(
        "Create two 1/1 red Goblin creature tokens.",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                &["Goblin"],
                &[ManaColor::Red],
                1,
                1,
            )))
            .with_count(ValueDef::Constant(2)),
        ),
    )]),
);

// ALA 100 — Flameblast Dragon
// Audit: unsupported — Needs an optional X-mana payment made as the attack trigger resolves.
pub(in crate::card::sets) static FLAMEBLAST_DRAGON: CardRecord = CardRecord::new(
    "Flameblast Dragon",
    "5544b26b-0bc4-4c1b-9616-613e9bf08557",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// ALA 104 — Hissing Iguanar
pub(in crate::card::sets) static HISSING_IGUANAR: CardRecord = CardRecord::new(
    "Hissing Iguanar",
    "4b8b8b90-cb6e-4910-bc40-d96b78b0d70c",
    "Brandon Kitkouski",
    // Every trade on the board becomes a point of reach, which is what makes
    // a fragile 3/1 worth playing in a deck that is already racing.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard"], 3, 1).with_ability(
        abilities::dies_trigger_matching_with_targets(
            "Whenever another creature dies, you may have this creature deal 1 damage to target \
             player or planeswalker.",
            // "Another": its own death does not feed it, so a board wipe
            // gives it one fewer ping than it looks like.
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// ALA 107 — Lightning Talons
pub(in crate::card::sets) static LIGHTNING_TALONS: CardRecord = CardRecord::new(
    "Lightning Talons",
    "8fc1ad4c-b394-48b9-9a5a-ec9f42bf6a00",
    "Pete Venters",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+0 and has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            ),
        ]),
);

// ALA 130 — Elvish Visionary
pub(in crate::card::sets) static ELVISH_VISIONARY: CardRecord = CardRecord::new(
    "Elvish Visionary",
    "faccfa5f-4d89-4a86-92d7-36cb5a16c5c9",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Shaman"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ALA 156 — Blightning
pub(in crate::card::sets) static BLIGHTNING: CardRecord = CardRecord::new(
    "Blightning",
    "3c05e8a2-b7d0-4f24-b2ae-8e4db30e5842",
    "Thomas M. Baxa",
// Three damage and two cards for three mana, which is why it was the
    // aggressive deck's answer to a control opponent rather than to a board.
    CardRules::new_sorcery(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Blightning deals 3 damage to target player or planeswalker. That player or that planeswalker's controller discards two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            // A target naming a player resolves to that player, so one
            // recipient says both halves of "that player or that
            // planeswalker's controller".
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// ALA 158 — Branching Bolt
pub(in crate::card::sets) static BRANCHING_BOLT: CardRecord = CardRecord::new(
    "Branching Bolt",
    "e7468876-f401-4a75-81c0-bed09cdda3e1",
    "Vance Kovacs",
    // Three mana for three damage is a poor rate until both modes are live,
    // which is the whole design: it is a two-for-one or it is overpriced.
    CardRules::new_instant(mana_cost!("{1}{R}{G}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one or both —",
            &[
                AbilityDef::spell_with_targets(
                    "Branching Bolt deals 3 damage to target creature with flying.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ]),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(3),
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Branching Bolt deals 3 damage to target creature without flying.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                KeywordAbility::Flying,
                            )),
                        ]),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(3),
                    ),
                ),
            ],
        )
        // Both modes may be chosen, and each carries its own target, so the
        // two halves never land on the same creature.
        .with_mode_selection(1, 2, false),
    ),
);

// ALA 194 — Sharuum the Hegemon
pub(in crate::card::sets) static SHARUUM_THE_HEGEMON_194: CardRecord = CardRecord::new(
    "Sharuum the Hegemon",
    "6589eaa8-95ec-4c97-8155-185487560ae6",
    "Izzy",
    CardRules::new_artifact_creature(mana_cost!("{3}{W}{U}{B}"), &["Sphinx"], 5, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::flying(),
abilities::enters_trigger_with_targets("When Sharuum enters, you may return target artifact card from your graveyard to the battlefield.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Artifact), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top) })
]),
);

// ALA 202 — Tidehollow Sculler
pub(in crate::card::sets) static TIDEHOLLOW_SCULLER: CardRecord = CardRecord::new(
    "Tidehollow Sculler",
    "1abecc77-07f2-43e4-8585-0a8199cdcf01",
    "rk post",
CardRules::new_artifact_creature(mana_cost!("{W}{B}"), &["Zombie"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent reveals their hand and you choose a nonland card from it. Exile that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_choose_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // Linked to the Sculler rather than exiled outright, which is the whole
                    // bargain: the card is gone only for as long as the body survives.
                    &EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        face_down: false,
                        then: None,
                    },
                )),
            ),
            // Leaves, not dies: bouncing or exiling the Sculler gives the card back
            // just as killing it does.
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Hand,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ),
        ]),
);

// ALA 218 — Relic of Progenitus
pub(in crate::card::sets) static RELIC_OF_PROGENITUS_218: CardRecord = CardRecord::new(
    "Relic of Progenitus",
    "90c41192-64ef-43aa-9af0-75f0d3f56688",
    "Jean-Sébastien Rossbach",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target player exiles a card from their graveyard.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::ChooseExact(crate::card::ChooseExactDef {
                binding: Binding!("graveyard_exile"),
                chooser: crate::card::PlayerRefDef::Target(TargetIndex::PRIMARY),
                candidates: ObjectSetDef::Query(crate::card::ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    crate::card::PlayerSetDef::One(crate::card::PlayerRefDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                )),
                exclude: None,
                amount: ValueDef::Constant(1),
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("graveyard_exile"))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            }),
        ),
        AbilityDef::activated(
            "{1}, Exile this artifact: Exile all graveyards. Draw a card.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::ExileSource],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Any,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// ALA 220 — Arcane Sanctum
pub(in crate::card::sets) static ARCANE_SANCTUM_220: CardRecord = CardRecord::new(
    "Arcane Sanctum",
    "6edc0681-4252-4d3d-baf3-f03c22af1208",
    "Anthony Francisco",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {W}, {U}, or {B}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue, ManaColor::Black]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELIC_BENEDICTION,
    &ELSPETH_KNIGHT_ERRANT,
    &ETHERSWORN_CANONIST_10,
    &GUARDIANS_OF_AKRASA,
    &RANGER_OF_EOS_21,
    &ETHERIUM_SCULPTOR_42,
    &FILIGREE_SAGES_44,
    &TEZZERET_THE_SEEKER_60,
    &AD_NAUSEAM_63,
    &BONE_SPLINTERS,
    &DEATH_BARON,
    &DRAGON_FODDER,
    &FLAMEBLAST_DRAGON,
    &HISSING_IGUANAR,
    &LIGHTNING_TALONS,
    &ELVISH_VISIONARY,
    &BLIGHTNING,
    &BRANCHING_BOLT,
    &SHARUUM_THE_HEGEMON_194,
    &TIDEHOLLOW_SCULLER,
    &RELIC_OF_PROGENITUS_218,
    &ARCANE_SANCTUM_220,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
