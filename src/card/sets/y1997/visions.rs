//! Visions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AddManaEffectDef;
use crate::ResolvedEffectDurationDef;
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    ArrivalAttachmentDef, BasicLandType, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef, PlayerRefDef,
    PlayerRelation, SpellAdditionalCostDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::card::{
    AppliedEffectDef, AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, CounterKind,
    EffectPaymentDef, PayOrDef, PlayerSetDef,
};
use crate::{TargetIndex, mana_cost};

// VIS 1 — Archangel
pub(in crate::card::sets) static ARCHANGEL: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Archangel",
    "368144bf-d415-48ab-a957-9d7ac1ceb353",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 5, 5)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// VIS 2 — Daraja Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARAJA_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Daraja Griffin",
    "2f7afcaa-9df8-4dd6-89ad-bc2e15f1ec4b",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// VIS 3 — Equipoise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EQUIPOISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Equipoise",
    "53783312-3551-4361-ab02-c9651ce2a926",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 4 — Eye of Singularity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EYE_OF_SINGULARITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Eye of Singularity",
    "fa84e4ad-738a-4d23-a84c-06c39ff4200b",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// VIS 5 — Freewind Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREEWIND_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Freewind Falcon",
    "33dc0244-319c-4e15-9083-8d21ad0364d8",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// VIS 6 — Gossamer Chains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOSSAMER_CHAINS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Gossamer Chains",
    "e9917a29-c6b4-4e0a-a301-21868bd27e17",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// VIS 7 — Honorable Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORABLE_PASSAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Honorable Passage",
    "6559d301-98bd-40a9-abf4-1079d7283214",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 8 — Hope Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPE_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Hope Charm",
    "a1a8980f-07ab-49b7-b83d-f394952ced57",
    "Greg Spalenka",
    crate::card::CardRules::unsupported(),
);

// VIS 9 — Infantry Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFANTRY_VETERAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Infantry Veteran",
    "0350470b-feea-4e15-bdf0-850b71dbeea6",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// VIS 10 — Jamuraan Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JAMURAAN_LION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Jamuraan Lion",
    "bfc681f5-9fff-48b6-98d9-e85c85e582a3",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// VIS 11 — Knight of Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_VALOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Knight of Valor",
    "25aa80ae-bb17-4e52-a269-efe75cf4c041",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 12 — Longbow Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONGBOW_ARCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Longbow Archer",
    "e2ee185d-f5ae-4b1d-90a4-840182f87ab8",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// VIS 13 — Miraculous Recovery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRACULOUS_RECOVERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Miraculous Recovery",
    "76fecb31-790a-4454-918e-5aeb253021f0",
    "Brian Horton",
    crate::card::CardRules::unsupported(),
);

// VIS 14 — Parapet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARAPET: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Parapet",
    "a7bbcaa9-edbf-48ad-bcd2-65e8fb9bb938",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// VIS 15 — Peace Talks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_TALKS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Peace Talks",
    "21da279d-a723-4902-bf84-dfe2c569d4c8",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// VIS 16 — Relic Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELIC_WARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Relic Ward",
    "f0459667-b7da-43bd-b981-0e515432d147",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// VIS 17 — Remedy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMEDY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Remedy",
    "2a0b7162-4422-4dfb-a6ca-8d89fa74e6dc",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// VIS 18 — Resistance Fighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESISTANCE_FIGHTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Resistance Fighter",
    "21250bdb-9431-41b3-9fef-d66a4d3f6ecd",
    "Cecil Fernando",
    crate::card::CardRules::unsupported(),
);

// VIS 19 — Retribution of the Meek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETRIBUTION_OF_THE_MEEK: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Retribution of the Meek",
    "860b8633-1bfc-426a-8666-5e6a584d4525",
    "Nathalie Hertz",
    crate::card::CardRules::unsupported(),
);

// VIS 20 — Righteous Aura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_AURA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Righteous Aura",
    "fed82843-2853-42d3-bcf6-b831032b7a69",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 21 — Sun Clasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_CLASP: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Sun Clasp",
    "e3f1fb74-bc08-4c3b-9fbe-da6973aaeaa2",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// VIS 22 — Teferi's Honor Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_HONOR_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Teferi's Honor Guard",
    "4177d5bf-db48-4bbf-bbd4-ee6313031920",
    "Cecil Fernando",
    crate::card::CardRules::unsupported(),
);

