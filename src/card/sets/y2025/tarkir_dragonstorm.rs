//! Tarkir: Dragonstorm card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PayOrDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayOptionDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::QuantifierDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellForm;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::CardPartId;
use crate::ids::ParentBinding;
use crate::ids::PlayOptionId;
use crate::ids::TargetIndex;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2015::dragons_of_tarkir as catalog_dtk;
use crate::card::sets::y2021::kaldheim as catalog_khm;
/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TDM",
    slug: "tarkir-dragonstorm",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

/// The delayed half of mobilize: the tokens this attack made go away at the
/// next end step, and it has to be exactly those. By then nothing about the
/// board could tell them from the ones the last attack made, or from a
/// Warrior that arrived some other way, so they are bound as they are
/// created and this names the binding.
static MOBILIZE_SACRIFICE: EffectDef = EffectDef::InstallTrigger(
    crate::card::InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, sacrifice those tokens.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
            ParentBinding,
        ))),
    )),
);

/// Mobilize N (CR 702.180a): "Whenever this creature attacks, create N tapped
/// and attacking 1/1 red Warrior creature tokens. Sacrifice them at the
/// beginning of the next end step."
///
/// Written out as the triggered ability it abbreviates. The caller supplies
/// the printed text because the reminder spells the number out in words.
#[must_use]
pub(in crate::card::sets) const fn mobilize(count: u16, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Warrior"], &[ManaColor::Red], 1, 1).with_art(
                    crate::card::CardArt::new(
                        "7edc0515-a130-45a7-aa09-0e23bba41587",
                        "Forrest Imel",
                    ),
                ),
            ))
            .with_amount(count)
            .entering_tapped()
            .entering_attacking()
            .with_created_tokens(CreatedTokensDef {
                binding: ParentBinding,
                then: &MOBILIZE_SACRIFICE,
            }),
        ),
    )
}


// TDM 1 — Ugin, Eye of the Storms
/// "Up to one target permanent that's one or more colors": colorless is what
/// Ugin does not touch, which is the whole bargain of the deck built around
/// him -- your own artifacts and Eldrazi are safe from every one of these
/// triggers.
static UP_TO_ONE_COLORED_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static UGIN_EXILES_IT: EffectDef = EffectDef::move_to_zone(
    EffectRecipientDef::Target(TargetIndex::PRIMARY),
    ZoneKind::Exile,
    ZonePlacement::Top,
);

pub(in crate::card::sets) static UGIN_EYE_OF_THE_STORMS: CardRecord = CardRecord::new(
    "Ugin, Eye of the Storms",
    "64a5d494-efa1-446b-bebe-2ad36e154376",
    "Joshua Raphael",
// Seven mana that answers something the moment it is cast and again for
    // every colorless spell after it, pays for the next one itself, and
    // eventually empties the library onto the table for free.
    CardRules::new_planeswalker(mana_cost!("{7}"), &["Ugin"], 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you cast this spell, exile up to one target permanent that's one or more colors.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                &UP_TO_ONE_COLORED_PERMANENT,
                UGIN_EXILES_IT,
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you cast a colorless spell, exile up to one target permanent that's one or \
                 more colors.",
                // A colorless spell you cast, which is every spell the deck around him is
                // made of. His own cast is not one of these: he is still on the stack, and
                // this clause is read off the battlefield.
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ColorCount(0),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                &UP_TO_ONE_COLORED_PERMANENT,
                UGIN_EXILES_IT,
            ),
            AbilityDef::activated(
                "+2: You gain 3 life and draw a card.",
                &[CostDef::Loyalty(2)],
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
            // A loyalty ability that makes mana is still a mana ability: it never
            // uses the stack, and it is still the one loyalty ability he may use
            // this turn.
            AbilityDef::activated_mana(
                "0: Add {C}{C}{C}.",
                &[CostDef::Loyalty(0)],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
            ),
            AbilityDef::activated(
                "\u{2212}11: Search your library for any number of colorless nonland cards, exile them, \
                 then shuffle. Until end of turn, you may cast those cards without paying their mana \
                 costs.",
                &[CostDef::Loyalty(-11)],
                // "Any number": the bound is the library, so the search offers everything
                // that matches and takes as many as its controller wants.
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ColorCount(0),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(i32::MAX),
                    reveal: false,
                    destination: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: Some(ParentBinding),
                    // "Until end of turn, you may cast those cards without paying their mana
                    // costs": the cards the search just exiled, named by what it bound rather
                    // than by anything about exile, since a card that was already there is not
                    // one of them.
                    then: Some(&EffectDef::MayPlayWithoutPaying(FreePlayDef {
                        objects: ObjectSetDef::Binding(ParentBinding),
                        // "Until end of turn" is printed, so this one outlives its resolution.
                        duration: FreePlayDurationDef::UntilEndOfTurn,
                        mandatory: false,
                        grants_haste: false,
                    })),
                },
            ),
        ]),
);

// TDM 2 — Anafenza, Unyielding Lineage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANAFENZA_UNYIELDING_LINEAGE: CardRecord = CardRecord::new(
    "Anafenza, Unyielding Lineage",
    "29957f49-9a6b-42f6-b2fb-b48f653ab725",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// TDM 3 — Arashin Sunshield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARASHIN_SUNSHIELD: CardRecord = CardRecord::new(
    "Arashin Sunshield",
    "dd7102d8-90b3-45a1-b66d-dcca469b1fb6",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TDM 4 — Bearer of Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEARER_OF_GLORY: CardRecord = CardRecord::new(
    "Bearer of Glory",
    "d6d91e42-43db-428d-a4dd-ef9d40306314",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// TDM 5 — Clarion Conqueror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLARION_CONQUEROR: CardRecord = CardRecord::new(
    "Clarion Conqueror",
    "f892d156-371c-4391-8ae6-25513c5032b0",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TDM 6 — Coordinated Maneuver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COORDINATED_MANEUVER: CardRecord = CardRecord::new(
    "Coordinated Maneuver",
    "c6569487-53c5-4b91-877d-e4e31bfa90c0",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// TDM 7 — Dalkovan Packbeasts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DALKOVAN_PACKBEASTS: CardRecord = CardRecord::new(
    "Dalkovan Packbeasts",
    "4df7b253-6107-47d6-b650-cb4d3e0aec6b",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// TDM 8 — Descendant of Storms
pub(in crate::card::sets) static DESCENDANT_OF_STORMS: CardRecord = CardRecord::new(
    "Descendant of Storms",
    "f632be90-9e7f-41f8-a52e-a2952354d730",
    "Lie Setiawan",
    // A one-mana 2/1 that attacks well early and has somewhere to put mana
    // late. Which half of endure you want changes with the board: the
    // counter makes the attack bigger, the Spirit makes the next one wider.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {1}{W}. If you do, it endures 1.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}{W}"))], // "It endures 1": the counter or the Spirit, and the attacking body is
                // what either one is about.
                &EffectDef::Endure {
                    object: EffectRecipientDef::Source,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ),
);

// TDM 9 — Dragonback Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONBACK_LANCER: CardRecord = CardRecord::new(
    "Dragonback Lancer",
    "0200a8c5-3293-48d0-a523-ba148680f588",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// TDM 10 — Duty Beyond Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUTY_BEYOND_DEATH: CardRecord = CardRecord::new(
    "Duty Beyond Death",
    "2e92640d-768b-4357-905f-bea017d351cc",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// TDM 11 — Elspeth, Storm Slayer (alternate printing)
const ELSPETH_STORM_SLAYER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELSPETH_STORM_SLAYER,
    1,
    "73a065e3-b530-4e62-ab3c-4f6f908184ec",
    "Ekaterina Burmak",
);

// TDM 12 — Fortress Kin-Guard
pub(in crate::card::sets) static FORTRESS_KIN_GUARD: CardRecord = CardRecord::new(
    "Fortress Kin-Guard",
    "b647a018-1d70-43a1-a265-928bcd863689",
    "Daneen Wilkerson",
    // Two mana for two bodies or one bigger one, and the choice is made
    // where it matters: a board that wants a blocker takes the Spirit.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog", "Soldier"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, it endures 1. (Put a +1/+1 counter on it or create a 1/1 \
             white Spirit creature token.)",
            EffectDef::Endure {
                object: EffectRecipientDef::Source,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TDM 13 — Furious Forebear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURIOUS_FOREBEAR: CardRecord = CardRecord::new(
    "Furious Forebear",
    "a4f247b6-8212-4e78-a452-d2d3be228d8e",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// TDM 14 — Lightfoot Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTFOOT_TECHNIQUE: CardRecord = CardRecord::new(
    "Lightfoot Technique",
    "baac1a41-d44d-4184-9147-b4233e73de65",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// TDM 15 — Loxodon Battle Priest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOXODON_BATTLE_PRIEST: CardRecord = CardRecord::new(
    "Loxodon Battle Priest",
    "a527cdb0-f54a-4b53-83a0-6b3e8cafa45e",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// TDM 16 — Mardu Devotee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARDU_DEVOTEE: CardRecord = CardRecord::new(
    "Mardu Devotee",
    "da45e9b0-a4f6-413b-9e62-666c511eb5b0",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// TDM 17 — Osseous Exhale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSSEOUS_EXHALE: CardRecord = CardRecord::new(
    "Osseous Exhale",
    "2300da2f-2297-4c2f-90c1-11ce2b42d91f",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// TDM 18 — Poised Practitioner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POISED_PRACTITIONER: CardRecord = CardRecord::new(
    "Poised Practitioner",
    "bb25366d-a647-4c5e-bcc7-7e54659aacbd",
    "Alessandra Pisano",
    crate::card::CardRules::unsupported(),
);

// TDM 19 — Rally the Monastery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RALLY_THE_MONASTERY: CardRecord = CardRecord::new(
    "Rally the Monastery",
    "b56e0037-8143-4c13-83e1-0c3f44e685ea",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// TDM 20 — Rebellious Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REBELLIOUS_STRIKE: CardRecord = CardRecord::new(
    "Rebellious Strike",
    "c9bafe19-3bd6-4da0-b3e5-e0b89262504c",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// TDM 21 — Riling Dawnbreaker
const fn riling_dawnbreaker_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Dragon"], 3, 4).with_abilities(
        &const {
            [
                abilities::flying(),
                abilities::vigilance(),
                AbilityDef::triggered_with_targets(
                    "At the beginning of combat on your turn, another target creature you \
                     control gets +1/+0 until end of turn.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::BeginningOfCombat,
                        player: PlayerRelation::You,
                    },
                    // "Another": the Dragon is already the biggest attacker,
                    // so the bonus always goes to something beside it.
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ]
        },
    )
}

fn riling_dawnbreaker_composition() -> CardComposition {
    let dragon = riling_dawnbreaker_rules();
    let roar = const {
        CardRules::new_sorcery(mana_cost!("{1}{W}"))
            .with_subtypes(&["Omen"])
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Soldier creature token.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 2, 2),
                    ))),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Riling Dawnbreaker", dragon),
            CardPart::new(CardPartId(1), "Signaling Roar", roar),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Omen,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Riling Dawnbreaker",
                SpellForm::Part(CardPartId::PRIMARY),
                dragon
                    .mana_cost()
                    .expect("the Dragon has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Signaling Roar",
                SpellForm::Part(CardPartId(1)),
                roar.mana_cost()
                    .expect("Signaling Roar has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static RILING_DAWNBREAKER: CardRecord = CardRecord::new(
    "Riling Dawnbreaker",
    "312f7072-3bf8-449f-bfb7-93727ef26c66",
    "Tuan Duong Chu",
    // A body early and a bigger one later out of one card, and the Dragon
    // pushes whatever the Omen left behind.
    riling_dawnbreaker_rules(),
)
.with_composition(riling_dawnbreaker_composition);

// TDM 22 — Sage of the Skies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGE_OF_THE_SKIES: CardRecord = CardRecord::new(
    "Sage of the Skies",
    "6ade6918-6d1d-448d-ab56-93996051e9a9",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// TDM 23 — Salt Road Packbeast
pub(in crate::card::sets) static SALT_ROAD_PACKBEAST: CardRecord = CardRecord::new(
    "Salt Road Packbeast",
    "98d548c9-42bc-4155-8211-0aea801c3724",
    "Ben Wootten",
    // Six mana printed, but a board that has already gone wide pays a
    // fraction of it, and the card it draws makes the turn no worse.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Beast"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for creatures (This spell costs {1} less to cast for each creature you \
             control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read while the card is in hand: this prices the spell, so it has
        // to apply from the zone the spell is cast out of.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TDM 24 — Smile at Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMILE_AT_DEATH: CardRecord = CardRecord::new(
    "Smile at Death",
    "ae2da18f-0d7d-446c-b463-8bf170ed95da",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// TDM 25 — Starry-Eyed Skyrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARRY_EYED_SKYRIDER: CardRecord = CardRecord::new(
    "Starry-Eyed Skyrider",
    "4b3cc15e-1c82-454e-b541-4ab47c44814e",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// TDM 26 — Static Snare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STATIC_SNARE: CardRecord = CardRecord::new(
    "Static Snare",
    "1ce50932-03a6-48bc-8aee-bc8defd896cf",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// TDM 27 — Stormbeacon Blade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMBEACON_BLADE: CardRecord = CardRecord::new(
    "Stormbeacon Blade",
    "f2f12684-c80a-422b-9c3f-ed4f31742b9d",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// TDM 28 — Stormplain Detainment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMPLAIN_DETAINMENT: CardRecord = CardRecord::new(
    "Stormplain Detainment",
    "39f3aab5-7b54-4b55-8114-c6f9f79c255d",
    "Livia Prima",
    crate::card::CardRules::unsupported(),
);

// TDM 29 — Sunpearl Kirin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNPEARL_KIRIN: CardRecord = CardRecord::new(
    "Sunpearl Kirin",
    "18292b9c-0f42-4ce2-8b85-35d06cf45a63",
    "Allen Morris",
    crate::card::CardRules::unsupported(),
);

// TDM 30 — Teeming Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEEMING_DRAGONSTORM: CardRecord = CardRecord::new(
    "Teeming Dragonstorm",
    "3b9d771f-24dc-4ed6-8051-62df576a2ba5",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// TDM 31 — Tempest Hawk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPEST_HAWK: CardRecord = CardRecord::new(
    "Tempest Hawk",
    "422f9453-ab12-4e3c-8c51-be87391395a1",
    "Abz J Harding",
    crate::card::CardRules::unsupported(),
);

// TDM 32 — United Battlefront
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNITED_BATTLEFRONT: CardRecord = CardRecord::new(
    "United Battlefront",
    "dff398be-4ba4-4976-9acc-be99d2e07a61",
    "Darren Tan",
    crate::card::CardRules::unsupported(),
);

// TDM 33 — Voice of Victory
pub(in crate::card::sets) static VOICE_OF_VICTORY: CardRecord = CardRecord::new(
    "Voice of Victory",
    "ec3de5f4-bb55-4ab9-995f-f3e0dc22c1bb",
    "Joshua Cairos",
// Two mana that adds two power to every attack and turns off every
    // instant your opponent was holding for the turn you attack.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Bard"], 1, 3)
        .with_abilities(&[
            mobilize(
                2,
                "Mobilize 2 (Whenever this creature attacks, create two tapped and attacking 1/1 red Warrior \
                 creature tokens. Sacrifice them at the beginning of the next end step.)",
            ),
            AbilityDef::static_ability(
                "Your opponents can't cast spells during your turn.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    // "During your turn" is the whole of the clause's timing, and it gates the
                    // restriction rather than narrowing who it names: on their own turn the
                    // same opponents may cast whatever they like.
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any))),
                    },
                },
            ),
        ]),
);

