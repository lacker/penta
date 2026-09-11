//! Commander Legends card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementAbilityDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMR",
    slug: "commander-legends",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "284ec798-2725-4741-8748-578c259d0623",
    "Alayna Danner",
));

// CMR 3 — Akroma's Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKROMA_S_WILL_3: CardRecord = CardRecord::new(
    "Akroma's Will",
    "c281997b-1566-4469-a14c-6645f81ab023",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// CMR 74 — Hullbreacher
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    "Hullbreacher",
    "4df8aabc-7fcb-4b7b-980b-18f499e6c170",
    "Sidharth Chaturvedi",
// Three mana at instant speed that turns their draw spell into your
    // mana, and a 3/2 body attached to it.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Pirate"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::defined_replacement(
                "If an opponent would draw a card except the first one they draw in each of their draw \
                 steps, instead you create a Treasure token. (It\'s an artifact with \"{T}, Sacrifice \
                 this token: Add one mana of any color.\")",
                // "Except the first one they draw in each of their draw steps": their
                // turn-based draw still happens, and everything after it does not.
                ReplacementAbilityDef::new()
                    .with_event(ReplacementEventDef::WouldDraw {
                        player: PlayerRelation::Opponent,
                        during_own_draw_step: false,
                        except_first_in_draw_step: true,
                    }),
                // The draw is replaced outright and the Treasure is the effect's
                // controller's, which is what makes this a tax on them rather than a gift:
                // the card they would have drawn stays in their library.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::ReplaceEventWithNothing,
                    ReplacementEffectDef::Perform(&EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN)))),
                ]),
            ),
        ]),
);

// CMR 79 — Malcolm, Keen-Eyed Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALCOLM_KEEN_EYED_NAVIGATOR_79: CardRecord = CardRecord::new(
    "Malcolm, Keen-Eyed Navigator",
    "bbc3bbda-a4bc-4302-a3fc-b1c89f0f5461",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// CMR 89 — Sakashima of a Thousand Faces
// Audit: unsupported — Needs the Partner deck-construction permission and an entry-copy exception that retains every other ability printed on the source.
pub(in crate::card::sets) static SAKASHIMA_OF_A_THOUSAND_FACES: CardRecord = CardRecord::new(
    "Sakashima of a Thousand Faces",
    "714c3a1f-7b30-4ed8-8f38-6176758741fb",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// CMR 141 — Opposition Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPPOSITION_AGENT_141: CardRecord = CardRecord::new(
    "Opposition Agent",
    "086f97e9-8b62-44f3-b467-149c2ac5ca78",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// CMR 172 — Dargo, the Shipwrecker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARGO_THE_SHIPWRECKER_172: CardRecord = CardRecord::new(
    "Dargo, the Shipwrecker",
    "5cd87cf8-4d5d-4aba-8dfa-800b1fb3799b",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// CMR 183 — Hellkite Courser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELLKITE_COURSER_183: CardRecord = CardRecord::new(
    "Hellkite Courser",
    "db45122e-b5ef-487b-8ea9-59ea066d3c88",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// CMR 185 — Impulsive Pilferer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPULSIVE_PILFERER_185: CardRecord = CardRecord::new(
    "Impulsive Pilferer",
    "55ba9bea-5549-45cf-896c-501a1c81fd5a",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// CMR 187 — Jeska's Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKA_S_WILL_187: CardRecord = CardRecord::new(
    "Jeska's Will",
    "4e91d96d-cc69-439b-b876-a7d57039022c",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// CMR 189 — Krark, the Thumbless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRARK_THE_THUMBLESS_189: CardRecord = CardRecord::new(
    "Krark, the Thumbless",
    "06a981cd-1951-438e-95c9-68294795638e",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// CMR 197 — Rograkh, Son of Rohgahh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROGRAKH_SON_OF_ROHGAHH_197: CardRecord = CardRecord::new(
    "Rograkh, Son of Rohgahh",
    "a4fab67f-00c2-4125-9262-d21a29411797",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// CMR 211 — Wheel of Misfortune
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHEEL_OF_MISFORTUNE_211: CardRecord = CardRecord::new(
    "Wheel of Misfortune",
    "74177b51-a300-49d9-8ea7-557b19cf80c7",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// CMR 216 — Annoyed Altisaur
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    "Annoyed Altisaur",
    "7536d618-0c98-45bb-913b-b8117b4acf87",
    "Lars Grant-West",
    // Seven mana with cascade attached, which is why a limited deck plays it
    // as two cards rather than as an expensive one.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 5).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::cascade(),
    ]),
);