// VIS 23 — Tithe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TITHE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Tithe",
    "aae08938-e563-4322-b2eb-db81913ea730",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// VIS 24 — Warrior's Honor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARRIOR_S_HONOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Warrior's Honor",
    "7babd273-3e20-4cf9-bf21-c602eb729fc5",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// VIS 25 — Zhalfirin Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZHALFIRIN_CRUSADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Zhalfirin Crusader",
    "d8ed802f-6e54-4fed-a71e-6d404c2c664b",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 26 — Betrayal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BETRAYAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Betrayal",
    "7f9b5c75-882e-4fe4-827f-584080e91485",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// VIS 27 — Breezekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREEZEKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Breezekeeper",
    "beaefa77-6e4a-4724-a443-fa6b45803db5",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 28 — Chronatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHRONATOG: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Chronatog",
    "05ada02f-04e9-4269-b04a-97a7eaac2c46",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// VIS 29 — Cloud Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Cloud Elemental",
    "4f2a5146-cf2e-40c0-b498-06e611343196",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 30 — Desertion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Desertion",
    "9a2a1779-af08-4a9a-aba4-e6892ce2332c",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// VIS 31 — Dream Tides
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_TIDES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Dream Tides",
    "3bd292a0-ec08-4250-8d75-0802e985d6e6",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// VIS 32 — Flooded Shoreline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_SHORELINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Flooded Shoreline",
    "49db9f58-380f-496e-9d3d-6776d30fb564",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 33 — Foreshadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORESHADOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Foreshadow",
    "d54c51de-bfac-4198-a7c4-37b4db74e525",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 34 — Impulse
pub(in crate::card::sets) static IMPULSE: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Impulse",
    "9d710a97-062f-4773-b6c6-8aeddeb3b6e8",
    "Bryan Talbot",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Look at the top four cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(4),
            ObjectPredicateDef::Any,
            1,
            1,
        ),
    )),
);

// VIS 35 — Inspiration
pub(in crate::card::sets) static INSPIRATION: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Inspiration",
    "5247d0b0-660e-4f27-8e76-62effbe12221",
    "Zina Saunders",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// VIS 36 — Knight of the Mists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_THE_MISTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Knight of the Mists",
    "37924cbc-fb9d-4906-9ad2-9b6d4ccfff0f",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// VIS 37 — Man-o'-War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAN_O_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Man-o'-War",
    "4dbf9bf9-75cd-4b25-a3a1-43b7e029700b",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// VIS 38 — Mystic Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_VEIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Mystic Veil",
    "7ddb640d-5c54-4d0a-b8c2-e22fe04f96c2",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// VIS 39 — Ovinomancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVINOMANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Ovinomancer",
    "ae4f0988-4194-4481-a6b7-27753261174a",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// VIS 40 — Prosperity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROSPERITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Prosperity",
    "3fa5e806-3cf2-4241-b45d-a05d2b715efd",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// VIS 41 — Rainbow Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAINBOW_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Rainbow Efreet",
    "1d6f03a6-3665-40e4-ae68-640913972770",
    "Nathalie Hertz",
    crate::card::CardRules::unsupported(),
);

// VIS 42 — Shimmering Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Shimmering Efreet",
    "23c5704f-5856-4422-9d82-14558dbe1434",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 43 — Shrieking Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEKING_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Shrieking Drake",
    "63971a64-c5f3-4d1f-ae0d-489d7d5b18f0",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// VIS 44 — Teferi's Realm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_REALM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Teferi's Realm",
    "aba3e4ea-2241-4f1e-a46b-70f512fe729e",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 45 — Three Wishes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_WISHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Three Wishes",
    "dbb2b253-7023-44d1-963b-eae98d48f498",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 46 — Time and Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_AND_TIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Time and Tide",
    "152b348a-0301-4d45-a2c1-d78802c445ba",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 47 — Undo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDO: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Undo",
    "2bef942e-9d17-4d40-a4c9-8be715e73a08",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 48 — Vanishing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VANISHING: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Vanishing",
    "8d1fb805-1382-458c-b98d-4491f13833b6",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// VIS 49 — Vision Charm
pub(in crate::card::sets) static VISION_CHARM: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Vision Charm",
    "78b384d3-3adf-493a-8b89-bfe68fd1c3e2",
    "Greg Spalenka",
    // One blue for whichever of three the turn calls for. The deck wants the
    // land mode to strand an opponent's colours, and the phase-out to answer
    // an artifact at instant speed.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        // The printed first choice is "a land type", which includes the nonbasic
        // ones. Nothing in this card pool carries a nonbasic land subtype, so the
        // choice offered is over the basic types alone.
        &[
            AbilityDef::spell_with_targets(
                "Target player mills four cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::spell(
                "Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.",
                EffectDef::SubstituteBasicLandTypeUntilEndOfTurn {
                    chooser: PlayerRefDef::EffectController,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target artifact phases out.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::PhaseOut {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ],
    )),
);

// VIS 50 — Waterspout Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERSPOUT_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Waterspout Djinn",
    "6946a75e-e9d1-4a56-86d1-dd81f7b1b125",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 51 — Aku Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKU_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Aku Djinn",
    "369a5df5-fc36-476c-84f4-ec4bdeb4f9d2",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 52 — Blanket of Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLANKET_OF_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Blanket of Night",
    "fe012fd0-9ff0-4436-a890-3ab436e42201",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 53 — Brood of Cockroaches
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROOD_OF_COCKROACHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Brood of Cockroaches",
    "30b6150e-7d0c-4361-b99b-79de96dfc53a",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 54 — Coercion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COERCION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Coercion",
    "f3b07d33-f5f5-45cc-b2ac-360eaf2d4146",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// VIS 55 — Crypt Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_RATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Crypt Rats",
    "736455f6-c1b3-4a5a-a91f-a0cd3986ed53",
    "Paul Lee",
    crate::card::CardRules::unsupported(),
);

