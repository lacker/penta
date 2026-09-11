//! Final Fantasy Commander card records.

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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELES_RUNE_KNIGHT_1: CardRecord = CardRecord::new(
    "Celes, Rune Knight",
    "30584c53-533b-4dc7-b07c-8600164a99b3",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// FIC 2 — Cloud, Ex-SOLDIER
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_EX_SOLDIER_2: CardRecord = CardRecord::new(
    "Cloud, Ex-SOLDIER",
    "07b4e4f8-6a31-4533-be51-668ce3ddc84f",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// FIC 43 — Espers to Magicite
// Audit: unsupported — Card rules have not been implemented.
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOGO_MYSTERIOUS_MIME_56: CardRecord = CardRecord::new(
    "Gogo, Mysterious Mime",
    "0db05dc8-03f8-4ab4-9ca3-2aaaa0099eb4",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// FIC 119 — Transpose (alternate printing)
const TRANSPOSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRANSPOSE,
    1,
    "51cb61ab-0508-4668-8680-051d38df7ccb",
    "Toni Infante",
);

// FIC 120 — Snort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNORT_120: CardRecord = CardRecord::new(
    "Snort",
    "2fbe13c7-af6c-43f4-b947-f32ea48a0edb",
    "ikeda_cpt",
    crate::card::CardRules::unsupported(),
);

// FIC 138 — Tataru Taru
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TATARU_TARU_138: CardRecord = CardRecord::new(
    "Tataru Taru",
    "8c832508-e6f8-4581-8424-744f4e24fad2",
    "Livia Prima",
    crate::card::CardRules::unsupported(),
);

// FIC 163 — Aerith, Last Ancient
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AERITH_LAST_ANCIENT_163: CardRecord = CardRecord::new(
    "Aerith, Last Ancient",
    "82518d3f-9557-416b-9b4d-dfe3ffa57f88",
    "Marta Nael",
    crate::card::CardRules::unsupported(),
);

// FIC 191 — Y'shtola, Night's Blessed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static Y_SHTOLA_NIGHT_S_BLESSED_191: CardRecord = CardRecord::new(
    "Y'shtola, Night's Blessed",
    "0bda4de9-d0ec-4d27-b92b-8a76779747cf",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// FIC 216 — Yuna, Grand Summoner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YUNA_GRAND_SUMMONER_216: CardRecord = CardRecord::new(
    "Yuna, Grand Summoner",
    "2819652e-c944-4c5d-a098-2d15e232366e",
    "Mai Okuma",
    crate::card::CardRules::unsupported(),
);

// FIC 225 — Tifa, Martial Artist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIFA_MARTIAL_ARTIST_225: CardRecord = CardRecord::new(
    "Tifa, Martial Artist",
    "09f09db5-ee5a-4a4b-9dbb-aca0dff04fcf",
    "Yumi Yaoshida",
    crate::card::CardRules::unsupported(),
);

// FIC 458 — Vivi's Persistence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIVI_S_PERSISTENCE_458: CardRecord = CardRecord::new(
    "Vivi's Persistence",
    "be6ba2e4-e657-4a2d-8f5f-255376d861b3",
    "Erion Makuo",
    crate::card::CardRules::unsupported(),
);

// FIC 463 — Flash Photography
// Audit: unsupported — Card rules have not been implemented.
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