// TDM 34 — Wayspeaker Bodyguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAYSPEAKER_BODYGUARD: CardRecord = CardRecord::new(
    "Wayspeaker Bodyguard",
    "5e5b2324-69fe-4105-b6f8-14dfbe359d59",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TDM 35 — Aegis Sculptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_SCULPTOR: CardRecord = CardRecord::new(
    "Aegis Sculptor",
    "19c1417a-9719-46f6-8749-d92b93ce0529",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TDM 36 — Agent of Kotis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_OF_KOTIS: CardRecord = CardRecord::new(
    "Agent of Kotis",
    "812d0462-0158-467f-951d-a7a121188a10",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// TDM 37 — Ambling Stormshell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBLING_STORMSHELL: CardRecord = CardRecord::new(
    "Ambling Stormshell",
    "c74d4a57-0f66-4965-9ed7-f88a08aa1d15",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// TDM 38 — Bewildering Blizzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEWILDERING_BLIZZARD: CardRecord = CardRecord::new(
    "Bewildering Blizzard",
    "91b25843-1aa0-484a-b6c7-0c284fe7214a",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// TDM 39 — Constrictor Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSTRICTOR_SAGE: CardRecord = CardRecord::new(
    "Constrictor Sage",
    "b2f160d7-c832-4b83-8f2e-aaeb190add3f",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// TDM 40 — Dirgur Island Dragon // Skimming Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRGUR_ISLAND_DRAGON: CardRecord = CardRecord::new(
    "Dirgur Island Dragon // Skimming Strike",
    "b1d21a9a-6b0c-4fbc-a427-81be885d326b",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// TDM 41 — Dispelling Exhale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISPELLING_EXHALE: CardRecord = CardRecord::new(
    "Dispelling Exhale",
    "1c9af3f1-711e-42ae-803a-1100eba3fb13",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// TDM 42 — Dragonologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONOLOGIST: CardRecord = CardRecord::new(
    "Dragonologist",
    "8810ebb4-9e51-46f0-a54a-a0b4d77b762a",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// TDM 43 — Dragonstorm Forecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONSTORM_FORECASTER: CardRecord = CardRecord::new(
    "Dragonstorm Forecaster",
    "75ec7a31-1893-493c-926b-dc3a8a770e72",
    "Kev Fang",
    crate::card::CardRules::unsupported(),
);

// TDM 44 — Essence Anchor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_ANCHOR: CardRecord = CardRecord::new(
    "Essence Anchor",
    "e91c4509-918e-44ba-aa13-1991199fee9f",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// TDM 45 — Focus the Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOCUS_THE_MIND: CardRecord = CardRecord::new(
    "Focus the Mind",
    "abb0ba34-6904-4c17-a04d-ea4f12c7cf21",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// TDM 46 — Fresh Start
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRESH_START: CardRecord = CardRecord::new(
    "Fresh Start",
    "96f7af08-ac05-45d0-979f-282943130c61",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// TDM 47 — Highspire Bell-Ringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGHSPIRE_BELL_RINGER: CardRecord = CardRecord::new(
    "Highspire Bell-Ringer",
    "e75dccf7-2894-4c4a-b516-3eee73acddd3",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// TDM 48 — Humbling Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUMBLING_ELDER: CardRecord = CardRecord::new(
    "Humbling Elder",
    "3a84c3f8-0030-4653-880e-b2d19272f5fa",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// TDM 49 — Iceridge Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICERIDGE_SERPENT: CardRecord = CardRecord::new(
    "Iceridge Serpent",
    "d13f117b-b8e4-48db-8ce9-5da9c7ce23a5",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TDM 50 — Kishla Trawlers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KISHLA_TRAWLERS: CardRecord = CardRecord::new(
    "Kishla Trawlers",
    "190fbc55-e8e9-4077-9532-1de7406baabf",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// TDM 51 — Marang River Regent // Coil and Catch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARANG_RIVER_REGENT: CardRecord = CardRecord::new(
    "Marang River Regent // Coil and Catch",
    "f890bdc7-32e6-4492-bac7-7cabf54a8bfd",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// TDM 52 — Naga Fleshcrafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAGA_FLESHCRAFTER: CardRecord = CardRecord::new(
    "Naga Fleshcrafter",
    "5df17423-9fdd-4432-8660-1d267c685595",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// TDM 53 — Ringing Strike Mastery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RINGING_STRIKE_MASTERY: CardRecord = CardRecord::new(
    "Ringing Strike Mastery",
    "ff4fc7ec-05f5-479a-8fbb-31e12a67b57e",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// TDM 54 — Riverwalk Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVERWALK_TECHNIQUE: CardRecord = CardRecord::new(
    "Riverwalk Technique",
    "043c25d5-13ee-4cab-98d5-fb89db9cf6e3",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// TDM 55 — Roiling Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROILING_DRAGONSTORM: CardRecord = CardRecord::new(
    "Roiling Dragonstorm",
    "455f4c96-684b-4b14-bd21-6799da2e1fa7",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 56 — Sibsig Appraiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIBSIG_APPRAISER: CardRecord = CardRecord::new(
    "Sibsig Appraiser",
    "670c5b96-bac6-449b-a2bd-cb43750d3911",
    "Ina Wong",
    crate::card::CardRules::unsupported(),
);

// TDM 57 — Snowmelt Stag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWMELT_STAG: CardRecord = CardRecord::new(
    "Snowmelt Stag",
    "a6b3b131-704a-4586-84f8-db465cd4a277",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// TDM 58 — Spectral Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_DENIAL: CardRecord = CardRecord::new(
    "Spectral Denial",
    "ee4e732a-1ffd-463d-92c2-26187659cfc3",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// TDM 59 — Stillness in Motion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STILLNESS_IN_MOTION: CardRecord = CardRecord::new(
    "Stillness in Motion",
    "a6289251-17e4-4987-96b9-2fb1a8f90e2a",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// TDM 60 — Taigam, Master Opportunist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAIGAM_MASTER_OPPORTUNIST: CardRecord = CardRecord::new(
    "Taigam, Master Opportunist",
    "8693d631-05f6-414d-9e49-6385746e8960",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// TDM 61 — Temur Devotee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMUR_DEVOTEE: CardRecord = CardRecord::new(
    "Temur Devotee",
    "a2ef698e-5466-43bd-985d-020f2e5d8205",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// TDM 62 — Unending Whisper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNENDING_WHISPER: CardRecord = CardRecord::new(
    "Unending Whisper",
    "fc48180a-ccac-469f-938d-c050821d0160",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// TDM 63 — Ureni's Rebuff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URENI_S_REBUFF: CardRecord = CardRecord::new(
    "Ureni's Rebuff",
    "722716df-9cea-40a7-924b-c28497e227e6",
    "Sergio Cosmai",
    crate::card::CardRules::unsupported(),
);

// TDM 64 — Veteran Ice Climber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_ICE_CLIMBER: CardRecord = CardRecord::new(
    "Veteran Ice Climber",
    "bcccfd7b-2846-4552-a89a-2b868bc9ab20",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TDM 65 — Wingblade Disciple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGBLADE_DISCIPLE: CardRecord = CardRecord::new(
    "Wingblade Disciple",
    "71de7dca-0231-4407-86a0-c7fc95f5aaa0",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// TDM 66 — Wingspan Stride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGSPAN_STRIDE: CardRecord = CardRecord::new(
    "Wingspan Stride",
    "339a0e24-c332-4558-bb60-f5504ddde88c",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// TDM 67 — Winternight Stories
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTERNIGHT_STORIES: CardRecord = CardRecord::new(
    "Winternight Stories",
    "64d9367c-f50c-4568-aa63-6760c44ecaeb",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// TDM 68 — Abzan Devotee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABZAN_DEVOTEE: CardRecord = CardRecord::new(
    "Abzan Devotee",
    "66555946-e747-46fa-b1ac-b103a8edcd93",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// TDM 69 — Adorned Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADORNED_CROCODILE: CardRecord = CardRecord::new(
    "Adorned Crocodile",
    "bb13a34b-6ac8-47cb-9e91-47106a585fc1",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TDM 70 — Aggressive Negotiations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSIVE_NEGOTIATIONS: CardRecord = CardRecord::new(
    "Aggressive Negotiations",
    "993ade84-031f-4a3e-bd68-55f61b559248",
    "Ovidio Cartagena",
    crate::card::CardRules::unsupported(),
);

// TDM 71 — Alchemist's Assistant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALCHEMIST_S_ASSISTANT: CardRecord = CardRecord::new(
    "Alchemist's Assistant",
    "4d305609-64f8-4f3f-bf67-cd5257f0d01e",
    "Eelis Kyttanen",
    crate::card::CardRules::unsupported(),
);

// TDM 72 — Alesha's Legacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALESHA_S_LEGACY: CardRecord = CardRecord::new(
    "Alesha's Legacy",
    "a9262bf6-df6a-446c-ba70-18270a09842d",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// TDM 73 — Avenger of the Fallen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGER_OF_THE_FALLEN: CardRecord = CardRecord::new(
    "Avenger of the Fallen",
    "d5397366-151f-46b0-b9b2-fa4d5bd892d8",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// TDM 74 — Caustic Exhale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUSTIC_EXHALE: CardRecord = CardRecord::new(
    "Caustic Exhale",
    "488152ce-2048-4ccb-b2d6-b9628958286f",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// TDM 75 — Corroding Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORRODING_DRAGONSTORM: CardRecord = CardRecord::new(
    "Corroding Dragonstorm",
    "e2a2d395-26d6-4eb2-9e8c-ed7dbbd3a8f5",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// TDM 76 — Cruel Truths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_TRUTHS: CardRecord = CardRecord::new(
    "Cruel Truths",
    "6852b4d5-74e0-44ba-ba44-20aa91e3c4c8",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TDM 77 — Delta Bloodflies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELTA_BLOODFLIES: CardRecord = CardRecord::new(
    "Delta Bloodflies",
    "119bb72d-aed9-47dc-9285-7bc836cc3776",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TDM 78 — Desperate Measures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_MEASURES: CardRecord = CardRecord::new(
    "Desperate Measures",
    "ccbc6fd0-42bc-4e8b-96bc-69a631ba7106",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 79 — Dragon's Prey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_S_PREY: CardRecord = CardRecord::new(
    "Dragon's Prey",
    "7a6004ff-4180-4332-8b51-960f8c7521d9",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// TDM 80 — Feral Deathgorger // Dusk Sight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERAL_DEATHGORGER: CardRecord = CardRecord::new(
    "Feral Deathgorger // Dusk Sight",
    "a147b94f-dfcf-44ce-8a73-b2fe6c4efc0e",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// TDM 81 — Gurmag Rakshasa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GURMAG_RAKSHASA: CardRecord = CardRecord::new(
    "Gurmag Rakshasa",
    "f05ad909-8860-473b-9a30-a322f7670b32",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// TDM 82 — Hundred-Battle Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNDRED_BATTLE_VETERAN: CardRecord = CardRecord::new(
    "Hundred-Battle Veteran",
    "e53adf93-2db5-4087-a2dc-c8f53401d700",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// TDM 83 — Kin-Tree Nurturer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIN_TREE_NURTURER: CardRecord = CardRecord::new(
    "Kin-Tree Nurturer",
    "2177ef64-28bf-4acf-b1f1-c1408f03c411",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// TDM 84 — Krumar Initiate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRUMAR_INITIATE: CardRecord = CardRecord::new(
    "Krumar Initiate",
    "bc66680f-24ab-433a-8197-feac3a174075",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// TDM 85 — Nightblade Brigade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTBLADE_BRIGADE: CardRecord = CardRecord::new(
    "Nightblade Brigade",
    "648debd9-d4cf-4788-8882-f1601a3d87f5",
    "Gary Laib",
    crate::card::CardRules::unsupported(),
);

// TDM 86 — Qarsi Revenant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QARSI_REVENANT: CardRecord = CardRecord::new(
    "Qarsi Revenant",
    "8c93a0f6-5e50-4dda-9ff6-da741fb839ff",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// TDM 87 — Rot-Curse Rakshasa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROT_CURSE_RAKSHASA: CardRecord = CardRecord::new(
    "Rot-Curse Rakshasa",
    "31276460-fa9d-47da-85c5-c4baa8074d0d",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// TDM 88 — Salt Road Skirmish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALT_ROAD_SKIRMISH: CardRecord = CardRecord::new(
    "Salt Road Skirmish",
    "8f529a2e-5102-492e-84ab-68541d83b5a3",
    "Arif Wijaya",
    crate::card::CardRules::unsupported(),
);

// TDM 89 — Sandskitter Outrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSKITTER_OUTRIDER: CardRecord = CardRecord::new(
    "Sandskitter Outrider",
    "1c4bfebe-f10f-44bd-9368-33e273ba5a55",
    "Arif Wijaya",
    crate::card::CardRules::unsupported(),
);

// TDM 90 — Scavenger Regent // Exude Toxin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCAVENGER_REGENT: CardRecord = CardRecord::new(
    "Scavenger Regent // Exude Toxin",
    "0d4b46a3-847a-44a7-9f68-2cb4657cad61",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// TDM 91 — The Sibsig Ceremony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SIBSIG_CEREMONY: CardRecord = CardRecord::new(
    "The Sibsig Ceremony",
    "5a9f2a62-1c61-4d2e-86d9-18cd84c31748",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// TDM 92 — Sidisi, Regent of the Mire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIDISI_REGENT_OF_THE_MIRE: CardRecord = CardRecord::new(
    "Sidisi, Regent of the Mire",
    "47374d23-662b-4ba7-a94f-37c9bc759cc6",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// TDM 93 — Sinkhole Surveyor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINKHOLE_SURVEYOR: CardRecord = CardRecord::new(
    "Sinkhole Surveyor",
    "37cb5599-7d2c-48e9-978b-902a01a74bde",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// TDM 94 — Strategic Betrayal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRATEGIC_BETRAYAL: CardRecord = CardRecord::new(
    "Strategic Betrayal",
    "95617742-548d-464a-bb89-a858ffa9018f",
    "Flavio Greco Paglia",
    crate::card::CardRules::unsupported(),
);

// TDM 95 — Unburied Earthcarver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNBURIED_EARTHCARVER: CardRecord = CardRecord::new(
    "Unburied Earthcarver",
    "3ab5e71e-dc8d-4ed8-bcef-6497177c4a9d",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TDM 96 — Unrooted Ancestor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNROOTED_ANCESTOR: CardRecord = CardRecord::new(
    "Unrooted Ancestor",
    "6394b125-21a8-4439-9958-94b76684138e",
    "Elizabeth Peiró",
    crate::card::CardRules::unsupported(),
);

// TDM 97 — Venerated Stormsinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENERATED_STORMSINGER: CardRecord = CardRecord::new(
    "Venerated Stormsinger",
    "a4a4e985-facd-47e6-b680-3023c82c2957",
    "Elizabeth Peiró",
    crate::card::CardRules::unsupported(),
);

// TDM 98 — Wail of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAIL_OF_WAR: CardRecord = CardRecord::new(
    "Wail of War",
    "7e9430dd-f583-400d-808a-64e2b5fa54f1",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// TDM 99 — Worthy Cost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORTHY_COST: CardRecord = CardRecord::new(
    "Worthy Cost",
    "adc18edc-01d8-4a7e-a87b-a854e50aa75e",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// TDM 100 — Yathan Tombguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YATHAN_TOMBGUARD: CardRecord = CardRecord::new(
    "Yathan Tombguard",
    "9e65d487-705a-4c3b-9bb6-69351e5dae81",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TDM 101 — Breaching Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREACHING_DRAGONSTORM: CardRecord = CardRecord::new(
    "Breaching Dragonstorm",
    "e2c2a069-7553-4879-abfb-b2aa3349e4b8",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// TDM 102 — Channeled Dragonfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANNELED_DRAGONFIRE: CardRecord = CardRecord::new(
    "Channeled Dragonfire",
    "24204881-690c-4043-8771-20cb93385072",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// TDM 103 — Cori-Steel Cutter (alternate printing)
const CORI_STEEL_CUTTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CORI_STEEL_CUTTER,
    1,
    "490eb213-9ae2-4b45-abec-6f1dfc83792a",
    "Xabi Gaztelua",
);