// VIS 56 — Dark Privilege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_PRIVILEGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Dark Privilege",
    "10d2cf44-cc20-4a37-81ae-930f8c6d0896",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 57 — Death Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_WATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Death Watch",
    "0e939d8f-6989-4884-989b-9cba566c9963",
    "Brian Horton",
    crate::card::CardRules::unsupported(),
);

// VIS 58 — Desolation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESOLATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Desolation",
    "3b186460-d2af-4912-ba19-95b2cb5f1639",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 59 — Fallen Askari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLEN_ASKARI: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Fallen Askari",
    "00107210-313f-49c1-84ff-92628f75b764",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// VIS 60 — Forbidden Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_RITUAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Forbidden Ritual",
    "f5327e6d-db4e-4b44-a00e-b764e80b8946",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// VIS 61 — Funeral Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNERAL_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Funeral Charm",
    "e79d7240-2014-4838-bace-80666192a73e",
    "Greg Spalenka",
    crate::card::CardRules::unsupported(),
);

// VIS 62 — Infernal Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Infernal Harvest",
    "ccf85ac9-f5d8-4a36-aa6c-3a31427a0348",
    "Nathalie Hertz",
    crate::card::CardRules::unsupported(),
);

// VIS 63 — Kaervek's Spite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAERVEK_S_SPITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Kaervek's Spite",
    "d385b9e5-e13d-4098-ba74-ea55bde164d9",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// VIS 64 — Necromancy
pub(in crate::card::sets) static NECROMANCY: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Necromancy",
    "311a6257-dd77-4bb6-81cb-c8e7862350f3",
    "Pete Venters",
    // Three mana for anything in any graveyard, at instant speed if you are
    // willing to give it back at cleanup. It is typed an Aura from the
    // start rather than becoming one as it enters: the difference is only
    // visible while the spell is on the stack, and nothing there reads it.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        // "Enchant creature put onto the battlefield with Necromancy" is
        // narrower than this, but the card guarantees the narrowing itself:
        // it only ever attaches to the creature it just reanimated.
        .enchanting(ObjectPredicateDef::HasType(CardType::Creature))
        .with_abilities(&[
            // "As though it had flash" and having flash differ only in what
            // reads the keyword, and nothing in the pool reads an
            // enchantment's.
            abilities::flash(),
            // Any graveyard, not only your own: the card is a reanimation spell for
            // whatever died, whoever owned it.
            AbilityDef::triggered_if_with_targets("When this enchantment enters, if it's on the battlefield, it becomes an Aura with \"enchant creature put onto the battlefield with Necromancy.\" Put target creature card from a graveyard onto the battlefield under your control and attach this enchantment to it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // "If it's on the battlefield" is an intervening if, read again as
                // the trigger resolves: an enchantment answered in that window
                // reanimates nothing rather than pulling a creature out of a
                // graveyard from somewhere else.
                &const { TriggerConditionDef::SourceOnBattlefield },
                &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            // The reanimation and the attachment are one step: what arrives is a new
            // object, so a following effect would have nothing left to name.
            )], EffectDef::Sequence(&const {
                [
                    // The reanimation and the attachment are one step: what arrives is a new
                    // object, so a following effect would have nothing left to name.
                    EffectDef::WithBattlefieldArrival {
                        effect: &const {
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                zone: ZoneKind::Battlefield,
                                placement: ZonePlacement::Top,
                            }
                        },
                        arrival: crate::card::BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            attachment: Some(ArrivalAttachmentDef::SourceToArrival),
                            ..crate::card::BattlefieldArrivalDef::DEFAULT
                        },
                    },
                    EffectDef::IfCondition {
                        condition: &const { TriggerConditionDef::SourceCastAtInstantSpeed },
                        // "The controller of the permanent it becomes sacrifices it at the
                        // beginning of the next cleanup step" -- the price of casting it at
                        // instant speed, and nothing at all when it was cast on your own turn.
                        then: &const {
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                                AbilityDef::triggered(
                                    "At the beginning of the next cleanup step, sacrifice this enchantment.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::Cleanup,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::Source,
                                    },
                                )
                            }))
                        },
                    },
                ]
            })),
            AbilityDef::triggered(
                "When this enchantment leaves the battlefield, that creature's controller sacrifices it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// VIS 65 — Necrosavant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECROSAVANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Necrosavant",
    "e70cd5fa-ae66-4ea4-90d2-28af2aa34dd4",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// VIS 66 — Nekrataal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEKRATAAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Nekrataal",
    "dba3e342-88b7-4692-a3f7-a3f56c0cf6b5",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// VIS 67 — Pillar Tombs of Aku
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILLAR_TOMBS_OF_AKU: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Pillar Tombs of Aku",
    "153f93fd-4f2c-4dce-a774-4483031ed532",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 68 — Python
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYTHON: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Python",
    "e7e99969-6c21-4de6-ba57-44ef7f9c8c47",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 69 — Suq'Ata Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUQ_ATA_ASSASSIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Suq'Ata Assassin",
    "1b7178c6-f989-437d-83e3-04b9817f2c54",
    "Gary Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 70 — Tar Pit Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAR_PIT_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Tar Pit Warrior",
    "e1283190-094e-4a9f-bf67-f9fd05778744",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 71 — Urborg Mindsucker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_MINDSUCKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Urborg Mindsucker",
    "78405864-fc83-47ab-9238-8e0464a700ec",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// VIS 72 — Vampiric Tutor
/// Imperial Seal prints this clause word for word, so the two share it: the
/// only difference between the cards is that one is an instant.
pub(in crate::card::sets) static VAMPIRIC_TUTOR_EFFECT: [EffectDef; 2] = [
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::Any,
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Library,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: false,
        attachment: None,
        binding: None,
        then: None,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

pub(in crate::card::sets) static VAMPIRIC_TUTOR: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Vampiric Tutor",
    "0a07cba3-2e8d-48ec-a6f8-4d2edfcd833d",
    "Gary Leach",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&VAMPIRIC_TUTOR_EFFECT),
    )),
);