// CMR 217 — Apex Devastator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APEX_DEVASTATOR_217: CardRecord = CardRecord::new(
    "Apex Devastator",
    "8fa281e1-5c48-4bba-b8e9-88c6f5f53abb",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// CMR 305 — Commander's Plate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDER_S_PLATE_305: CardRecord = CardRecord::new(
    "Commander's Plate",
    "19992dbd-7a6a-43d3-b1db-01716b2eed27",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// CMR 354 — Rejuvenating Springs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REJUVENATING_SPRINGS_354: CardRecord = CardRecord::new(
    "Rejuvenating Springs",
    "51e69910-0d90-48a0-af29-3cddaeec5151",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// CMR 356 — Spectator Seating
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTATOR_SEATING_356: CardRecord = CardRecord::new(
    "Spectator Seating",
    "2f6f1453-fe93-4a29-965c-5f867a81e8b3",
    "Ravenna Tran",
    crate::card::CardRules::unsupported(),
);

// CMR 360 — Vault of Champions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAULT_OF_CHAMPIONS_360: CardRecord = CardRecord::new(
    "Vault of Champions",
    "0e144ae1-500d-4485-b476-5783b14380d9",
    "Cliff Childs",
    crate::card::CardRules::unsupported(),
);

// CMR 361 — War Room
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_ROOM_361: CardRecord = CardRecord::new(
    "War Room",
    "48d6ce7c-5dc8-449b-acbd-db259ae687ed",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// CMR 573 — Kediss, Emberclaw Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEDISS_EMBERCLAW_FAMILIAR_573: CardRecord = CardRecord::new(
    "Kediss, Emberclaw Familiar",
    "23766fa0-e673-46c7-a29f-3fb140844b1c",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// CMR 669 — Port Razer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PORT_RAZER_669: CardRecord = CardRecord::new(
    "Port Razer",
    "77222431-9db0-4bc8-80be-7bfc7c48bc5e",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// CMR 713 — Training Center
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINING_CENTER_713: CardRecord = CardRecord::new(
    "Training Center",
    "15a0efa5-6559-446b-a4d9-47585f6e94fb",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// CMR 714 — Undergrowth Stadium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROWTH_STADIUM_714: CardRecord = CardRecord::new(
    "Undergrowth Stadium",
    "35bd7a70-0cd1-48f2-96b6-7869c003a8c7",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMA_S_WILL_3,
    &HULLBREACHER,
    &MALCOLM_KEEN_EYED_NAVIGATOR_79,
    &SAKASHIMA_OF_A_THOUSAND_FACES,
    &OPPOSITION_AGENT_141,
    &DARGO_THE_SHIPWRECKER_172,
    &HELLKITE_COURSER_183,
    &IMPULSIVE_PILFERER_185,
    &JESKA_S_WILL_187,
    &KRARK_THE_THUMBLESS_189,
    &ROGRAKH_SON_OF_ROHGAHH_197,
    &WHEEL_OF_MISFORTUNE_211,
    &ANNOYED_ALTISAUR,
    &APEX_DEVASTATOR_217,
    &COMMANDER_S_PLATE_305,
    &REJUVENATING_SPRINGS_354,
    &SPECTATOR_SEATING_356,
    &VAULT_OF_CHAMPIONS_360,
    &WAR_ROOM_361,
    &KEDISS_EMBERCLAW_FAMILIAR_573,
    &PORT_RAZER_669,
    &TRAINING_CENTER_713,
    &UNDERGROWTH_STADIUM_714,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