// TDM 104 — Devoted Duelist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOTED_DUELIST: CardRecord = CardRecord::new(
    "Devoted Duelist",
    "bbf9c673-37b4-48ed-a9ea-13f8e3e6c47b",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TDM 105 — Dracogenesis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRACOGENESIS: CardRecord = CardRecord::new(
    "Dracogenesis",
    "0d5674f9-22b2-45f9-902d-4fd245485c60",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// TDM 106 — Equilibrium Adept
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EQUILIBRIUM_ADEPT: CardRecord = CardRecord::new(
    "Equilibrium Adept",
    "a4ba6d74-c6be-4a5e-8859-b791bb6b8f51",
    "Leroy Steinmann",
    crate::card::CardRules::unsupported(),
);

// TDM 107 — Fire-Rim Form
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRE_RIM_FORM: CardRecord = CardRecord::new(
    "Fire-Rim Form",
    "32dc1bf4-a135-449f-848f-361a5360fae1",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// TDM 108 — Fleeting Effigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETING_EFFIGY: CardRecord = CardRecord::new(
    "Fleeting Effigy",
    "1971fd6c-0a1c-41b2-93a6-886a176fbb73",
    "Darren Tan",
    crate::card::CardRules::unsupported(),
);

// TDM 109 — Iridescent Tiger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRIDESCENT_TIGER: CardRecord = CardRecord::new(
    "Iridescent Tiger",
    "e3abbc8b-2bf8-478e-a541-f8019d150054",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TDM 110 — Jeskai Devotee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKAI_DEVOTEE: CardRecord = CardRecord::new(
    "Jeskai Devotee",
    "27f31f9c-7149-4608-9b18-b3530a2efd4a",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TDM 111 — Magmatic Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMATIC_HELLKITE: CardRecord = CardRecord::new(
    "Magmatic Hellkite",
    "b3b3aec8-d931-4c7f-86b5-1e7dfb717b59",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// TDM 112 — Meticulous Artisan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METICULOUS_ARTISAN: CardRecord = CardRecord::new(
    "Meticulous Artisan",
    "baf4c9dd-0546-41ac-a7ba-0bc312fef31e",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// TDM 113 — Molten Exhale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_EXHALE: CardRecord = CardRecord::new(
    "Molten Exhale",
    "0ab95aab-a4bf-4131-83a0-2c138b6f20c3",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// TDM 114 — Narset's Rebuke
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NARSET_S_REBUKE: CardRecord = CardRecord::new(
    "Narset's Rebuke",
    "5098bd73-d51c-4db4-bf06-fd4854089d37",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// TDM 115 — Overwhelming Surge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERWHELMING_SURGE: CardRecord = CardRecord::new(
    "Overwhelming Surge",
    "bd7af85f-354e-468a-990b-bd774e68240f",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 116 — Rescue Leopard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESCUE_LEOPARD: CardRecord = CardRecord::new(
    "Rescue Leopard",
    "056136a8-84be-477c-b654-63238fb8236e",
    "Hector Ortiz",
    crate::card::CardRules::unsupported(),
);

// TDM 117 — Reverberating Summons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERBERATING_SUMMONS: CardRecord = CardRecord::new(
    "Reverberating Summons",
    "1af19ce8-bc0c-420c-9e3b-9059b4df32cb",
    "Marco Gorlei",
    crate::card::CardRules::unsupported(),
);

// TDM 118 — Sarkhan, Dragon Ascendant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARKHAN_DRAGON_ASCENDANT: CardRecord = CardRecord::new(
    "Sarkhan, Dragon Ascendant",
    "c2200646-7b7c-489d-bbae-16b03e1d7fb2",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// TDM 119 — Seize Opportunity
pub(in crate::card::sets) static SEIZE_OPPORTUNITY: CardRecord = CardRecord::new(
    "Seize Opportunity",
    "f7818d28-b9a5-4341-9adc-666070b8878d",
    "Josiah \"Jo\" Cameron",
    // Cards when the board is empty, reach when it is not. Neither half is
    // worth three mana alone; being able to pick at instant speed is.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Exile the top two cards of your library. Until the end of your next turn, you \
                 may play those cards.",
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    free: false,
                    face_down: false,
                    duration: ExilePlayDurationDef::UntilEndOfYourNextTurn,
                    spend_any_color: false,
                    play_condition: None,
                    cast_only: false,
                },
            ),
            AbilityDef::spell_with_targets(
                "Up to two target creatures each get +2/+1 until end of turn.",
                // "Up to two" and not "two": cast for this half with a single
                // creature on the board, it still resolves on that one.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// TDM 120 — Shock Brigade
pub(in crate::card::sets) static SHOCK_BRIGADE: CardRecord = CardRecord::new(
    "Shock Brigade",
    "66940466-8e9d-4a85-bfb0-e92189b7a121",
    "Fajareka Setiawan",
    // A 1/3 body nobody blocks profitably, attacking as two creatures. The
    // Warrior is gone by the end step, so what mobilize buys is damage on
    // this attack rather than a board.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Soldier"], 1, 3).with_abilities(&[
        abilities::menace(),
        mobilize(
            1,
            "Mobilize 1 (Whenever this creature attacks, create a tapped and attacking 1/1 red \
             Warrior creature token. Sacrifice it at the beginning of the next end step.)",
        ),
    ]),
);

// TDM 121 — Shocking Sharpshooter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCKING_SHARPSHOOTER: CardRecord = CardRecord::new(
    "Shocking Sharpshooter",
    "4a10342d-ca04-4d1e-bca9-79f531951a16",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// TDM 122 — Stadium Headliner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STADIUM_HEADLINER: CardRecord = CardRecord::new(
    "Stadium Headliner",
    "37d4ab2a-a06a-4768-b5e1-e1def957d7f4",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// TDM 123 — Stormscale Scion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCALE_SCION: CardRecord = CardRecord::new(
    "Stormscale Scion",
    "0ac43386-bd32-425c-8776-cec00b064cbc",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// TDM 124 — Stormshriek Feral // Flush Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSHRIEK_FERAL: CardRecord = CardRecord::new(
    "Stormshriek Feral // Flush Out",
    "0ec92c44-7cf0-48a5-a3ca-bc633496d887",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// TDM 125 — Summit Intimidator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUMMIT_INTIMIDATOR: CardRecord = CardRecord::new(
    "Summit Intimidator",
    "e3cba0b1-7c22-4e51-b9cf-5bf01e67a222",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// TDM 126 — Sunset Strikemaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSET_STRIKEMASTER: CardRecord = CardRecord::new(
    "Sunset Strikemaster",
    "f8f1a2f2-526d-4b2c-985b-0acfdc21a2ee",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// TDM 127 — Tersa Lightshatter
pub(in crate::card::sets) static TERSA_LIGHTSHATTER: CardRecord = CardRecord::new(
    "Tersa Lightshatter",
    "99e96b34-b1c4-4647-a38e-2cf1aedaaace",
    "Olivier Bernard",
// Three mana for a 3/3 that attacks immediately and turns a spent hand
    // into a card a turn. What she asks for is the graveyard the deck was
    // filling anyway.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Wizard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            abilities::enters_trigger(
                "When Tersa Lightshatter enters, discard up to two cards, then draw that many cards.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Private,
                    // "Discard up to two cards, then draw that many." The size is the player's
                    // to choose, so the discard is a choice with a floor of none rather than a
                    // fixed number, and what is drawn is however many that turned out to be.
                    then: &EffectDef::Sequence(&[
                        EffectDef::discard_cards(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(ParentBinding),
                        )),
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::BoundObjectCount(ParentBinding),
                        },
                    ]),
                }),
            ),
            AbilityDef::triggered_if(
                "Whenever Tersa Lightshatter attacks, if there are seven or more cards in your graveyard, \
                 exile a card at random from your graveyard. You may play that card this turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // Seven cards is a real threshold rather than a formality: the attack that
                // turns it on is the one that has already spent a hand.
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        effect: &EffectDef::SelectAtRandomFromZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Graveyard,
                            object: ObjectPredicateDef::Any,
                            amount: ValueDef::Constant(1),
                        },
                        binding: Binding!("random_graveyard_card"),
                    },
                    EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            Binding!("random_graveyard_card"),
                        )),
                    },
                ]),
            ),
        ]),
);

// TDM 128 — Twin Bolt (reprint)
const TWIN_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::TWIN_BOLT,
    "688d8e93-d071-4089-9ef9-565ac4ae9ae0",
    "Craig J Spearing",
);

// TDM 129 — Underfoot Underdogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERFOOT_UNDERDOGS: CardRecord = CardRecord::new(
    "Underfoot Underdogs",
    "049acc79-1d68-410f-a081-88a7d40e823a",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// TDM 130 — Unsparing Boltcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSPARING_BOLTCASTER: CardRecord = CardRecord::new(
    "Unsparing Boltcaster",
    "204f5e5e-d87f-4aee-84e3-28afe8e21591",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 131 — War Effort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_EFFORT: CardRecord = CardRecord::new(
    "War Effort",
    "dd7f0413-c009-4c08-b877-9c1b776cbf26",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// TDM 132 — Wild Ride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_RIDE: CardRecord = CardRecord::new(
    "Wild Ride",
    "abc8c6f5-6135-428e-8476-1751f82623f9",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// TDM 133 — Zurgo's Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZURGO_S_VANGUARD: CardRecord = CardRecord::new(
    "Zurgo's Vanguard",
    "a1aa3501-5738-4063-a7f4-51d2600b0041",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TDM 134 — Ainok Wayfarer
pub(in crate::card::sets) static AINOK_WAYFARER: CardRecord = CardRecord::new(
    "Ainok Wayfarer",
    "57695a9b-8f72-4ccc-a946-5d5037b09b8f",
    "Filipe Pagliuso",
    // Never a blank: it finds a land when the draw is short and grows when
    // it is not, which is what two mana is buying.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dog", "Scout"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, mill three cards. You may put a land card from among \
             them into your hand. If you don't, put a +1/+1 counter on this creature. (To mill \
             three cards, put the top three cards of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    binding: Binding!("milled_cards"),
                },
                // A minimum of zero is the "you may", and a pile with no land
                // in it never asks.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // "From among them" is what this mill just put there,
                    // not what the graveyard already held.
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Binding(Binding!("milled_cards")),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                    },
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        // "If you don't" is read off what was taken rather
                        // than off what was offered: declining and having
                        // nothing to take both leave the counter.
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::BoundObjectCount(ParentBinding),
                                comparison: ComparisonDef::LessOrEqual,
                                right: ValueDef::Constant(0),
                            }),
                            then: &EffectDef::AddCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ]),
                }),
            ]),
        ),
    ),
);

// TDM 135 — Attuned Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATTUNED_HUNTER: CardRecord = CardRecord::new(
    "Attuned Hunter",
    "d1a4f502-86a9-49fb-9cb9-7918d13c5313",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// TDM 136 — Bloomvine Regent // Claim Territory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOMVINE_REGENT: CardRecord = CardRecord::new(
    "Bloomvine Regent // Claim Territory",
    "10e0a9a3-f63a-4f92-a083-9d181580e498",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// TDM 137 — Champion of Dusan
pub(in crate::card::sets) static CHAMPION_OF_DUSAN: CardRecord = CardRecord::new(
    "Champion of Dusan",
    "c51dcdab-38ee-4804-8859-09adc353c182",
    "Bastien L. Deharme",
// A 4/2 trades early and then hands its trample to something better
    // from the graveyard, which is the whole arc of the card.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Warrior"], 4, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Renew — {1}{G}, Exile this card from your graveyard: Put a +1/+1 counter and a trample counter on target creature. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}{G}")),
                CostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // A trample counter rather than a granted keyword: it stays on
            // the creature and survives anything that ends a duration.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Trample,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// TDM 138 — Craterhoof Behemoth (reprint)
const CRATERHOOF_BEHEMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_avr::CRATERHOOF_BEHEMOTH,
    "276f5cee-a501-4658-bd4d-7a044bf1ccbc",
    "Magali Villeneuve",
);

// TDM 139 — Dragon Sniper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_SNIPER: CardRecord = CardRecord::new(
    "Dragon Sniper",
    "074b1e00-45bb-4436-8f5e-058512b2d08a",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// TDM 140 — Dragonbroods' Relic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONBROODS_RELIC: CardRecord = CardRecord::new(
    "Dragonbroods' Relic",
    "3d634087-77ba-4543-aa7a-8a3774d69cd7",
    "Racrufi",
    crate::card::CardRules::unsupported(),
);

// TDM 141 — Dusyut Earthcarver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSYUT_EARTHCARVER: CardRecord = CardRecord::new(
    "Dusyut Earthcarver",
    "b98ecc96-f557-479a-8685-2b5487d5b407",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// TDM 142 — Encroaching Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCROACHING_DRAGONSTORM: CardRecord = CardRecord::new(
    "Encroaching Dragonstorm",
    "4ddd4477-f8c9-4d05-9177-f8344ba8f40b",
    "Marco Gorlei",
    crate::card::CardRules::unsupported(),
);

// TDM 143 — Formation Breaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORMATION_BREAKER: CardRecord = CardRecord::new(
    "Formation Breaker",
    "67ab8e8f-3ef6-4339-8c66-68c5aca4867a",
    "Eelis Kyttanen",
    crate::card::CardRules::unsupported(),
);

// TDM 144 — Herd Heirloom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERD_HEIRLOOM: CardRecord = CardRecord::new(
    "Herd Heirloom",
    "a88c7713-b3a9-4685-b1d3-623d35b62365",
    "Allen Morris",
    crate::card::CardRules::unsupported(),
);

// TDM 145 — Heritage Reclamation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERITAGE_RECLAMATION: CardRecord = CardRecord::new(
    "Heritage Reclamation",
    "4f8fee37-a050-4329-8b10-46d150e7a95e",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// TDM 146 — Inspirited Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRITED_VANGUARD: CardRecord = CardRecord::new(
    "Inspirited Vanguard",
    "c642d6ac-f0f0-4b4c-a7ee-50631531f6d1",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// TDM 147 — Knockout Maneuver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNOCKOUT_MANEUVER: CardRecord = CardRecord::new(
    "Knockout Maneuver",
    "9d218831-2a41-46a3-8e9d-93462cae5cab",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// TDM 148 — Krotiq Nestguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROTIQ_NESTGUARD: CardRecord = CardRecord::new(
    "Krotiq Nestguard",
    "a5d0a9fb-1068-478d-a78c-6fd77cc313f0",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// TDM 149 — Lasyd Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASYD_PROWLER: CardRecord = CardRecord::new(
    "Lasyd Prowler",
    "c7f5c8ef-8e9e-4264-a7d2-126a30a5d341",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// TDM 150 — Nature's Rhythm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_RHYTHM: CardRecord = CardRecord::new(
    "Nature's Rhythm",
    "1397d904-c51d-451e-8505-7f3118acc1f6",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// TDM 151 — Piercing Exhale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIERCING_EXHALE: CardRecord = CardRecord::new(
    "Piercing Exhale",
    "b2a0deb9-5bc3-42d5-9e1e-5f463d176aef",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// TDM 152 — Rainveil Rejuvenator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAINVEIL_REJUVENATOR: CardRecord = CardRecord::new(
    "Rainveil Rejuvenator",
    "9bc5c316-6a41-48ba-864b-da3030dd3e0e",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// TDM 153 — Rite of Renewal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITE_OF_RENEWAL: CardRecord = CardRecord::new(
    "Rite of Renewal",
    "f737698a-d934-4851-b238-828959ef4835",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 154 — Roamer's Routine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROAMER_S_ROUTINE: CardRecord = CardRecord::new(
    "Roamer's Routine",
    "fb8c2d5c-ba0c-4d50-8898-5c6574b1e974",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// TDM 155 — Sage of the Fang
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGE_OF_THE_FANG: CardRecord = CardRecord::new(
    "Sage of the Fang",
    "1ebf4a9d-d90c-4017-9f00-fca89899f301",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// TDM 156 — Sagu Pummeler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGU_PUMMELER: CardRecord = CardRecord::new(
    "Sagu Pummeler",
    "def9cb5b-4062-481e-b682-3a30443c2e56",
    "Francisco Badilla",
    crate::card::CardRules::unsupported(),
);

// TDM 157 — Sagu Wildling
const fn sagu_wildling_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Dragon"], 3, 3).with_abilities(
        &const {
            [
                abilities::flying(),
                abilities::enters_trigger(
                    "When this creature enters, you gain 3 life.",
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ),
            ]
        },
    )
}

fn sagu_wildling_composition() -> CardComposition {
    let wildling = sagu_wildling_rules();
    let seek = const {
        CardRules::new_sorcery(mana_cost!("{G}"))
            .with_subtypes(&["Omen"])
            .with_ability(
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
                )
                // An Omen shuffles itself back in rather than exiling, which
                // is the whole difference from an Adventure: the creature is
                // drawn again later instead of waiting in exile.
                .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Sagu Wildling", wildling),
            CardPart::new(CardPartId(1), "Roost Seek", seek),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Omen,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Sagu Wildling",
                SpellForm::Part(CardPartId::PRIMARY),
                wildling
                    .mana_cost()
                    .expect("the Dragon has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Roost Seek",
                SpellForm::Part(CardPartId(1)),
                seek.mana_cost()
                    .expect("Roost Seek has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static SAGU_WILDLING: CardRecord = CardRecord::new(
    "Sagu Wildling",
    "d8b43b00-f4d1-436c-bf3f-6d414cd4ce38",
    "Gaboleps",
    // A land on turn one and a five-drop later out of the same card, which
    // is what an Omen buys over a plain fetch spell.
    sagu_wildling_rules(),
)
.with_composition(sagu_wildling_composition);

// TDM 158 — Sarkhan's Resolve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARKHAN_S_RESOLVE: CardRecord = CardRecord::new(
    "Sarkhan's Resolve",
    "cae56fef-b661-4bc5-b9a1-3871ae06e491",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// TDM 159 — Snakeskin Veil (reprint)
const SNAKESKIN_VEIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SNAKESKIN_VEIL,
    "a3d2c692-7566-468e-9c86-47a9f768fde2",
    "Monztre",
);

// TDM 160 — Sultai Devotee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULTAI_DEVOTEE: CardRecord = CardRecord::new(
    "Sultai Devotee",
    "c32487e9-f3ac-472e-b6ea-81bd9254770c",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// TDM 161 — Surrak, Elusive Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURRAK_ELUSIVE_HUNTER: CardRecord = CardRecord::new(
    "Surrak, Elusive Hunter",
    "e4775a26-0b66-40e9-8f64-41d9308ca032",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// TDM 162 — Synchronized Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYNCHRONIZED_CHARGE: CardRecord = CardRecord::new(
    "Synchronized Charge",
    "1f721f8d-fd2f-480b-8645-4bf6ce38dde9",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// TDM 163 — Trade Route Envoy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_ROUTE_ENVOY: CardRecord = CardRecord::new(
    "Trade Route Envoy",
    "f0c89d95-d697-4cfa-9dfa-52d7adb96176",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// TDM 164 — Traveling Botanist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELING_BOTANIST: CardRecord = CardRecord::new(
    "Traveling Botanist",
    "890b11b4-777a-4f1e-8c4d-21c5ebbfb0a2",
    "Daneen Wilkerson",
    crate::card::CardRules::unsupported(),
);

// TDM 165 — Undergrowth Leopard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROWTH_LEOPARD: CardRecord = CardRecord::new(
    "Undergrowth Leopard",
    "67ab8f9a-b17c-452f-b4ef-a3f91909e3de",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// TDM 166 — Warden of the Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARDEN_OF_THE_GROVE: CardRecord = CardRecord::new(
    "Warden of the Grove",
    "2414db96-0e2b-4f7c-9b97-41f8e310b752",
    "Alexander Ostrowski",
    crate::card::CardRules::unsupported(),
);