// VIS 73 — Vampirism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRISM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Vampirism",
    "2dff2817-1813-410f-aca7-96e8f9f4ce81",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// VIS 74 — Wake of Vultures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAKE_OF_VULTURES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Wake of Vultures",
    "52420b80-7f34-4426-ac97-a6e15167c7a9",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 75 — Wicked Reward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICKED_REWARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Wicked Reward",
    "ee32f8ba-3547-4913-a555-d43ee2978ba9",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// VIS 76 — Bogardan Phoenix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGARDAN_PHOENIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Bogardan Phoenix",
    "253db28a-3873-4364-80d7-a8164000ea9e",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

// VIS 77 — Dwarven Vigilantes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_VIGILANTES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Dwarven Vigilantes",
    "077d33bb-41bf-440d-939b-67ab5aacb092",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 78 — Elkin Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_LAIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Elkin Lair",
    "bcb625ba-3718-4988-962c-bf2e11eb4c16",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// VIS 79 — Fireblast
pub(in crate::card::sets) static FIREBLAST: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Fireblast",
    "b1eb5b2c-1f02-48a6-a287-88eb189d6780",
    "Michael Danza",
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fireblast deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::alternative_cast(
            crate::mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may sacrifice two Mountains rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // Two Mountains off the battlefield, which is why the card is a finisher
        // rather than a burn spell: it is cast from an empty board on the turn the
        // lands stop mattering.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::sacrifice(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
            CostQuantityDef::Fixed(2),
        )),
    ]),
);

// VIS 80 — Goblin Recruiter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_RECRUITER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Goblin Recruiter",
    "6ee791d5-1d48-40e8-b65f-b6aa889f3467",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// VIS 81 — Goblin Swine-Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SWINE_RIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Goblin Swine-Rider",
    "49980982-d534-4204-bc15-3e6c4ffa1a53",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 82 — Hearth Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTH_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Hearth Charm",
    "caa9ac66-51b7-4aec-92dc-0f0656b0f7fe",
    "Greg Spalenka",
    crate::card::CardRules::unsupported(),
);

// VIS 83 — Heat Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAT_WAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Heat Wave",
    "42dd0810-4528-4a88-add8-923bb2057821",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 84 — Hulking Cyclops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULKING_CYCLOPS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Hulking Cyclops",
    "a3ee5ea8-7023-4dde-ab51-d3ba234d74b9",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// VIS 85 — Keeper of Kookus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_KOOKUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Keeper of Kookus",
    "d11b6df4-449f-44ea-a4fa-f079bcd26a54",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// VIS 86 — Kookus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOOKUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Kookus",
    "8fb90922-99d2-4b36-9039-bb806fd01756",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// VIS 87 — Lightning Cloud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_CLOUD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Lightning Cloud",
    "7fcfc2ad-a1a4-4f65-a239-f11383aaafe1",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// VIS 88 — Mob Mentality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOB_MENTALITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Mob Mentality",
    "e428d56a-9445-4e86-b281-656e2d251e0b",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// VIS 89 — Ogre Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_ENFORCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Ogre Enforcer",
    "b0f072d6-7489-4eb0-8c53-1fa42ad806a4",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 90 — Raging Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_GORILLA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Raging Gorilla",
    "07c284ce-33b8-4fb2-9dd9-4c477bedc774",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 91 — Relentless Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELENTLESS_ASSAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Relentless Assault",
    "747161ea-cb65-4960-84dd-a05bfe5f3ba0",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 92 — Rock Slide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_SLIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Rock Slide",
    "7e01717a-d6ed-42c1-9a9a-f3f4a3d73bca",
    "Mike Kerr",
    crate::card::CardRules::unsupported(),
);

// VIS 93 — Solfatara
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLFATARA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Solfatara",
    "c5d4bd6f-b019-4594-aa41-138fa58ba529",
    "Omaha Pérez",
    crate::card::CardRules::unsupported(),
);

