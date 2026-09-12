//! Final Fantasy Commander card records.

use crate::TargetIndex;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::DrawEventMatcherDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueComparisonDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "FIC",
    slug: "final-fantasy-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// FIC 1 — Celes, Rune Knight
// Audit: unsupported — There is no grouped creature-entry trigger preserving the whole entering set and each creature's entry/cast provenance. Per-creature triggers would add too many counters for simultaneous entries.
pub(in crate::card::sets) static CELES_RUNE_KNIGHT_1: CardRecord = CardRecord::new(
    "Celes, Rune Knight",
    "30584c53-533b-4dc7-b07c-8600164a99b3",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// FIC 2 — Cloud, Ex-SOLDIER
// Audit: unsupported — Object queries can select Equipment attached to a source, but cannot select each distinct equipped attacking creature. Counting attached Equipment overcounts creatures carrying multiple Equipment, and there is no equipment-host projection for the draw count.
pub(in crate::card::sets) static CLOUD_EX_SOLDIER_2: CardRecord = CardRecord::new(
    "Cloud, Ex-SOLDIER",
    "07b4e4f8-6a31-4533-be51-668ce3ddc84f",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// FIC 43 — Espers to Magicite
// Audit: unsupported — The collection continuation cannot install a reflexive trigger whose legal targets are restricted to the exact newly exiled set. Choosing a card during resolution would bypass targeting and the response window.
pub(in crate::card::sets) static ESPERS_TO_MAGICITE_43: CardRecord = CardRecord::new(
    "Espers to Magicite",
    "6cb18871-dd23-4ce8-a535-16adefae63c0",
    "AKAGI",
    crate::card::CardRules::unsupported(),
);

// FIC 52 — Transpose
pub(in crate::card::sets) static TRANSPOSE: CardRecord = CardRecord::new(
    "Transpose",
    "66392b0e-8691-42a4-bc84-03b017174a73",
    "Toni Infante",
CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card, then discard a card. You lose 1 life. If this spell was cast from your hand, create a 0/1 black Wizard creature token with \"Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.\"",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Wizard"], &[ManaColor::Black], 0, 1).with_abilities(&[
                            AbilityDef::triggered(
                                "Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.",
                                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::NoncreatureSpell,
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ])),
                                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
                            ),
                        ]),
                    ))),
                },
            ]),
        ),
        abilities::rebound(),
    ]),
);

// FIC 55 — Gau, Feral Youth
pub(in crate::card::sets) static GAU_FERAL_YOUTH: CardRecord = CardRecord::new(
    "Gau, Feral Youth",
    "89175ce1-0746-4ba1-970e-617d134b0527",
    "Eglė Mosakaitė",
// Two mana that grows every attack and, in a deck that is already using
    // its graveyard, throws that growth at the opponent every end step.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Berserker"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // "Rage" is an ability word: flavour on the front of an ordinary attack
            // trigger, and nothing the rules read.
            AbilityDef::triggered(
                "Rage — Whenever Gau attacks, put a +1/+1 counter on it.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            // Each end step, not just yours: a graveyard emptied on their turn pays
            // out on their turn too.
            AbilityDef::triggered_if(
                "At the beginning of each end step, if a card left your graveyard this turn, Gau deals \
                 damage equal to its power to each opponent.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                // An intervening-if, so it is checked twice: once when the end step begins
                // and again as the ability resolves. A graveyard that gave a card up and
                // then got it back is still a graveyard a card left.
                &TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn,
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::SourcePower),
            ),
        ]),
);

// FIC 56 — Gogo, Mysterious Mime
pub(in crate::card::sets) static GOGO_MYSTERIOUS_MIME_56: CardRecord = CardRecord::new(
    "Gogo, Mysterious Mime",
    "0db05dc8-03f8-4ab4-9ca3-2aaaa0099eb4",
    "Lee Woo-chul",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Wizard"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered_with_targets("At the beginning of combat on your turn, you may have Gogo become a copy of another target creature you control until end of turn, except its name is Gogo, Mysterious Mime. If you do, Gogo and that creature each get +2/+0 and gain haste until end of turn and attack this turn if able.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::IfCondition { condition: &TriggerConditionDef::SourceOnBattlefield, then: &EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Sequence(&[EffectDef::BecomeCopyOf { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), copier: None, exceptions: CopyExceptionsDef::NONE.with_name("Gogo, Mysterious Mime"), duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn) }, EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Union(&[ObjectSetDef::One(ObjectRefDef::Source), ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)])), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)), AppliedEffectDef::add_ability(&abilities::haste()), AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }]) } })
]),
);

// FIC 119 — Transpose (alternate printing)
const TRANSPOSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRANSPOSE,
    1,
    "51cb61ab-0508-4668-8680-051d38df7ccb",
    "Toni Infante",
);

// FIC 120 — Snort
// Audit: unsupported — There is no simultaneous optional whole-hand discard procedure that freezes each player's choice in APNAP order, then performs the selected discards and draws before the damage. Separate May branches would reveal one player's new hand or discard outcome before the other chooses.
pub(in crate::card::sets) static SNORT_120: CardRecord = CardRecord::new(
    "Snort",
    "2fbe13c7-af6c-43f4-b947-f32ea48a0edb",
    "ikeda_cpt",
    crate::card::CardRules::unsupported(),
);