// TDM 167 — All-Out Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALL_OUT_ASSAULT: CardRecord = CardRecord::new(
    "All-Out Assault",
    "b74876d8-f6a6-4b47-b960-b01a331bab01",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// TDM 168 — Armament Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMAMENT_DRAGON: CardRecord = CardRecord::new(
    "Armament Dragon",
    "17f61c01-0a41-4fa1-ac34-ffa83baad989",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// TDM 169 — Auroral Procession
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURORAL_PROCESSION: CardRecord = CardRecord::new(
    "Auroral Procession",
    "672f94ad-65d6-4c7d-925d-165ef264626f",
    "Marco Gorlei",
    crate::card::CardRules::unsupported(),
);

// TDM 170 — Awaken the Honored Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AWAKEN_THE_HONORED_DEAD: CardRecord = CardRecord::new(
    "Awaken the Honored Dead",
    "14078a49-2230-4ad7-aea0-0c253813c646",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 171 — Barrensteppe Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRENSTEPPE_SIEGE: CardRecord = CardRecord::new(
    "Barrensteppe Siege",
    "2556a35b-2229-42c7-8cb3-c8c668403dd2",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// TDM 172 — Betor, Kin to All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BETOR_KIN_TO_ALL: CardRecord = CardRecord::new(
    "Betor, Kin to All",
    "b475b071-5545-483e-a397-89451f258602",
    "Alexander Ostrowski",
    crate::card::CardRules::unsupported(),
);

// TDM 173 — Bone-Cairn Butcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_CAIRN_BUTCHER: CardRecord = CardRecord::new(
    "Bone-Cairn Butcher",
    "78bf36bc-6702-4c5d-b52d-ab7217cc8787",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// TDM 174 — Call the Spirit Dragons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_THE_SPIRIT_DRAGONS: CardRecord = CardRecord::new(
    "Call the Spirit Dragons",
    "b1ad91db-5f16-4392-baf1-f8400ec11e0a",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// TDM 175 — Cori Mountain Stalwart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORI_MOUNTAIN_STALWART: CardRecord = CardRecord::new(
    "Cori Mountain Stalwart",
    "b6cbf54e-f30e-4e7b-b17c-217fa424971c",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// TDM 176 — Death Begets Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_BEGETS_LIFE: CardRecord = CardRecord::new(
    "Death Begets Life",
    "1faab43d-587d-44f6-9516-c8e3965bbc20",
    "Justin Hernandez & Alexis Hernandez",
    crate::card::CardRules::unsupported(),
);

// TDM 177 — Defibrillating Current
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFIBRILLATING_CURRENT: CardRecord = CardRecord::new(
    "Defibrillating Current",
    "bf3a18cf-03db-4eb0-8d53-0c1a71e184da",
    "Isis",
    crate::card::CardRules::unsupported(),
);

// TDM 178 — Disruptive Stormbrood // Petty Revenge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPTIVE_STORMBROOD: CardRecord = CardRecord::new(
    "Disruptive Stormbrood // Petty Revenge",
    "bd78e8ae-e927-40e7-9580-966c5e81f53c",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// TDM 179 — Dragonback Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONBACK_ASSAULT: CardRecord = CardRecord::new(
    "Dragonback Assault",
    "d54cc838-d79d-433a-99fb-d6e4d1c1431d",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// TDM 180 — Dragonclaw Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONCLAW_STRIKE: CardRecord = CardRecord::new(
    "Dragonclaw Strike",
    "bc7692ef-7091-4365-85a8-1edbd374f279",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// TDM 181 — Effortless Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EFFORTLESS_MASTER: CardRecord = CardRecord::new(
    "Effortless Master",
    "0ae03ca5-cd4b-42b7-8cd5-3f7e753b4147",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// TDM 182 — Eshki Dragonclaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESHKI_DRAGONCLAW: CardRecord = CardRecord::new(
    "Eshki Dragonclaw",
    "0d369c44-78ee-4f3c-bf2b-cddba7fe26d4",
    "Tran Nguyen",
    crate::card::CardRules::unsupported(),
);

// TDM 183 — Fangkeeper's Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANGKEEPER_S_FAMILIAR: CardRecord = CardRecord::new(
    "Fangkeeper's Familiar",
    "655fa2e1-3e1c-424c-b17a-daa7b8fface4",
    "David Szabo",
    crate::card::CardRules::unsupported(),
);

// TDM 184 — Felothar, Dawn of the Abzan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FELOTHAR_DAWN_OF_THE_ABZAN: CardRecord = CardRecord::new(
    "Felothar, Dawn of the Abzan",
    "83e11f20-6524-4fba-9603-0b97e2d69aac",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// TDM 185 — Flamehold Grappler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEHOLD_GRAPPLER: CardRecord = CardRecord::new(
    "Flamehold Grappler",
    "cc8443a6-282f-4218-9dc8-144b5570d891",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// TDM 186 — Frontline Rush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRONTLINE_RUSH: CardRecord = CardRecord::new(
    "Frontline Rush",
    "2ce8a205-99d6-4a9c-83a7-18b7220177d3",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// TDM 187 — Frostcliff Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROSTCLIFF_SIEGE: CardRecord = CardRecord::new(
    "Frostcliff Siege",
    "a750aabb-9788-494a-841f-bf75717970a7",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// TDM 188 — Glacial Dragonhunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_DRAGONHUNT: CardRecord = CardRecord::new(
    "Glacial Dragonhunt",
    "95994c88-e404-4a4f-8be6-b99d703d4609",
    "Igor Grechanyi",
    crate::card::CardRules::unsupported(),
);

// TDM 189 — Glacierwood Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIERWOOD_SIEGE: CardRecord = CardRecord::new(
    "Glacierwood Siege",
    "0f37fad7-2385-409b-8375-fa5dfbcad833",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TDM 190 — Gurmag Nightwatch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GURMAG_NIGHTWATCH: CardRecord = CardRecord::new(
    "Gurmag Nightwatch",
    "de731430-6bbf-4782-953e-b69c46353959",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// TDM 191 — Hardened Tactician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARDENED_TACTICIAN: CardRecord = CardRecord::new(
    "Hardened Tactician",
    "86b225cb-5c45-4da1-a64e-b04091e483e8",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// TDM 192 — Hollowmurk Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOWMURK_SIEGE: CardRecord = CardRecord::new(
    "Hollowmurk Siege",
    "5ac0e136-8877-4bfc-a831-2bf7b7b5ad1e",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// TDM 193 — Host of the Hereafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOST_OF_THE_HEREAFTER: CardRecord = CardRecord::new(
    "Host of the Hereafter",
    "0f182957-8133-45a7-80a3-1944bead4d43",
    "Annie Stegg",
    crate::card::CardRules::unsupported(),
);

// TDM 194 — Inevitable Defeat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INEVITABLE_DEFEAT: CardRecord = CardRecord::new(
    "Inevitable Defeat",
    "9d677980-b608-407e-9f17-790a81263f15",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// TDM 195 — Jeskai Brushmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKAI_BRUSHMASTER: CardRecord = CardRecord::new(
    "Jeskai Brushmaster",
    "2eb06c36-cf7e-47a9-819e-adfc54284153",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// TDM 196 — Jeskai Revelation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKAI_REVELATION: CardRecord = CardRecord::new(
    "Jeskai Revelation",
    "3cac0ad3-5107-4ed6-a688-d44bbd65e407",
    "Igor Grechanyi",
    crate::card::CardRules::unsupported(),
);

// TDM 197 — Jeskai Shrinekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKAI_SHRINEKEEPER: CardRecord = CardRecord::new(
    "Jeskai Shrinekeeper",
    "6ec8fa0b-c695-4326-aebd-042cb1974925",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// TDM 198 — Karakyk Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARAKYK_GUARDIAN: CardRecord = CardRecord::new(
    "Karakyk Guardian",
    "a4c77b08-c3f6-4458-8636-f226f9843b6d",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// TDM 199 — Kheru Goldkeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KHERU_GOLDKEEPER: CardRecord = CardRecord::new(
    "Kheru Goldkeeper",
    "8d11183a-57f5-4ddb-8a6e-15fff704b114",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// TDM 200 — Kin-Tree Severance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KIN_TREE_SEVERANCE: CardRecord = CardRecord::new(
    "Kin-Tree Severance",
    "b577e246-3377-42aa-856e-b9fa89f3603a",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// TDM 201 — Kishla Skimmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KISHLA_SKIMMER: CardRecord = CardRecord::new(
    "Kishla Skimmer",
    "b5f1acb0-d73e-4814-8158-3645daf5c4cc",
    "James Ryman",
    crate::card::CardRules::unsupported(),
);

// TDM 202 — Kotis, the Fangkeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOTIS_THE_FANGKEEPER: CardRecord = CardRecord::new(
    "Kotis, the Fangkeeper",
    "d3736f17-f80b-4b2c-b919-2c963bc14682",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// TDM 203 — Lie in Wait
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIE_IN_WAIT: CardRecord = CardRecord::new(
    "Lie in Wait",
    "96fff22c-282b-4849-82ce-890013b53262",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// TDM 204 — Lotuslight Dancers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUSLIGHT_DANCERS: CardRecord = CardRecord::new(
    "Lotuslight Dancers",
    "82aa2593-4a79-46f1-a2bd-b71fb504d0ab",
    "Jodie Muir",
    crate::card::CardRules::unsupported(),
);

// TDM 205 — Mammoth Bellow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAMMOTH_BELLOW: CardRecord = CardRecord::new(
    "Mammoth Bellow",
    "468b17b4-79ce-4dfa-8873-a9cfc347e38f",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// TDM 206 — Mardu Siegebreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARDU_SIEGEBREAKER: CardRecord = CardRecord::new(
    "Mardu Siegebreaker",
    "3044b232-edf4-4000-9273-cc4653ad653a",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// TDM 207 — Marshal of the Lost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARSHAL_OF_THE_LOST: CardRecord = CardRecord::new(
    "Marshal of the Lost",
    "64fbaa16-67c3-4ed2-9545-39abbbde61dc",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// TDM 208 — Monastery Messenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONASTERY_MESSENGER: CardRecord = CardRecord::new(
    "Monastery Messenger",
    "0c9eeced-6464-41f0-bbea-05b3af4cc005",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// TDM 209 — Narset, Jeskai Waymaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NARSET_JESKAI_WAYMASTER: CardRecord = CardRecord::new(
    "Narset, Jeskai Waymaster",
    "6b77cbc1-dbc8-44d9-aa29-15cbb19afecd",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// TDM 210 — Neriv, Heart of the Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NERIV_HEART_OF_THE_STORM: CardRecord = CardRecord::new(
    "Neriv, Heart of the Storm",
    "b58112b0-a05c-4b98-b650-fd27ad97789f",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// TDM 211 — New Way Forward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEW_WAY_FORWARD: CardRecord = CardRecord::new(
    "New Way Forward",
    "d9d48f9e-79f0-478c-9db0-ff7ac4a8f401",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// TDM 212 — Perennation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERENNATION: CardRecord = CardRecord::new(
    "Perennation",
    "ffe7071e-a214-44e8-a571-129f0db44f76",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// TDM 213 — Purging Stormbrood // Absorb Essence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURGING_STORMBROOD: CardRecord = CardRecord::new(
    "Purging Stormbrood // Absorb Essence",
    "3988dc76-072c-4f43-849d-2e73c6f6ff58",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// TDM 214 — Rakshasa's Bargain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKSHASA_S_BARGAIN: CardRecord = CardRecord::new(
    "Rakshasa's Bargain",
    "5c409f4f-3b2c-4c33-b850-55b2a46f51ca",
    "Yigit Koroglu",
    crate::card::CardRules::unsupported(),
);

// TDM 215 — Rediscover the Way
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDISCOVER_THE_WAY: CardRecord = CardRecord::new(
    "Rediscover the Way",
    "79d6decf-afd5-4e96-b87e-fd7ab7e3c068",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 216 — Reigning Victor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REIGNING_VICTOR: CardRecord = CardRecord::new(
    "Reigning Victor",
    "a394112a-032b-4047-887a-6522cf7b83d5",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// TDM 217 — Reputable Merchant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPUTABLE_MERCHANT: CardRecord = CardRecord::new(
    "Reputable Merchant",
    "b7d0591e-7fb7-40ea-ba2a-cfe544d40216",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// TDM 218 — Revival of the Ancestors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVAL_OF_THE_ANCESTORS: CardRecord = CardRecord::new(
    "Revival of the Ancestors",
    "fd742ff5-f0ea-4f4b-911e-4c09e2154dba",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 219 — Riverwheel Sweep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVERWHEEL_SWEEP: CardRecord = CardRecord::new(
    "Riverwheel Sweep",
    "686fe623-ee50-407d-87c9-664fb039f4d9",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// TDM 220 — Roar of Endless Song
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROAR_OF_ENDLESS_SONG: CardRecord = CardRecord::new(
    "Roar of Endless Song",
    "7a9c3531-61a8-43f5-82a2-5166e5f5a6b9",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 221 — Runescale Stormbrood // Chilling Screech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNESCALE_STORMBROOD: CardRecord = CardRecord::new(
    "Runescale Stormbrood // Chilling Screech",
    "317744d1-ed78-4b53-a4d8-8c7ecfd9c4ae",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// TDM 222 — Severance Priest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEVERANCE_PRIEST: CardRecord = CardRecord::new(
    "Severance Priest",
    "bc779a1b-128c-4c74-bebd-bdb687867f68",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// TDM 223 — Shiko, Paragon of the Way
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIKO_PARAGON_OF_THE_WAY: CardRecord = CardRecord::new(
    "Shiko, Paragon of the Way",
    "8138cf10-1e3e-483f-86ad-cc399192657d",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// TDM 224 — Skirmish Rhino
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRMISH_RHINO: CardRecord = CardRecord::new(
    "Skirmish Rhino",
    "4a2e9ba1-c254-41e3-9845-4e81f9fec38d",
    "James Bousema",
    crate::card::CardRules::unsupported(),
);

// TDM 225 — Songcrafter Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONGCRAFTER_MAGE: CardRecord = CardRecord::new(
    "Songcrafter Mage",
    "9523bc07-49e5-409c-ae6b-b28e305eef36",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// TDM 226 — Sonic Shrieker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONIC_SHRIEKER: CardRecord = CardRecord::new(
    "Sonic Shrieker",
    "7c231437-8bec-42e0-9175-af74c752b119",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// TDM 227 — Stalwart Successor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STALWART_SUCCESSOR: CardRecord = CardRecord::new(
    "Stalwart Successor",
    "4a7b206f-8190-46e6-bb9e-44763d3eb4ac",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// TDM 228 — Temur Battlecrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMUR_BATTLECRIER: CardRecord = CardRecord::new(
    "Temur Battlecrier",
    "72184791-0767-4108-920c-763e92dae2d4",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// TDM 229 — Temur Tawnyback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMUR_TAWNYBACK: CardRecord = CardRecord::new(
    "Temur Tawnyback",
    "3cdb383f-bc04-46d1-aa3a-7459d57f1fec",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TDM 230 — Teval, Arbiter of Virtue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEVAL_ARBITER_OF_VIRTUE: CardRecord = CardRecord::new(
    "Teval, Arbiter of Virtue",
    "27a93f5b-7b32-49f0-a179-b897828fe49a",
    "Alexander Ostrowski",
    crate::card::CardRules::unsupported(),
);

// TDM 231 — Thunder of Unity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_OF_UNITY: CardRecord = CardRecord::new(
    "Thunder of Unity",
    "5c953b36-f5e4-4258-91cb-f07e799321f7",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 232 — Twinmaw Stormbrood // Charring Bite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWINMAW_STORMBROOD: CardRecord = CardRecord::new(
    "Twinmaw Stormbrood // Charring Bite",
    "2999e3b1-6510-42b2-9429-28c07a64a44f",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// TDM 233 — Ureni, the Song Unending
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URENI_THE_SONG_UNENDING: CardRecord = CardRecord::new(
    "Ureni, the Song Unending",
    "227802c0-4ff6-43a8-a850-ed0f546dc5ac",
    "Alexander Ostrowski",
    crate::card::CardRules::unsupported(),
);

// TDM 234 — Whirlwing Stormbrood // Dynamic Soar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLWING_STORMBROOD: CardRecord = CardRecord::new(
    "Whirlwing Stormbrood // Dynamic Soar",
    "56a25eb1-bdb8-4f86-8d9a-3055ad1b2a13",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// TDM 235 — Windcrag Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDCRAG_SIEGE: CardRecord = CardRecord::new(
    "Windcrag Siege",
    "31a8329b-23a1-4c49-a579-a5da8d01435a",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// TDM 236 — Yathan Roadwatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YATHAN_ROADWATCHER: CardRecord = CardRecord::new(
    "Yathan Roadwatcher",
    "8e77339b-dd82-481c-9ee2-4156ca69ad35",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// TDM 237 — Zurgo, Thunder's Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZURGO_THUNDER_S_DECREE: CardRecord = CardRecord::new(
    "Zurgo, Thunder's Decree",
    "bd93fb95-4268-45dc-8f0d-590c481a526d",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// TDM 238 — Abzan Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABZAN_MONUMENT: CardRecord = CardRecord::new(
    "Abzan Monument",
    "d2da9024-3b58-4a57-8f7d-4094c193daee",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// TDM 239 — Boulderborn Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOULDERBORN_DRAGON: CardRecord = CardRecord::new(
    "Boulderborn Dragon",
    "50c6e815-bfe7-4599-9227-d36504a3640f",
    "Alexander Ostrowski",
    crate::card::CardRules::unsupported(),
);

// TDM 240 — Dragonfire Blade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONFIRE_BLADE: CardRecord = CardRecord::new(
    "Dragonfire Blade",
    "031afea3-fbfb-4663-a8cc-9b7eb7b16020",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// TDM 241 — Dragonstorm Globe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONSTORM_GLOBE: CardRecord = CardRecord::new(
    "Dragonstorm Globe",
    "7f50aa6e-ce6a-4479-9725-202926245f2c",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// TDM 242 — Embermouth Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERMOUTH_SENTINEL: CardRecord = CardRecord::new(
    "Embermouth Sentinel",
    "485f75d5-da5b-4605-885a-561ccd999cc6",
    "Stephanie Cheung",
    crate::card::CardRules::unsupported(),
);

// TDM 243 — Jade-Cast Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JADE_CAST_SENTINEL: CardRecord = CardRecord::new(
    "Jade-Cast Sentinel",
    "516ce5fa-bd00-429b-ba22-b38c7dd9306c",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// TDM 244 — Jeskai Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKAI_MONUMENT: CardRecord = CardRecord::new(
    "Jeskai Monument",
    "d0193ad6-39b7-4558-bd3e-36f809332ea2",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// TDM 245 — Mardu Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARDU_MONUMENT: CardRecord = CardRecord::new(
    "Mardu Monument",
    "9bd0c794-77bc-4d4a-a769-3829e2ce4bdf",
    "Salvatorre Zee Yazzie",
    crate::card::CardRules::unsupported(),
);

// TDM 246 — Mox Jasper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOX_JASPER: CardRecord = CardRecord::new(
    "Mox Jasper",
    "a851d2d3-7e93-4887-bee5-4d6c9aaf9419",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// TDM 247 — Sultai Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULTAI_MONUMENT: CardRecord = CardRecord::new(
    "Sultai Monument",
    "45308e0e-b515-49ac-9960-a24e898dd321",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// TDM 248 — Temur Monument
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMUR_MONUMENT: CardRecord = CardRecord::new(
    "Temur Monument",
    "55e97b40-d898-4da5-8159-cca48eb298eb",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

// TDM 249 — Watcher of the Wayside
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATCHER_OF_THE_WAYSIDE: CardRecord = CardRecord::new(
    "Watcher of the Wayside",
    "2dcafdac-a293-4adc-a540-3b3f469cf6f3",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// TDM 250 — Bloodfell Caves (reprint)
const BLOODFELL_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOODFELL_CAVES,
    "1dde3c68-6f29-4c00-b668-c25ac9e3e13b",
    "Piotr Dura",
);

// TDM 251 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOSSOMING_SANDS,
    "0a9df994-e0f4-4919-af99-4f643eb9199c",
    "Piotr Dura",
);

// TDM 252 — Cori Mountain Monastery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORI_MOUNTAIN_MONASTERY: CardRecord = CardRecord::new(
    "Cori Mountain Monastery",
    "9312821a-2059-4f44-9b20-c9522b827e38",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// TDM 253 — Dalkovan Encampment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DALKOVAN_ENCAMPMENT: CardRecord = CardRecord::new(
    "Dalkovan Encampment",
    "98ad5f0c-8775-4e89-8e92-84a6ade93e35",
    "Marina Ortega Lorente",
    crate::card::CardRules::unsupported(),
);

// TDM 254 — Dismal Backwater (reprint)
const DISMAL_BACKWATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISMAL_BACKWATER,
    "082b52c9-c46e-44d3-b723-546ba528e07b",
    "Alayna Danner",
);