// VIS 94 — Song of Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONG_OF_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Song of Blood",
    "4497a1d7-6604-4f2d-9484-1f1d77a6228f",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// VIS 95 — Spitting Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Spitting Drake",
    "c9f6ef97-587f-4f7b-98a2-e3cc8b39df8b",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 96 — Suq'Ata Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUQ_ATA_LANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Suq'Ata Lancer",
    "2884d8df-7fd5-4247-9da5-38c31333ff5d",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 97 — Talruum Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALRUUM_CHAMPION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Talruum Champion",
    "33730a07-754c-4606-bfac-d73454af9567",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 98 — Talruum Piper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALRUUM_PIPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Talruum Piper",
    "ca2cb9a7-5063-4b31-9782-8bfd784bca0a",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 99 — Tremor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Tremor",
    "a9d64665-c1e0-40ab-a358-247f82966379",
    "Michael Danza",
    crate::card::CardRules::unsupported(),
);

// VIS 100 — Viashino Sandstalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_SANDSTALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Viashino Sandstalker",
    "01770e13-ebd4-4c83-9e72-99374239a63d",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// VIS 101 — Bull Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULL_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Bull Elephant",
    "fa7f5f41-ed30-412b-b51e-37d26e9e6455",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 102 — City of Solitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITY_OF_SOLITUDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "City of Solitude",
    "be499b81-bb2d-4f1d-9deb-c8bfcdca8e13",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 103 — Creeping Mold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CREEPING_MOLD: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Creeping Mold",
    "36e7691f-c771-4451-ac54-3532ca10d48f",
    "David Seeley",
    crate::card::CardRules::unsupported(),
);

// VIS 104 — Elephant Grass
static ELEPHANT_GRASS_BLACK_CREATURES: ObjectPredicateDef =
    ObjectPredicateDef::Color(ManaColor::Black);

pub(in crate::card::sets) static ELEPHANT_GRASS: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Elephant Grass",
    "f4c1f5a7-0d28-43ab-9b66-937e963f42cd",
    "Tony Roberts",
    CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceOnBattlefield,
                then: &EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("age"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::PayOr(PayOrDef::unless(
                        EffectPaymentDef::generic_mana(
                            PlayerSetDef::One(PlayerRefDef::EffectController),
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                        ),
                        &EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                    )),
                ]),
            },
        ),
        AbilityDef::static_ability(
            "Black creatures can't attack you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::prohibit(
                        ELEPHANT_GRASS_BLACK_CREATURES,
                        AttackDefenderScopeDef::AffectedPlayer,
                    ),
                )),
            },
        ),
        AbilityDef::static_ability(
            "Nonblack creatures can't attack you unless their controller pays {2} for each creature they control that's attacking you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::unless_paid(
                        ObjectPredicateDef::Not(&ELEPHANT_GRASS_BLACK_CREATURES),
                        AttackDefenderScopeDef::AffectedPlayer,
                        mana_cost!("{2}"),
                    ),
                )),
            },
        ),
    ]),
);

// VIS 105 — Elven Cache
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_CACHE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Elven Cache",
    "80fa078f-c74a-42b2-af97-7ca2c29dc316",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// VIS 106 — Emerald Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMERALD_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Emerald Charm",
    "e9c9199b-61b3-4794-878b-f065058f50f3",
    "Greg Spalenka",
    crate::card::CardRules::unsupported(),
);

// VIS 107 — Feral Instinct
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERAL_INSTINCT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Feral Instinct",
    "20dec7cf-2865-4642-9022-d3006fd7ac30",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// VIS 108 — Giant Caterpillar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_CATERPILLAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Giant Caterpillar",
    "b7f602a6-3d35-49a3-b5cb-d754e03a9573",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// VIS 109 — Katabatic Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KATABATIC_WINDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Katabatic Winds",
    "97b34ce8-1eb2-44eb-813a-09d0308e27a0",
    "Gary Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 110 — King Cheetah
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KING_CHEETAH: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "King Cheetah",
    "38149d49-8661-427c-9338-93c11a2a8093",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 111 — Kyscu Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYSCU_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Kyscu Drake",
    "b6f14bbe-2436-4a5a-8e2a-8066b740b715",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 112 — Lichenthrope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LICHENTHROPE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Lichenthrope",
    "76f0c356-a81d-41d4-a8b7-8c159146a8b8",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// VIS 113 — Mortal Wound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTAL_WOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Mortal Wound",
    "808830ff-496a-41dc-8b64-334ddaca9435",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// VIS 114 — Natural Order
pub(in crate::card::sets) static NATURAL_ORDER: CardRecord = CardRecord::new(
    CardSet::Visions,
    "Natural Order",
    "0845f0b0-9413-4ddd-861d-9607636bebc6",
    "Terese Nielsen",
    // Four mana and a Llanowar Elves for whatever the deck is built around.
    // The search is mandatory and the sacrifice is a cost, so the card is a
    // dead draw exactly when the board is empty.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a green creature.\nSearch your \
             library for a green creature card, put it onto the battlefield, then shuffle.",
            &[],
            // Paid as the spell is cast, so a board with nothing green on it cannot
            // cast this at all.
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// VIS 115 — Panther Warriors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANTHER_WARRIORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Panther Warriors",
    "76c9bc99-28e3-4d64-8383-2b92011104ed",
    "Cecil Fernando",
    crate::card::CardRules::unsupported(),
);