// FIC 138 — Tataru Taru
pub(in crate::card::sets) static TATARU_TARU_138: CardRecord = CardRecord::new(
    "Tataru Taru",
    "8c832508-e6f8-4581-8424-744f4e24fad2",
    "Livia Prima",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dwarf", "Advisor"], 0, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::enters_trigger_with_targets("When Tataru Taru enters, you draw a card and target opponent may draw a card.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), EffectDef::May { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: &EffectDef::DrawCards { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(1) } }])),
AbilityDef::triggered_if("Scions' Secretary — Whenever an opponent draws a card, if it isn't that player's turn, create a tapped Treasure token. This ability triggers only once each turn.", TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)), &TriggerConditionDef::Not(&TriggerConditionDef::ActivePlayer(PlayerRelation::EventPlayer)), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure())).entering_tapped())).triggering_at_most(1)
]),
);

// FIC 163 — Aerith, Last Ancient
pub(in crate::card::sets) static AERITH_LAST_ANCIENT_163: CardRecord = CardRecord::new(
    "Aerith, Last Ancient",
    "82518d3f-9557-416b-9b4d-dfe3ffa57f88",
    "Marta Nael",
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Human", "Cleric", "Druid"], 3, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::lifelink(),
AbilityDef::triggered_if_with_targets("Raise — At the beginning of your end step, if you gained life this turn, return target creature card from your graveyard to your hand. If you gained 7 or more life this turn, return that card to the battlefield instead.", TriggerEventDef::StepBegins { step: TurnStepDef::End, player: PlayerRelation::You }, &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::LifeGainedThisTurn(PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(1) }), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::IfElseCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::LifeGainedThisTurn(PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(7) }), then: &EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top), otherwise: &EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top) })
]),
);

// FIC 191 — Y'shtola, Night's Blessed
// Audit: unsupported — Turn history does not record each player's total life lost this turn. Damage history and current life totals cannot answer the four-life-loss threshold.
pub(in crate::card::sets) static Y_SHTOLA_NIGHT_S_BLESSED_191: CardRecord = CardRecord::new(
    "Y'shtola, Night's Blessed",
    "0bda4de9-d0ec-4d27-b92b-8a76779747cf",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// FIC 216 — Yuna, Grand Summoner
// Audit: unsupported — There is no next-creature-cast entry-counter rider independent of spending the generated mana. The death trigger also needs a sum across all counter kinds rather than one named kind.
pub(in crate::card::sets) static YUNA_GRAND_SUMMONER_216: CardRecord = CardRecord::new(
    "Yuna, Grand Summoner",
    "2819652e-c944-4c5d-a098-2d15e232366e",
    "Mai Okuma",
    crate::card::CardRules::unsupported(),
);

// FIC 225 — Tifa, Martial Artist
// Audit: unsupported — The trigger cannot ask whether the current combat is the first combat phase of the turn. First-attack history is not equivalent.
pub(in crate::card::sets) static TIFA_MARTIAL_ARTIST_225: CardRecord = CardRecord::new(
    "Tifa, Martial Artist",
    "09f09db5-ee5a-4a4b-9dbb-aca0dff04fcf",
    "Yumi Yaoshida",
    crate::card::CardRules::unsupported(),
);

// FIC 458 — Vivi's Persistence
pub(in crate::card::sets) static VIVI_S_PERSISTENCE_458: CardRecord = CardRecord::new(
    "Vivi's Persistence",
    "be6ba2e4-e657-4a2d-8f5f-255376d861b3",
    "Erion Makuo",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
AbilityDef::spell("Create a 0/1 black Wizard creature token with \"Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.\"", EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Wizard"], &[ManaColor::Black], 0, 1).with_abilities(&[AbilityDef::triggered("Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::NoncreatureSpell, ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)))]))))),
AbilityDef::triggered("Whenever your commander enters or attacks, you may pay {2}. If you do, return this card from your graveyard to your hand.", TriggerEventDef::AnyOf(&[TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::OwnedBy(PlayerRelation::You)]), None, Some(ZoneKind::Battlefield)), TriggerEventDef::attacks(ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::OwnedBy(PlayerRelation::You)]))]), EffectDef::PayOr(PayOrDef::optional(&[CostDef::Mana(mana_cost!("{2}"))], &EffectDef::move_to_zone(EffectRecipientDef::Source, ZoneKind::Hand, ZonePlacement::Top)))).with_source_zones(&[ZoneKind::Graveyard])
]),
);

// FIC 463 — Flash Photography
// Audit: unsupported — Conditional cast-timing permission cannot depend on the targets being chosen for the prospective cast. Flash would otherwise authorize copying opponents' permanents at instant speed.
pub(in crate::card::sets) static FLASH_PHOTOGRAPHY_463: CardRecord = CardRecord::new(
    "Flash Photography",
    "2bca2cd2-4d4a-44e4-87c3-732692b77921",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CELES_RUNE_KNIGHT_1,
    &CLOUD_EX_SOLDIER_2,
    &ESPERS_TO_MAGICITE_43,
    &TRANSPOSE,
    &GAU_FERAL_YOUTH,
    &GOGO_MYSTERIOUS_MIME_56,
    &SNORT_120,
    &TATARU_TARU_138,
    &AERITH_LAST_ANCIENT_163,
    &Y_SHTOLA_NIGHT_S_BLESSED_191,
    &YUNA_GRAND_SUMMONER_216,
    &TIFA_MARTIAL_ARTIST_225,
    &VIVI_S_PERSISTENCE_458,
    &FLASH_PHOTOGRAPHY_463,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[TRANSPOSE_ALTERNATE_1];