// TDM 255 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "62209251-4118-4843-895b-46afb7284c75",
    "Leon Tukker",
);

// TDM 256 — Frontier Bivouac (reprint)
const FRONTIER_BIVOUAC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::FRONTIER_BIVOUAC,
    "679fff07-4796-4d91-8dd6-4e294383ce88",
    "Andrea Piparo",
);

// TDM 257 — Great Arashin City
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_ARASHIN_CITY: CardRecord = CardRecord::new(
    "Great Arashin City",
    "ecba23b6-9f3a-431e-bc22-f1fb04d27b68",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// TDM 258 — Jungle Hollow (reprint)
const JUNGLE_HOLLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::JUNGLE_HOLLOW,
    "ea13440b-3f7b-4182-9541-27c1fa3121e5",
    "Cristi Balanescu",
);

// TDM 259 — Kishla Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KISHLA_VILLAGE: CardRecord = CardRecord::new(
    "Kishla Village",
    "9f0ff90d-7312-44df-afc5-29c768fa7758",
    "Bruce Brenneise",
    crate::card::CardRules::unsupported(),
);

// TDM 260 — Maelstrom of the Spirit Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAELSTROM_OF_THE_SPIRIT_DRAGON: CardRecord = CardRecord::new(
    "Maelstrom of the Spirit Dragon",
    "c4e90bfb-d9a5-48a9-9ff9-b0f50a813eee",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// TDM 261 — Mistrise Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTRISE_VILLAGE: CardRecord = CardRecord::new(
    "Mistrise Village",
    "d44bccbf-6fab-46e4-8ddb-6577e27ec6e8",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// TDM 262 — Mystic Monastery (reprint)
const MYSTIC_MONASTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::MYSTIC_MONASTERY,
    "c7b8a01c-c400-47c7-8270-78902efe850e",
    "Leon Tukker",
);

// TDM 263 — Nomad Outpost (reprint)
const NOMAD_OUTPOST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::NOMAD_OUTPOST,
    "a68fbeaa-941f-4d53-becd-f93ed22b9a54",
    "Alayna Danner",
);

// TDM 264 — Opulent Palace (reprint)
const OPULENT_PALACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::OPULENT_PALACE,
    "21cb3b3b-0738-4c2e-a3fc-927fd6b9d3fb",
    "Sergey Glushakov",
);

// TDM 265 — Rugged Highlands (reprint)
const RUGGED_HIGHLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::RUGGED_HIGHLANDS,
    "31261eca-28ad-407c-84ef-0c124d0d7451",
    "Carlos Palma Cruchaga",
);

// TDM 266 — Sandsteppe Citadel (reprint)
const SANDSTEPPE_CITADEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SANDSTEPPE_CITADEL,
    "47f47e7f-39ba-4807-8e32-7262a61dfbba",
    "Diego Gisbert",
);

// TDM 267 — Scoured Barrens (reprint)
const SCOURED_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SCOURED_BARRENS,
    "b4b47b80-69ed-44b0-afa0-ca90206dc16d",
    "Brent Hollowell",
);

// TDM 268 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SWIFTWATER_CLIFFS,
    "ca53fb19-b8ca-485b-af1a-5117ae54bfe3",
    "Piotr Dura",
);

// TDM 269 — Thornwood Falls (reprint)
const THORNWOOD_FALLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::THORNWOOD_FALLS,
    "ebb502c2-5fd0-46a9-b77d-010f4a942056",
    "Alexander Ostrowski",
);

// TDM 270 — Tranquil Cove (reprint)
const TRANQUIL_COVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::TRANQUIL_COVE,
    "1c4efa6c-4f29-41cd-a728-bf0e479ace05",
    "Kevin Sidharta",
);

// TDM 271 — Wind-Scarred Crag (reprint)
const WIND_SCARRED_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::WIND_SCARRED_CRAG,
    "4912e4d0-b16a-4aa6-a583-3430d26bd591",
    "Filip Burburan",
);

// TDM 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "0d0f1dd6-9564-4adc-af7d-f83252e8581a",
    "Sergey Glushakov",
);

// TDM 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "4208e66c-8c98-4c48-ab07-8523c0b26ca4",
    "Sergey Glushakov",
);

// TDM 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "ef235170-8276-4ef0-bdfd-ba68d5b218ec",
    "Sergey Glushakov",
);

// TDM 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "fe0865ba-47c0-40bc-b0c6-e1ea5ae08a98",
    "Sergey Glushakov",
);

// TDM 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "48811e13-5774-4da1-95ec-6ea5dc4976ad",
    "Sergey Glushakov",
);

// TDM 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "12cff32a-a365-43ee-a196-8ce32b3bb9fd",
    "John Avon",
);

// TDM 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "b8c391f2-b340-43c7-89e6-afac5b70491f",
    "Valera Lutfullina",
);

// TDM 279 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "1ff6acc9-581c-468f-894d-41f725da7f33",
    "Sam Burley",
);

// TDM 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "15be7923-6efc-4650-b8d1-f61cb33ef81d",
    "Constantin Marin",
);

// TDM 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "f0bfdb9e-318f-4acd-9fbd-41b98a8875d6",
    "Alexander Ostrowski",
);

// TDM 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "ac885eb7-9dae-4c48-b45c-97ef9c62c99e",
    "Arthur Yuan",
);

// TDM 283 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "bfa10a88-12e0-4b79-80bb-6f4620277e20",
    "Ralph Horsley",
);

// TDM 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "3df7c206-97b6-49d7-ba01-7a35fd8c61d9",
    "Josu Solano",
);

// TDM 285 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "8100bceb-ffba-487a-bb45-4fe2a156a8dc",
    "Jesper Ejsing",
);

// TDM 286 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "8e3e83d2-96ba-4d5c-a1ed-6c08a90b339c",
    "Valera Lutfullina",
);

// TDM 287 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "3e8c67e5-587a-43b2-af47-bbad1f8b52e9",
    "Ron Spencer",
);

// TDM 288 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "b300be80-6618-4284-b5c3-95c1ab373e6f",
    "Ron Spencer",
);

// TDM 289 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "57da24a0-89a7-4756-b4ca-4dea132e8f67",
    "Ron Spencer",
);

// TDM 290 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "a4db1b7a-93f2-40a5-b649-80a099ddeb62",
    "Ron Spencer",
);

// TDM 291 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "7e33e540-2828-46ad-a441-366552843d9c",
    "Ron Spencer",
);

// TDM 292 — Riling Dawnbreaker // Signaling Roar (alternate printing)
const RILING_DAWNBREAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RILING_DAWNBREAKER,
    1,
    "48b73810-3abd-4469-a6f0-993b6fedc315",
    "Jarel Threat",
);

// TDM 293 — Teeming Dragonstorm (alternate printing)
const TEEMING_DRAGONSTORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEEMING_DRAGONSTORM,
    1,
    "f717301c-1ae8-44b4-b6e5-d3bdf052f5da",
    "Dibujante Nocturno",
);

// TDM 294 — Dirgur Island Dragon // Skimming Strike (alternate printing)
const DIRGUR_ISLAND_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIRGUR_ISLAND_DRAGON,
    1,
    "f7def6d6-c80a-4597-8a3f-3855423bc960",
    "Dibujante Nocturno",
);

// TDM 295 — Dragonologist (alternate printing)
const DRAGONOLOGIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAGONOLOGIST,
    1,
    "97e8d7d3-7e80-4742-9951-eb6679a0aa66",
    "Jarel Threat",
);

// TDM 296 — Roiling Dragonstorm (alternate printing)
const ROILING_DRAGONSTORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROILING_DRAGONSTORM,
    1,
    "3c725add-1cca-4003-8f37-c68f8f8fcc33",
    "Dan Mumford",
);

// TDM 297 — Corroding Dragonstorm (alternate printing)
const CORRODING_DRAGONSTORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CORRODING_DRAGONSTORM,
    1,
    "a8f19964-b37b-4877-bdd5-c5c3022439ef",
    "Cabrol",
);

// TDM 298 — Feral Deathgorger // Dusk Sight (alternate printing)
const FERAL_DEATHGORGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FERAL_DEATHGORGER,
    1,
    "58c446e6-8ffb-45ed-aca2-95161ac88d5c",
    "DZO",
);

// TDM 299 — Breaching Dragonstorm (alternate printing)
const BREACHING_DRAGONSTORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BREACHING_DRAGONSTORM,
    1,
    "6bbfdf32-8b21-4f7a-aa30-42d5362ee352",
    "Justine Jones",
);

// TDM 300 — Dracogenesis (alternate printing)
const DRACOGENESIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRACOGENESIS,
    1,
    "30f7c75a-c8f7-4f34-bc86-9d0441dc3a40",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// TDM 301 — Magmatic Hellkite (alternate printing)
const MAGMATIC_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGMATIC_HELLKITE,
    1,
    "b8a8906f-8593-4ae8-ba65-f2a1cc4c8fa6",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// TDM 302 — Sarkhan, Dragon Ascendant (alternate printing)
const SARKHAN_DRAGON_ASCENDANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SARKHAN_DRAGON_ASCENDANT,
    1,
    "57c03255-e3dc-44c2-982b-7efa188280df",
    "DZO",
);

// TDM 303 — Stormscale Scion (alternate printing)
const STORMSCALE_SCION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORMSCALE_SCION,
    1,
    "c250cbd2-2b78-4721-9977-02de20c3d7d1",
    "Massiveface",
);

// TDM 304 — Stormshriek Feral // Flush Out (alternate printing)
const STORMSHRIEK_FERAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORMSHRIEK_FERAL,
    1,
    "322e4880-f3f0-44d8-8f95-48b496af0e81",
    "Justine Jones",
);

// TDM 305 — Encroaching Dragonstorm (alternate printing)
const ENCROACHING_DRAGONSTORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENCROACHING_DRAGONSTORM,
    1,
    "8d1e6797-f938-4161-a231-dac2da23b573",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// TDM 306 — Sagu Wildling // Roost Seek (alternate printing)
const SAGU_WILDLING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAGU_WILDLING,
    1,
    "b72ee8f9-5e79-4f77-ae7e-e4c274f78187",
    "Justine Jones",
);

// TDM 307 — Armament Dragon (alternate printing)
const ARMAMENT_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARMAMENT_DRAGON,
    1,
    "77a49553-fc4a-427d-9818-dc8b33fe6127",
    "Jarel Threat",
);

// TDM 308 — Betor, Kin to All (alternate printing)
const BETOR_KIN_TO_ALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BETOR_KIN_TO_ALL,
    1,
    "f1969dec-4d6b-493a-8233-76faf8fa3cea",
    "Dan Mumford",
);

// TDM 309 — Call the Spirit Dragons (alternate printing)
const CALL_THE_SPIRIT_DRAGONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CALL_THE_SPIRIT_DRAGONS,
    1,
    "9473ac65-acb4-454b-84ce-2a505387cc24",
    "Dibujante Nocturno",
);

// TDM 310 — Disruptive Stormbrood // Petty Revenge (alternate printing)
const DISRUPTIVE_STORMBROOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISRUPTIVE_STORMBROOD,
    1,
    "24ca444d-f4ab-4375-a670-63f29eb863dd",
    "Dan Mumford",
);

// TDM 311 — Jeskai Shrinekeeper (alternate printing)
const JESKAI_SHRINEKEEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JESKAI_SHRINEKEEPER,
    1,
    "171ba15b-f981-4b0f-8062-24e4c78fc213",
    "Justine Jones",
);

// TDM 312 — Karakyk Guardian (alternate printing)
const KARAKYK_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KARAKYK_GUARDIAN,
    1,
    "8b8a7d6a-c429-4b66-b5b0-953335c5108e",
    "Dibujante Nocturno",
);

// TDM 313 — Kheru Goldkeeper (alternate printing)
const KHERU_GOLDKEEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KHERU_GOLDKEEPER,
    1,
    "9d85ba44-8f29-4c49-b77f-8a6692d23c8c",
    "Jarel Threat",
);

// TDM 314 — Neriv, Heart of the Storm (alternate printing)
const NERIV_HEART_OF_THE_STORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NERIV_HEART_OF_THE_STORM,
    1,
    "9dc53504-0eab-4ed2-b498-d8a5267bd40f",
    "Michael Walsh",
);

// TDM 315 — Purging Stormbrood // Absorb Essence (alternate printing)
const PURGING_STORMBROOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PURGING_STORMBROOD,
    1,
    "fb293f4f-9ba2-48f5-a4fb-d902aa531bfc",
    "WolfSkullJack",
);

// TDM 316 — Runescale Stormbrood // Chilling Screech (alternate printing)
const RUNESCALE_STORMBROOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUNESCALE_STORMBROOD,
    1,
    "72e8f916-5a01-4918-bcb5-7fd69fe32785",
    "Massiveface",
);

// TDM 317 — Shiko, Paragon of the Way (alternate printing)
const SHIKO_PARAGON_OF_THE_WAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHIKO_PARAGON_OF_THE_WAY,
    1,
    "e465b6a8-3b8a-47c6-b3d0-119552556d35",
    "Dibujante Nocturno",
);

// TDM 318 — Sonic Shrieker (alternate printing)
const SONIC_SHRIEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SONIC_SHRIEKER,
    1,
    "46a8ee3f-cee5-4971-9112-393f639a210e",
    "Jarel Threat",
);

// TDM 319 — Teval, Arbiter of Virtue (alternate printing)
const TEVAL_ARBITER_OF_VIRTUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEVAL_ARBITER_OF_VIRTUE,
    1,
    "4d3e165a-60e2-4e50-a0de-1cf7c46cb406",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// TDM 320 — Twinmaw Stormbrood // Charring Bite (alternate printing)