// VIS 116 — Quirion Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Quirion Druid",
    "8ca5319a-5c26-487f-ba87-d317633122ba",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// VIS 117 — Quirion Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_RANGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Quirion Ranger",
    "56efe72c-6d7f-44f6-ac74-01af9305c4b6",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 118 — River Boa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_BOA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "River Boa",
    "2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 119 — Rowen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Rowen",
    "07144d84-f7f3-4101-805d-07cce8342a64",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// VIS 120 — Spider Climb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_CLIMB: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Spider Climb",
    "a1818812-4cb8-4fe1-98c0-b40086b4991c",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// VIS 121 — Stampeding Wildebeests
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDING_WILDEBEESTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Stampeding Wildebeests",
    "ddb5f524-fad6-4a63-b20f-3348a844fefa",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// VIS 122 — Summer Bloom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUMMER_BLOOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Summer Bloom",
    "35d78f4e-d95d-49bc-9971-06a68a4e35fd",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// VIS 123 — Uktabi Orangutan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UKTABI_ORANGUTAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Uktabi Orangutan",
    "101c7d58-43cc-4ebd-87f1-2016fbff56dd",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// VIS 124 — Warthog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARTHOG: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Warthog",
    "dd2510b8-52d6-4d2e-89a5-31b27b732dd8",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 125 — Wind Shear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIND_SHEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Wind Shear",
    "b8324f44-c7f5-41ee-bc8d-16822bd8942f",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// VIS 126 — Army Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMY_ANTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Army Ants",
    "7e129be5-e2c5-4f69-b8e8-539ac2085c7a",
    "Geofrey Darrow & I. Rabarot",
    crate::card::CardRules::unsupported(),
);

// VIS 127 — Breathstealer's Crypt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATHSTEALER_S_CRYPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Breathstealer's Crypt",
    "f87ace53-d77c-4df5-b200-4be2ac2b7fdb",
    "Blackie del Rio",
    crate::card::CardRules::unsupported(),
);

// VIS 128 — Corrosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORROSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Corrosion",
    "176122b2-f60f-4150-8c0c-757c8f8914d2",
    "Michael Danza",
    crate::card::CardRules::unsupported(),
);

// VIS 129 — Femeref Enchantress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEMEREF_ENCHANTRESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Femeref Enchantress",
    "20ba72c7-7957-4d02-b41e-c0132fe1f2e6",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// VIS 130 — Firestorm Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESTORM_HELLKITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Firestorm Hellkite",
    "def23574-4a41-4323-84d9-49f58b2ca322",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 131 — Guiding Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDING_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Guiding Spirit",
    "5f96d184-0ef8-40f7-98bc-bd4c53c57072",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 132 — Mundungu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUNDUNGU: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Mundungu",
    "d6e320ca-848b-4743-93f1-ec04ef1ce402",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 133 — Pygmy Hippo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_HIPPO: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Pygmy Hippo",
    "2e3f6220-6ead-46b4-8663-57609ef5a12e",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 134 — Righteous War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Righteous War",
    "bbcacb8e-1aff-4807-b70c-a17d6703d279",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// VIS 135 — Scalebane's Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALEBANE_S_ELITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Scalebane's Elite",
    "b3bff610-783a-46b7-bd15-061da41027bb",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// VIS 136 — Simoon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIMOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Simoon",
    "642d9239-82e0-4696-ad99-10796042d1f8",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// VIS 137 — Squandered Resources
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUANDERED_RESOURCES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Squandered Resources",
    "fcddbea7-3025-47b1-a597-2d2b2711fb81",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 138 — Suleiman's Legacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULEIMAN_S_LEGACY: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Suleiman's Legacy",
    "3a15e970-e605-425a-b4ec-391d9cacde38",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// VIS 139 — Tempest Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPEST_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Tempest Drake",
    "54aa5262-d0d9-4b4a-8027-00393568b3df",
    "Gerry Grace",
    crate::card::CardRules::unsupported(),
);

// VIS 140 — Viashivan Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHIVAN_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Viashivan Dragon",
    "7172ef0b-ca9e-47cf-8ec6-2d8cb18f2283",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// VIS 141 — Anvil of Bogardan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANVIL_OF_BOGARDAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Anvil of Bogardan",
    "7ff965dd-54b4-4f21-a52f-81c0dd1e691e",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// VIS 142 — Brass-Talon Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_TALON_CHIMERA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Brass-Talon Chimera",
    "200c9655-e51c-4b63-96cf-7f3fba3ec75c",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// VIS 143 — Diamond Kaleidoscope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIAMOND_KALEIDOSCOPE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Diamond Kaleidoscope",
    "548ff852-274d-4068-818d-58a883e74a5f",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// VIS 144 — Dragon Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Dragon Mask",
    "f098e329-adc8-42dd-b779-d00d9ccc3dbd",
    "Craig Hooper",
    crate::card::CardRules::unsupported(),
);