const TWINMAW_STORMBROOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINMAW_STORMBROOD,
    1,
    "ef466256-7d9c-46d0-a860-a0db6930db61",
    "Michael Walsh",
);

// TDM 321 — Ureni, the Song Unending (alternate printing)
const URENI_THE_SONG_UNENDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &URENI_THE_SONG_UNENDING,
    1,
    "a683c32f-325a-42f5-826f-5cc978b8333c",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// TDM 322 — Whirlwing Stormbrood // Dynamic Soar (alternate printing)
const WHIRLWING_STORMBROOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHIRLWING_STORMBROOD,
    1,
    "a621ea7f-f6d5-4663-897a-bbdc2556d665",
    "Dan Mumford",
);

// TDM 323 — Boulderborn Dragon (alternate printing)
const BOULDERBORN_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOULDERBORN_DRAGON,
    1,
    "970e11f0-337a-46b5-9bff-4bcb7843ed3a",
    "Cabrol",
);

// TDM 324 — Dragonfire Blade (alternate printing)
const DRAGONFIRE_BLADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAGONFIRE_BLADE,
    1,
    "45f039ce-cbfd-46d7-a575-5e6c049f83ff",
    "WolfSkullJack",
);

// TDM 325 — Mox Jasper (alternate printing)
const MOX_JASPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOX_JASPER,
    1,
    "2c0372d8-362d-486e-96c0-5738427a1087",
    "DZO",
);

// TDM 326 — Maelstrom of the Spirit Dragon (alternate printing)
const MAELSTROM_OF_THE_SPIRIT_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAELSTROM_OF_THE_SPIRIT_DRAGON,
    1,
    "48b89e6d-da58-465e-a9b1-69629da159f6",
    "Cabrol",
);

// TDM 327 — Anafenza, Unyielding Lineage (alternate printing)
const ANAFENZA_UNYIELDING_LINEAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANAFENZA_UNYIELDING_LINEAGE,
    1,
    "c0f31696-2307-4ec6-a568-c255a25b59b6",
    "Martina Fačková",
);

// TDM 328 — Sage of the Skies (alternate printing)
const SAGE_OF_THE_SKIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAGE_OF_THE_SKIES,
    1,
    "53e7ddf5-5aaf-4233-834d-c9992a9c2b0e",
    "Wayne Wu",
);

// TDM 329 — Smile at Death (alternate printing)
const SMILE_AT_DEATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMILE_AT_DEATH,
    1,
    "5121b5a4-5f91-4bd3-a8b1-c2dc2a449378",
    "Billy Christian",
);

// TDM 330 — United Battlefront (alternate printing)
const UNITED_BATTLEFRONT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNITED_BATTLEFRONT,
    1,
    "3c6d8e5c-4b3d-4c5c-89c5-a2746cd4b578",
    "Aaron J. Riley",
);

// TDM 331 — Voice of Victory (alternate printing)
const VOICE_OF_VICTORY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOICE_OF_VICTORY,
    1,
    "73f24785-c7b3-46ab-833e-666af3d86c63",
    "Kevin Glint",
);

// TDM 332 — Ambling Stormshell (alternate printing)
const AMBLING_STORMSHELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMBLING_STORMSHELL,
    1,
    "7c67235e-13d3-40ba-9cb7-03c1db6d455e",
    "Tomas Duchek",
);

// TDM 333 — Naga Fleshcrafter (alternate printing)
const NAGA_FLESHCRAFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NAGA_FLESHCRAFTER,
    1,
    "95b4863c-51bc-445d-97a5-289b5a87c871",
    "Tuan Duong Chu",
);

// TDM 334 — Stillness in Motion (alternate printing)
const STILLNESS_IN_MOTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STILLNESS_IN_MOTION,
    1,
    "70ae35fd-5fb6-440a-9e82-13998b928ee3",
    "Kevin Glint",
);

// TDM 335 — Taigam, Master Opportunist (alternate printing)
const TAIGAM_MASTER_OPPORTUNIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAIGAM_MASTER_OPPORTUNIST,
    1,
    "0ea8c278-781d-4c9c-9f1c-99d799384a29",
    "Jeremy Chong",
);

// TDM 336 — Winternight Stories (alternate printing)
const WINTERNIGHT_STORIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINTERNIGHT_STORIES,
    1,
    "c94538c3-320c-4903-a689-bb8e9f4ae40f",
    "Tomas Duchek",
);

// TDM 337 — Avenger of the Fallen (alternate printing)
const AVENGER_OF_THE_FALLEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVENGER_OF_THE_FALLEN,
    1,
    "bae9ca3b-cc32-410f-82e9-85cb9c4fa447",
    "Tomas Duchek",
);

// TDM 338 — Qarsi Revenant (alternate printing)
const QARSI_REVENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QARSI_REVENANT,
    1,
    "4ca69694-c345-4255-9c92-0110aa5c8004",
    "Kevin Glint",
);

// TDM 339 — Rot-Curse Rakshasa (alternate printing)
const ROT_CURSE_RAKSHASA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROT_CURSE_RAKSHASA,
    1,
    "cd34da53-1a96-4f06-aaf6-e70581de112d",
    "Tomas Duchek",
);

// TDM 340 — The Sibsig Ceremony (alternate printing)
const THE_SIBSIG_CEREMONY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SIBSIG_CEREMONY,
    1,
    "6daa156c-478f-47dd-9284-b95e82ccfd68",
    "Tomas Duchek",
);

// TDM 341 — Sidisi, Regent of the Mire (alternate printing)
const SIDISI_REGENT_OF_THE_MIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIDISI_REGENT_OF_THE_MIRE,
    1,
    "5f9e3ddc-a4e7-4304-bfd8-890c9c71f53d",
    "Tomas Duchek",
);

// TDM 342 — Sinkhole Surveyor (alternate printing)
const SINKHOLE_SURVEYOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SINKHOLE_SURVEYOR,
    1,
    "74b155cd-c3a0-4f27-8d3c-7778354abbd4",
    "Kevin Glint",
);

// TDM 343 — Cori-Steel Cutter
pub(in crate::card::sets) static CORI_STEEL_CUTTER: CardRecord = CardRecord::new(
    "Cori-Steel Cutter",
    "470dd3c8-07c9-42ef-aa9e-3c73b23607ff",
    "Tomas Duchek",
    // Two mana that turns every second spell into a hasty attacker, and
    // moves itself onto the new one for free every time.
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has trample and haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            AbilityDef::triggered_if(
                "Flurry — Whenever you cast your second spell each turn, create a 1/1 white Monk \
                 creature token with prowess. You may attach this Equipment to it.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read.
                &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Monk"], &[ManaColor::White], 1, 1)
                            .with_abilities(&[abilities::prowess()])
                            .with_art(CardArt::new(
                                "633d2d10-def7-426f-8496-ed6b45684299",
                                "Elizabeth Peiró",
                            )),
                    ))
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        // "You may attach this Equipment to it": the Monk is named rather than
                        // targeted, so the token the trigger just made is the one it moves onto --
                        // and declining leaves the Equipment where it was.
                        then: &EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Attach {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                            },
                        },
                    }),
                ),
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}{R}"))], "Equip {1}{R}"),
        ]),
);

static ELSPETH_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// TDM 344 — Stadium Headliner (alternate printing)
const STADIUM_HEADLINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STADIUM_HEADLINER,
    1,
    "ef7a1ddf-bf52-4d44-92cd-8d8127472bd9",
    "Tomas Duchek",
);

// TDM 345 — Tersa Lightshatter (alternate printing)
const TERSA_LIGHTSHATTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERSA_LIGHTSHATTER,
    1,
    "7a1689bb-f7a4-4b53-8473-75b7ce7b496d",
    "Tomas Duchek",
);

// TDM 346 — Craterhoof Behemoth (alternate printing)
const CRATERHOOF_BEHEMOTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CRATERHOOF_BEHEMOTH,
    1,
    "c966c8a0-e73d-4484-9307-a793a65222ea",
    "Tuan Duong Chu",
);

// TDM 347 — Herd Heirloom (alternate printing)
const HERD_HEIRLOOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HERD_HEIRLOOM,
    1,
    "c5e7d936-60f8-40fc-b6a9-be677a97395b",
    "Clint Lockwood",
);

// TDM 348 — Lasyd Prowler (alternate printing)
const LASYD_PROWLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LASYD_PROWLER,
    1,
    "2683ba05-13aa-44ca-8465-d9fa19ae610d",
    "Wonchun Choi",
);

// TDM 349 — Nature's Rhythm (alternate printing)
const NATURE_S_RHYTHM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NATURE_S_RHYTHM,
    1,
    "ce0ee309-b6c4-455d-8af6-48d8ac1426cb",
    "Valera Lutfullina",
);

// TDM 350 — Surrak, Elusive Hunter (alternate printing)
const SURRAK_ELUSIVE_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SURRAK_ELUSIVE_HUNTER,
    1,
    "dc80d937-a166-42e7-a7b3-56150e11d27e",
    "Jose Cabrera",
);

// TDM 351 — Warden of the Grove (alternate printing)
const WARDEN_OF_THE_GROVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARDEN_OF_THE_GROVE,
    1,
    "2e3d7969-5dcb-434d-8a8b-fb16da288bc4",
    "Kevin Glint",
);

// TDM 352 — All-Out Assault (alternate printing)
const ALL_OUT_ASSAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALL_OUT_ASSAULT,
    1,
    "e0febe8c-7e00-485a-bd06-1d7c4d4e816e",
    "Valera Lutfullina",
);

// TDM 353 — Betor, Kin to All (alternate printing)
const BETOR_KIN_TO_ALL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BETOR_KIN_TO_ALL,
    2,
    "7c8cf348-7db8-4f93-8f83-8b1f2035ed4e",
    "Anna Podedworna",
);

// TDM 354 — Death Begets Life (alternate printing)
const DEATH_BEGETS_LIFE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEATH_BEGETS_LIFE,
    1,
    "e8e94b23-955b-45c0-9cef-713a0a6c38ac",
    "Joshua Raphael",
);

// TDM 355 — Dragonback Assault (alternate printing)
const DRAGONBACK_ASSAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAGONBACK_ASSAULT,
    1,
    "87970548-bbec-4f07-b534-e463c9128469",
    "Anastasia Ovchinnikova",
);

// TDM 356 — Eshki Dragonclaw (alternate printing)
const ESHKI_DRAGONCLAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ESHKI_DRAGONCLAW,
    1,
    "aafaa59e-87e1-4953-8c04-8e7a3a509827",
    "Valera Lutfullina",
);

// TDM 357 — Fangkeeper's Familiar (alternate printing)
const FANGKEEPER_S_FAMILIAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FANGKEEPER_S_FAMILIAR,
    1,
    "8178e63d-caa6-4088-aaef-367fb24638a4",
    "Kevin Glint",
);

// TDM 358 — Felothar, Dawn of the Abzan (alternate printing)
const FELOTHAR_DAWN_OF_THE_ABZAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FELOTHAR_DAWN_OF_THE_ABZAN,
    1,
    "9c4f9d0f-11fa-4986-a7e8-64a922681906",
    "Wayne Wu",
);

// TDM 359 — Flamehold Grappler (alternate printing)
const FLAMEHOLD_GRAPPLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLAMEHOLD_GRAPPLER,
    1,
    "60667979-40c1-4144-a3f8-0115fb77341d",
    "Wayne Wu",
);

// TDM 360 — Inevitable Defeat (alternate printing)
const INEVITABLE_DEFEAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INEVITABLE_DEFEAT,
    1,
    "e4f049ce-6bc7-437d-a530-8c4278151569",
    "Tomas Duchek",
);

// TDM 361 — Jeskai Revelation (alternate printing)
const JESKAI_REVELATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JESKAI_REVELATION,
    1,
    "7679a6a2-7704-4f37-9fdd-24414d411599",
    "Francis Tneh",
);

// TDM 362 — Kotis, the Fangkeeper (alternate printing)
const KOTIS_THE_FANGKEEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOTIS_THE_FANGKEEPER,
    1,
    "f70098f2-e5a8-4056-b5b3-1229fc290c51",
    "Tomas Duchek",
);

// TDM 363 — Lotuslight Dancers (alternate printing)
const LOTUSLIGHT_DANCERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOTUSLIGHT_DANCERS,
    1,
    "79dc69dc-6245-43fc-95a2-85b2c2957182",
    "Fajareka Setiawan",
);

// TDM 364 — Mardu Siegebreaker (alternate printing)
const MARDU_SIEGEBREAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARDU_SIEGEBREAKER,
    1,
    "4fa10a98-1d1f-4e66-b81d-615ffaf43ca1",
    "Joshua Cairos",
);

// TDM 365 — Narset, Jeskai Waymaster (alternate printing)
const NARSET_JESKAI_WAYMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NARSET_JESKAI_WAYMASTER,
    1,
    "cccfb58a-4844-466c-81ea-5fb73863bccf",
    "Kevin Glint",
);

// TDM 366 — Neriv, Heart of the Storm (alternate printing)
const NERIV_HEART_OF_THE_STORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NERIV_HEART_OF_THE_STORM,
    2,
    "f72f191b-81d5-4db4-ac42-c5482f15385d",
    "Kevin Glint",
);

// TDM 367 — New Way Forward (alternate printing)
const NEW_WAY_FORWARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NEW_WAY_FORWARD,
    1,
    "6ebc8ee7-3a1c-49f7-aa67-ff68c377e38c",
    "Wayne Wu",
);

// TDM 368 — Perennation (alternate printing)
const PERENNATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERENNATION,
    1,
    "5596f0c7-8007-4136-bab0-58a9cd852a6e",
    "Martina Fačková",
);

// TDM 369 — Severance Priest (alternate printing)
const SEVERANCE_PRIEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEVERANCE_PRIEST,
    1,
    "585ccfa2-24e3-47aa-b244-31e29b216058",
    "Tomas Duchek",
);

// TDM 370 — Shiko, Paragon of the Way (alternate printing)
const SHIKO_PARAGON_OF_THE_WAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SHIKO_PARAGON_OF_THE_WAY,
    2,
    "47fd0437-bfb4-4a9e-9109-d172bfb3faab",
    "Anastasia Ovchinnikova",
);

// TDM 371 — Songcrafter Mage (alternate printing)
const SONGCRAFTER_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SONGCRAFTER_MAGE,
    1,
    "584eb844-91e2-47fb-b4e0-f5def65b824a",
    "Jeremy Chong",
);

// TDM 372 — Temur Battlecrier (alternate printing)
const TEMUR_BATTLECRIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEMUR_BATTLECRIER,
    1,
    "8141492f-f971-4b7f-afdd-e37537f4d3f5",
    "Christina Kraus",
);

// TDM 373 — Teval, Arbiter of Virtue (alternate printing)
const TEVAL_ARBITER_OF_VIRTUE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TEVAL_ARBITER_OF_VIRTUE,
    2,
    "a19c38bc-946c-438a-ac8b-f59ff0b4c613",
    "Andrey Kuzinskiy",
);

// TDM 374 — Ureni, the Song Unending (alternate printing)
const URENI_THE_SONG_UNENDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &URENI_THE_SONG_UNENDING,
    2,
    "e8ea13cf-2fa3-411f-9dc5-13d75f0c67dd",
    "Andrey Kuzinskiy",
);

// TDM 375 — Yathan Roadwatcher (alternate printing)
const YATHAN_ROADWATCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YATHAN_ROADWATCHER,
    1,
    "715afdde-ef3b-40c0-8b1d-59c59381a54e",
    "Kevin Glint",
);

// TDM 376 — Zurgo, Thunder's Decree (alternate printing)
const ZURGO_THUNDER_S_DECREE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZURGO_THUNDER_S_DECREE,
    1,
    "d899dde2-68e6-4807-b0e9-3f4e28824822",
    "Chuck Lukacs",
);

// TDM 377 — Clarion Conqueror // Clarion Conqueror (alternate printing)
const CLARION_CONQUEROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLARION_CONQUEROR,
    1,
    "b3aecdfc-9d37-4f1f-9123-fc07b669d747",
    "Crystal Sully",
);

// TDM 378 — Marang River Regent // Coil and Catch // Marang River Regent (alternate printing)
const MARANG_RIVER_REGENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARANG_RIVER_REGENT,
    1,
    "484b5580-b179-4dce-8bdf-d714eb4635e5",
    "Brian Valeza",
);

// TDM 379 — Scavenger Regent // Exude Toxin // Scavenger Regent (alternate printing)
const SCAVENGER_REGENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCAVENGER_REGENT,
    1,
    "9cf54062-7b5b-4e46-ae1e-fab7e419a9fa",
    "Allen Douglas",
);

// TDM 380 — Magmatic Hellkite // Magmatic Hellkite (alternate printing)
const MAGMATIC_HELLKITE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MAGMATIC_HELLKITE,
    2,
    "4981dc79-4efd-40e1-9fc1-c08e284aff22",
    "Alison Johnstun",
);

// TDM 381 — Bloomvine Regent // Claim Territory // Bloomvine Regent (alternate printing)
const BLOOMVINE_REGENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOOMVINE_REGENT,
    1,
    "081f2de5-251a-41c9-a62f-11487f54d355",
    "Brooklyn Smith",
);