// VIS 145 — Helm of Awakening
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELM_OF_AWAKENING: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Helm of Awakening",
    "41bba882-39b8-42db-9a01-54c6712b8019",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 146 — Iron-Heart Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_HEART_CHIMERA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Iron-Heart Chimera",
    "5899a575-a97d-4850-b55c-22ad6900ba20",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// VIS 147 — Juju Bubble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUJU_BUBBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Juju Bubble",
    "a5fa8208-7d65-4f8f-b07e-f5c3a66e1143",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// VIS 148 — Lead-Belly Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_BELLY_CHIMERA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Lead-Belly Chimera",
    "5d89b377-80d2-42a0-b84e-a455a72ed9fe",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// VIS 149 — Magma Mine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_MINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Magma Mine",
    "1aecc3df-7ce6-419c-b3d6-60fc28bfe941",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// VIS 150 — Matopi Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MATOPI_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Matopi Golem",
    "92378d6f-89ee-49dc-8964-0e9c55daeffc",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 151 — Phyrexian Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_MARAUDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Phyrexian Marauder",
    "29a75dc8-1c24-4063-8944-d7e71b4a5755",
    "David Seeley",
    crate::card::CardRules::unsupported(),
);

// VIS 152 — Phyrexian Walker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_WALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Phyrexian Walker",
    "9f8a3979-2947-4692-8b2f-d4c07c534777",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// VIS 153 — Sands of Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDS_OF_TIME: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Sands of Time",
    "a782ee95-bde4-41f4-a947-b073cc4c1e7c",
    "Paul Lee",
    crate::card::CardRules::unsupported(),
);

// VIS 154 — Sisay's Ring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SISAY_S_RING: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Sisay's Ring",
    "a08becd3-ca5e-4150-8d28-52436a3eaffd",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// VIS 155 — Snake Basket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAKE_BASKET: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Snake Basket",
    "bfda9a16-9cdb-494a-b662-ac24e3b89d0c",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// VIS 156 — Teferi's Puzzle Box
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_PUZZLE_BOX: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Teferi's Puzzle Box",
    "1377dab4-b814-46cc-a097-24a3cf8d0f8f",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// VIS 157 — Tin-Wing Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIN_WING_CHIMERA: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Tin-Wing Chimera",
    "3375dcc6-9399-48eb-9aa4-7b40c3686cc5",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// VIS 158 — Triangle of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIANGLE_OF_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Triangle of War",
    "4c1d7d4d-bed7-4d28-a304-ad33f42e9831",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// VIS 159 — Wand of Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAND_OF_DENIAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Wand of Denial",
    "0b1c856f-6d29-4bfc-976e-7875d60abd52",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// VIS 160 — Coral Atoll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORAL_ATOLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Coral Atoll",
    "5d7c4619-e5af-4aa0-bd3f-6bf0e1fdc1fc",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// VIS 161 — Dormant Volcano
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DORMANT_VOLCANO: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Dormant Volcano",
    "6aa92be7-883f-42bd-8623-00eb2df28a98",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// VIS 162 — Everglades
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERGLADES: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Everglades",
    "c1f2eaf7-7f08-446b-892f-5a844f74808f",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// VIS 163 — Griffin Canyon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIFFIN_CANYON: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Griffin Canyon",
    "705d8194-3ad0-41b7-ae32-9c0cd8cd46b9",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// VIS 164 — Jungle Basin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNGLE_BASIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Jungle Basin",
    "cc3146db-2f86-4728-9af1-ff651f871652",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// VIS 165 — Karoo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAROO: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Karoo",
    "d786815c-53ec-483e-ad56-382778a57b1a",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// VIS 166 — Quicksand