// TDM 382 — Ugin, Eye of the Storms // Ugin, Eye of the Storms (alternate printing)
const UGIN_EYE_OF_THE_STORMS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UGIN_EYE_OF_THE_STORMS,
    2,
    "53b11c30-1c4e-4238-9d42-2e1480df60c1",
    "Antonio José Manzanedo",
);

// TDM 383 — Awaken the Honored Dead (alternate printing)
const AWAKEN_THE_HONORED_DEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AWAKEN_THE_HONORED_DEAD,
    1,
    "76bd76c7-7a1e-4119-8f4a-12b536b30a32",
    "Flavio Greco Paglia",
);

// TDM 384 — Barrensteppe Siege (alternate printing)
const BARRENSTEPPE_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARRENSTEPPE_SIEGE,
    1,
    "c09d4015-f101-4529-a603-c66192dcfd92",
    "Clint Lockwood",
);

// TDM 385 — Frostcliff Siege (alternate printing)
const FROSTCLIFF_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FROSTCLIFF_SIEGE,
    1,
    "b32ab782-5f99-489c-895a-49c5c5ea249d",
    "Francesca Baerald",
);

// TDM 386 — Glacierwood Siege (alternate printing)
const GLACIERWOOD_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLACIERWOOD_SIEGE,
    1,
    "6626cc5e-3a9f-4832-a88a-abf6466e2bae",
    "Clint Lockwood",
);

// TDM 387 — Hollowmurk Siege (alternate printing)
const HOLLOWMURK_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOLLOWMURK_SIEGE,
    1,
    "bd9a6427-09cc-4ddf-88a6-fc23498a7c08",
    "Raymond Bonilla",
);

// TDM 388 — Rediscover the Way (alternate printing)
const REDISCOVER_THE_WAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDISCOVER_THE_WAY,
    1,
    "9f0797b4-7e06-4f64-95f3-3a7d694d601a",
    "Justyna Dura",
);

// TDM 389 — Revival of the Ancestors (alternate printing)
const REVIVAL_OF_THE_ANCESTORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REVIVAL_OF_THE_ANCESTORS,
    1,
    "6ae833e4-b1b8-44cf-a831-d10b78328b81",
    "Miklós Ligeti",
);

// TDM 390 — Roar of Endless Song (alternate printing)
const ROAR_OF_ENDLESS_SONG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROAR_OF_ENDLESS_SONG,
    1,
    "b2624c7f-1c10-49c7-be74-e7b2dc8dac12",
    "Miklós Ligeti",
);

// TDM 391 — Thunder of Unity (alternate printing)
const THUNDER_OF_UNITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDER_OF_UNITY,
    1,
    "5fd218be-b4c1-4dc7-9672-a16892f1b1e7",
    "Lie Setiawan",
);

// TDM 392 — Windcrag Siege (alternate printing)
const WINDCRAG_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINDCRAG_SIEGE,
    1,
    "b32111e6-c389-4dcd-9dcd-29ee7ee238e6",
    "Francesca Baerald",
);

// TDM 393 — Cori Mountain Monastery (alternate printing)
const CORI_MOUNTAIN_MONASTERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CORI_MOUNTAIN_MONASTERY,
    1,
    "85b84c4c-465d-4d8d-8d38-2ed08a9213b3",
    "Constantin Marin",
);

// TDM 394 — Dalkovan Encampment (alternate printing)
const DALKOVAN_ENCAMPMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DALKOVAN_ENCAMPMENT,
    1,
    "5af006f6-135e-4ea0-8ce4-7824934e87da",
    "Raymond Bonilla",
);

// TDM 395 — Great Arashin City (alternate printing)
const GREAT_ARASHIN_CITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_ARASHIN_CITY,
    1,
    "d98fdfc0-5dd2-4059-8fd6-73378235de55",
    "Constantin Marin",
);

// TDM 396 — Kishla Village (alternate printing)
const KISHLA_VILLAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KISHLA_VILLAGE,
    1,
    "687459d1-f487-4ef6-9532-d68425d71210",
    "Marina Ortega Lorente",
);

// TDM 397 — Mistrise Village (alternate printing)
const MISTRISE_VILLAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MISTRISE_VILLAGE,
    1,
    "8c4f775c-98dd-4506-a02c-d22024f31d67",
    "Marco Gorlei",
);

// TDM 398 — Elspeth, Storm Slayer
pub(in crate::card::sets) static ELSPETH_STORM_SLAYER: CardRecord = CardRecord::new(
    "Elspeth, Storm Slayer",
    "1fdf9438-fd5f-4638-8f41-dae35ae8f257",
    "Jeremy Wilson",
// Five mana whose first line is worth more than the three below it: in a
    // deck that makes tokens at all, everything it was already doing happens
    // twice.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{W}"), &["Elspeth"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // The doubling is what every other line on the card is written against:
            // her plus makes two Soldiers, and so does anything else you were
            // already doing.
            AbilityDef::static_ability(
                "If one or more tokens would be created under your control, twice that many of those \
                 tokens are created instead.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoublesTokensCreated),
                },
            ),
            AbilityDef::activated(
                "+1: Create a 1/1 white Soldier creature token.",
                &[CostDef::Loyalty(1)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
                ))),
            ),
            AbilityDef::activated(
                "0: Put a +1/+1 counter on each creature you control. Those creatures gain flying until \
                 your next turn.",
                &[CostDef::Loyalty(0)],
                // "Those creatures" is the set the counters went on. Nothing can join or
                // leave the battlefield between the two halves of one resolution, so
                // naming the same query twice names the same creatures -- and unlike a
                // binding it says outright that they are on the battlefield.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "−3: Destroy target creature an opponent controls with mana value 3 or greater.",
                &[CostDef::Loyalty(-3)],
                // "Mana value 3 or greater", which for a whole number is everything that is
                // not two or less.
                &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::Opponent),
                            owner: None,
                        },
                    )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
        ]),
);

// TDM 399 — Ugin, Eye of the Storms (alternate printing)
const UGIN_EYE_OF_THE_STORMS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &UGIN_EYE_OF_THE_STORMS,
    3,
    "43085bc6-4d16-4a78-af31-b10cea602fc8",
    "Joshua Raphael",
);

// TDM 400 — Clarion Conqueror (alternate printing)
const CLARION_CONQUEROR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CLARION_CONQUEROR,
    2,
    "5aa73d25-c887-487a-ba77-0d4ca992f106",
    "Nathaniel Himawan",
);

// TDM 401 — Elspeth, Storm Slayer (alternate printing)
const ELSPETH_STORM_SLAYER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELSPETH_STORM_SLAYER,
    2,
    "f421da3b-b88d-4e9f-865b-61120bff917a",
    "Ekaterina Burmak",
);

// TDM 402 — Dracogenesis (alternate printing)
const DRACOGENESIS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DRACOGENESIS,
    2,
    "737d2ab6-bb45-432c-9ce2-e9ecb513ee4d",
    "Kai Carpenter",
);

// TDM 403 — Sarkhan, Dragon Ascendant (alternate printing)
const SARKHAN_DRAGON_ASCENDANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SARKHAN_DRAGON_ASCENDANT,
    2,
    "e3be50dc-3735-47a6-9af3-e8d8e425b5b2",
    "Billy Christian",
);

// TDM 404 — Craterhoof Behemoth (alternate printing)
const CRATERHOOF_BEHEMOTH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CRATERHOOF_BEHEMOTH,
    2,
    "0e172790-7ab4-4dea-9439-e3cedd3e5cab",
    "Magali Villeneuve",
);

// TDM 405 — All-Out Assault (alternate printing)
const ALL_OUT_ASSAULT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ALL_OUT_ASSAULT,
    2,
    "42c0f348-2435-4c62-9bf7-c1efded1fca0",
    "Joshua Cairos",
);

// TDM 406 — Death Begets Life (alternate printing)
const DEATH_BEGETS_LIFE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEATH_BEGETS_LIFE,
    2,
    "08cb3168-6872-43b6-9980-35ddc20cf192",
    "Justin Hernandez & Alexis Hernandez",
);

// TDM 407 — Narset, Jeskai Waymaster (alternate printing)
const NARSET_JESKAI_WAYMASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NARSET_JESKAI_WAYMASTER,
    2,
    "40a1c532-e0f6-456a-a6d9-5f7bf1a6b47c",
    "Randy Vargas",
);

// TDM 408 — Skirmish Rhino (alternate printing)
const SKIRMISH_RHINO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKIRMISH_RHINO,
    1,
    "0eaca731-0886-4617-b012-451a5ba768db",
    "James Bousema",
);

// TDM 409 — Ugin, Eye of the Storms (alternate printing)
const UGIN_EYE_OF_THE_STORMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UGIN_EYE_OF_THE_STORMS,
    1,
    "2e7cb37b-3ab5-42d0-860a-0c0760924850",
    "Joshua Raphael",
);

// TDM 410 — Clarion Conqueror (alternate printing)
const CLARION_CONQUEROR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CLARION_CONQUEROR,
    3,
    "d5eada03-eaca-4091-8fa0-f8e996a402ad",
    "Nathaniel Himawan",
);

// TDM 411 — Elspeth, Storm Slayer (alternate printing)
const ELSPETH_STORM_SLAYER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ELSPETH_STORM_SLAYER,
    3,
    "89b98fd0-e2e5-4533-af2b-5230af2c88bd",
    "Ekaterina Burmak",
);

// TDM 412 — Dracogenesis (alternate printing)
const DRACOGENESIS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DRACOGENESIS,
    3,
    "38b6d099-e31f-45b5-b78a-72a4b38d60f0",
    "Kai Carpenter",
);

// TDM 413 — Sarkhan, Dragon Ascendant (alternate printing)
const SARKHAN_DRAGON_ASCENDANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SARKHAN_DRAGON_ASCENDANT,
    3,
    "a267ced0-34af-483c-ba42-517f3f7e22dc",
    "Billy Christian",
);

// TDM 414 — Craterhoof Behemoth (alternate printing)
const CRATERHOOF_BEHEMOTH_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CRATERHOOF_BEHEMOTH,
    3,
    "c13f37b1-48ff-45b5-8625-d089073ca90b",
    "Magali Villeneuve",
);

// TDM 415 — All-Out Assault (alternate printing)
const ALL_OUT_ASSAULT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ALL_OUT_ASSAULT,
    3,
    "37821af8-a873-497a-82cc-51095f1eed37",
    "Joshua Cairos",
);

// TDM 416 — Death Begets Life (alternate printing)
const DEATH_BEGETS_LIFE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEATH_BEGETS_LIFE,
    3,
    "4b1251fb-1f39-4afb-b902-140032f20192",
    "Justin Hernandez & Alexis Hernandez",
);

// TDM 417 — Narset, Jeskai Waymaster (alternate printing)
const NARSET_JESKAI_WAYMASTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NARSET_JESKAI_WAYMASTER,
    3,
    "9f104106-2922-404e-a959-5d6d071aad74",
    "Randy Vargas",
);

// TDM 418 — Skirmish Rhino (alternate printing)
const SKIRMISH_RHINO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SKIRMISH_RHINO,
    2,
    "5346269a-aa11-4a93-9fbc-109421afe579",
    "James Bousema",
);

// TDM 419 — Mox Jasper (alternate printing)
const MOX_JASPER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOX_JASPER,
    2,
    "ec33ea23-c8e8-4066-91b9-5e0ad191bcdb",
    "Dan Frazier",
);

// TDM 420 — Static Snare (alternate printing)
const STATIC_SNARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STATIC_SNARE,
    1,
    "4cf3c128-e4a9-4d21-8cf4-dfc122cc0957",
    "Yohann Schepacz",
);

// TDM 421 — Roiling Dragonstorm (alternate printing)
const ROILING_DRAGONSTORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ROILING_DRAGONSTORM,
    2,
    "31cd0d01-8d3f-4a00-acfd-a43a93e14e7d",
    "Gaboleps",
);

// TDM 422 — Strategic Betrayal (alternate printing)
const STRATEGIC_BETRAYAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STRATEGIC_BETRAYAL,
    1,
    "dc8e99f9-7557-45e3-af72-c5cb87927202",
    "Flavio Greco Paglia",
);

// TDM 423 — Channeled Dragonfire (alternate printing)
const CHANNELED_DRAGONFIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHANNELED_DRAGONFIRE,
    1,
    "377aac92-3278-4c81-9095-04ff7d7a81dc",
    "Jorge Jacinto",
);

// TDM 424 — Encroaching Dragonstorm (alternate printing)
const ENCROACHING_DRAGONSTORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENCROACHING_DRAGONSTORM,
    2,
    "f07d668d-bff0-4bae-a42e-4130fdc1016d",
    "Marco Gorlei",
);

// TDM 425 — Temur Battlecrier (alternate printing)
const TEMUR_BATTLECRIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TEMUR_BATTLECRIER,
    2,
    "ee706aa4-3188-47ee-b164-35287b26e677",
    "Valera Lutfullina",
);