pub(in crate::card::sets) static QUICKSAND: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Quicksand",
    "11370658-8d80-4d2f-afa5-ec6df6dee369",
    "Roger Raupp",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Target attacking creature without flying gets -1/-2 until end of turn.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// VIS 167 — Undiscovered Paradise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDISCOVERED_PARADISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Visions,
    "Undiscovered Paradise",
    "5f6e8830-5e62-4945-8b73-60f0628d38e7",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGEL,
    &DARAJA_GRIFFIN,
    &EQUIPOISE,
    &EYE_OF_SINGULARITY,
    &FREEWIND_FALCON,
    &GOSSAMER_CHAINS,
    &HONORABLE_PASSAGE,
    &HOPE_CHARM,
    &INFANTRY_VETERAN,
    &JAMURAAN_LION,
    &KNIGHT_OF_VALOR,
    &LONGBOW_ARCHER,
    &MIRACULOUS_RECOVERY,
    &PARAPET,
    &PEACE_TALKS,
    &RELIC_WARD,
    &REMEDY,
    &RESISTANCE_FIGHTER,
    &RETRIBUTION_OF_THE_MEEK,
    &RIGHTEOUS_AURA,
    &SUN_CLASP,
    &TEFERI_S_HONOR_GUARD,
    &TITHE,
    &WARRIOR_S_HONOR,
    &ZHALFIRIN_CRUSADER,
    &BETRAYAL,
    &BREEZEKEEPER,
    &CHRONATOG,
    &CLOUD_ELEMENTAL,
    &DESERTION,
    &DREAM_TIDES,
    &FLOODED_SHORELINE,
    &FORESHADOW,
    &IMPULSE,
    &INSPIRATION,
    &KNIGHT_OF_THE_MISTS,
    &MAN_O_WAR,
    &MYSTIC_VEIL,
    &OVINOMANCER,
    &PROSPERITY,
    &RAINBOW_EFREET,
    &SHIMMERING_EFREET,
    &SHRIEKING_DRAKE,
    &TEFERI_S_REALM,
    &THREE_WISHES,
    &TIME_AND_TIDE,
    &UNDO,
    &VANISHING,
    &VISION_CHARM,
    &WATERSPOUT_DJINN,
    &AKU_DJINN,
    &BLANKET_OF_NIGHT,
    &BROOD_OF_COCKROACHES,
    &COERCION,
    &CRYPT_RATS,
    &DARK_PRIVILEGE,
    &DEATH_WATCH,
    &DESOLATION,
    &FALLEN_ASKARI,
    &FORBIDDEN_RITUAL,
    &FUNERAL_CHARM,
    &INFERNAL_HARVEST,
    &KAERVEK_S_SPITE,
    &NECROMANCY,
    &NECROSAVANT,
    &NEKRATAAL,
    &PILLAR_TOMBS_OF_AKU,
    &PYTHON,
    &SUQ_ATA_ASSASSIN,
    &TAR_PIT_WARRIOR,
    &URBORG_MINDSUCKER,
    &VAMPIRIC_TUTOR,
    &VAMPIRISM,
    &WAKE_OF_VULTURES,
    &WICKED_REWARD,
    &BOGARDAN_PHOENIX,
    &DWARVEN_VIGILANTES,
    &ELKIN_LAIR,
    &FIREBLAST,
    &GOBLIN_RECRUITER,
    &GOBLIN_SWINE_RIDER,
    &HEARTH_CHARM,
    &HEAT_WAVE,
    &HULKING_CYCLOPS,
    &KEEPER_OF_KOOKUS,
    &KOOKUS,
    &LIGHTNING_CLOUD,
    &MOB_MENTALITY,
    &OGRE_ENFORCER,
    &RAGING_GORILLA,
    &RELENTLESS_ASSAULT,
    &ROCK_SLIDE,
    &SOLFATARA,
    &SONG_OF_BLOOD,
    &SPITTING_DRAKE,
    &SUQ_ATA_LANCER,
    &TALRUUM_CHAMPION,
    &TALRUUM_PIPER,
    &TREMOR,
    &VIASHINO_SANDSTALKER,
    &BULL_ELEPHANT,
    &CITY_OF_SOLITUDE,
    &CREEPING_MOLD,
    &ELEPHANT_GRASS,
    &ELVEN_CACHE,
    &EMERALD_CHARM,
    &FERAL_INSTINCT,
    &GIANT_CATERPILLAR,
    &KATABATIC_WINDS,
    &KING_CHEETAH,
    &KYSCU_DRAKE,
    &LICHENTHROPE,
    &MORTAL_WOUND,
    &NATURAL_ORDER,
    &PANTHER_WARRIORS,
    &QUIRION_DRUID,
    &QUIRION_RANGER,
    &RIVER_BOA,
    &ROWEN,
    &SPIDER_CLIMB,
    &STAMPEDING_WILDEBEESTS,
    &SUMMER_BLOOM,
    &UKTABI_ORANGUTAN,
    &WARTHOG,
    &WIND_SHEAR,
    &ARMY_ANTS,
    &BREATHSTEALER_S_CRYPT,
    &CORROSION,
    &FEMEREF_ENCHANTRESS,
    &FIRESTORM_HELLKITE,
    &GUIDING_SPIRIT,
    &MUNDUNGU,
    &PYGMY_HIPPO,
    &RIGHTEOUS_WAR,
    &SCALEBANE_S_ELITE,
    &SIMOON,
    &SQUANDERED_RESOURCES,
    &SULEIMAN_S_LEGACY,
    &TEMPEST_DRAKE,
    &VIASHIVAN_DRAGON,
    &ANVIL_OF_BOGARDAN,
    &BRASS_TALON_CHIMERA,
    &DIAMOND_KALEIDOSCOPE,
    &DRAGON_MASK,
    &HELM_OF_AWAKENING,
    &IRON_HEART_CHIMERA,
    &JUJU_BUBBLE,
    &LEAD_BELLY_CHIMERA,
    &MAGMA_MINE,
    &MATOPI_GOLEM,
    &PHYREXIAN_MARAUDER,
    &PHYREXIAN_WALKER,
    &SANDS_OF_TIME,
    &SISAY_S_RING,
    &SNAKE_BASKET,
    &TEFERI_S_PUZZLE_BOX,
    &TIN_WING_CHIMERA,
    &TRIANGLE_OF_WAR,
    &WAND_OF_DENIAL,
    &CORAL_ATOLL,
    &DORMANT_VOLCANO,
    &EVERGLADES,
    &GRIFFIN_CANYON,
    &JUNGLE_BASIN,
    &KAROO,
    &QUICKSAND,
    &UNDISCOVERED_PARADISE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