// TDM 426 — Qarsi Revenant (alternate printing)
const QARSI_REVENANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QARSI_REVENANT,
    2,
    "03f54ff0-e10a-4f22-ada8-43b61d46ee75",
    "Diana Franco",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &UGIN_EYE_OF_THE_STORMS,
    &ANAFENZA_UNYIELDING_LINEAGE,
    &ARASHIN_SUNSHIELD,
    &BEARER_OF_GLORY,
    &CLARION_CONQUEROR,
    &COORDINATED_MANEUVER,
    &DALKOVAN_PACKBEASTS,
    &DESCENDANT_OF_STORMS,
    &DRAGONBACK_LANCER,
    &DUTY_BEYOND_DEATH,
    &FORTRESS_KIN_GUARD,
    &FURIOUS_FOREBEAR,
    &LIGHTFOOT_TECHNIQUE,
    &LOXODON_BATTLE_PRIEST,
    &MARDU_DEVOTEE,
    &OSSEOUS_EXHALE,
    &POISED_PRACTITIONER,
    &RALLY_THE_MONASTERY,
    &REBELLIOUS_STRIKE,
    &RILING_DAWNBREAKER,
    &SAGE_OF_THE_SKIES,
    &SALT_ROAD_PACKBEAST,
    &SMILE_AT_DEATH,
    &STARRY_EYED_SKYRIDER,
    &STATIC_SNARE,
    &STORMBEACON_BLADE,
    &STORMPLAIN_DETAINMENT,
    &SUNPEARL_KIRIN,
    &TEEMING_DRAGONSTORM,
    &TEMPEST_HAWK,
    &UNITED_BATTLEFRONT,
    &VOICE_OF_VICTORY,
    &WAYSPEAKER_BODYGUARD,
    &AEGIS_SCULPTOR,
    &AGENT_OF_KOTIS,
    &AMBLING_STORMSHELL,
    &BEWILDERING_BLIZZARD,
    &CONSTRICTOR_SAGE,
    &DIRGUR_ISLAND_DRAGON,
    &DISPELLING_EXHALE,
    &DRAGONOLOGIST,
    &DRAGONSTORM_FORECASTER,
    &ESSENCE_ANCHOR,
    &FOCUS_THE_MIND,
    &FRESH_START,
    &HIGHSPIRE_BELL_RINGER,
    &HUMBLING_ELDER,
    &ICERIDGE_SERPENT,
    &KISHLA_TRAWLERS,
    &MARANG_RIVER_REGENT,
    &NAGA_FLESHCRAFTER,
    &RINGING_STRIKE_MASTERY,
    &RIVERWALK_TECHNIQUE,
    &ROILING_DRAGONSTORM,
    &SIBSIG_APPRAISER,
    &SNOWMELT_STAG,
    &SPECTRAL_DENIAL,
    &STILLNESS_IN_MOTION,
    &TAIGAM_MASTER_OPPORTUNIST,
    &TEMUR_DEVOTEE,
    &UNENDING_WHISPER,
    &URENI_S_REBUFF,
    &VETERAN_ICE_CLIMBER,
    &WINGBLADE_DISCIPLE,
    &WINGSPAN_STRIDE,
    &WINTERNIGHT_STORIES,
    &ABZAN_DEVOTEE,
    &ADORNED_CROCODILE,
    &AGGRESSIVE_NEGOTIATIONS,
    &ALCHEMIST_S_ASSISTANT,
    &ALESHA_S_LEGACY,
    &AVENGER_OF_THE_FALLEN,
    &CAUSTIC_EXHALE,
    &CORRODING_DRAGONSTORM,
    &CRUEL_TRUTHS,
    &DELTA_BLOODFLIES,
    &DESPERATE_MEASURES,
    &DRAGON_S_PREY,
    &FERAL_DEATHGORGER,
    &GURMAG_RAKSHASA,
    &HUNDRED_BATTLE_VETERAN,
    &KIN_TREE_NURTURER,
    &KRUMAR_INITIATE,
    &NIGHTBLADE_BRIGADE,
    &QARSI_REVENANT,
    &ROT_CURSE_RAKSHASA,
    &SALT_ROAD_SKIRMISH,
    &SANDSKITTER_OUTRIDER,
    &SCAVENGER_REGENT,
    &THE_SIBSIG_CEREMONY,
    &SIDISI_REGENT_OF_THE_MIRE,
    &SINKHOLE_SURVEYOR,
    &STRATEGIC_BETRAYAL,
    &UNBURIED_EARTHCARVER,
    &UNROOTED_ANCESTOR,
    &VENERATED_STORMSINGER,
    &WAIL_OF_WAR,
    &WORTHY_COST,
    &YATHAN_TOMBGUARD,
    &BREACHING_DRAGONSTORM,
    &CHANNELED_DRAGONFIRE,
    &DEVOTED_DUELIST,
    &DRACOGENESIS,
    &EQUILIBRIUM_ADEPT,
    &FIRE_RIM_FORM,
    &FLEETING_EFFIGY,
    &IRIDESCENT_TIGER,
    &JESKAI_DEVOTEE,
    &MAGMATIC_HELLKITE,
    &METICULOUS_ARTISAN,
    &MOLTEN_EXHALE,
    &NARSET_S_REBUKE,
    &OVERWHELMING_SURGE,
    &RESCUE_LEOPARD,
    &REVERBERATING_SUMMONS,
    &SARKHAN_DRAGON_ASCENDANT,
    &SEIZE_OPPORTUNITY,
    &SHOCK_BRIGADE,
    &SHOCKING_SHARPSHOOTER,
    &STADIUM_HEADLINER,
    &STORMSCALE_SCION,
    &STORMSHRIEK_FERAL,
    &SUMMIT_INTIMIDATOR,
    &SUNSET_STRIKEMASTER,
    &TERSA_LIGHTSHATTER,
    &UNDERFOOT_UNDERDOGS,
    &UNSPARING_BOLTCASTER,
    &WAR_EFFORT,
    &WILD_RIDE,
    &ZURGO_S_VANGUARD,
    &AINOK_WAYFARER,
    &ATTUNED_HUNTER,
    &BLOOMVINE_REGENT,
    &CHAMPION_OF_DUSAN,
    &DRAGON_SNIPER,
    &DRAGONBROODS_RELIC,
    &DUSYUT_EARTHCARVER,
    &ENCROACHING_DRAGONSTORM,
    &FORMATION_BREAKER,
    &HERD_HEIRLOOM,
    &HERITAGE_RECLAMATION,
    &INSPIRITED_VANGUARD,
    &KNOCKOUT_MANEUVER,
    &KROTIQ_NESTGUARD,
    &LASYD_PROWLER,
    &NATURE_S_RHYTHM,
    &PIERCING_EXHALE,
    &RAINVEIL_REJUVENATOR,
    &RITE_OF_RENEWAL,
    &ROAMER_S_ROUTINE,
    &SAGE_OF_THE_FANG,
    &SAGU_PUMMELER,
    &SAGU_WILDLING,
    &SARKHAN_S_RESOLVE,
    &SULTAI_DEVOTEE,
    &SURRAK_ELUSIVE_HUNTER,
    &SYNCHRONIZED_CHARGE,
    &TRADE_ROUTE_ENVOY,
    &TRAVELING_BOTANIST,
    &UNDERGROWTH_LEOPARD,
    &WARDEN_OF_THE_GROVE,
    &ALL_OUT_ASSAULT,
    &ARMAMENT_DRAGON,
    &AURORAL_PROCESSION,
    &AWAKEN_THE_HONORED_DEAD,
    &BARRENSTEPPE_SIEGE,
    &BETOR_KIN_TO_ALL,
    &BONE_CAIRN_BUTCHER,
    &CALL_THE_SPIRIT_DRAGONS,
    &CORI_MOUNTAIN_STALWART,
    &DEATH_BEGETS_LIFE,
    &DEFIBRILLATING_CURRENT,
    &DISRUPTIVE_STORMBROOD,
    &DRAGONBACK_ASSAULT,
    &DRAGONCLAW_STRIKE,
    &EFFORTLESS_MASTER,
    &ESHKI_DRAGONCLAW,
    &FANGKEEPER_S_FAMILIAR,
    &FELOTHAR_DAWN_OF_THE_ABZAN,
    &FLAMEHOLD_GRAPPLER,
    &FRONTLINE_RUSH,
    &FROSTCLIFF_SIEGE,
    &GLACIAL_DRAGONHUNT,
    &GLACIERWOOD_SIEGE,
    &GURMAG_NIGHTWATCH,
    &HARDENED_TACTICIAN,
    &HOLLOWMURK_SIEGE,
    &HOST_OF_THE_HEREAFTER,
    &INEVITABLE_DEFEAT,
    &JESKAI_BRUSHMASTER,
    &JESKAI_REVELATION,
    &JESKAI_SHRINEKEEPER,
    &KARAKYK_GUARDIAN,
    &KHERU_GOLDKEEPER,
    &KIN_TREE_SEVERANCE,
    &KISHLA_SKIMMER,
    &KOTIS_THE_FANGKEEPER,
    &LIE_IN_WAIT,
    &LOTUSLIGHT_DANCERS,
    &MAMMOTH_BELLOW,
    &MARDU_SIEGEBREAKER,
    &MARSHAL_OF_THE_LOST,
    &MONASTERY_MESSENGER,
    &NARSET_JESKAI_WAYMASTER,
    &NERIV_HEART_OF_THE_STORM,
    &NEW_WAY_FORWARD,
    &PERENNATION,
    &PURGING_STORMBROOD,
    &RAKSHASA_S_BARGAIN,
    &REDISCOVER_THE_WAY,
    &REIGNING_VICTOR,
    &REPUTABLE_MERCHANT,
    &REVIVAL_OF_THE_ANCESTORS,
    &RIVERWHEEL_SWEEP,
    &ROAR_OF_ENDLESS_SONG,
    &RUNESCALE_STORMBROOD,
    &SEVERANCE_PRIEST,
    &SHIKO_PARAGON_OF_THE_WAY,
    &SKIRMISH_RHINO,
    &SONGCRAFTER_MAGE,
    &SONIC_SHRIEKER,
    &STALWART_SUCCESSOR,
    &TEMUR_BATTLECRIER,
    &TEMUR_TAWNYBACK,
    &TEVAL_ARBITER_OF_VIRTUE,
    &THUNDER_OF_UNITY,
    &TWINMAW_STORMBROOD,
    &URENI_THE_SONG_UNENDING,
    &WHIRLWING_STORMBROOD,
    &WINDCRAG_SIEGE,
    &YATHAN_ROADWATCHER,
    &ZURGO_THUNDER_S_DECREE,
    &ABZAN_MONUMENT,
    &BOULDERBORN_DRAGON,
    &DRAGONFIRE_BLADE,
    &DRAGONSTORM_GLOBE,
    &EMBERMOUTH_SENTINEL,
    &JADE_CAST_SENTINEL,
    &JESKAI_MONUMENT,
    &MARDU_MONUMENT,
    &MOX_JASPER,
    &SULTAI_MONUMENT,
    &TEMUR_MONUMENT,
    &WATCHER_OF_THE_WAYSIDE,
    &CORI_MOUNTAIN_MONASTERY,
    &DALKOVAN_ENCAMPMENT,
    &GREAT_ARASHIN_CITY,
    &KISHLA_VILLAGE,
    &MAELSTROM_OF_THE_SPIRIT_DRAGON,
    &MISTRISE_VILLAGE,
    &CORI_STEEL_CUTTER,
    &ELSPETH_STORM_SLAYER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ELSPETH_STORM_SLAYER_ALTERNATE_1,
    CORI_STEEL_CUTTER_ALTERNATE_1,
    TWIN_BOLT_REPRINT,
    CRATERHOOF_BEHEMOTH_REPRINT,
    SNAKESKIN_VEIL_REPRINT,
    BLOODFELL_CAVES_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    DISMAL_BACKWATER_REPRINT,
    EVOLVING_WILDS_REPRINT,
    FRONTIER_BIVOUAC_REPRINT,
    JUNGLE_HOLLOW_REPRINT,
    MYSTIC_MONASTERY_REPRINT,
    NOMAD_OUTPOST_REPRINT,
    OPULENT_PALACE_REPRINT,
    RUGGED_HIGHLANDS_REPRINT,
    SANDSTEPPE_CITADEL_REPRINT,
    SCOURED_BARRENS_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
    THORNWOOD_FALLS_REPRINT,
    TRANQUIL_COVE_REPRINT,
    WIND_SCARRED_CRAG_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_3,
    RILING_DAWNBREAKER_ALTERNATE_1,
    TEEMING_DRAGONSTORM_ALTERNATE_1,
    DIRGUR_ISLAND_DRAGON_ALTERNATE_1,
    DRAGONOLOGIST_ALTERNATE_1,
    ROILING_DRAGONSTORM_ALTERNATE_1,
    CORRODING_DRAGONSTORM_ALTERNATE_1,
    FERAL_DEATHGORGER_ALTERNATE_1,
    BREACHING_DRAGONSTORM_ALTERNATE_1,
    DRACOGENESIS_ALTERNATE_1,
    MAGMATIC_HELLKITE_ALTERNATE_1,
    SARKHAN_DRAGON_ASCENDANT_ALTERNATE_1,
    STORMSCALE_SCION_ALTERNATE_1,
    STORMSHRIEK_FERAL_ALTERNATE_1,
    ENCROACHING_DRAGONSTORM_ALTERNATE_1,
    SAGU_WILDLING_ALTERNATE_1,
    ARMAMENT_DRAGON_ALTERNATE_1,
    BETOR_KIN_TO_ALL_ALTERNATE_1,
    CALL_THE_SPIRIT_DRAGONS_ALTERNATE_1,
    DISRUPTIVE_STORMBROOD_ALTERNATE_1,
    JESKAI_SHRINEKEEPER_ALTERNATE_1,
    KARAKYK_GUARDIAN_ALTERNATE_1,
    KHERU_GOLDKEEPER_ALTERNATE_1,
    NERIV_HEART_OF_THE_STORM_ALTERNATE_1,
    PURGING_STORMBROOD_ALTERNATE_1,
    RUNESCALE_STORMBROOD_ALTERNATE_1,
    SHIKO_PARAGON_OF_THE_WAY_ALTERNATE_1,
    SONIC_SHRIEKER_ALTERNATE_1,
    TEVAL_ARBITER_OF_VIRTUE_ALTERNATE_1,
    TWINMAW_STORMBROOD_ALTERNATE_1,
    URENI_THE_SONG_UNENDING_ALTERNATE_1,
    WHIRLWING_STORMBROOD_ALTERNATE_1,
    BOULDERBORN_DRAGON_ALTERNATE_1,
    DRAGONFIRE_BLADE_ALTERNATE_1,
    MOX_JASPER_ALTERNATE_1,
    MAELSTROM_OF_THE_SPIRIT_DRAGON_ALTERNATE_1,
    ANAFENZA_UNYIELDING_LINEAGE_ALTERNATE_1,
    SAGE_OF_THE_SKIES_ALTERNATE_1,
    SMILE_AT_DEATH_ALTERNATE_1,
    UNITED_BATTLEFRONT_ALTERNATE_1,
    VOICE_OF_VICTORY_ALTERNATE_1,
    AMBLING_STORMSHELL_ALTERNATE_1,
    NAGA_FLESHCRAFTER_ALTERNATE_1,
    STILLNESS_IN_MOTION_ALTERNATE_1,
    TAIGAM_MASTER_OPPORTUNIST_ALTERNATE_1,
    WINTERNIGHT_STORIES_ALTERNATE_1,
    AVENGER_OF_THE_FALLEN_ALTERNATE_1,
    QARSI_REVENANT_ALTERNATE_1,
    ROT_CURSE_RAKSHASA_ALTERNATE_1,
    THE_SIBSIG_CEREMONY_ALTERNATE_1,
    SIDISI_REGENT_OF_THE_MIRE_ALTERNATE_1,
    SINKHOLE_SURVEYOR_ALTERNATE_1,
    STADIUM_HEADLINER_ALTERNATE_1,
    TERSA_LIGHTSHATTER_ALTERNATE_1,
    CRATERHOOF_BEHEMOTH_ALTERNATE_1,
    HERD_HEIRLOOM_ALTERNATE_1,
    LASYD_PROWLER_ALTERNATE_1,
    NATURE_S_RHYTHM_ALTERNATE_1,
    SURRAK_ELUSIVE_HUNTER_ALTERNATE_1,
    WARDEN_OF_THE_GROVE_ALTERNATE_1,
    ALL_OUT_ASSAULT_ALTERNATE_1,
    BETOR_KIN_TO_ALL_ALTERNATE_2,
    DEATH_BEGETS_LIFE_ALTERNATE_1,
    DRAGONBACK_ASSAULT_ALTERNATE_1,
    ESHKI_DRAGONCLAW_ALTERNATE_1,
    FANGKEEPER_S_FAMILIAR_ALTERNATE_1,
    FELOTHAR_DAWN_OF_THE_ABZAN_ALTERNATE_1,
    FLAMEHOLD_GRAPPLER_ALTERNATE_1,
    INEVITABLE_DEFEAT_ALTERNATE_1,
    JESKAI_REVELATION_ALTERNATE_1,
    KOTIS_THE_FANGKEEPER_ALTERNATE_1,
    LOTUSLIGHT_DANCERS_ALTERNATE_1,
    MARDU_SIEGEBREAKER_ALTERNATE_1,
    NARSET_JESKAI_WAYMASTER_ALTERNATE_1,
    NERIV_HEART_OF_THE_STORM_ALTERNATE_2,
    NEW_WAY_FORWARD_ALTERNATE_1,
    PERENNATION_ALTERNATE_1,
    SEVERANCE_PRIEST_ALTERNATE_1,
    SHIKO_PARAGON_OF_THE_WAY_ALTERNATE_2,
    SONGCRAFTER_MAGE_ALTERNATE_1,
    TEMUR_BATTLECRIER_ALTERNATE_1,
    TEVAL_ARBITER_OF_VIRTUE_ALTERNATE_2,
    URENI_THE_SONG_UNENDING_ALTERNATE_2,
    YATHAN_ROADWATCHER_ALTERNATE_1,
    ZURGO_THUNDER_S_DECREE_ALTERNATE_1,
    CLARION_CONQUEROR_ALTERNATE_1,
    MARANG_RIVER_REGENT_ALTERNATE_1,
    SCAVENGER_REGENT_ALTERNATE_1,
    MAGMATIC_HELLKITE_ALTERNATE_2,
    BLOOMVINE_REGENT_ALTERNATE_1,
    UGIN_EYE_OF_THE_STORMS_ALTERNATE_2,
    AWAKEN_THE_HONORED_DEAD_ALTERNATE_1,
    BARRENSTEPPE_SIEGE_ALTERNATE_1,
    FROSTCLIFF_SIEGE_ALTERNATE_1,
    GLACIERWOOD_SIEGE_ALTERNATE_1,
    HOLLOWMURK_SIEGE_ALTERNATE_1,
    REDISCOVER_THE_WAY_ALTERNATE_1,
    REVIVAL_OF_THE_ANCESTORS_ALTERNATE_1,
    ROAR_OF_ENDLESS_SONG_ALTERNATE_1,
    THUNDER_OF_UNITY_ALTERNATE_1,
    WINDCRAG_SIEGE_ALTERNATE_1,
    CORI_MOUNTAIN_MONASTERY_ALTERNATE_1,
    DALKOVAN_ENCAMPMENT_ALTERNATE_1,
    GREAT_ARASHIN_CITY_ALTERNATE_1,
    KISHLA_VILLAGE_ALTERNATE_1,
    MISTRISE_VILLAGE_ALTERNATE_1,
    UGIN_EYE_OF_THE_STORMS_ALTERNATE_3,
    CLARION_CONQUEROR_ALTERNATE_2,
    ELSPETH_STORM_SLAYER_ALTERNATE_2,
    DRACOGENESIS_ALTERNATE_2,
    SARKHAN_DRAGON_ASCENDANT_ALTERNATE_2,
    CRATERHOOF_BEHEMOTH_ALTERNATE_2,
    ALL_OUT_ASSAULT_ALTERNATE_2,
    DEATH_BEGETS_LIFE_ALTERNATE_2,
    NARSET_JESKAI_WAYMASTER_ALTERNATE_2,
    SKIRMISH_RHINO_ALTERNATE_1,
    UGIN_EYE_OF_THE_STORMS_ALTERNATE_1,
    CLARION_CONQUEROR_ALTERNATE_3,
    ELSPETH_STORM_SLAYER_ALTERNATE_3,
    DRACOGENESIS_ALTERNATE_3,
    SARKHAN_DRAGON_ASCENDANT_ALTERNATE_3,
    CRATERHOOF_BEHEMOTH_ALTERNATE_3,
    ALL_OUT_ASSAULT_ALTERNATE_3,
    DEATH_BEGETS_LIFE_ALTERNATE_3,
    NARSET_JESKAI_WAYMASTER_ALTERNATE_3,
    SKIRMISH_RHINO_ALTERNATE_2,
    MOX_JASPER_ALTERNATE_2,
    STATIC_SNARE_ALTERNATE_1,
    ROILING_DRAGONSTORM_ALTERNATE_2,
    STRATEGIC_BETRAYAL_ALTERNATE_1,
    CHANNELED_DRAGONFIRE_ALTERNATE_1,
    ENCROACHING_DRAGONSTORM_ALTERNATE_2,
    TEMUR_BATTLECRIER_ALTERNATE_2,
    QARSI_REVENANT_ALTERNATE_2,
];
