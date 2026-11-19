//! Star Trek card inventory.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::abilities;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    CardRules, CardSupertype, CardType, ComparisonDef, CostDef, CounterKind, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, ResolvedEffectDurationDef, StackObjectEventDef,
    StackObjectEventMatcherDef, SubtypeDef, TriggerConditionDef, TriggerEventDef,
    ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TRK",
    slug: "star-trek",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TRK 1 — 70,000 Light-Years from Home
// Audit: unsupported — Needs an executable declaration for Exile target creature or Spacecraft. Its controller may
// search their library for a basic land card, reveal it, put it into their hand, then shuffle.
pub(in crate::card::sets) static SEVENTY_THOUSAND_LIGHT_YEARS_FROM_HOME: CardRecord =
    CardRecord::new(
        "70,000 Light-Years from Home",
        "02080f42-863a-471b-992b-67e35ef1b7b7",
        "Ariel Orea",
        CardRules::unsupported(),
    );

// TRK 2 — Andromedan Automatons
// Audit: unsupported — Needs an executable declaration for When this creature enters, you may search your library for
// any number of cards named Andromedan Automatons, reveal them, put them into your hand, then
// shuffle.
pub(in crate::card::sets) static ANDROMEDAN_AUTOMATONS: CardRecord = CardRecord::new(
    "Andromedan Automatons",
    "fc386662-0fe3-432f-85a8-9a06e1274245",
    "Uzuri",
    CardRules::unsupported(),
);

// TRK 3 — Antagonist Incorporator
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control have "Whenever this creature
// deals combat damage to a player, you may pay {1}. If you do, create a 1/1 colorless Borg
// artifact creature token."
pub(in crate::card::sets) static ANTAGONIST_INCORPORATOR: CardRecord = CardRecord::new(
    "Antagonist Incorporator",
    "63918e59-0c03-4d98-9216-e58ee157b7ea",
    "Miklós Ligeti",
    CardRules::unsupported(),
);

// TRK 4 — Assembly Specialist
// Audit: unsupported — Needs an executable declaration for When this creature enters, create a 1/1 colorless Borg
// artifact creature token.
pub(in crate::card::sets) static ASSEMBLY_SPECIALIST: CardRecord = CardRecord::new(
    "Assembly Specialist",
    "a4d339a5-198c-49bd-845e-2632476746e0",
    "Sean Vo",
    CardRules::unsupported(),
);

// TRK 5 — Brad Boimler, Eager Ensign
// Audit: unsupported — Needs an executable declaration for Lifelink Whenever Brad Boimler becomes tapped, until end of
// turn, if one or more counters would be put on a permanent you control, that many plus one of
// each of those kinds of counters are put on that permanent instead.
pub(in crate::card::sets) static BRAD_BOIMLER_EAGER_ENSIGN: CardRecord = CardRecord::new(
    "Brad Boimler, Eager Ensign",
    "a6bf1525-2212-46d0-ad4d-1dbaa2e3b3cd",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// TRK 6 — Captain Benjamin Sisko
// Audit: unsupported — Needs an executable declaration for At the beginning of your end step, create a 1/1 red Officer
// creature token for each dilemma you've faced this turn. {2}{W}{U}{B}{R}{G}, {T}: Exile the
// top five cards of your library. You may cast any number of spells with mana value less than
// or equal to the number of Officers you control from among those cards without paying their
// mana costs.
pub(in crate::card::sets) static CAPTAIN_BENJAMIN_SISKO: CardRecord = CardRecord::new(
    "Captain Benjamin Sisko",
    "51338fb2-13a3-41b8-981f-05fa35e87e2e",
    "Chris Rallis",
    CardRules::unsupported(),
);

// TRK 7 — Captain Jean-Luc Picard
// Audit: unsupported — Needs an executable declaration for Other creatures you control get +1/+1. Whenever you cast a
// creature or Spacecraft spell, create a 1/1 red Officer creature token. Tap seven untapped
// Officers you control: Search your library for a Spacecraft card, put it onto the battlefield
// with seven charge counters on it, then shuffle.
pub(in crate::card::sets) static CAPTAIN_JEAN_LUC_PICARD: CardRecord = CardRecord::new(
    "Captain Jean-Luc Picard",
    "b7114271-eb05-41e2-ad96-a34f53492312",
    "Anna Pavleeva",
    CardRules::unsupported(),
);

// TRK 8 — Command Decision
// Audit: unsupported — Needs an executable declaration for Choose one — • Destroy target creature with power 4 or
// greater. • Draw a card.
pub(in crate::card::sets) static COMMAND_DECISION: CardRecord = CardRecord::new(
    "Command Decision",
    "9f1f789b-5c08-4a4d-b3a3-4d51ad9f1a4d",
    "Raymond Bonilla",
    CardRules::unsupported(),
);

// TRK 9 — Command Subadjunct
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control have vigilance.
pub(in crate::card::sets) static COMMAND_SUBADJUNCT: CardRecord = CardRecord::new(
    "Command Subadjunct",
    "853bf74d-0b61-43df-9018-0b40b624a9d1",
    "Alex Negrea",
    CardRules::unsupported(),
);

// TRK 10 — Conquest Mission
// Audit: unsupported — Needs an executable declaration for Whenever a nontoken artifact you control enters, put a
// quest counter on this enchantment. Create a 1/1 colorless Borg artifact creature token. If
// this enchantment has six or more quest counters on it, create two of those tokens instead.
pub(in crate::card::sets) static CONQUEST_MISSION: CardRecord = CardRecord::new(
    "Conquest Mission",
    "f91755d7-6fb9-4caf-b4b4-f21ed5572d52",
    "Mark Spears",
    CardRules::unsupported(),
);

// TRK 11 — Deanna Troi, Psychic Counsel
// Audit: unsupported — Needs an executable declaration for Other creatures you control enter with an additional +1/+1
// counter on them.
pub(in crate::card::sets) static DEANNA_TROI_PSYCHIC_COUNSEL: CardRecord = CardRecord::new(
    "Deanna Troi, Psychic Counsel",
    "1a027761-84e7-4cbf-a629-ae9bd3d8467b",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// TRK 12 — DOT-7 Repair Squad
// Audit: unsupported — Needs an executable declaration for Flying {1}{W}: Untap target creature. Activate only as a
// sorcery and only once each turn.
pub(in crate::card::sets) static DOT_7_REPAIR_SQUAD: CardRecord = CardRecord::new(
    "DOT-7 Repair Squad",
    "fca1528c-84cc-42d2-99ae-9f10f0e28743",
    "Colin Searle",
    CardRules::unsupported(),
);

// TRK 13 — Dr. Beverly Crusher
// Audit: unsupported — Needs an executable declaration for If you gaining life causes a triggered ability of a
// permanent you control to trigger, that ability triggers an additional time. Whenever you
// gain life, put a +1/+1 counter on Dr. Crusher.
pub(in crate::card::sets) static DR_BEVERLY_CRUSHER: CardRecord = CardRecord::new(
    "Dr. Beverly Crusher",
    "ef9c5790-d909-4271-90bd-5fa4816ddd6b",
    "Mattia Rangoni",
    CardRules::unsupported(),
);

// TRK 14 — Dr. McCoy, Surly Surgeon
// Audit: unsupported — Needs an executable declaration for When Dr. McCoy enters, put a +1/+1 counter on another
// target creature you control. If another creature you control with a +1/+1 counter on it
// would be destroyed, instead remove all +1/+1 counters from it and all damage already dealt
// to it is healed.
pub(in crate::card::sets) static DR_MCCOY_SURLY_SURGEON: CardRecord = CardRecord::new(
    "Dr. McCoy, Surly Surgeon",
    "b300d814-cf74-44ba-b253-ec179ec54a20",
    "David Robert Hovey",
    CardRules::unsupported(),
);

// TRK 15 — Emergency Medical Hologram
// Audit: unsupported — Needs an executable declaration for When this creature enters, put a +1/+1 counter on target
// creature. You gain 1 life.
pub(in crate::card::sets) static EMERGENCY_MEDICAL_HOLOGRAM: CardRecord = CardRecord::new(
    "Emergency Medical Hologram",
    "dda6f965-cbdf-4dbc-b0b1-00c27e32f5f2",
    "Kirk DouPonce",
    CardRules::unsupported(),
);

// TRK 16 — Excalbian Gamemaster
// Audit: unsupported — Needs an executable declaration for Strange new worlds — When this creature enters, create X
// 1/1 red Officer creature tokens, where X is the number of differently named lands you
// control. Whenever this creature or another creature you control dies, put a +1/+1 counter on
// target creature you control.
pub(in crate::card::sets) static EXCALBIAN_GAMEMASTER: CardRecord = CardRecord::new(
    "Excalbian Gamemaster",
    "31b5780c-8c5f-4836-8848-1eea1d416a93",
    "Charles Chaisson",
    CardRules::unsupported(),
);

// TRK 17 — Federation Field Medic
// Audit: unsupported — Needs an executable declaration for Vigilance When this creature enters, choose one — • Put a
// +1/+1 counter on target creature. It gains vigilance until end of turn. • Draw a card if you
// control a permanent with a counter on it.
pub(in crate::card::sets) static FEDERATION_FIELD_MEDIC: CardRecord = CardRecord::new(
    "Federation Field Medic",
    "b312dacd-b87f-44c1-a804-bee221e70f52",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// TRK 18 — Federation Resupply
// Audit: unsupported — Needs an executable declaration for Search your library for an Aura, Equipment, or Spacecraft
// card, reveal it, put it into your hand, then shuffle. Beam me up {3}{W} (You may cast this
// card from your graveyard for {3}{W} if you also return a creature you control to its owner's
// hand. Then exile this spell.)
pub(in crate::card::sets) static FEDERATION_RESUPPLY: CardRecord = CardRecord::new(
    "Federation Resupply",
    "5626c58f-9245-437a-9844-b023320cf709",
    "Ryan Valle",
    CardRules::unsupported(),
);

// TRK 19 — Free Borg Revolutionaries
// Audit: unsupported — Needs an executable declaration for When this creature enters, creatures you control get +1/+1
// until end of turn. Basic landcycling {2} ({2}, Discard this card: Search your library for a
// basic land card, reveal it, put it into your hand, then shuffle.)
pub(in crate::card::sets) static FREE_BORG_REVOLUTIONARIES: CardRecord = CardRecord::new(
    "Free Borg Revolutionaries",
    "6711e910-e5a8-4602-9559-de5374bffff2",
    "Smirtouille",
    CardRules::unsupported(),
);

// TRK 20 — Ghost of Illyria
// Audit: unsupported — Needs an executable declaration for Flying When this creature enters, exile up to one other
// target nonland permanent you control. Return that card to the battlefield under its owner's
// control at the beginning of the next end step.
pub(in crate::card::sets) static GHOST_OF_ILLYRIA: CardRecord = CardRecord::new(
    "Ghost of Illyria",
    "99830058-c97a-4326-8c2f-8658cf8a1561",
    "Pace Wilder",
    CardRules::unsupported(),
);

// TRK 21 — Hologram Janeway
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, put a +1/+1 counter on target creature.
pub(in crate::card::sets) static HOLOGRAM_JANEWAY: CardRecord = CardRecord::new(
    "Hologram Janeway",
    "02d29047-a7b8-43fb-8b90-b35ba34da5f9",
    "Khadijah Khatib",
    CardRules::unsupported(),
);

// TRK 22 — Malfunctioning Holodeck
// Audit: unsupported — Needs an executable declaration for Ward {2} (Whenever this artifact becomes the target of a
// spell or ability an opponent controls, counter it unless that player pays {2}.) When this
// artifact enters, exile target nonland permanent an opponent controls until this artifact
// leaves the battlefield.
pub(in crate::card::sets) static MALFUNCTIONING_HOLODECK: CardRecord = CardRecord::new(
    "Malfunctioning Holodeck",
    "0faeffee-bb07-4b9d-8cdd-e9731bbd2396",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// TRK 23 — The Measure of a Man
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — Exile up to one target nontoken artifact or creature. II
// — Investigate. (Create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw
// a card.") III — Return the exiled card to the battlefield under its owner's control. You
// gain 2 life.
pub(in crate::card::sets) static THE_MEASURE_OF_A_MAN: CardRecord = CardRecord::new(
    "The Measure of a Man",
    "e9e6b01d-7eb5-4632-9d51-0012b58006da",
    "Lazare Gvimradze",
    CardRules::unsupported(),
);

// TRK 24 — Miles O'Brien, Transporter Ops
// Audit: unsupported — Needs an executable declaration for When Miles O'Brien enters, return up to one other target
// creature or artifact you control to its owner's hand. At the beginning of each end step, if
// a permanent was returned to your hand from the battlefield this turn, draw a card.
pub(in crate::card::sets) static MILES_O_BRIEN_TRANSPORTER_OPS: CardRecord = CardRecord::new(
    "Miles O'Brien, Transporter Ops",
    "e4f041b3-e785-45ba-9a7e-33878614d3b0",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// TRK 25 — The Needs of the Many
// Audit: unsupported — Needs an executable declaration for As an additional cost to cast this spell, sacrifice a
// creature. Creatures you control gain hexproof and indestructible until end of turn. You gain
// hexproof until end of turn. Prevent all damage that would be dealt to you this turn. Exile
// The Needs of the Many.
pub(in crate::card::sets) static THE_NEEDS_OF_THE_MANY: CardRecord = CardRecord::new(
    "The Needs of the Many",
    "c779279f-681b-4107-b62c-5202c544d3e8",
    "Carlos C Diaz",
    CardRules::unsupported(),
);

// TRK 26 — Oberth-Class Starship
// Audit: unsupported — Needs an executable declaration for When this Spacecraft enters, search your library for a
// basic Plains card, reveal it, put it into your hand, then shuffle. Station (Tap another
// creature you control: Put charge counters equal to its power on this Spacecraft. Station
// only as a sorcery. It's an artifact creature at 6+.) 6+ | Flying
pub(in crate::card::sets) static OBERTH_CLASS_STARSHIP: CardRecord = CardRecord::new(
    "Oberth-Class Starship",
    "7efe3788-ecf7-4b56-8970-bc324092941c",
    "Jessica Rossier",
    CardRules::unsupported(),
);

// TRK 27 — Peace in Our Time
// Audit: unsupported — Needs an executable declaration for At the beginning of combat on each player's turn, that
// player may draw a card. If they do, they can't attack this turn and you put a peace counter
// on this enchantment. At the beginning of your upkeep, if this enchantment has twelve or more
// peace counters on it, you win the game.
pub(in crate::card::sets) static PEACE_IN_OUR_TIME: CardRecord = CardRecord::new(
    "Peace in Our Time",
    "9dd522dd-a894-496e-9d31-68583dba7ecc",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// TRK 28 — Pet Sehlat
// Audit: unsupported — Needs an executable declaration for This creature can't attack alone.
pub(in crate::card::sets) static PET_SEHLAT: CardRecord = CardRecord::new(
    "Pet Sehlat",
    "477a6a46-7fda-4666-87d9-a35c953dd94d",
    "Narendra Bintara Adi",
    CardRules::unsupported(),
);

// TRK 29 — Picard, Leading by Example
// Audit: unsupported — Needs an executable declaration for {1}, {T}: Another target creature gets +X/+X until end of
// turn, where X is Picard's power. Activate only as a sorcery.
pub(in crate::card::sets) static PICARD_LEADING_BY_EXAMPLE: CardRecord = CardRecord::new(
    "Picard, Leading by Example",
    "f9fa20c5-f241-4da2-9c99-c9b650095d74",
    "Julie Dillon",
    CardRules::unsupported(),
);

// TRK 30 — Primary Unimatrix Adjunct
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control have double strike.
pub(in crate::card::sets) static PRIMARY_UNIMATRIX_ADJUNCT: CardRecord = CardRecord::new(
    "Primary Unimatrix Adjunct",
    "b94a582c-11bb-4463-a826-bd0c79366752",
    "Jodie Muir",
    CardRules::unsupported(),
);

// TRK 31 — Pure Energy
// Audit: unsupported — Needs an executable declaration for Distribute two +1/+1 counters among one or two target
// creatures. Beam me up {2}{W} (You may cast this card from your graveyard for {2}{W} if you
// also return a creature you control to its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static PURE_ENERGY: CardRecord = CardRecord::new(
    "Pure Energy",
    "215709a8-044f-4fdb-b3ca-bd5a394b2a0f",
    "Evan Shipard",
    CardRules::unsupported(),
);

// TRK 32 — Rutherford, Upgraded Ensign
// Audit: unsupported — Needs an executable declaration for When Rutherford enters, look at the top six cards of your
// library. You may reveal an artifact card from among them and put it into your hand. Put the
// rest on the bottom of your library in a random order. Whenever you cast a Spacecraft spell,
// put a +1/+1 counter on target creature.
pub(in crate::card::sets) static RUTHERFORD_UPGRADED_ENSIGN: CardRecord = CardRecord::new(
    "Rutherford, Upgraded Ensign",
    "16a4d8ed-a918-4dfe-9976-0b68adb26afa",
    "Charles Chaisson",
    CardRules::unsupported(),
);

// TRK 33 — Saavik, Stoic Student
// Audit: unsupported — Needs an executable declaration for Flash Federation — When Saavik enters, she deals X damage
// to target attacking or blocking creature an opponent controls, where X is the number of
// creature types among non-Borg creatures you control.
pub(in crate::card::sets) static SAAVIK_STOIC_STUDENT: CardRecord = CardRecord::new(
    "Saavik, Stoic Student",
    "3783c530-a2da-4dc1-811a-0a9a412c8e78",
    "Mushk Rizvi",
    CardRules::unsupported(),
);

// TRK 34 — Set Phasers to . . .
// Audit: unsupported — Needs an executable declaration for Choose one — • Stun — Tap target creature and put a stun
// counter on it. (If a permanent with a stun counter would become untapped, remove one from it
// instead.) • Kill — This spell deals 3 damage to target attacking creature.
pub(in crate::card::sets) static SET_PHASERS_TO: CardRecord = CardRecord::new(
    "Set Phasers to . . .",
    "54dbf416-973d-46c8-b9be-e863283d328b",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// TRK 35 — Shields Up!
// Audit: unsupported — Needs an executable declaration for Target artifact or creature you control gains hexproof and
// indestructible until end of turn. If it's a creature, put a +1/+1 counter on it. (It can't
// be the target of spells or abilities your opponents control. Damage and effects that say
// "destroy" don't destroy it.)
pub(in crate::card::sets) static SHIELDS_UP: CardRecord = CardRecord::new(
    "Shields Up!",
    "0b60c9af-8ea9-491d-a4a4-226a2143e7f3",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// TRK 36 — Shuttle Ace
// Audit: unsupported — Needs an executable declaration for During your turn, this creature has flying. Whenever this
// creature becomes tapped, you gain 2 life.
pub(in crate::card::sets) static SHUTTLE_ACE: CardRecord = CardRecord::new(
    "Shuttle Ace",
    "559f838a-24ac-4f18-8c40-23cfde6f65cc",
    "Joan Piqué Llorens",
    CardRules::unsupported(),
);

// TRK 37 — Sulu, Space Swashbuckler
// Audit: unsupported — Needs an executable declaration for First strike Whenever Sulu becomes tapped, if an Equipment
// or Spacecraft entered the battlefield under your control this turn, draw a card.
pub(in crate::card::sets) static SULU_SPACE_SWASHBUCKLER: CardRecord = CardRecord::new(
    "Sulu, Space Swashbuckler",
    "9b16f967-4bc5-4508-8433-b46323ce2e71",
    "Micah Epstein",
    CardRules::unsupported(),
);

// TRK 38 — Tactical Officer
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, put a +1/+1 counter on this creature. Untap it.
pub(in crate::card::sets) static TACTICAL_OFFICER: CardRecord = CardRecord::new(
    "Tactical Officer",
    "317704a8-a3c2-400c-bbe0-2cbc6fe10710",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// TRK 39 — Test of Conviction
// Audit: unsupported — Needs an executable declaration for Choose one — • Battle Stations — Put a +1/+1 counter on
// each creature you control. • Begin Negotiations — Target opponent draws a card. You draw two
// cards.
pub(in crate::card::sets) static TEST_OF_CONVICTION: CardRecord = CardRecord::new(
    "Test of Conviction",
    "837d7cec-f3f8-4555-86f2-c5ea1849d7d1",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// TRK 40 — There's Coffee in that Nebula
// Audit: unsupported — Needs an executable declaration for Look at the top five cards of your library. You may reveal
// a Plains card or nonland permanent card with mana value 3 or less from among them and put it
// into your hand. Put the rest on the bottom in a random order. Beam me up {2}{W} (You may
// cast this card from your graveyard for {2}{W} if you also return a creature you control to
// its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static THERE_S_COFFEE_IN_THAT_NEBULA: CardRecord = CardRecord::new(
    "There's Coffee in that Nebula",
    "8dd7e0c6-84f6-4570-af93-4c64e78c2868",
    "Pauline Voss",
    CardRules::unsupported(),
);

// TRK 41 — Troubling Tribbles
// Audit: unsupported — Needs an executable declaration for This creature can't block. At the beginning of your upkeep,
// create a token that's a copy of this creature.
pub(in crate::card::sets) static TROUBLING_TRIBBLES: CardRecord = CardRecord::new(
    "Troubling Tribbles",
    "0af3e535-83e6-422b-85c5-39d830cb072c",
    "Filipe Pagliuso",
    CardRules::unsupported(),
);

// TRK 42 — Una Chin-Riley, Number One
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, target attacking creature gets +2/+0 and gains first strike until end of
// turn.
pub(in crate::card::sets) static UNA_CHIN_RILEY_NUMBER_ONE: CardRecord = CardRecord::new(
    "Una Chin-Riley, Number One",
    "5585c90d-3fe7-41f4-9728-36b9410ecc53",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// TRK 43 — U.S.S. Enterprise-A, Free Spirit
// Audit: unsupported — Needs an executable declaration for Ward {2} When U.S.S. Enterprise-A enters, create three 1/1
// red Officer creature tokens. Station (Tap another creature you control: Put charge counters
// equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact creature
// at 6+.) 6+ | Flying, lifelink
pub(in crate::card::sets) static U_S_S_ENTERPRISE_A_FREE_SPIRIT: CardRecord = CardRecord::new(
    "U.S.S. Enterprise-A, Free Spirit",
    "99429015-6e5d-4ecf-aaf7-eba181bcffc7",
    "Evan Shipard",
    CardRules::unsupported(),
);

// TRK 44 — U.S.S. Excelsior, Cutting Edge
// Audit: unsupported — Needs an executable declaration for When U.S.S. Excelsior enters, destroy target tapped
// creature an opponent controls. Station (Tap another creature you control: Put charge
// counters equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact
// creature at 8+.) 8+ | Flying
pub(in crate::card::sets) static U_S_S_EXCELSIOR_CUTTING_EDGE: CardRecord = CardRecord::new(
    "U.S.S. Excelsior, Cutting Edge",
    "de62b452-3780-4eeb-8163-f44176495615",
    "Jetfreak-7",
    CardRules::unsupported(),
);

// TRK 45 — Vexing Drone
// Audit: unsupported — Needs an executable declaration for When this creature enters, tap target creature an opponent
// controls.
pub(in crate::card::sets) static VEXING_DRONE: CardRecord = CardRecord::new(
    "Vexing Drone",
    "6f4b2476-8f93-4d4e-b664-233e82fcf212",
    "Kristina WermoongRey",
    CardRules::unsupported(),
);

// TRK 46 — Anthropological Mission
pub(in crate::card::sets) static ANTHROPOLOGICAL_MISSION: CardRecord = CardRecord::new(
    "Anthropological Mission",
    "93b458aa-a928-48ef-b685-1306cc65ad06",
    "Pablo Mendoza",
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever you cast a noncreature spell, put a quest counter on \
             this enchantment. Draw a card. Then discard a card unless this \
             enchantment has three or more quest counters on it.",
        TriggerEventDef::StackObject(StackObjectEventMatcherDef {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ]),
            event: StackObjectEventDef::Cast { from: None },
        }),
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("quest"),
                amount: ValueDef::Constant(1),
            },
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountersOnSource(CounterKind::named("quest")),
                    comparison: ComparisonDef::Less,
                    right: ValueDef::Constant(3),
                }),
                then: &EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            },
        ]),
    )]),
);

// TRK 47 — The Caretaker
// Audit: unsupported — Needs an executable declaration for Vigilance, ward {2} At the beginning of each end step, if
// you faced a dilemma this turn, target opponent exiles cards from the top of their library
// until they exile a nonland card. You may cast that card without paying its mana cost. (You
// face a dilemma as you choose one or more modes for a spell or ability.)
pub(in crate::card::sets) static THE_CARETAKER: CardRecord = CardRecord::new(
    "The Caretaker",
    "ec6a1967-78fc-4ef4-9816-6d9a9dde61fb",
    "Sam Kennedy",
    CardRules::unsupported(),
);

// TRK 48 — Cetacean Ops
// Audit: unsupported — Needs an executable declaration for Ward {1} (Whenever this creature becomes the target of a
// spell or ability an opponent controls, counter it unless that player pays {1}.) Whenever you
// cast a noncreature spell, put a +1/+1 counter on this creature.
pub(in crate::card::sets) static CETACEAN_OPS: CardRecord = CardRecord::new(
    "Cetacean Ops",
    "a8dc7ecd-e6f0-4050-9d5b-8436f60a1395",
    "Arthur Haas",
    CardRules::unsupported(),
);

// TRK 49 — Cloak and Phaser
// Audit: unsupported — Needs an executable declaration for Strange new worlds — This spell costs {X} less to cast,
// where X is the number of differently named lands you control. Create two 2/2 black Romulan
// creature tokens. Draw two cards, then discard a card.
pub(in crate::card::sets) static CLOAK_AND_PHASER: CardRecord = CardRecord::new(
    "Cloak and Phaser",
    "2d9fb7e7-582d-41e7-be9e-98372505ea44",
    "David Szabo",
    CardRules::unsupported(),
);

// TRK 50 — Cloistered Telepath
// Audit: unsupported — Needs an executable declaration for Flash When this creature enters, draw a card, then discard
// a card.
pub(in crate::card::sets) static CLOISTERED_TELEPATH: CardRecord = CardRecord::new(
    "Cloistered Telepath",
    "959318c1-3f7e-4418-91ce-be529af27972",
    "Monztre",
    CardRules::unsupported(),
);

// TRK 51 — Consider the Prime Directive
// Audit: unsupported — Needs an executable declaration for Choose one — • Interfere — Counter target spell unless its
// controller pays {3}. • Observe — Tap up to one target creature and put a stun counter on it.
// Draw a card. (If a permanent with a stun counter would become untapped, remove one from it
// instead.)
pub(in crate::card::sets) static CONSIDER_THE_PRIME_DIRECTIVE: CardRecord = CardRecord::new(
    "Consider the Prime Directive",
    "4e8f75b8-cca7-4ed3-90db-bcac02e74611",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// TRK 52 — Data, New Life
// Audit: unsupported — Needs an executable declaration for Whenever you draw a card, put a +1/+1 counter on Data.
pub(in crate::card::sets) static DATA_NEW_LIFE: CardRecord = CardRecord::new(
    "Data, New Life",
    "96b7e757-c64d-48c8-bd9c-caad5f3c78fb",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// TRK 53 — Deep Space Scan
// Audit: unsupported — Needs an executable declaration for Strange new worlds — Draw X cards, where X is the number of
// differently named lands you control. You have no maximum hand size until your next turn.
pub(in crate::card::sets) static DEEP_SPACE_SCAN: CardRecord = CardRecord::new(
    "Deep Space Scan",
    "15ffeda3-28ac-4adf-9844-d7b051aae227",
    "Andreia Ugrai",
    CardRules::unsupported(),
);

// TRK 54 — Dominion Saboteur
// Audit: unsupported — Needs an executable declaration for You may have this creature enter as a copy of any artifact
// or creature on the battlefield, except it isn't legendary. If you do, it enters with
// additional counters on it equal to the same number and kinds of counters the copied
// permanent has on it.
pub(in crate::card::sets) static DOMINION_SABOTEUR: CardRecord = CardRecord::new(
    "Dominion Saboteur",
    "207c4369-220a-4eff-81dc-65872d6fc264",
    "Alex Negrea",
    CardRules::unsupported(),
);

// TRK 55 — Emergency Holographic Crew
// Audit: unsupported — Needs an executable declaration for When this creature enters, choose one — • Return target
// Spacecraft card from your graveyard to your hand. • Surveil 2. (Look at the top two cards of
// your library, then put any number of them into your graveyard and the rest on top of your
// library in any order.)
pub(in crate::card::sets) static EMERGENCY_HOLOGRAPHIC_CREW: CardRecord = CardRecord::new(
    "Emergency Holographic Crew",
    "53943948-8f24-495a-b6eb-e788fa169fcd",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// TRK 56 — Energize
// Audit: unsupported — Needs an executable declaration for Return target creature an opponent controls to its owner's
// hand. Beam me up {3}{U} (You may cast this card from your graveyard for {3}{U} if you also
// return a creature you control to its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static ENERGIZE: CardRecord = CardRecord::new(
    "Energize",
    "24e72db3-a8d9-4553-92fe-ba6e40c4f62e",
    "Carlos C Diaz",
    CardRules::unsupported(),
);

// TRK 57 — Ensign Pavel Chekov
// Audit: unsupported — Needs an executable declaration for Strange new worlds — When Ensign Chekov enters, each
// opponent exiles the top X cards of their library, where X is the number of differently named
// lands you control. For each opponent, choose a land card that player owns exiled this way.
// You may play the chosen cards for as long as they remain exiled.
pub(in crate::card::sets) static ENSIGN_PAVEL_CHEKOV: CardRecord = CardRecord::new(
    "Ensign Pavel Chekov",
    "a441c12e-1a2f-4b04-83dd-407bf6040efe",
    "Gal Or",
    CardRules::unsupported(),
);

// TRK 58 — Enterprise NX-01
// Audit: unsupported — Needs an executable declaration for When Enterprise NX-01 enters, tap target creature an
// opponent controls and put a stun counter on it. (If a permanent with a stun counter would
// become untapped, remove one from it instead.) Station (Tap another creature you control: Put
// charge counters equal to its power on this Spacecraft. Station only as a sorcery. It's an
// artifact creature at 6+.) 6+ | Flying, vigilance
pub(in crate::card::sets) static ENTERPRISE_NX_01: CardRecord = CardRecord::new(
    "Enterprise NX-01",
    "385638fe-cafb-4541-ba37-19bc57520c48",
    "Colin Searle",
    CardRules::unsupported(),
);

// TRK 59 — Federation Probe
// Audit: unsupported — Needs an executable declaration for Flying When this creature enters, choose one — • Target
// creature gets +1/+1 until end of turn. • Surveil 1. (Look at the top card of your library.
// You may put it into your graveyard.)
pub(in crate::card::sets) static FEDERATION_PROBE: CardRecord = CardRecord::new(
    "Federation Probe",
    "f988ba84-fc78-41a2-b2f2-eebf78cee591",
    "Racrufi",
    CardRules::unsupported(),
);

// TRK 60 — Guidance Failure
// Audit: unsupported — Needs an executable declaration for This spell costs {1} less to cast if it targets an
// attacking creature. Target creature or Spacecraft's owner puts it on their choice of the top
// or bottom of their library.
pub(in crate::card::sets) static GUIDANCE_FAILURE: CardRecord = CardRecord::new(
    "Guidance Failure",
    "86b790e6-3c99-4a95-8327-eb90ed9380f2",
    "David Álvarez",
    CardRules::unsupported(),
);

// TRK 61 — Gwyndala, the Progeny
// Audit: unsupported — Needs an executable declaration for When Gwyndala enters, create a colorless Equipment artifact
// token named Neuroflux Heirloom with "Equipped creature gets +1/+1" and equip {2}. Attach it
// to Gwyndala. {2}: Each Equipment token you control becomes a copy of target Equipment you
// control until end of turn.
pub(in crate::card::sets) static GWYNDALA_THE_PROGENY: CardRecord = CardRecord::new(
    "Gwyndala, the Progeny",
    "a82f5db5-787e-4e4a-ab3c-4a488c6f4990",
    "Pierre Raveneau",
    CardRules::unsupported(),
);

// TRK 62 — Highly Illogical
// Audit: unsupported — Needs an executable declaration for Kicker {1}{U} (You may pay an additional {1}{U} as you cast
// this spell.) Counter target spell with mana value 2 or less. If this spell was kicked,
// instead counter target spell.
pub(in crate::card::sets) static HIGHLY_ILLOGICAL: CardRecord = CardRecord::new(
    "Highly Illogical",
    "11c94c5f-abf4-429e-8703-c5db9abd79bf",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 63 — H.M.S. Bounty, Unlikely Timeship
// Audit: unsupported — Needs an executable declaration for When H.M.S. Bounty enters, you may search your library for
// a Whale card, reveal it, put it into your hand, then shuffle. Station 8+ | Flying Whenever
// H.M.S. Bounty deals combat damage to a player, draw a card. If you control a Whale and
// haven't begun an extra turn this game, take an extra turn after this one.
pub(in crate::card::sets) static H_M_S_BOUNTY_UNLIKELY_TIMESHIP: CardRecord = CardRecord::new(
    "H.M.S. Bounty, Unlikely Timeship",
    "d75f0991-39a3-474e-8c01-9b3363c41a9f",
    "Jetfreak-7",
    CardRules::unsupported(),
);

// TRK 64 — Hoshi Sato, Exolinguist
// Audit: unsupported — Needs an executable declaration for Federation — When Hoshi Sato enters, look at the top X
// cards of your library, where X is the number of creature types among non-Borg creatures you
// control. Put one of them into your hand and the rest on the bottom of your library in a
// random order.
pub(in crate::card::sets) static HOSHI_SATO_EXOLINGUIST: CardRecord = CardRecord::new(
    "Hoshi Sato, Exolinguist",
    "24589f4d-c530-4f6a-b2d0-450ef70fca71",
    "Jason Seiler",
    CardRules::unsupported(),
);

// TRK 65 — Humpback Whales
// Audit: unsupported — Needs an executable declaration for Vigilance, ward {2} (Whenever this creature becomes the
// target of a spell or ability an opponent controls, counter it unless that player pays {2}.)
// Basic landcycling {2} ({2}, Discard this card: Search your library for a basic land card,
// reveal it, put it into your hand, then shuffle.)
pub(in crate::card::sets) static HUMPBACK_WHALES: CardRecord = CardRecord::new(
    "Humpback Whales",
    "605263f0-08b6-4cc7-aba7-eb49850f8832",
    "Evan Shipard",
    CardRules::unsupported(),
);

// TRK 66 — Interstellar Effigy
// Audit: unsupported — Needs an executable declaration for When this creature enters, return target creature an
// opponent controls to its owner's hand.
pub(in crate::card::sets) static INTERSTELLAR_EFFIGY: CardRecord = CardRecord::new(
    "Interstellar Effigy",
    "4b7d505f-2a6b-4525-87c7-19e06d49c9df",
    "Cliff Nielsen",
    CardRules::unsupported(),
);

// TRK 67 — Jett Reno, Problem Solver
// Audit: unsupported — Needs an executable declaration for Whenever Jett Reno becomes tapped, surveil 1. If her power
// is 4 or greater, instead draw a card. Whenever you face a dilemma, put a +1/+1 counter on
// Jett Reno. (You face a dilemma as you choose one or more modes for a spell or ability.)
pub(in crate::card::sets) static JETT_RENO_PROBLEM_SOLVER: CardRecord = CardRecord::new(
    "Jett Reno, Problem Solver",
    "6dc34d44-397a-4dda-a8e3-0c2ff7da110f",
    "Riccardo Federici",
    CardRules::unsupported(),
);

// TRK 68 — Kobayashi Maru Scenario
// Audit: unsupported — Needs an executable declaration for At the beginning of your upkeep, choose one that hasn't
// been chosen — • Hail — Return up to one target creature an opponent controls with mana value
// 3 or less to its owner's hand. • Sensors — Draw two cards. • Transport — Create two 1/1 red
// Officer creature tokens. • Abandon Ship — You lose the game.
pub(in crate::card::sets) static KOBAYASHI_MARU_SCENARIO: CardRecord = CardRecord::new(
    "Kobayashi Maru Scenario",
    "c7fb72cc-1bc6-4774-9798-44824de46c72",
    "Kevin Glint",
    CardRules::unsupported(),
);

// TRK 69 — Lieutenant Commander Data
// Audit: unsupported — Needs an executable declaration for Whenever a player casts a noncreature spell, put a +1/+1
// counter on Lieutenant Commander Data. {1}, Remove three counters from among creatures you
// control: Draw a card.
pub(in crate::card::sets) static LIEUTENANT_COMMANDER_DATA: CardRecord = CardRecord::new(
    "Lieutenant Commander Data",
    "c6d5da51-6c74-4dd5-a532-cf4c2a7b601c",
    "Irvin Rodriguez",
    CardRules::unsupported(),
);

// TRK 70 — Lt. Cmdr. Geordi La Forge
// Audit: unsupported — Needs an executable declaration for If you would put one or more charge counters on an artifact
// you control, put twice that many charge counters on that artifact instead.
pub(in crate::card::sets) static LT_CMDR_GEORDI_LA_FORGE: CardRecord = CardRecord::new(
    "Lt. Cmdr. Geordi La Forge",
    "3a558d82-7efd-49b3-9f7b-ace639b44a35",
    "Julie Dillon",
    CardRules::unsupported(),
);

// TRK 71 — Make It So
// Audit: unsupported — Needs an executable declaration for Choose one — • Untap target permanent you control. Target
// player draws two cards. • Create a 1/1 red Officer creature token and draw two cards.
pub(in crate::card::sets) static MAKE_IT_SO: CardRecord = CardRecord::new(
    "Make It So",
    "7149df19-4833-47ca-8add-b897f55b930c",
    "Javier Charro",
    CardRules::unsupported(),
);

// TRK 72 — Marooned
// Audit: unsupported — Needs an executable declaration for Enchant artifact or creature When this Aura enters, you may
// pay {3}. If you do, tap enchanted permanent. Enchanted permanent doesn't untap during its
// controller's untap step and can't be beamed up.
pub(in crate::card::sets) static MAROONED: CardRecord = CardRecord::new(
    "Marooned",
    "0820a78e-e392-423c-9d62-ddfda5cb3f03",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 73 — Melkotian Adjudicator
// Audit: unsupported — Needs an executable declaration for Flying, vigilance Whenever you face a dilemma, put a +1/+1
// counter on this creature. (You face a dilemma as you choose one or more modes for a spell or
// ability.)
pub(in crate::card::sets) static MELKOTIAN_ADJUDICATOR: CardRecord = CardRecord::new(
    "Melkotian Adjudicator",
    "873c9933-7939-4376-873c-6a5b3bbc98d4",
    "Jason Raish",
    CardRules::unsupported(),
);

// TRK 74 — Mr. Spock, Science Officer
// Audit: unsupported — Needs an executable declaration for If you would draw a card, draw two cards instead.
pub(in crate::card::sets) static MR_SPOCK_SCIENCE_OFFICER: CardRecord = CardRecord::new(
    "Mr. Spock, Science Officer",
    "d19fc127-669e-4264-935a-e5991e6c0964",
    "Colin Boyer",
    CardRules::unsupported(),
);

// TRK 75 — Nyota Uhura, Communications
// Audit: unsupported — Needs an executable declaration for Hailing Frequencies — As Nyota Uhura enters, choose a
// number between 0 and 10. Open a Channel — Whenever an opponent casts a spell with mana value
// equal to the chosen number, you draw a card.
pub(in crate::card::sets) static NYOTA_UHURA_COMMUNICATIONS: CardRecord = CardRecord::new(
    "Nyota Uhura, Communications",
    "b04066ef-ad43-4b40-b4ae-5d94f154261f",
    "Charles Chaisson",
    CardRules::unsupported(),
);

// TRK 76 — Open Communications
// Audit: unsupported — Needs an executable declaration for Draw a card. Beam me up {2}{U} (You may cast this card from
// your graveyard for {2}{U} if you also return a creature you control to its owner's hand.
// Then exile this spell.)
pub(in crate::card::sets) static OPEN_COMMUNICATIONS: CardRecord = CardRecord::new(
    "Open Communications",
    "8eeec269-13f0-4923-9f13-e995acd73e00",
    "David Robert Hovey",
    CardRules::unsupported(),
);

// TRK 77 — Romulan Bird-of-Prey
// Audit: unsupported — Needs an executable declaration for Cloaking Device — As long as this Spacecraft isn't a
// creature, it has hexproof. Whenever you cast a noncreature spell, if this Spacecraft isn't a
// creature, put two charge counters on it. Station 8+ | Flying
pub(in crate::card::sets) static ROMULAN_BIRD_OF_PREY: CardRecord = CardRecord::new(
    "Romulan Bird-of-Prey",
    "6a461d90-6b7b-4777-958a-8a5d8365ad8c",
    "Edouard",
    CardRules::unsupported(),
);

// TRK 78 — Rules of Acquisition
// Audit: unsupported — Needs an executable declaration for Choose one — • Follow the 3rd Rule — Gain control of target
// nonland permanent an opponent controls. That opponent draws three cards. • Follow the 98th
// Rule — Target opponent gains control of target nonland permanent you control. If they do,
// you draw three cards.
pub(in crate::card::sets) static RULES_OF_ACQUISITION: CardRecord = CardRecord::new(
    "Rules of Acquisition",
    "307c36ec-6fa1-4d4b-afcd-c3c83b876ee8",
    "Racrufi",
    CardRules::unsupported(),
);

// TRK 79 — Scotty, Miracle Worker
// Audit: unsupported — Needs an executable declaration for During your main phase, each card in your graveyard has
// beam me up. The beam me up cost is equal to that spell's mana cost. Scotty can't be beamed
// up to cast a spell this way. (Only instant and sorcery spells can be cast this way. You may
// cast a card from your graveyard for its beam me up cost if you also return another creature
// you control to its owner's hand. Then exile that spell.)
pub(in crate::card::sets) static SCOTTY_MIRACLE_WORKER: CardRecord = CardRecord::new(
    "Scotty, Miracle Worker",
    "42f77f9e-5e56-45ac-aa93-36353c92bca9",
    "Sam Kennedy",
    CardRules::unsupported(),
);

// TRK 80 — Seven of Nine, Unassimilated
// Audit: unsupported — Needs an executable declaration for At the beginning of your first main phase, draw a card.
// Then discard a card unless you control five or more differently named lands. {6}{U}: Seven
// of Nine's base power and toughness become 7/9. Activate only if you control seven or more
// differently named lands.
pub(in crate::card::sets) static SEVEN_OF_NINE_UNASSIMILATED: CardRecord = CardRecord::new(
    "Seven of Nine, Unassimilated",
    "ddc517bd-27e1-4688-8941-a41c312bd782",
    "Toraji",
    CardRules::unsupported(),
);

// TRK 81 — Starfleet Spanner Jockey
// Audit: unsupported — Needs an executable declaration for Defender As long as you've cast a noncreature spell this
// turn, this creature can attack as though it didn't have defender.
pub(in crate::card::sets) static STARFLEET_SPANNER_JOCKEY: CardRecord = CardRecord::new(
    "Starfleet Spanner Jockey",
    "16bc9660-2bef-4926-b0e6-f49b22817bea",
    "Chris Rallis",
    CardRules::unsupported(),
);

// TRK 82 — Subcommander T'Pol
// Audit: unsupported — Needs an executable declaration for Flash When Subcommander T'Pol enters, choose one — •
// Counter target activated or triggered ability. • Tap up to one target artifact or creature.
// Put a stun counter on it. (If a permanent with a stun counter would become untapped, remove
// one from it instead.)
pub(in crate::card::sets) static SUBCOMMANDER_T_POL: CardRecord = CardRecord::new(
    "Subcommander T'Pol",
    "7d90016d-c2cb-457b-9d97-c3b9411c46e3",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// TRK 83 — Suurok-Class Science Vessel
// Audit: unsupported — Needs an executable declaration for When this Spacecraft enters, draw a card. Station (Tap
// another creature you control: Put charge counters equal to its power on this Spacecraft.
// Station only as a sorcery. It's an artifact creature at 5+.) 5+ | Flying
pub(in crate::card::sets) static SUUROK_CLASS_SCIENCE_VESSEL: CardRecord = CardRecord::new(
    "Suurok-Class Science Vessel",
    "7608943e-329e-41f9-b098-a0a94b01e6a1",
    "Pace Wilder",
    CardRules::unsupported(),
);

// TRK 84 — Suus Mahna Practitioner
// Audit: unsupported — Needs an executable declaration for Strange new worlds — {4}{U}, {T}: Tap target creature. This
// ability costs {X} less to activate, where X is the number of differently named lands you
// control.
pub(in crate::card::sets) static SUUS_MAHNA_PRACTITIONER: CardRecord = CardRecord::new(
    "Suus Mahna Practitioner",
    "cb204e41-06a9-4ad2-81f2-a4d62f771cfa",
    "Lucas Durham",
    CardRules::unsupported(),
);

// TRK 85 — Test of Priorities
// Audit: unsupported — Needs an executable declaration for Choose one — • Power to Engines — Surveil 2, then draw a
// card. • Power to Life Support — Exile target creature you control. Return that card to the
// battlefield under its owner's control at the beginning of the next end step.
pub(in crate::card::sets) static TEST_OF_PRIORITIES: CardRecord = CardRecord::new(
    "Test of Priorities",
    "89e3ec23-e4dc-4d58-aeda-9f73ed4504a5",
    "Paolo Puggioni",
    CardRules::unsupported(),
);

// TRK 86 — Those Old Scientists
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — Draw two cards, then discard two cards. II — Return up to
// one target artifact, instant, or sorcery card from your graveyard to your hand. III — Choose
// up to two target cards in graveyards. The owners of those cards shuffle them into their
// libraries.
pub(in crate::card::sets) static THOSE_OLD_SCIENTISTS: CardRecord = CardRecord::new(
    "Those Old Scientists",
    "44280b43-b07e-43a4-9ae3-2eac0baeb918",
    "Gregg Schigiel",
    CardRules::unsupported(),
);

// TRK 87 — V'Ger, the Intruder
// Audit: unsupported — Needs an executable declaration for Flying Whenever V'Ger enters or attacks, choose one — •
// Look at target opponent's hand, then draw a card. • Creatures your opponents control get
// -1/-0 until your next turn.
pub(in crate::card::sets) static V_GER_THE_INTRUDER: CardRecord = CardRecord::new(
    "V'Ger, the Intruder",
    "b082d454-d331-4960-8120-754f107ffa42",
    "Jessica Rossier",
    CardRules::unsupported(),
);

// TRK 88 — Vulcan Mind-Meld
// Audit: unsupported — Needs an executable declaration for Target opponent looks at the top card of your library, then
// exiles it face down. You look at the top two cards of their library, then exile them face
// down. For each card exiled this way, the player who exiled it may play it for as long as it
// remains exiled. Mana of any type can be spent to cast spells this way.
pub(in crate::card::sets) static VULCAN_MIND_MELD: CardRecord = CardRecord::new(
    "Vulcan Mind-Meld",
    "dd6c0c12-01b8-4f68-9401-7430a7f94bf7",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// TRK 89 — Wesley Crusher, Starfleet Cadet
// Audit: unsupported — Needs an executable declaration for Flash When Wesley Crusher enters, choose two — • Counter
// target noncreature spell unless its controller pays {2}. • Return target creature with mana
// value 2 or less to its owner's hand. • Target creature can't be blocked this turn. • Target
// player draws two cards, then discards two cards.
pub(in crate::card::sets) static WESLEY_CRUSHER_STARFLEET_CADET: CardRecord = CardRecord::new(
    "Wesley Crusher, Starfleet Cadet",
    "956eeb71-fd5f-4a26-aff6-77127c0ad4e7",
    "Nia Kovalevski",
    CardRules::unsupported(),
);

// TRK 90 — Zefram Cochrane, Warp Pioneer
// Audit: unsupported — Needs an executable declaration for You may look at the top card of your library any time. Once
// during each of your turns, you may cast an artifact spell from the top of your library.
// Spacecraft you control enter with two additional charge counters on them.
pub(in crate::card::sets) static ZEFRAM_COCHRANE_WARP_PIONEER: CardRecord = CardRecord::new(
    "Zefram Cochrane, Warp Pioneer",
    "43ce2e91-9dce-4e86-a4b8-8d21e4e368ea",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// TRK 91 — Armus, Skin of Evil
// Audit: unsupported — Needs an executable declaration for Menace At the beginning of your end step, target opponent
// discards two cards. {2}{B}, Discard this card: Target creature gets -4/-4 until end of turn.
pub(in crate::card::sets) static ARMUS_SKIN_OF_EVIL: CardRecord = CardRecord::new(
    "Armus, Skin of Evil",
    "c3ba1e0d-5e3d-4fa7-9b58-5b90944b43c5",
    "Camille Alquier",
    CardRules::unsupported(),
);

// TRK 92 — Assault Drone
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control get +1/+0.
pub(in crate::card::sets) static ASSAULT_DRONE: CardRecord = CardRecord::new(
    "Assault Drone",
    "5ca2da7a-afbf-4b32-8ff6-ae899b9f70ad",
    "Filipe Pagliuso",
    CardRules::unsupported(),
);

// TRK 93 — Automated Warfare System
// Audit: unsupported — Needs an executable declaration for Flying Whenever this creature attacks, you may sacrifice
// another creature or artifact. If you do, draw a card.
pub(in crate::card::sets) static AUTOMATED_WARFARE_SYSTEM: CardRecord = CardRecord::new(
    "Automated Warfare System",
    "3deae89b-b207-487b-9e82-1fa7fcd7854b",
    "Pierre Raveneau",
    CardRules::unsupported(),
);

// TRK 94 — Bat'leth
// Audit: unsupported — Needs an executable declaration for When this Equipment enters, target creature an opponent
// controls gets -2/-2 until end of turn. Equipped creature gets +2/+0. Equip {2} ({2}: Attach
// to target creature you control. Equip only as a sorcery.)
pub(in crate::card::sets) static BAT_LETH: CardRecord = CardRecord::new(
    "Bat'leth",
    "888e6e0f-43c3-4516-88ff-8a8d814478d4",
    "Tony Foti",
    CardRules::unsupported(),
);

// TRK 95 — Battle-Scarred Survivalist
// Audit: unsupported — Needs an executable declaration for Menace (This creature can't be blocked except by two or
// more creatures.) Whenever another creature dies, put a +1/+1 counter on this creature. This
// ability triggers only once each turn.
pub(in crate::card::sets) static BATTLE_SCARRED_SURVIVALIST: CardRecord = CardRecord::new(
    "Battle-Scarred Survivalist",
    "c567253e-a2ee-45de-b3c6-3aeafbf8237b",
    "John Di Giovanni",
    CardRules::unsupported(),
);

// TRK 96 — Blastoneuron
// Audit: unsupported — Needs an executable declaration for Flying When this creature enters, exile target creature
// card with mana value 3 or less from your graveyard. Create a token that's a copy of it. That
// token gains haste. Sacrifice it at the beginning of the next end step. Whenever one or more
// cards leave your graveyard, each opponent loses 1 life and you gain 1 life.
pub(in crate::card::sets) static BLASTONEURON: CardRecord = CardRecord::new(
    "Blastoneuron",
    "c1855a4c-c7d1-407c-b134-4c43968cbdd3",
    "Bastien L. Deharme",
    CardRules::unsupported(),
);

// TRK 97 — Cha'DIch Investigator
// Audit: unsupported — Needs an executable declaration for When this creature enters, create a 2/1 red Klingon Warrior
// creature token with haste.
pub(in crate::card::sets) static CHA_DICH_INVESTIGATOR: CardRecord = CardRecord::new(
    "Cha'DIch Investigator",
    "16281200-4660-4871-afd2-d4720e6a3197",
    "Javier Charro",
    CardRules::unsupported(),
);

// TRK 98 — Collation Trimatrix
// Audit: unsupported — Needs an executable declaration for Tap three untapped artifact creatures you control: You draw
// a card and lose 1 life.
pub(in crate::card::sets) static COLLATION_TRIMATRIX: CardRecord = CardRecord::new(
    "Collation Trimatrix",
    "92c19351-0592-4269-90f4-2475d5529400",
    "Bryan Sola",
    CardRules::unsupported(),
);

// TRK 99 — Collective Drone
// Audit: unsupported — Needs an executable declaration for Collective Drone's power is equal to the number of artifact
// creatures you control.
pub(in crate::card::sets) static COLLECTIVE_DRONE: CardRecord = CardRecord::new(
    "Collective Drone",
    "65b74daa-4606-4118-9752-b0dcd7a3f486",
    "José Parodi",
    CardRules::unsupported(),
);

// TRK 100 — Commander Raffi Musiker
// Audit: unsupported — Needs an executable declaration for When Commander Musiker enters, choose one — • Exile up to
// one target card from a graveyard. You gain 2 life. • Investigate. You lose 2 life. (Create a
// Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a card.")
pub(in crate::card::sets) static COMMANDER_RAFFI_MUSIKER: CardRecord = CardRecord::new(
    "Commander Raffi Musiker",
    "7e60bd56-3da1-4211-b790-320471eef242",
    "Runa I. Rosenberger",
    CardRules::unsupported(),
);

// TRK 101 — D'Ghor Bloodletter
// Audit: unsupported — Needs an executable declaration for When this creature enters, choose one — • All other
// creatures get -1/-1 until end of turn. • Each player sacrifices a creature of their choice.
pub(in crate::card::sets) static D_GHOR_BLOODLETTER: CardRecord = CardRecord::new(
    "D'Ghor Bloodletter",
    "5075645d-ef69-4559-953e-c3140153bd75",
    "Patricia Pria",
    CardRules::unsupported(),
);

// TRK 102 — Death Is Irrelevant
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control have indestructible. At the
// beginning of your end step, assimilate target creature card from a graveyard. (Put it onto
// the battlefield under your control with a +1/+1 counter. It's a Borg artifact creature and
// loses all other creature types.)
pub(in crate::card::sets) static DEATH_IS_IRRELEVANT: CardRecord = CardRecord::new(
    "Death Is Irrelevant",
    "408cb5dd-1a79-47a9-98ea-41f3f4502c5e",
    "Narendra Bintara Adi",
    CardRules::unsupported(),
);

// TRK 103 — The Destruction of Praxis
// Audit: unsupported — Needs an executable declaration for Strange new worlds — All creatures get -X/-X until end of
// turn, where X is the number of differently named lands you control. If X is 6 or more,
// create two 2/1 red Klingon Warrior creature tokens with haste.
pub(in crate::card::sets) static THE_DESTRUCTION_OF_PRAXIS: CardRecord = CardRecord::new(
    "The Destruction of Praxis",
    "3d33ac2a-73a9-4804-bf9f-c8cf4eea1c76",
    "Pace Wilder",
    CardRules::unsupported(),
);

// TRK 104 — De-evolve
// Audit: unsupported — Needs an executable declaration for Enchant creature Enchanted creature gets -4/-0, can't
// block, and is a Salamander. (It loses all other creature types.) Threshold — At the
// beginning of each upkeep, if there are seven or more cards in your graveyard, exile
// enchanted creature.
pub(in crate::card::sets) static DE_EVOLVE: CardRecord = CardRecord::new(
    "De-evolve",
    "81d798f1-07dd-46b1-b0b3-411280a2e257",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// TRK 105 — Eject the Warp Core
// Audit: unsupported — Needs an executable declaration for Kicker—Sacrifice an artifact or creature. (You may
// sacrifice an artifact or creature in addition to any other costs as you cast this spell.)
// Target creature gets -2/-2 until end of turn. If this spell was kicked, that creature gets
// -6/-6 until end of turn instead.
pub(in crate::card::sets) static EJECT_THE_WARP_CORE: CardRecord = CardRecord::new(
    "Eject the Warp Core",
    "04a479be-8093-4775-a5d9-8b0e1683bf12",
    "Pierre Raveneau",
    CardRules::unsupported(),
);

// TRK 106 — Elim Garak, Simple Tailor
// Audit: unsupported — Needs an executable declaration for Menace Whenever you face a dilemma, each opponent loses 2
// life and you gain 2 life. (You face a dilemma as you choose one or more modes for a spell or
// ability.)
pub(in crate::card::sets) static ELIM_GARAK_SIMPLE_TAILOR: CardRecord = CardRecord::new(
    "Elim Garak, Simple Tailor",
    "113bd51a-dfbb-4106-adf3-e567057fcd06",
    "Alexander Gering",
    CardRules::unsupported(),
);

// TRK 107 — Fugitive Listener
pub(in crate::card::sets) static FUGITIVE_LISTENER: CardRecord = CardRecord::new(
    "Fugitive Listener",
    "bd04c738-b4e3-481e-9a68-d257524bf40e",
    "Bastien L. Deharme",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["El-Aurian", "Scout"], 3, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// TRK 108 — Gabriel Lorca, Mirror Menace
// Audit: unsupported — Needs an executable declaration for Menace Whenever you face a dilemma, you may pay {B}. If you
// do, return this card from your graveyard to your hand. (You face a dilemma as you choose one
// or more modes for a spell or ability.)
pub(in crate::card::sets) static GABRIEL_LORCA_MIRROR_MENACE: CardRecord = CardRecord::new(
    "Gabriel Lorca, Mirror Menace",
    "a98d0f01-8e5e-4ff1-88e5-684b8f4c015c",
    "Kevin Glint",
    CardRules::unsupported(),
);

// TRK 109 — General Chang, Cold Warrior
// Audit: unsupported — Needs an executable declaration for {1}, Sacrifice a creature: Put a +1/+1 counter on each
// Klingon you control.
pub(in crate::card::sets) static GENERAL_CHANG_COLD_WARRIOR: CardRecord = CardRecord::new(
    "General Chang, Cold Warrior",
    "c46a0910-215c-4757-8bfe-7eca479157a5",
    "Cliff Nielsen",
    CardRules::unsupported(),
);

// TRK 110 — Georgiou, Terran Emperor
// Audit: unsupported — Needs an executable declaration for You don't lose the game for having 0 or less life. At the
// beginning of your end step, you lose 3 life and draw a card. Then if your life total is 0 or
// less, create ten 1/1 red Officer creature tokens.
pub(in crate::card::sets) static GEORGIOU_TERRAN_EMPEROR: CardRecord = CardRecord::new(
    "Georgiou, Terran Emperor",
    "a68361a3-83fa-44b5-92b5-b7905866fc99",
    "Julia Metzger",
    CardRules::unsupported(),
);

// TRK 111 — A Good Day to Die
// Audit: unsupported — Needs an executable declaration for Target creature gets +2/+0 until end of turn. When that
// creature dies this turn, you draw a card.
pub(in crate::card::sets) static A_GOOD_DAY_TO_DIE: CardRecord = CardRecord::new(
    "A Good Day to Die",
    "e2b350b2-0eda-4c2b-b9d6-b58ab4848545",
    "Monztre",
    CardRules::unsupported(),
);

// TRK 112 — Grand Nagus Zek
// Audit: unsupported — Needs an executable declaration for Grand Nagus Zek gets +1/+1 for each Treasure you control.
// Whenever another creature dies, create a Treasure token. This ability triggers only once
// each turn. (It's an artifact with "{T}, Sacrifice this token: Add one mana of any color.")
pub(in crate::card::sets) static GRAND_NAGUS_ZEK: CardRecord = CardRecord::new(
    "Grand Nagus Zek",
    "47488f1e-512d-4dc6-a82d-1ead24d3276a",
    "Wei Guan",
    CardRules::unsupported(),
);

// TRK 113 — Gul Dukat, Colonial Prefect
// Audit: unsupported — Needs an executable declaration for Menace At the beginning of each end step, each opponent
// loses life equal to the life that player lost this turn. You gain life equal to the life
// lost this way. (Damage causes loss of life.)
pub(in crate::card::sets) static GUL_DUKAT_COLONIAL_PREFECT: CardRecord = CardRecord::new(
    "Gul Dukat, Colonial Prefect",
    "7adc6373-1196-448f-8aee-cee7a890c724",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 114 — He's Dead, Jim
pub(in crate::card::sets) static HE_S_DEAD_JIM: CardRecord = CardRecord::new(
    "He's Dead, Jim",
    "2c309b0d-94b4-405e-be6c-18b407323e51",
    "Crystal Fae",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. Surveil 1. (Look at the top card of your \
             library. You may put it into your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::destroy_target(TargetIndex::PRIMARY),
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// TRK 115 — Hive Mind Coprocessor
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control have "Whenever this creature
// deals combat damage to a player, you may pay 1 life. If you do, draw a card."
pub(in crate::card::sets) static HIVE_MIND_COPROCESSOR: CardRecord = CardRecord::new(
    "Hive Mind Coprocessor",
    "a4734c59-ff41-47ac-bbb4-c8c3bf052fe6",
    "Andrey Kuzinskiy",
    CardRules::unsupported(),
);

// TRK 116 — Impossible Choice
// Audit: unsupported — Needs an executable declaration for Choose one — • Surgical Separation — Destroy target
// creature. Its controller creates two 1/1 red Officer creature tokens. • Remain Fused — You
// draw two cards and each player loses 2 life.
pub(in crate::card::sets) static IMPOSSIBLE_CHOICE: CardRecord = CardRecord::new(
    "Impossible Choice",
    "6c5a87cf-7790-4335-8497-ff793bbdcb05",
    "Douglas Shuler",
    CardRules::unsupported(),
);

// TRK 117 — In the Pale Moonlight
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — Each opponent sacrifices a nontoken creature of their
// choice. II — Investigate. (Create a Clue token. It's an artifact with "{2}, Sacrifice this
// token: Draw a card.") III — You may sacrifice an artifact or creature. If you do, create two
// 2/2 black Romulan creature tokens.
pub(in crate::card::sets) static IN_THE_PALE_MOONLIGHT: CardRecord = CardRecord::new(
    "In the Pale Moonlight",
    "47c8262c-e0c6-4c1a-b1c1-3b5fe5252a9c",
    "Charles Chaisson",
    CardRules::unsupported(),
);

// TRK 118 — Kai Winn, Faithless Leader
// Audit: unsupported — Needs an executable declaration for As Kai Winn enters, choose a nonland card name. Whenever an
// opponent casts a spell with the chosen name or activates an ability of a source with the
// chosen name, that player loses 3 life.
pub(in crate::card::sets) static KAI_WINN_FAITHLESS_LEADER: CardRecord = CardRecord::new(
    "Kai Winn, Faithless Leader",
    "973ab6cd-fc23-4816-aa08-f77c93e1b1d6",
    "Wei Guan",
    CardRules::unsupported(),
);

// TRK 119 — Khan, Vengeful Heart
// Audit: unsupported — Needs an executable declaration for When Khan enters, you may sacrifice another creature or
// artifact. If you do, draw two cards. You may cast this card from exile.
pub(in crate::card::sets) static KHAN_VENGEFUL_HEART: CardRecord = CardRecord::new(
    "Khan, Vengeful Heart",
    "a6f47b34-02fb-41d2-978a-a6d0a47ee8ba",
    "Sam Kennedy",
    CardRules::unsupported(),
);

// TRK 120 — Obsidian Order Operative
// Audit: unsupported — Needs an executable declaration for Whenever you face a dilemma, each opponent loses 1 life and
// you gain 1 life. (You face a dilemma as you choose one or more modes for a spell or
// ability.)
pub(in crate::card::sets) static OBSIDIAN_ORDER_OPERATIVE: CardRecord = CardRecord::new(
    "Obsidian Order Operative",
    "17a7b0a8-baaf-4e6a-8536-f511c796a713",
    "Wero Gallo",
    CardRules::unsupported(),
);

// TRK 121 — Organic Avulsion Unit
// Audit: unsupported — Needs an executable declaration for {2}: Artifact creatures you control gain menace until end
// of turn. (A creature with menace can't be blocked except by two or more creatures.)
pub(in crate::card::sets) static ORGANIC_AVULSION_UNIT: CardRecord = CardRecord::new(
    "Organic Avulsion Unit",
    "8cc06121-6ecd-44c3-90f9-34d492b031d4",
    "Andrey Kuzinskiy",
    CardRules::unsupported(),
);

// TRK 122 — Overflow Submatrix
// Audit: unsupported — Needs an executable declaration for Once during each of your turns, you may cast an artifact
// creature spell from your graveyard. If you cast a spell this way, that artifact creature
// enters with a finality counter on it. (If a creature with a finality counter on it would
// die, exile it instead.)
pub(in crate::card::sets) static OVERFLOW_SUBMATRIX: CardRecord = CardRecord::new(
    "Overflow Submatrix",
    "7e5654a7-ce83-4801-8978-7294a39dbef8",
    "Carlos Dattoli",
    CardRules::unsupported(),
);

// TRK 123 — Pattern Buffer Preservation
// Audit: unsupported — Needs an executable declaration for Look at the top two cards of your library. Put one of them
// into your hand and the other into your graveyard. Beam me up {1}{B} (You may cast this card
// from your graveyard for {1}{B} if you also return a creature you control to its owner's
// hand. Then exile this spell.)
pub(in crate::card::sets) static PATTERN_BUFFER_PRESERVATION: CardRecord = CardRecord::new(
    "Pattern Buffer Preservation",
    "23166580-9676-4025-916b-864568c2e59d",
    "Alex Negrea",
    CardRules::unsupported(),
);

// TRK 124 — Relentless Drednok
// Audit: unsupported — Needs an executable declaration for Lifelink {2}{B}, Exile this card from your graveyard: You
// draw a card and lose 1 life.
pub(in crate::card::sets) static RELENTLESS_DREDNOK: CardRecord = CardRecord::new(
    "Relentless Drednok",
    "ce538db8-373d-4554-821e-cd96e795e37a",
    "Edouard",
    CardRules::unsupported(),
);

// TRK 125 — Resistance Is Futile
// Audit: unsupported — Needs an executable declaration for Choose one — • Destroy target nonartifact creature. •
// Return target artifact creature card with mana value 3 or less from your graveyard to the
// battlefield.
pub(in crate::card::sets) static RESISTANCE_IS_FUTILE: CardRecord = CardRecord::new(
    "Resistance Is Futile",
    "81d88257-466d-4729-862f-591885b3d875",
    "Matt Stewart",
    CardRules::unsupported(),
);

// TRK 126 — Rogue Artificial Intelligence
// Audit: unsupported — Needs an executable declaration for When this creature enters, choose one — • You gain 3 life.
// • Surveil 2. (Look at the top two cards of your library, then put any number of them into
// your graveyard and the rest on top of your library in any order.) Basic landcycling {2}
// ({2}, Discard this card: Search your library for a basic land card, reveal it, put it into
// your hand, then shuffle.)
pub(in crate::card::sets) static ROGUE_ARTIFICIAL_INTELLIGENCE: CardRecord = CardRecord::new(
    "Rogue Artificial Intelligence",
    "edcce061-88c9-40bf-97f8-d0d3018e24dd",
    "José Parodi",
    CardRules::unsupported(),
);

// TRK 127 — Salt Vampire
// Audit: unsupported — Needs an executable declaration for Lifelink When this creature enters, look at target
// opponent's hand. You may choose a nonland card from it. If you do, that player exiles that
// card, then draws a card.
pub(in crate::card::sets) static SALT_VAMPIRE: CardRecord = CardRecord::new(
    "Salt Vampire",
    "78faba81-01a3-489f-927e-1adaa8fe4436",
    "Bastien L. Deharme",
    CardRules::unsupported(),
);

// TRK 128 — Sela, Tal Shiar Paradox
// Audit: unsupported — Needs an executable declaration for Whenever Sela attacks, exile up to one target card from a
// graveyard. Whenever one or more cards leave your graveyard, create a tapped 2/2 black
// Romulan creature token.
pub(in crate::card::sets) static SELA_TAL_SHIAR_PARADOX: CardRecord = CardRecord::new(
    "Sela, Tal Shiar Paradox",
    "cb680a44-3395-4c96-a73d-af1e0b7b8a9f",
    "Tony Foti",
    CardRules::unsupported(),
);

// TRK 129 — Storied Veteran
// Audit: unsupported — Needs an executable declaration for This creature can't block. You may cast this card from your
// graveyard. If you do, it enters with a finality counter on it. (If a creature with a
// finality counter on it would die, exile it instead.)
pub(in crate::card::sets) static STORIED_VETERAN: CardRecord = CardRecord::new(
    "Storied Veteran",
    "3b80995c-8e9a-4a9c-8854-59439a58166f",
    "Ana Teresa Rivera",
    CardRules::unsupported(),
);

// TRK 130 — Stray from History
// Audit: unsupported — Needs an executable declaration for Choose one — • Rewrite the Past — Return one or two target
// creature and/or Spacecraft cards from your graveyard to your hand. • Repair the Timeline —
// Target opponent reveals their hand. You choose a nonland card from it. Exile that card.
pub(in crate::card::sets) static STRAY_FROM_HISTORY: CardRecord = CardRecord::new(
    "Stray from History",
    "aac57a7f-a28b-486a-8d85-c620702a9cc0",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 131 — Tal Shiar Operative
// Audit: unsupported — Needs an executable declaration for Whenever this creature attacks, choose one — • Target
// creature you control gains deathtouch until end of turn. • Surveil 1. (Look at the top card
// of your library. You may put it into your graveyard.)
pub(in crate::card::sets) static TAL_SHIAR_OPERATIVE: CardRecord = CardRecord::new(
    "Tal Shiar Operative",
    "ebe9c7bd-5528-41fd-9c51-2306086c7887",
    "Irina Nordsol",
    CardRules::unsupported(),
);

// TRK 132 — Tell Me What You See
// Audit: unsupported — Needs an executable declaration for When this enchantment enters, target opponent may have you
// draw a card. If they don't, they lose 4 life and you surveil 1. Whenever you face a dilemma,
// you may return this enchantment to its owner's hand. (You face a dilemma as you choose one
// or more modes for a spell or ability.)
pub(in crate::card::sets) static TELL_ME_WHAT_YOU_SEE: CardRecord = CardRecord::new(
    "Tell Me What You See",
    "e339c8fb-26d2-4514-aeb7-014274df0c9c",
    "Douglas Shuler",
    CardRules::unsupported(),
);

// TRK 133 — Tenacious Drone
// Audit: unsupported — Needs an executable declaration for When this creature dies, create a 1/1 colorless Borg
// artifact creature token.
pub(in crate::card::sets) static TENACIOUS_DRONE: CardRecord = CardRecord::new(
    "Tenacious Drone",
    "206ae042-09af-4fc1-8441-69cecdd087de",
    "Anton Solovianchyk",
    CardRules::unsupported(),
);

// TRK 134 — Test of Loyalty
// Audit: unsupported — Needs an executable declaration for Choose one — • Plain, Simple Garak — Exile target card from
// a graveyard. Draw a card. • Of the Obsidian Order — Return target creature card from your
// graveyard to your hand.
pub(in crate::card::sets) static TEST_OF_LOYALTY: CardRecord = CardRecord::new(
    "Test of Loyalty",
    "722eeb21-be3c-4ae6-abd2-c3281d53c035",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// TRK 135 — U.S.S. Reliant, From Hell's Heart
// Audit: unsupported — Needs an executable declaration for Whenever U.S.S. Reliant enters or deals combat damage to a
// player, return target creature card with mana value 4 or less from your graveyard to the
// battlefield with a finality counter on it. Station (Tap another creature you control: Put
// charge counters equal to its power on this Spacecraft. Station only as a sorcery. It's an
// artifact creature at 10+.) 10+ | Flying
pub(in crate::card::sets) static U_S_S_RELIANT_FROM_HELL_S_HEART: CardRecord = CardRecord::new(
    "U.S.S. Reliant, From Hell's Heart",
    "a4c6fc55-f585-4529-b549-31d4d838c9af",
    "Dofresh",
    CardRules::unsupported(),
);

// TRK 136 — Vengeance Mission
// Audit: unsupported — Needs an executable declaration for Whenever a creature dies, put a quest counter on this
// enchantment. Target opponent loses 1 life. That player loses 2 life instead if there are
// seven or more quest counters on this enchantment.
pub(in crate::card::sets) static VENGEANCE_MISSION: CardRecord = CardRecord::new(
    "Vengeance Mission",
    "059e3331-43bc-43a8-90e9-86fef37e032a",
    "Daarken",
    CardRules::unsupported(),
);

// TRK 137 — Academy Instructor
// Audit: unsupported — Needs an executable declaration for This creature gets +1/+0 for each Officer token you
// control. Whenever you cast a noncreature spell, create a 1/1 red Officer creature token.
pub(in crate::card::sets) static ACADEMY_INSTRUCTOR: CardRecord = CardRecord::new(
    "Academy Instructor",
    "660ec8bc-dc03-4b42-84f3-091c19e2a853",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// TRK 138 — Beckett Mariner, Impetuous Ensign
// Audit: unsupported — Needs an executable declaration for First strike Reckless Behavior — Whenever Beckett Mariner
// becomes tapped, put a promotion counter on her. Then if there are three or more promotion
// counters on her, remove those counters and she deals 2 damage to each opponent.
pub(in crate::card::sets) static BECKETT_MARINER_IMPETUOUS_ENSIGN: CardRecord = CardRecord::new(
    "Beckett Mariner, Impetuous Ensign",
    "f2001bbd-63ee-4a6e-8e01-3a91aaf1e3b9",
    "Lucas Durham",
    CardRules::unsupported(),
);

// TRK 139 — Boarding Action
// Audit: unsupported — Needs an executable declaration for Create a 2/1 red Klingon Warrior creature token with haste.
// Beam me up {2}{R} (You may cast this card from your graveyard for {2}{R} if you also return
// a creature you control to its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static BOARDING_ACTION: CardRecord = CardRecord::new(
    "Boarding Action",
    "3a0d33cd-f81c-4950-81ed-9536061eca8a",
    "Douglas Shuler",
    CardRules::unsupported(),
);

// TRK 140 — Brazen Bluff
// Audit: unsupported — Needs an executable declaration for Choose target noncreature spell an opponent controls. That
// player may exile it. If they don't, you may cast a noncreature spell from your hand without
// paying its mana cost.
pub(in crate::card::sets) static BRAZEN_BLUFF: CardRecord = CardRecord::new(
    "Brazen Bluff",
    "5b306901-1bba-488b-866e-83aa162dab5a",
    "Filipe Pagliuso",
    CardRules::unsupported(),
);

// TRK 141 — Cantankerous Captain
// Audit: unsupported — Needs an executable declaration for At the beginning of combat on your turn, target creature
// you control gets +1/+0 until end of turn. Then that creature gains menace until end of turn
// if creatures you control have total power 8 or greater. (It can't be blocked except by two
// or more creatures.)
pub(in crate::card::sets) static CANTANKEROUS_CAPTAIN: CardRecord = CardRecord::new(
    "Cantankerous Captain",
    "2c053ec5-d1ea-4526-8a0d-e4dfafb8ecdf",
    "Edouard",
    CardRules::unsupported(),
);

// TRK 142 — Captain James T. Kirk
// Audit: unsupported — Needs conditional one-or-more mode selection on a triggered ability when its
// controller has no cards in hand; current trigger placement selects exactly one mode.
pub(in crate::card::sets) static CAPTAIN_JAMES_T_KIRK: CardRecord = CardRecord::new(
    "Captain James T. Kirk",
    "40a0466d-20bb-4ed5-858d-947fb9144580",
    "Jason Pastrana",
    CardRules::unsupported(),
);

// TRK 143 — Captain Michael Burnham
// Audit: unsupported — Needs an executable declaration for Trample Landfall — Whenever a land you control enters,
// Captain Burnham deals 1 damage to each opponent. If this is the second time this ability has
// resolved this turn, put a +1/+1 counter on Captain Burnham.
pub(in crate::card::sets) static CAPTAIN_MICHAEL_BURNHAM: CardRecord = CardRecord::new(
    "Captain Michael Burnham",
    "e10d6d71-9360-4ecf-97c8-22afb22aaeb7",
    "Romana Kendelic",
    CardRules::unsupported(),
);

// TRK 144 — Captain's Tutelage
// Audit: unsupported — Needs an executable declaration for Flash Enchant creature When this Aura enters, enchanted
// creature gains first strike until end of turn. Enchanted creature gets +2/+0.
pub(in crate::card::sets) static CAPTAIN_S_TUTELAGE: CardRecord = CardRecord::new(
    "Captain's Tutelage",
    "0c340eba-645d-416a-9ed4-f2303aa8c919",
    "Mark Spears",
    CardRules::unsupported(),
);

// TRK 145 — D-7 Battle Cruiser
// Audit: unsupported — Needs an executable declaration for Station (Tap another creature you control: Put charge
// counters equal to its power on this Spacecraft. Station only as a sorcery. It's an artifact
// creature at 8+.) 3+ | At the beginning of combat on your turn, create a 2/1 red Klingon
// Warrior creature token with haste. Sacrifice it at the beginning of the next end step. 8+ |
// Flying
pub(in crate::card::sets) static D_7_BATTLE_CRUISER: CardRecord = CardRecord::new(
    "D-7 Battle Cruiser",
    "6c22998d-7bd7-433d-95b8-50f2fd8b7b7f",
    "Kevin Glint",
    CardRules::unsupported(),
);

// TRK 146 — Dahar Master
// Audit: unsupported — Needs an executable declaration for When this creature enters, you may sacrifice another
// nonland permanent. When you do, this creature deals 2 damage to any target.
pub(in crate::card::sets) static DAHAR_MASTER: CardRecord = CardRecord::new(
    "Dahar Master",
    "1122fe1c-97f7-409d-ad37-0fed46f9f963",
    "Grant Griffin",
    CardRules::unsupported(),
);

// TRK 147 — Dominion Infiltration
// Audit: unsupported — Needs an executable declaration for Choose X target creatures your opponents control. For each
// of them, create a token that's a copy of that creature, except it has changeling. Those
// tokens gain haste. Exile those tokens at the beginning of the next end step. (They're every
// creature type.)
pub(in crate::card::sets) static DOMINION_INFILTRATION: CardRecord = CardRecord::new(
    "Dominion Infiltration",
    "38b7cc7c-e6f0-4460-83c0-0bcd1ab5b4cf",
    "Jonathan Bartlett",
    CardRules::unsupported(),
);

// TRK 148 — Encounter the Unknown
// Audit: unsupported — Needs an executable declaration for Choose one — • Come in Peace — Exile the top two cards of
// your library. Until the end of your next turn, you may play those cards. • Shoot to Kill —
// Encounter the Unknown deals 2 damage to each creature and each player.
pub(in crate::card::sets) static ENCOUNTER_THE_UNKNOWN: CardRecord = CardRecord::new(
    "Encounter the Unknown",
    "9a49f66c-7017-4ddd-8787-21d68fe1bff5",
    "Neil Blevins",
    CardRules::unsupported(),
);

// TRK 149 — Erica Ortegas, Plucky Pilot
// Audit: unsupported — Needs an executable declaration for Haste Whenever Erica Ortegas becomes tapped, you may put a
// land card from your hand onto the battlefield. If you do, return a land you control to its
// owner's hand at the beginning of the next end step.
pub(in crate::card::sets) static ERICA_ORTEGAS_PLUCKY_PILOT: CardRecord = CardRecord::new(
    "Erica Ortegas, Plucky Pilot",
    "3667eb41-598e-4543-97ed-6da961f8b567",
    "Romana Kendelic",
    CardRules::unsupported(),
);

// TRK 150 — Exploratory Mission
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, put a quest
// counter on this enchantment. Then exile cards from the top of your library until you exile a
// nonland card. You may play that card this turn. If this enchantment has five or more quest
// counters on it, add {R}{R}.
pub(in crate::card::sets) static EXPLORATORY_MISSION: CardRecord = CardRecord::new(
    "Exploratory Mission",
    "61bb2ec5-5f04-4c34-ba72-7fa226922c5c",
    "Jason Pastrana",
    CardRules::unsupported(),
);

// TRK 151 — Genesis Effect
// Audit: unsupported — Needs an executable declaration for Genesis Effect deals 6 damage to target creature. Destroy
// target land you control. Create a tapped Planet land token named New Planet with "{T}: Add
// one mana of any color."
pub(in crate::card::sets) static GENESIS_EFFECT: CardRecord = CardRecord::new(
    "Genesis Effect",
    "f0996bab-c2e4-4483-a76e-1355c81f1577",
    "Anthony Devine",
    CardRules::unsupported(),
);

// TRK 152 — Horta
// Audit: unsupported — Needs an executable declaration for Trample Basic landcycling {2} ({2}, Discard this card:
// Search your library for a basic land card, reveal it, put it into your hand, then shuffle.)
pub(in crate::card::sets) static HORTA: CardRecord = CardRecord::new(
    "Horta",
    "abf44662-62ad-47fb-9cfe-6bed47133de5",
    "Arthur Haas",
    CardRules::unsupported(),
);

// TRK 153 — Infinite Diversity
// Audit: unsupported — Needs an executable declaration for Federation — Exile target instant or sorcery card from your
// graveyard. Copy that card X times, where X is the number of creature types among non-Borg
// creatures you control. You may cast the copies without paying their mana costs. Exile
// Infinite Diversity.
pub(in crate::card::sets) static INFINITE_DIVERSITY: CardRecord = CardRecord::new(
    "Infinite Diversity",
    "33ade614-3593-4055-8ecc-2d9e4858ba19",
    "Julie Dillon",
    CardRules::unsupported(),
);

// TRK 154 — Khaaaaaaaaaaaannn!
// Audit: unsupported — Needs an executable declaration for Khaaaaaaaaaaaannn! deals twice X damage to target creature.
// If that creature would die this turn, exile it instead.
pub(in crate::card::sets) static KHAAAAAAAAAAAANNN: CardRecord = CardRecord::new(
    "Khaaaaaaaaaaaannn!",
    "22c60c17-74ec-460b-bf7a-e56e42239826",
    "Evan Shipard",
    CardRules::unsupported(),
);

// TRK 155 — Khan, Despot by Design
// Audit: unsupported — Needs an executable declaration for Trample When Khan enters, he deals 1 damage to each
// opponent and each creature your opponents control.
pub(in crate::card::sets) static KHAN_DESPOT_BY_DESIGN: CardRecord = CardRecord::new(
    "Khan, Despot by Design",
    "fd283ef1-fd8c-4288-8efa-6e81c92b3415",
    "Kirk DouPonce",
    CardRules::unsupported(),
);

// TRK 156 — La'An Noonien-Singh, Security
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, target creature can't block this turn.
pub(in crate::card::sets) static LA_AN_NOONIEN_SINGH_SECURITY: CardRecord = CardRecord::new(
    "La'An Noonien-Singh, Security",
    "afe08bef-b184-4b91-8d69-0101e183c3ff",
    "Pauline Voss",
    CardRules::unsupported(),
);

// TRK 157 — Lwaxana Troi, Charming Guest
// Audit: unsupported — Needs an executable declaration for You may choose not to untap Lwaxana Troi during your untap
// step. {2}, {T}: Choose another target creature you control. For as long as Lwaxana Troi
// remains tapped, if a triggered ability of that creature triggers, that ability triggers an
// additional time.
pub(in crate::card::sets) static LWAXANA_TROI_CHARMING_GUEST: CardRecord = CardRecord::new(
    "Lwaxana Troi, Charming Guest",
    "19bf2f2f-4be2-4958-bfab-eb46c9ca1995",
    "Marta Nael",
    CardRules::unsupported(),
);

// TRK 158 — Munitions Enthusiast
// Audit: unsupported — Needs an executable declaration for Trample When this creature dies, it deals 1 damage to any
// target.
pub(in crate::card::sets) static MUNITIONS_ENTHUSIAST: CardRecord = CardRecord::new(
    "Munitions Enthusiast",
    "6d73bb1c-39e3-429c-b6bb-96dae2254708",
    "Lucio Parrillo",
    CardRules::unsupported(),
);

// TRK 159 — The Naked Time
pub(in crate::card::sets) static THE_NAKED_TIME: CardRecord = CardRecord::new(
    "The Naked Time",
    "627c2ff6-88be-4c37-a676-0eddf453e8c6",
    "Jason Raish",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Search your library for a basic Mountain card, put it onto \
             the battlefield tapped, then shuffle.",
                EffectDef::SearchZone {
                    exile_face_down: false,
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Mountain")),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            abilities::saga_chapter(
                2,
                "II — Whenever a creature you control enters this turn, put a \
             +1/+1 counter on it and it gains haste until end of turn.",
                EffectDef::InstallTrigger(InstalledTriggerDef::this_turn(&AbilityDef::triggered(
                    "Whenever a creature you control enters, put a +1/+1 counter on it \
             and it gains haste until end of turn.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::TriggeringObject,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::TriggeringObject,
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                ))),
            ),
            abilities::saga_chapter(
                3,
                "III — Sacrifice a land.",
                crate::card::actions::choose_sacrifice(1)
                    .matching(ObjectPredicateDef::HasType(CardType::Land))
                    .as_effect(),
            ),
        ]),
);

// TRK 160 — Pah-Wraith
// Audit: unsupported — Needs an executable declaration for First strike, haste Strange new worlds — Whenever this
// creature attacks, creatures you control get +X/+0 until end of turn, where X is the number
// of differently named lands you control.
pub(in crate::card::sets) static PAH_WRAITH: CardRecord = CardRecord::new(
    "Pah-Wraith",
    "24c7e179-c016-4db8-ad8b-aa5629152b5c",
    "Nino Vecia",
    CardRules::unsupported(),
);

// TRK 161 — Perils of the Past
// Audit: unsupported — Needs an executable declaration for Choose one — • Use the Relic — Target creature gets +3/+2
// until end of turn. • Destroy the Relic — Destroy target artifact.
pub(in crate::card::sets) static PERILS_OF_THE_PAST: CardRecord = CardRecord::new(
    "Perils of the Past",
    "3df10e9a-efb6-4945-9b1b-29200688702a",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// TRK 162 — Planetary Bombardment
// Audit: unsupported — Needs an executable declaration for Planetary Bombardment deals 2 damage to target creature.
// Planetary Bombardment deals 4 damage to that creature instead if a creature died this turn.
pub(in crate::card::sets) static PLANETARY_BOMBARDMENT: CardRecord = CardRecord::new(
    "Planetary Bombardment",
    "4d9c39ce-f598-48d1-afa0-22f9513f1483",
    "Smirtouille",
    CardRules::unsupported(),
);

// TRK 163 — Planetary Patrol
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, this
// creature gets +1/+1 until end of turn.
pub(in crate::card::sets) static PLANETARY_PATROL: CardRecord = CardRecord::new(
    "Planetary Patrol",
    "163f5474-138d-4b47-a6f5-b1a0974e56a4",
    "Runa I. Rosenberger",
    CardRules::unsupported(),
);

// TRK 164 — Plasma Cascade
// Audit: unsupported — Needs an executable declaration for Plasma Cascade deals 3 damage to any target. You may
// discard a card. If you do, draw a card.
pub(in crate::card::sets) static PLASMA_CASCADE: CardRecord = CardRecord::new(
    "Plasma Cascade",
    "3c786a2f-3de4-4b24-a3a0-15b1f2a07324",
    "Narendra Bintara Adi",
    CardRules::unsupported(),
);

// TRK 165 — Q, Stacking the Deck
// Audit: unsupported — Needs to grant a hand-exile activated cost to an exiled card, then create and
// optionally cast a copy of that card from exile with an own-turn activation restriction.
pub(in crate::card::sets) static Q_STACKING_THE_DECK: CardRecord = CardRecord::new(
    "Q, Stacking the Deck",
    "dcbe34db-0065-4960-92b8-3ea1d336e803",
    "Ryan Valle",
    CardRules::unsupported(),
);

// TRK 166 — Quark, Humble Proprietor
// Audit: unsupported — Needs an executable declaration for You can't sacrifice Treasure tokens. At the beginning of
// each upkeep, create a Treasure token. (It's an artifact with "{T}, Sacrifice this token: Add
// one mana of any color.")
pub(in crate::card::sets) static QUARK_HUMBLE_PROPRIETOR: CardRecord = CardRecord::new(
    "Quark, Humble Proprietor",
    "96104754-57e2-46e9-948d-628aab2aa0ad",
    "Alex Negrea",
    CardRules::unsupported(),
);

// TRK 167 — Ravenous Cosmozoan
pub(in crate::card::sets) static RAVENOUS_COSMOZOAN: CardRecord = CardRecord::new(
    "Ravenous Cosmozoan",
    "8eeb471b-412d-4ea1-bec0-d7dd0a557d76",
    "Bruce Brenneise",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Fish"], 3, 3).with_abilities(&[
        abilities::this_spell_cost_reduction(
            "This spell costs {1} less to cast for each noncreature, nonland \
             card in your graveyard.",
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ]),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            )),
        ),
        abilities::flying(),
        abilities::haste(),
    ]),
);

// TRK 168 — Reckless Impulse (reprint)
const RECKLESS_IMPULSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::innistrad_crimson_vow::RECKLESS_IMPULSE,
    "5bca43d8-7c69-47f4-8f82-48dd2b6a7cba",
    "Simon Dominic",
);

// TRK 169 — Red Alert!
// Audit: unsupported — Needs an executable declaration for This spell costs {2} less to cast if a creature is
// attacking you. Creatures you control get +2/+0 until end of turn.
pub(in crate::card::sets) static RED_ALERT: CardRecord = CardRecord::new(
    "Red Alert!",
    "0f9b10d3-ba76-45ec-a85c-fb7b26da5561",
    "Carlos C Diaz",
    CardRules::unsupported(),
);

// TRK 170 — Sarcophagus, Ship of the Dead
// Audit: unsupported — Needs an executable declaration for Other creatures you control get +1/+0. Whenever a creature
// dies, if Sarcophagus isn't a creature, put a charge counter on it. Station (Tap another
// creature you control: Put charge counters equal to its power on this Spacecraft. Station
// only as a sorcery. It's an artifact creature at 10+.) 10+ | Flying
pub(in crate::card::sets) static SARCOPHAGUS_SHIP_OF_THE_DEAD: CardRecord = CardRecord::new(
    "Sarcophagus, Ship of the Dead",
    "b5ea32b5-aa58-4f87-ac2e-4e3b8dd4a752",
    "Joan Piqué Llorens",
    CardRules::unsupported(),
);

// TRK 171 — Savvy Profiteer
// Audit: unsupported — Needs an executable declaration for {T}: Add {R}. {3}, {T}: Create a Treasure token. (It's an
// artifact with "{T}, Sacrifice this token: Add one mana of any color.")
pub(in crate::card::sets) static SAVVY_PROFITEER: CardRecord = CardRecord::new(
    "Savvy Profiteer",
    "8256376f-011f-4cc8-9be4-47f5121ca75e",
    "Andreia Ugrai",
    CardRules::unsupported(),
);

// TRK 172 — Silicate Surveyor
// Audit: unsupported — Needs an executable declaration for Reach When this creature enters, exile the top card of your
// library. Until the end of your next turn, you may play that card.
pub(in crate::card::sets) static SILICATE_SURVEYOR: CardRecord = CardRecord::new(
    "Silicate Surveyor",
    "6359ab5d-4151-4a0c-af13-d59c1b381bb0",
    "Joan Piqué Llorens",
    CardRules::unsupported(),
);

// TRK 173 — Starfleet Security
// Audit: unsupported — Needs an executable declaration for Haste When this creature dies during your turn,
// investigate. (Create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a
// card.")
pub(in crate::card::sets) static STARFLEET_SECURITY: CardRecord = CardRecord::new(
    "Starfleet Security",
    "62acde0e-791d-4259-83bc-3690e498f3b0",
    "Edouard",
    CardRules::unsupported(),
);

// TRK 174 — Tal Shiar Analyst
// Audit: unsupported — Needs an executable declaration for {T}: This creature deals 1 damage to each opponent.
// Whenever you cast a noncreature spell, untap this creature.
pub(in crate::card::sets) static TAL_SHIAR_ANALYST: CardRecord = CardRecord::new(
    "Tal Shiar Analyst",
    "66daae4f-8024-4d87-b42f-e5271d29e646",
    "Lucas Durham",
    CardRules::unsupported(),
);

// TRK 175 — Targ Wrangler
// Audit: unsupported — Needs an executable declaration for Menace (This creature can't be blocked except by two or
// more creatures.) At the beginning of your end step, if this creature attacked this turn,
// sacrifice a nonland permanent.
pub(in crate::card::sets) static TARG_WRANGLER: CardRecord = CardRecord::new(
    "Targ Wrangler",
    "eabc4252-cf58-48c3-9483-7862450e03d0",
    "Harkalé Linaï",
    CardRules::unsupported(),
);

// TRK 176 — Tasha Yar, Out of Time
// Audit: unsupported — Needs an executable declaration for Haste When Tasha Yar dies, exile the top card of your
// library. Until the end of your next turn, you may play that card. {2}{R}, Exile this card
// from your graveyard: Create a token that's a copy of this card. Sacrifice it at the
// beginning of the next end step. Activate only as a sorcery.
pub(in crate::card::sets) static TASHA_YAR_OUT_OF_TIME: CardRecord = CardRecord::new(
    "Tasha Yar, Out of Time",
    "5f6cbe6c-5cd8-4eec-82b7-25d5bf5641c0",
    "Patricia Pria",
    CardRules::unsupported(),
);

// TRK 177 — Test of Identity
// Audit: unsupported — Needs an executable declaration for Choose one — • I Am Klingon — Target Klingon gets +1/+1 and
// gains double strike until end of turn. • I Am Starfleet — Discard a card. If you do, draw
// two cards.
pub(in crate::card::sets) static TEST_OF_IDENTITY: CardRecord = CardRecord::new(
    "Test of Identity",
    "7e1c0423-bf58-47a1-a838-c15bbf40eaba",
    "Andrey Kuzinskiy",
    CardRules::unsupported(),
);

// TRK 178 — Transporter Malfunction
// Audit: unsupported — Needs an executable declaration for Transporter Malfunction deals 3 damage to any target. Beam
// me up {3}{R}{R}{R} (You may cast this card from your graveyard for {3}{R}{R}{R} if you also
// return a creature you control to its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static TRANSPORTER_MALFUNCTION: CardRecord = CardRecord::new(
    "Transporter Malfunction",
    "85b108c3-00b5-4dfb-90de-57fa483c4926",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// TRK 179 — Will Riker, First in the Field
// Audit: unsupported — Needs an executable declaration for At the beginning of combat on your turn, reveal cards from
// the top of your library until you reveal two land cards. Put those land cards onto the
// battlefield and the rest on the bottom of your library in a random order. Sacrifice those
// lands at the beginning of your next end step.
pub(in crate::card::sets) static WILL_RIKER_FIRST_IN_THE_FIELD: CardRecord = CardRecord::new(
    "Will Riker, First in the Field",
    "9422d025-9460-44b6-8071-d896f20b87ea",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// TRK 180 — Worf, Intelligence Contractor
// Audit: unsupported — Needs an executable declaration for When Worf enters, for each opponent, gain control of up to
// one target creature that player controls with mana value 3 or less until end of turn. Untap
// those creatures. Until end of turn, they become Warriors in addition to their other types
// and gain haste.
pub(in crate::card::sets) static WORF_INTELLIGENCE_CONTRACTOR: CardRecord = CardRecord::new(
    "Worf, Intelligence Contractor",
    "7df6c8c1-1f85-4c5d-a4f6-9ffcdad1b12e",
    "Jodie Muir",
    CardRules::unsupported(),
);

// TRK 181 — The Wrath of Khan
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — This Saga deals 4 damage to target creature an opponent
// controls. At the beginning of the next combat this turn, gain control of that creature until
// end of turn. Untap it. It gains haste until end of turn. II — You may exile a card from your
// hand. If you do, draw a card. III — Each opponent sacrifices a nontoken permanent of their
// choice.
pub(in crate::card::sets) static THE_WRATH_OF_KHAN: CardRecord = CardRecord::new(
    "The Wrath of Khan",
    "426d627e-bb88-4879-8d9f-db1babf22b15",
    "Kirk DouPonce",
    CardRules::unsupported(),
);

// TRK 182 — Adversary by Algorithm
// Audit: unsupported — Needs an executable declaration for Create a 3/3 colorless Illusion artifact creature token.
// Beam me up {3}{G} (You may cast this card from your graveyard for {3}{G} if you also return
// a creature you control to its owner's hand. Then exile this spell.)
pub(in crate::card::sets) static ADVERSARY_BY_ALGORITHM: CardRecord = CardRecord::new(
    "Adversary by Algorithm",
    "41644f06-d08f-406c-a56a-47e1b5273982",
    "Tony Foti",
    CardRules::unsupported(),
);

// TRK 183 — Amok Time
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — Target creature you control fights up to one target
// creature an opponent controls. II — You gain 3 life. III — Return target creature card from
// your graveyard to your hand.
pub(in crate::card::sets) static AMOK_TIME: CardRecord = CardRecord::new(
    "Amok Time",
    "d31447b6-484f-4dda-b6af-ab068a84f896",
    "Jason Raish",
    CardRules::unsupported(),
);

// TRK 184 — Astute Terraformer
// Audit: unsupported — Needs an executable declaration for Strange new worlds — {6}: This creature gets +2/+2 until
// end of turn. This ability costs {X} less to activate, where X is the number of differently
// named lands you control. Activate only once each turn.
pub(in crate::card::sets) static ASTUTE_TERRAFORMER: CardRecord = CardRecord::new(
    "Astute Terraformer",
    "2ef7c30c-3ee6-413d-bd24-476322f36cb8",
    "Andreia Ugrai",
    CardRules::unsupported(),
);

// TRK 185 — Branching Timeline
// Audit: unsupported — Needs an executable declaration for Strange new worlds — Reveal the top X cards of your
// library, where X is the number of differently named lands you control. Put all permanent
// cards revealed this way onto the battlefield tapped and the rest on the bottom of your
// library in a random order.
pub(in crate::card::sets) static BRANCHING_TIMELINE: CardRecord = CardRecord::new(
    "Branching Timeline",
    "4a2231ff-677d-4041-9413-b8d683846920",
    "Borja Pindado",
    CardRules::unsupported(),
);

// TRK 186 — Captain Jonathan Archer
// Audit: unsupported — Needs an executable declaration for When Captain Archer enters, create Porthos, a legendary 1/1
// green Dog creature token. Federation — {G}, {T}: Target creature you control gets +X/+X
// until end of turn, where X is the number of creature types among non-Borg creatures you
// control. Activate only as a sorcery.
pub(in crate::card::sets) static CAPTAIN_JONATHAN_ARCHER: CardRecord = CardRecord::new(
    "Captain Jonathan Archer",
    "a4518ea3-5f56-4f1d-9024-2bf06ac98786",
    "Julia Metzger",
    CardRules::unsupported(),
);

// TRK 187 — Captain Kathryn Janeway
// Audit: unsupported — Needs an executable declaration for You may play an additional land on each of your turns.
// Whenever Captain Janeway or another creature you control enters, that creature explores.
// (Reveal the top card of your library. Put that card into your hand if it's a land.
// Otherwise, put a +1/+1 counter on the creature, then put the card back or put it into your
// graveyard.)
pub(in crate::card::sets) static CAPTAIN_KATHRYN_JANEWAY: CardRecord = CardRecord::new(
    "Captain Kathryn Janeway",
    "d17ea45c-7f9c-411f-b7d3-ca7100ffe53b",
    "Jason Pastrana",
    CardRules::unsupported(),
);

// TRK 188 — Captain Pike, Fullness of Time
// Audit: unsupported — Needs an executable declaration for Strange new worlds — Captain Pike's power and toughness are
// each equal to the number of differently named lands you control. You may look at the top
// card of your library any time. You may play lands from the top of your library.
pub(in crate::card::sets) static CAPTAIN_PIKE_FULLNESS_OF_TIME: CardRecord = CardRecord::new(
    "Captain Pike, Fullness of Time",
    "743be4c4-5c91-42bb-9942-115e903041a9",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 189 — Cold-Blooded Crew
// Audit: unsupported — Needs an executable declaration for When this creature enters, put two +1/+1 counters on target
// creature. This creature can't be blocked by creatures with power 2 or less. Basic
// landcycling {2} ({2}, Discard this card: Search your library for a basic land card, reveal
// it, put it into your hand, then shuffle.)
pub(in crate::card::sets) static COLD_BLOODED_CREW: CardRecord = CardRecord::new(
    "Cold-Blooded Crew",
    "4f2d1830-baea-4735-b029-cb8c0b538d83",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// TRK 190 — Common Goal
// Audit: unsupported — Needs an executable declaration for One or two target creatures you control each deal damage
// equal to their power to target creature an opponent controls.
pub(in crate::card::sets) static COMMON_GOAL: CardRecord = CardRecord::new(
    "Common Goal",
    "4fddc950-fbc5-4cf7-b694-dcd54ffccb31",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// TRK 191 — Culber and Stamets, Star-Crossed
// Audit: unsupported — Needs an executable declaration for When Culber and Stamets enter, choose one — • Put a +1/+1
// counter on target creature. • Add two mana of any one color.
pub(in crate::card::sets) static CULBER_AND_STAMETS_STAR_CROSSED: CardRecord = CardRecord::new(
    "Culber and Stamets, Star-Crossed",
    "962752f7-5c94-41e4-ab7d-3e8741c7a502",
    "Harkalé Linaï",
    CardRules::unsupported(),
);

// TRK 192 — Dathon and Picard at El-Adrel
// Audit: unsupported — Needs an executable declaration for Choose one — • Temba, His Arms Wide — Target creature gets
// +3/+3 and gains trample until end of turn. • Shaka, When the Walls Fell — Each opponent
// sacrifices a nontoken artifact or enchantment of their choice.
pub(in crate::card::sets) static DATHON_AND_PICARD_AT_EL_ADREL: CardRecord = CardRecord::new(
    "Dathon and Picard at El-Adrel",
    "94e9d400-f2bb-46e7-8d41-6038ebdc6c8b",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// TRK 193 — David and Carol Marcus, Creators
// Audit: unsupported — Needs an executable declaration for When David and Carol Marcus enter, create a tapped Planet
// land token named New Planet with "{T}: Add one mana of any color."
pub(in crate::card::sets) static DAVID_AND_CAROL_MARCUS_CREATORS: CardRecord = CardRecord::new(
    "David and Carol Marcus, Creators",
    "55d21155-647e-4892-b349-86b8d4e6d9b9",
    "KarrahE",
    CardRules::unsupported(),
);

// TRK 194 — Dominion Assault Force
// Audit: unsupported — Needs an executable declaration for Vigilance When this creature enters, choose one — • Exile
// target card from a graveyard. • Surveil 1. (Look at the top card of your library. You may
// put it into your graveyard.)
pub(in crate::card::sets) static DOMINION_ASSAULT_FORCE: CardRecord = CardRecord::new(
    "Dominion Assault Force",
    "28a55f63-1cbd-4362-a192-e42a06f44686",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// TRK 195 — Dominion Supervisor
// Audit: unsupported — Needs an executable declaration for When this creature enters, put a +1/+1 counter on target
// creature. Creatures you control with counters on them have trample.
pub(in crate::card::sets) static DOMINION_SUPERVISOR: CardRecord = CardRecord::new(
    "Dominion Supervisor",
    "5fa76320-24c0-4294-8ae4-fb39e171fbe6",
    "Kevin Glint",
    CardRules::unsupported(),
);

// TRK 196 — Dr. Joseph M'Benga
// Audit: unsupported — Needs an executable declaration for Other creatures you control with counters on them have
// hexproof. If you would put one or more counters on a creature you control, put twice that
// many of each of those kinds of counters on that creature instead.
pub(in crate::card::sets) static DR_JOSEPH_M_BENGA: CardRecord = CardRecord::new(
    "Dr. Joseph M'Benga",
    "c27e6865-40ba-406c-9dce-d8a7a9b8035b",
    "Romana Kendelic",
    CardRules::unsupported(),
);

// TRK 197 — Dr. Leonard "Bones" McCoy
// Audit: unsupported — Needs an executable declaration for Flash When Dr. McCoy enters, choose one — • Return target
// non-Doctor permanent card from your graveyard to your hand. • Choose up to one other target
// creature. When it dies this turn, return that card to the battlefield under its owner's
// control with a +1/+1 counter on it at the beginning of the next end step.
pub(in crate::card::sets) static DR_LEONARD_BONES_MCCOY: CardRecord = CardRecord::new(
    "Dr. Leonard \"Bones\" McCoy",
    "d8593bf7-5248-4f4a-bf8b-9816d2f76c6b",
    "Sam Kennedy",
    CardRules::unsupported(),
);

// TRK 198 — Final Frontier
// Audit: unsupported — Needs an executable declaration for Search your library for a basic land card or Planet card,
// put it onto the battlefield tapped, then shuffle. Beam me up {1}{G} (You may cast this card
// from your graveyard for {1}{G} if you also return a creature you control to its owner's
// hand. Then exile this spell.)
pub(in crate::card::sets) static FINAL_FRONTIER: CardRecord = CardRecord::new(
    "Final Frontier",
    "34ea6df3-36e3-42c5-a4ed-a4cd18ec6dd0",
    "Jessica Rossier",
    CardRules::unsupported(),
);

// TRK 199 — First Contact
// Audit: unsupported — Needs an executable declaration for Mill four cards. You may put a permanent card from among
// them into your hand. If this spell is the first spell you've cast this game, you gain 2
// life. (To mill four cards, put the top four cards of your library into your graveyard.)
pub(in crate::card::sets) static FIRST_CONTACT: CardRecord = CardRecord::new(
    "First Contact",
    "5407e8f4-0c8e-4e66-9d2f-f22277aa38f4",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// TRK 200 — Genesis Device
// Audit: unsupported — Needs an executable declaration for When Genesis Device enters, search your library for a basic
// land card or Planet card, put it onto the battlefield tapped, then shuffle. {3}{G}{G}, {T},
// Sacrifice Genesis Device: Sacrifice any number of lands you control, then search your
// library for up to that many land cards, put them onto the battlefield tapped, then shuffle.
// Activate only as a sorcery.
pub(in crate::card::sets) static GENESIS_DEVICE: CardRecord = CardRecord::new(
    "Genesis Device",
    "6a5b94ba-ac1f-478e-981f-7dfc31ad02f9",
    "Cliff Nielsen",
    CardRules::unsupported(),
);

// TRK 201 — Holo-Engineer
// Audit: unsupported — Needs an executable declaration for When this creature enters, create a 3/3 colorless Illusion
// artifact creature token. If one or more tokens would be created under your control, that
// many 3/3 colorless Illusion artifact creature tokens are created instead.
pub(in crate::card::sets) static HOLO_ENGINEER: CardRecord = CardRecord::new(
    "Holo-Engineer",
    "acfaf5f3-a4f5-418a-a29a-211b33bf18ee",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// TRK 202 — Hologram Stalker
// Audit: unsupported — Needs an executable declaration for When this creature enters, return target permanent card
// from your graveyard to your hand. Whenever a card leaves your graveyard, create a 3/3
// colorless Illusion artifact creature token. This ability triggers only once each turn.
pub(in crate::card::sets) static HOLOGRAM_STALKER: CardRecord = CardRecord::new(
    "Hologram Stalker",
    "6bd8a87b-ca7a-4a15-b96f-1a673ee44f3e",
    "Hector Ortiz",
    CardRules::unsupported(),
);

// TRK 203 — I'm a Doctor, Not a . . .
// Audit: unsupported — Needs an executable declaration for Put a +1/+1 counter on target creature you control. Untap
// it. Until end of turn, it gets +1/+1 and becomes a Doctor. (It loses all other creature
// types.)
pub(in crate::card::sets) static I_M_A_DOCTOR_NOT_A: CardRecord = CardRecord::new(
    "I'm a Doctor, Not a . . .",
    "c07979d4-8a58-4cfb-bffa-74e8bd97c7ff",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 204 — Jadzia Dax, Living Legacy
// Audit: unsupported — Needs an executable declaration for At the beginning of your first main phase, put a +1/+1
// counter on Jadzia Dax, then add an amount of {G} equal to the number of counters on her.
// Transfer Symbiont — {2}{G}, Discard this card: Put two +1/+1 counters on target creature.
pub(in crate::card::sets) static JADZIA_DAX_LIVING_LEGACY: CardRecord = CardRecord::new(
    "Jadzia Dax, Living Legacy",
    "bd342f8b-9072-487d-8a33-b1bb149f1502",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// TRK 205 — Jake Sisko, War Correspondent
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, you gain 1
// life. If this is the second time this ability has resolved this turn, investigate. (Create a
// Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a card.")
pub(in crate::card::sets) static JAKE_SISKO_WAR_CORRESPONDENT: CardRecord = CardRecord::new(
    "Jake Sisko, War Correspondent",
    "9e47c6b6-f99b-46be-98a1-92c98416aefe",
    "Ariel Orea",
    CardRules::unsupported(),
);

// TRK 206 — MACO Strike Team
// Audit: unsupported — Needs an executable declaration for Whenever a creature you control becomes the target of a
// spell or ability an opponent controls, create a tapped 1/1 red Officer creature token.
pub(in crate::card::sets) static MACO_STRIKE_TEAM: CardRecord = CardRecord::new(
    "MACO Strike Team",
    "98a38e3f-e5ff-402f-9bcd-a03edc264688",
    "Aaron J. Riley",
    CardRules::unsupported(),
);

// TRK 207 — Moopsy
pub(in crate::card::sets) static MOOPSY: CardRecord = CardRecord::new(
    "Moopsy",
    "dfbaf597-6be2-4aa0-88ae-cc4c94878add",
    "Brooklyn Smith",
    CardRules::new_creature(mana_cost!("{G}"), &["Beast"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated(
            "{2}: This creature gains reach until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::reach()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TRK 208 — Mugato
pub(in crate::card::sets) static MUGATO: CardRecord = CardRecord::new(
    "Mugato",
    "f3c68b20-f202-4d55-bef1-398d96e55c57",
    "Chuck Lukacs",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Ape", "Beast"], 6, 6).with_abilities(&[
        abilities::reach(),
        abilities::vigilance(),
        abilities::deathtouch(),
    ]),
);

// TRK 209 — Neelix, Moral Support
// Audit: unsupported — Needs an executable declaration for When Neelix enters, create a Food token. (It's an artifact
// with "{2}, {T}, Sacrifice this token: You gain 3 life.") Whenever you gain life, put a +1/+1
// counter on Neelix.
pub(in crate::card::sets) static NEELIX_MORAL_SUPPORT: CardRecord = CardRecord::new(
    "Neelix, Moral Support",
    "09c0daa9-9eca-4817-a427-868d1efcfa20",
    "JB Casacop",
    CardRules::unsupported(),
);

// TRK 210 — Photosynthetic Physician
// Audit: unsupported — Needs an executable declaration for Reach {G}, Exile this card from your graveyard: Create a
// 1/1 red Officer creature token. Activate only as a sorcery.
pub(in crate::card::sets) static PHOTOSYNTHETIC_PHYSICIAN: CardRecord = CardRecord::new(
    "Photosynthetic Physician",
    "88b87ddb-fbf8-4e6e-9895-1b30ae5ad63a",
    "Stella Spente",
    CardRules::unsupported(),
);

// TRK 211 — Rok-Tahk, Budding Scientist
// Audit: unsupported — Needs an executable declaration for Creatures you control have "{T}: Add {G}{G}." When Rok-Tahk
// enters, create Murf, a legendary 0/1 green Ooze Slug creature token with indestructible.
pub(in crate::card::sets) static ROK_TAHK_BUDDING_SCIENTIST: CardRecord = CardRecord::new(
    "Rok-Tahk, Budding Scientist",
    "787bd19b-586e-4451-850b-60e0857bada0",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// TRK 212 — Saru, Unthreatened
// Audit: unsupported — Needs an executable declaration for Flash When Saru enters, another target creature you control
// gains hexproof until end of turn. (It can't be the target of spells or abilities your
// opponents control.)
pub(in crate::card::sets) static SARU_UNTHREATENED: CardRecord = CardRecord::new(
    "Saru, Unthreatened",
    "445de44d-8af9-4ea3-b165-9e8eb8939820",
    "Ron Lemen",
    CardRules::unsupported(),
);

// TRK 213 — Saurian Explorer
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, this
// creature gets +2/+2 until end of turn. {6}: Create a tapped Planet land token named New
// Planet with "{T}: Add one mana of any color." Activate only as a sorcery.
pub(in crate::card::sets) static SAURIAN_EXPLORER: CardRecord = CardRecord::new(
    "Saurian Explorer",
    "197abe5e-cde2-4b29-a10f-d6b789776bfa",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// TRK 214 — Species 8472
// Audit: unsupported — Needs an executable declaration for Strange new worlds — When this creature enters, you gain
// life equal to the number of differently named lands you control. This creature can't be
// blocked by artifact creatures. This creature can't be the target of abilities from artifact
// sources your opponents control.
pub(in crate::card::sets) static SPECIES_8472: CardRecord = CardRecord::new(
    "Species 8472",
    "b3d74723-4c9e-4fe7-8276-05d5f1b7ba0c",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// TRK 215 — Stable Wormhole
// Audit: unsupported — Needs an executable declaration for Whenever you attack, create a tapped Planet land token
// named New Planet with "{T}: Add one mana of any color."
pub(in crate::card::sets) static STABLE_WORMHOLE: CardRecord = CardRecord::new(
    "Stable Wormhole",
    "c6fc3447-55de-4f5d-8d06-8d9e2ff4d5d0",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// TRK 216 — Support Mission
// Audit: unsupported — Needs an executable declaration for Whenever a creature you control enters, put a quest counter
// on this enchantment. Put a +1/+1 counter on that creature. If this enchantment has five or
// more quest counters on it, instead put two +1/+1 counters on that creature.
pub(in crate::card::sets) static SUPPORT_MISSION: CardRecord = CardRecord::new(
    "Support Mission",
    "9e8b404c-6e2a-4966-ac4b-c9981c1c6fee",
    "Nadia Hurianova",
    CardRules::unsupported(),
);

// TRK 217 — Sylvia Tilly, Joy of Discovery
// Audit: unsupported — Needs an executable declaration for When Sylvia Tilly enters, surveil 2. (Look at the top two
// cards of your library, then put any number of them into your graveyard and the rest on top
// of your library in any order.) At the beginning of combat on your turn, you may pay {1} and
// exile a card from your graveyard. When you do, put a +1/+1 counter on target creature.
pub(in crate::card::sets) static SYLVIA_TILLY_JOY_OF_DISCOVERY: CardRecord = CardRecord::new(
    "Sylvia Tilly, Joy of Discovery",
    "fe6e03b5-cd96-430b-9c74-96eca0212a81",
    "Ron Lemen",
    CardRules::unsupported(),
);

// TRK 218 — Syndicate Liquidators
// Audit: unsupported — Needs an executable declaration for Trample Landfall — Whenever a land you control enters, put
// two +1/+1 counters on target creature you control.
pub(in crate::card::sets) static SYNDICATE_LIQUIDATORS: CardRecord = CardRecord::new(
    "Syndicate Liquidators",
    "18bb2edd-8dee-4714-a707-6542b6ebbd53",
    "Carlos C Diaz",
    CardRules::unsupported(),
);

// TRK 219 — Temporal Conqueror
// Audit: unsupported — Needs an executable declaration for When this creature enters, mill three cards. Then return a
// land card from your graveyard to your hand. If you can't, create a Treasure token. (To mill
// three cards, put the top three cards of your library into your graveyard.)
pub(in crate::card::sets) static TEMPORAL_CONQUEROR: CardRecord = CardRecord::new(
    "Temporal Conqueror",
    "b0bb51a7-1ae2-45d0-8be9-5454a3c8f751",
    "Douglas Shuler",
    CardRules::unsupported(),
);

// TRK 220 — Test of Courage
pub(in crate::card::sets) static TEST_OF_COURAGE: CardRecord = CardRecord::new(
    "Test of Courage",
    "291b9b6b-54bf-4e71-98f1-cb9301ea0727",
    "Zoltan Boros",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Alter the Past — Exile target card from a graveyard. You gain 2 \
             life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        owner: None,
                        controller: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Obliterate the Transwarp Network — Destroy target artifact or \
             enchantment with mana value 3 or less.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
        ],
    )]),
);

// TRK 221 — U.S.S. Discovery, at Black Alert
// Audit: unsupported — Needs an executable declaration for Whenever U.S.S. Discovery enters or attacks, discover 5.
// (Exile cards from the top of your library until you exile a nonland card with mana value 5
// or less. Cast it without paying its mana cost or put it into your hand. Put the rest on the
// bottom in a random order.) Station 6+ | Flying
pub(in crate::card::sets) static U_S_S_DISCOVERY_AT_BLACK_ALERT: CardRecord = CardRecord::new(
    "U.S.S. Discovery, at Black Alert",
    "69b1f87f-ff02-4945-9f07-18dab5035e2d",
    "Kevin Glint",
    CardRules::unsupported(),
);

// TRK 222 — U.S.S. Voyager, Lone Wanderer
// Audit: unsupported — Needs an executable declaration for When U.S.S. Voyager enters, reveal cards from the top of
// your library until you reveal two land cards. Put those land cards onto the battlefield
// tapped and the rest on the bottom of your library in a random order. Station (Tap another
// creature you control: Put charge counters equal to its power on this Spacecraft. Station
// only as a sorcery. It's an artifact creature at 8+.) 8+ | Flying, vigilance
pub(in crate::card::sets) static U_S_S_VOYAGER_LONE_WANDERER: CardRecord = CardRecord::new(
    "U.S.S. Voyager, Lone Wanderer",
    "aaf8a0b2-c6cb-41cb-913d-7ae1d75e555d",
    "Hector Ortiz",
    CardRules::unsupported(),
);

// TRK 223 — Warp Speed
// Audit: unsupported — Needs an executable declaration for Lands you control enter untapped. When this enchantment
// enters, search your library for a basic land card or Planet card, reveal it, put it into
// your hand, then shuffle. Strange new worlds — {7}{U}{U}, Sacrifice this enchantment: Draw
// two cards. This ability costs {X} less to activate, where X is the number of differently
// named lands you control.
pub(in crate::card::sets) static WARP_SPEED: CardRecord = CardRecord::new(
    "Warp Speed",
    "fdb5b940-9f62-47ad-8e80-5996daad0c9b",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// TRK 224 — Xenobotanist
pub(in crate::card::sets) static XENOBOTANIST: CardRecord = CardRecord::new(
    "Xenobotanist",
    "20a97cda-42b2-44f1-8e77-f64dd1c98f6f",
    "Marta Nael",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Vulcan", "Scientist"], 1, 3).with_ability(
        abilities::tap_for_mana(
            "{T}: Add one mana of any color.",
            AddManaEffectDef::any_color(),
        ),
    ),
);

// TRK 225 — Xindi Surveyors
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, put a +1/+1
// counter on this creature.
pub(in crate::card::sets) static XINDI_SURVEYORS: CardRecord = CardRecord::new(
    "Xindi Surveyors",
    "803608aa-ae69-45d2-ae48-f1688d2a6a0d",
    "Joan Piqué Llorens",
    CardRules::unsupported(),
);

// TRK 226 — All Good Things...
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after IV.) I, II, III — Draw two cards, then exile a card from your hand.
// IV — If all cards exiled with this Saga share a card type, you gain 10 life.
pub(in crate::card::sets) static ALL_GOOD_THINGS: CardRecord = CardRecord::new(
    "All Good Things...",
    "fe5dda31-159c-49c7-962e-a04b771256ea",
    "Lazare Gvimradze",
    CardRules::unsupported(),
);

// TRK 227 — Borg Cube
// Audit: unsupported — Needs an executable declaration for Affinity for artifact creatures Whenever this Spacecraft
// enters or attacks, assimilate a creature card exiled with it. If you can't, exile up to one
// target creature. (To assimilate a card, put it onto the battlefield under your control with
// a +1/+1 counter. It's a Borg artifact creature and loses all other creature types.) Station
// 12+ | Artifact creatures you control have flying.
pub(in crate::card::sets) static BORG_CUBE: CardRecord = CardRecord::new(
    "Borg Cube",
    "e58f7310-3fd5-48d5-b6b7-a5d3e1808bbc",
    "Alexander Forssberg",
    CardRules::unsupported(),
);

// TRK 228 — The City on the Edge of Forever
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — All artifacts and creatures phase out. Create a 0/1 white
// Human creature token with "Permanents can't phase in." (Treat phased-out permanents and
// anything attached to them as though they don't exist.) II — You draw two cards and each
// player loses 2 life. III — Sacrifice a creature token. If you do, you gain 5 life.
pub(in crate::card::sets) static THE_CITY_ON_THE_EDGE_OF_FOREVER: CardRecord = CardRecord::new(
    "The City on the Edge of Forever",
    "aac6f64d-d39a-4994-bc70-6b477b2640f8",
    "Peter Diamond",
    CardRules::unsupported(),
);

// TRK 229 — Cleveland Booker, Empath
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, create a 2/2
// red and green Worm creature token with "This token can't block." {8}: Creatures you control
// get +2/+0 until end of turn.
pub(in crate::card::sets) static CLEVELAND_BOOKER_EMPATH: CardRecord = CardRecord::new(
    "Cleveland Booker, Empath",
    "6b2e52e2-5cb9-4360-92d6-5c0d9e435d09",
    "Kaitlyn McCulley",
    CardRules::unsupported(),
);

// TRK 230 — Constable Odo
// Audit: unsupported — Needs an executable declaration for When Constable Odo enters, choose one — • Tap target
// creature an opponent controls. Put two stun counters on it. (If a permanent with a stun
// counter would become untapped, remove one from it instead.) • Investigate twice. (To
// investigate, create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a
// card.")
pub(in crate::card::sets) static CONSTABLE_ODO: CardRecord = CardRecord::new(
    "Constable Odo",
    "3df582e4-3bfb-4413-9b39-898f33a5c1fd",
    "Jason Rainville",
    CardRules::unsupported(),
);

// TRK 231 — Dal R'El, Augment Orphan
// Audit: unsupported — Needs an executable declaration for Landfall — Whenever a land you control enters, add {R} or
// {G}. {3}, Sacrifice a land: Draw a card.
pub(in crate::card::sets) static DAL_R_EL_AUGMENT_ORPHAN: CardRecord = CardRecord::new(
    "Dal R'El, Augment Orphan",
    "0da52d3a-dd83-4662-8e6d-3f4d4d389c3d",
    "Stella Spente",
    CardRules::unsupported(),
);

// TRK 232 — Daniels, Temporal Agent
// Audit: unsupported — Needs an executable declaration for When Daniels enters, return target creature card from your
// graveyard to your hand. {2}, Put this card from your graveyard on the bottom of your
// library: You gain 2 life.
pub(in crate::card::sets) static DANIELS_TEMPORAL_AGENT: CardRecord = CardRecord::new(
    "Daniels, Temporal Agent",
    "b64dd792-7dc7-4d22-9776-9b9e25e1173d",
    "Kaitlyn McCulley",
    CardRules::unsupported(),
);

// TRK 233 — Dathon, His Heart Open
// Audit: unsupported — Needs an executable declaration for Whenever you cast an enchantment spell, draw a card. If
// it's a Saga spell, Dathon deals 2 damage to each opponent. {2}: Choose a counter on target
// enchantment you control. Put another counter of that kind on it. Activate only as a sorcery.
pub(in crate::card::sets) static DATHON_HIS_HEART_OPEN: CardRecord = CardRecord::new(
    "Dathon, His Heart Open",
    "f2fe4663-eaaf-42c3-90f9-07e68129352c",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// TRK 234 — The Doctor, EMH
// Audit: unsupported — Needs an executable declaration for When The Doctor enters, put a +1/+1 counter on target
// creature. Then put a +1/+1 counter on each creature you control with a counter on it.
pub(in crate::card::sets) static THE_DOCTOR_EMH: CardRecord = CardRecord::new(
    "The Doctor, EMH",
    "8fbad0dd-5f74-40c0-9e3e-c06be8eabb14",
    "David Robert Hovey",
    CardRules::unsupported(),
);

// TRK 235 — The Duras Sisters
// Audit: unsupported — Needs an executable declaration for When The Duras Sisters die, create a 2/1 red Klingon
// Warrior creature token with haste.
pub(in crate::card::sets) static THE_DURAS_SISTERS: CardRecord = CardRecord::new(
    "The Duras Sisters",
    "7eb112eb-2abf-4ba3-b9bd-b45ffa80f7eb",
    "Harkalé Linaï",
    CardRules::unsupported(),
);

// TRK 236 — D'Vana Tendi, Optimistic Ensign
// Audit: unsupported — Needs an executable declaration for {T}: Untap another target permanent. {3}: Surveil 1. (Look
// at the top card of your library. You may put it into your graveyard.)
pub(in crate::card::sets) static D_VANA_TENDI_OPTIMISTIC_ENSIGN: CardRecord = CardRecord::new(
    "D'Vana Tendi, Optimistic Ensign",
    "206239ca-341a-4f07-8149-a24ccf20ee45",
    "Micah Epstein",
    CardRules::unsupported(),
);

// TRK 237 — Gowron, Eager for Battle
// Audit: unsupported — Needs an executable declaration for Whenever another creature dies, put a +1/+1 counter on
// Gowron. Sacrifice another creature: Gowron gains trample until end of turn. If you
// sacrificed a Klingon this way, you draw a card and lose 1 life.
pub(in crate::card::sets) static GOWRON_EAGER_FOR_BATTLE: CardRecord = CardRecord::new(
    "Gowron, Eager for Battle",
    "d60613c2-5655-4051-babd-fc0f3a73fc77",
    "Grant Griffin",
    CardRules::unsupported(),
);

// TRK 238 — Gray and Adira, Forever Joined
// Audit: unsupported — Needs an executable declaration for At the beginning of combat on your turn, if you've cast a
// noncreature spell this turn, create a token that's a copy of target creature you control,
// except it isn't legendary. It gains haste until end of turn. Sacrifice it at the beginning
// of the next end step.
pub(in crate::card::sets) static GRAY_AND_ADIRA_FOREVER_JOINED: CardRecord = CardRecord::new(
    "Gray and Adira, Forever Joined",
    "eede6822-c8bc-4344-81a9-bc608245d5b6",
    "Julia Metzger",
    CardRules::unsupported(),
);

// TRK 239 — Hemmer, Fixing What Is Broken
// Audit: unsupported — Needs an executable declaration for During your turn, Spacecraft you control have hexproof.
// (They can't be the targets of spells or abilities your opponents control.) Whenever Hemmer
// becomes tapped, draw a card, then discard a card.
pub(in crate::card::sets) static HEMMER_FIXING_WHAT_IS_BROKEN: CardRecord = CardRecord::new(
    "Hemmer, Fixing What Is Broken",
    "f3e730d7-3fd1-4bd2-a35b-6fb85bfd3c65",
    "Riccardo Federici",
    CardRules::unsupported(),
);

// TRK 240 — Hit It!
// Audit: unsupported — Needs an executable declaration for Creatures you control have haste. When this enchantment
// enters, look at the top five cards of your library. Put a land card from among them onto the
// battlefield tapped and the rest into your graveyard.
pub(in crate::card::sets) static HIT_IT: CardRecord = CardRecord::new(
    "Hit It!",
    "6266506f-fabf-4b53-8f93-6e2115a3e5a4",
    "Aaron J. Riley",
    CardRules::unsupported(),
);

// TRK 241 — The Inner Light
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I — Return target green or white creature card from your
// graveyard to the battlefield. II — Choose target creature. Whenever it deals combat damage
// to a player this turn, you draw that many cards. III — You gain life equal to the greatest
// power among creatures you control. Create a Treasure token.
pub(in crate::card::sets) static THE_INNER_LIGHT: CardRecord = CardRecord::new(
    "The Inner Light",
    "af92a773-b237-4cd8-b50d-4f7c3b9290ed",
    "Marko Manev",
    CardRules::unsupported(),
);

// TRK 242 — Kira Nerys, Firebrand
// Audit: unsupported — Needs an executable declaration for Trample If a creature you control would enter while you
// control an attacking creature, instead it enters tapped and attacking. Whenever another
// creature you control enters, it gets +1/+1 until end of turn.
pub(in crate::card::sets) static KIRA_NERYS_FIREBRAND: CardRecord = CardRecord::new(
    "Kira Nerys, Firebrand",
    "81ad6427-e730-47c9-8310-6d3ca4b10c82",
    "Pauline Voss",
    CardRules::unsupported(),
);

// TRK 243 — Kirk, Commanding Officer
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, exile the top card of your library. You may play that card this turn.
// {2}, Sacrifice Kirk: Destroy target artifact or enchantment.
pub(in crate::card::sets) static KIRK_COMMANDING_OFFICER: CardRecord = CardRecord::new(
    "Kirk, Commanding Officer",
    "245d0430-0967-44f1-8c59-985104a8db30",
    "Cliff Nielsen",
    CardRules::unsupported(),
);

// TRK 244 — Live Long and Prosper
// Audit: unsupported — Needs an executable declaration for This spell can't be countered. Double your life total. Draw
// cards equal to the number of cards in your hand. You have no maximum hand size until your
// next turn.
pub(in crate::card::sets) static LIVE_LONG_AND_PROSPER: CardRecord = CardRecord::new(
    "Live Long and Prosper",
    "0fc50ffc-3b4d-4fb3-898f-c308a63eb6e9",
    "Stella Spente",
    CardRules::unsupported(),
);

// TRK 245 — Locutus, the One Who Speaks
// Audit: unsupported — Needs an executable declaration for Artifact creatures you control get +1/+1. {W}{B}, Discard
// this card: It deals 2 damage to target creature. You gain 2 life.
pub(in crate::card::sets) static LOCUTUS_THE_ONE_WHO_SPEAKS: CardRecord = CardRecord::new(
    "Locutus, the One Who Speaks",
    "296f1127-14a0-42c4-9e4a-c0d929720748",
    "Ariel Orea",
    CardRules::unsupported(),
);

// TRK 246 — Lore, Positronic Provocateur
// Audit: unsupported — Needs an executable declaration for When Lore enters, choose one — • Target creature gets +1/+0
// until end of turn and can't be blocked this turn. • Surveil 2. (Look at the top two cards of
// your library, then put any number of them into your graveyard and the rest on top of your
// library in any order.)
pub(in crate::card::sets) static LORE_POSITRONIC_PROVOCATEUR: CardRecord = CardRecord::new(
    "Lore, Positronic Provocateur",
    "62afa55d-d4ba-48d5-9991-0f8408f9c92c",
    "Wei Guan",
    CardRules::unsupported(),
);

// TRK 247 — L'Rell, Fierce Visionary
// Audit: unsupported — Needs an executable declaration for At the beginning of combat on your turn, create a 2/1 red
// Klingon Warrior creature token with haste. Sacrifice it at the beginning of the next end
// step. {2}, Sacrifice another creature or artifact: Surveil 1. (Look at the top card of your
// library. You may put it into your graveyard.)
pub(in crate::card::sets) static L_RELL_FIERCE_VISIONARY: CardRecord = CardRecord::new(
    "L'Rell, Fierce Visionary",
    "7b06bbb9-7c6d-4889-b55a-1de77c27fd18",
    "Patricia Pria",
    CardRules::unsupported(),
);

// TRK 248 — Mirror Universe Intrusion
// Audit: unsupported — Needs an executable declaration for Kicker {4}{U}{B} (You may pay an additional {4}{U}{B} as
// you cast this spell.) Create a token that's a copy of target creature an opponent controls,
// except it has menace. If this spell was kicked, instead for each creature target opponent
// controls, create a token that's a copy of that creature, except it has menace.
pub(in crate::card::sets) static MIRROR_UNIVERSE_INTRUSION: CardRecord = CardRecord::new(
    "Mirror Universe Intrusion",
    "3507f99b-15fb-4f66-9484-8e8115e7f5de",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// TRK 249 — Nog, Ambitious Cadet
// Audit: unsupported — Needs an executable declaration for Menace {1}: If Nog is a Rogue, he becomes a Ferengi Scout
// with base power and toughness 3/4 and gains "Whenever Nog becomes tapped, surveil 2." {3}:
// If Nog is a Scout, he becomes a Ferengi Officer and gains "You may play lands from your
// graveyard" and "You may play an additional land on each of your turns."
pub(in crate::card::sets) static NOG_AMBITIOUS_CADET: CardRecord = CardRecord::new(
    "Nog, Ambitious Cadet",
    "b513dd60-8573-467b-9c6a-855a0ba86fca",
    "Darren Tan",
    CardRules::unsupported(),
);

// TRK 250 — Phlox, Unorthodox Doctor
// Audit: unsupported — Needs an executable declaration for Trample Whenever a creature you control with a counter on
// it deals combat damage to a player, draw a card.
pub(in crate::card::sets) static PHLOX_UNORTHODOX_DOCTOR: CardRecord = CardRecord::new(
    "Phlox, Unorthodox Doctor",
    "2695e76e-c27d-49fc-9ef2-d9eaff449f3d",
    "Jason Seiler",
    CardRules::unsupported(),
);

// TRK 251 — Sarek, Federation Ambassador
// Audit: unsupported — Needs an executable declaration for Federation — At the beginning of combat on your turn, draw
// a card for every five creature types among non-Borg creatures you control. If you draw three
// or more cards this way, put two +1/+1 counters on each creature you control.
pub(in crate::card::sets) static SAREK_FEDERATION_AMBASSADOR: CardRecord = CardRecord::new(
    "Sarek, Federation Ambassador",
    "94ad9063-9663-49d8-b7ba-05be865bd20a",
    "Darren Tan",
    CardRules::unsupported(),
);

// TRK 252 — Seven of Nine
// Audit: unsupported — Needs an executable declaration for Whenever you face a dilemma, draw a card. (You face a
// dilemma as you choose one or more modes for a spell or ability.)
pub(in crate::card::sets) static SEVEN_OF_NINE: CardRecord = CardRecord::new(
    "Seven of Nine",
    "6204cfdb-6fb4-4030-96b6-9306c8266bcf",
    "Bastien L. Deharme",
    CardRules::unsupported(),
);

// TRK 253 — Spock, Always Your Friend
// Audit: unsupported — Needs an executable declaration for When Spock enters, look at the top four cards of your
// library. You may reveal a noncreature, nonland card from among them and put it into your
// hand. Put the rest on the bottom of your library in any order.
pub(in crate::card::sets) static SPOCK_ALWAYS_YOUR_FRIEND: CardRecord = CardRecord::new(
    "Spock, Always Your Friend",
    "1be731d5-8b21-4883-91c0-dd9070b1eadd",
    "Chris Rallis",
    CardRules::unsupported(),
);

// TRK 254 — The Traveler
// Audit: unsupported — Needs an executable declaration for Reach, vigilance Strange new worlds — The Traveler's power
// is equal to the number of differently named lands you control.
pub(in crate::card::sets) static THE_TRAVELER: CardRecord = CardRecord::new(
    "The Traveler",
    "171932ba-26da-48e9-b8a2-c71aafe5c6a6",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// TRK 255 — Vash, Freelance Archaeologist
// Audit: unsupported — Needs an executable declaration for At the beginning of your upkeep, surveil 1. (Look at the
// top card of your library. You may put it into your graveyard.) Whenever a card leaves your
// graveyard, draw a card. This ability triggers only once each turn.
pub(in crate::card::sets) static VASH_FREELANCE_ARCHAEOLOGIST: CardRecord = CardRecord::new(
    "Vash, Freelance Archaeologist",
    "d1f51765-f336-4ca2-a0f5-e42dc352c0fa",
    "Nicky Barkla",
    CardRules::unsupported(),
);

// TRK 256 — Worf, Honor-Bound Warrior
// Audit: unsupported — Needs an executable declaration for Whenever you attack while creatures you control have total
// power 8 or greater, double Worf's power until end of turn. Tap two untapped artifacts and/or
// creatures you control: Put a +1/+1 counter on Worf. Activate only as a sorcery.
pub(in crate::card::sets) static WORF_HONOR_BOUND_WARRIOR: CardRecord = CardRecord::new(
    "Worf, Honor-Bound Warrior",
    "b2d1955f-e834-4157-a139-9b6ec94bcd2a",
    "Alexander Mokhov",
    CardRules::unsupported(),
);

// TRK 257 — Year of Hell
// Audit: unsupported — Needs an executable declaration for (As this Saga enters and after your draw step, add a lore
// counter. Sacrifice after III.) I, II, III — Each player draws a card. This Saga deals 2
// damage to each player.
pub(in crate::card::sets) static YEAR_OF_HELL: CardRecord = CardRecord::new(
    "Year of Hell",
    "4ab931e9-fb07-4da2-a0d9-acfec2ac9e19",
    "Jonathan Bartlett",
    CardRules::unsupported(),
);

// TRK 258 — Zero, Incorporeal Sensate
// Audit: unsupported — Needs an executable declaration for Lifelink When Zero enters, look at target opponent's hand.
// You may exile a nonland card from it. For as long as that card remains exiled, its owner may
// play it. A spell cast this way costs {2} more to cast.
pub(in crate::card::sets) static ZERO_INCORPOREAL_SENSATE: CardRecord = CardRecord::new(
    "Zero, Incorporeal Sensate",
    "93f616fe-6906-4e5d-b9fe-b432cccdb0ec",
    "Racrufi",
    CardRules::unsupported(),
);

// TRK 259 — Class-F Shuttlecraft
// Audit: unsupported — Needs an executable declaration for {T}: Add one mana of any color. Station (Tap another
// creature you control: Put charge counters equal to its power on this Spacecraft. Station
// only as a sorcery. It's an artifact creature at 6+.) 6+ | Flying, vigilance
pub(in crate::card::sets) static CLASS_F_SHUTTLECRAFT: CardRecord = CardRecord::new(
    "Class-F Shuttlecraft",
    "55e0a183-4145-472f-8bed-a1f247febe06",
    "William Phifer",
    CardRules::unsupported(),
);

// TRK 260 — Combadge
// Audit: unsupported — Needs an executable declaration for When you cast this spell, copy it. (The copy becomes a
// token.) Equipped creature has "Whenever this creature attacks while you control another
// creature with a Communicator attached to it, this creature can't be blocked this turn."
// Equip {2} ({2}: Attach to target creature you control. Equip only as a sorcery.)
pub(in crate::card::sets) static COMBADGE: CardRecord = CardRecord::new(
    "Combadge",
    "8bcf6bb5-f689-4d61-8f69-3290eeab539d",
    "Paolo Puggioni",
    CardRules::unsupported(),
);

// TRK 261 — Crystalline Entity
// Audit: unsupported — Needs an executable declaration for Flying When this creature enters, if you cast it, destroy
// all nonartifact creatures.
pub(in crate::card::sets) static CRYSTALLINE_ENTITY: CardRecord = CardRecord::new(
    "Crystalline Entity",
    "cbc4ac70-0a4c-4973-a366-dd67d8901e21",
    "David Álvarez",
    CardRules::unsupported(),
);

// TRK 262 — Cybernetic Specialist
// Audit: unsupported — Needs an executable declaration for When this creature enters, surveil 1. (Look at the top card
// of your library. You may put it into your graveyard.) {1}, {T}: Put target card from your
// graveyard on the bottom of your library.
pub(in crate::card::sets) static CYBERNETIC_SPECIALIST: CardRecord = CardRecord::new(
    "Cybernetic Specialist",
    "b147f16e-a2d6-4d43-9213-9a2723b0d3ef",
    "Joan Piqué Llorens",
    CardRules::unsupported(),
);

// TRK 263 — Deep Space 9
// Audit: unsupported — Needs an executable declaration for When Deep Space 9 enters and at the beginning of your
// upkeep, exile the top four cards of your library. Put all permanent cards exiled this way
// onto the battlefield. Station (Tap another creature you control: Put charge counters equal
// to its power on this Spacecraft. Station only as a sorcery.) 9+ | Creatures you control get
// +3/+3.
pub(in crate::card::sets) static DEEP_SPACE_9: CardRecord = CardRecord::new(
    "Deep Space 9",
    "76b039c5-e0a1-410a-954b-52f304e31f7f",
    "Paolo Puggioni",
    CardRules::unsupported(),
);

// TRK 264 — Engineered Explosives (reprint)
const ENGINEERED_EXPLOSIVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::fifth_dawn::ENGINEERED_EXPLOSIVES,
    "ef2d8db7-3f86-4d87-97ca-368444dc7f16",
    "David Álvarez",
);

// TRK 265 — The Holodeck
// Audit: unsupported — Needs an executable declaration for Once during each of your turns, you may play a permanent
// card you own from outside the game or from your graveyard.
pub(in crate::card::sets) static THE_HOLODECK: CardRecord = CardRecord::new(
    "The Holodeck",
    "57e830f5-564d-4899-87c0-74192c561fc8",
    "Jessica Rossier",
    CardRules::unsupported(),
);

// TRK 266 — Isolinear Processor
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. Spend this mana only to cast an artifact
// spell or activate an ability of an artifact source. {2}, {T}: Draw a card, then discard a
// card.
pub(in crate::card::sets) static ISOLINEAR_PROCESSOR: CardRecord = CardRecord::new(
    "Isolinear Processor",
    "8c1a2c13-5e36-4707-9d90-18c2e58a2b26",
    "David Álvarez",
    CardRules::unsupported(),
);

// TRK 267 — Replicator
// Audit: unsupported — Needs an executable declaration for When this artifact enters, create a Food token. (It's an
// artifact with "{2}, {T}, Sacrifice this token: You gain 3 life.") {2}, {T}: Create a tapped
// token that's a copy of target token you control.
pub(in crate::card::sets) static REPLICATOR: CardRecord = CardRecord::new(
    "Replicator",
    "ae846f45-d87b-47d4-87f6-2d9aa19c3c40",
    "William Phifer",
    CardRules::unsupported(),
);

// TRK 268 — Sickbay
// Audit: unsupported — Needs an executable declaration for Whenever a creature you control enters, you gain 1 life. At
// the beginning of combat on your turn, put X +1/+1 counters on target creature you control,
// where X is one plus the amount of life you gained this turn.
pub(in crate::card::sets) static SICKBAY: CardRecord = CardRecord::new(
    "Sickbay",
    "05735357-3cca-42be-a3ae-5b4fc1fd5f47",
    "Joel Chaim Holtzman",
    CardRules::unsupported(),
);

// TRK 269 — Tea, Earl Grey, Hot
// Audit: unsupported — Needs an executable declaration for When this artifact enters, draw a card. {2}, {T}, Sacrifice
// this artifact: Choose one — • This artifact deals 2 damage to target opponent. • You gain 3
// life.
pub(in crate::card::sets) static TEA_EARL_GREY_HOT: CardRecord = CardRecord::new(
    "Tea, Earl Grey, Hot",
    "289e8ebe-ab0f-466b-a4e6-d5cf8411e724",
    "Raymond Bonilla",
    CardRules::unsupported(),
);

// TRK 270 — Tricorder
// Audit: unsupported — Needs an executable declaration for Equipped creature gets +1/+1. Whenever equipped creature
// attacks, surveil 1. (Look at the top card of your library. You may put it into your
// graveyard.) Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)
pub(in crate::card::sets) static TRICORDER: CardRecord = CardRecord::new(
    "Tricorder",
    "3c872f02-80ac-49e3-a777-3526d35ad7cc",
    "Tony Foti",
    CardRules::unsupported(),
);

// TRK 271 — Type-2 Phaser
// Audit: unsupported — Needs an executable declaration for When this Equipment enters, tap target creature an opponent
// controls and put a stun counter on it. (If a permanent with a stun counter would become
// untapped, remove one from it instead.) Equipped creature gets +3/+0. Equip {3} ({3}: Attach
// to target creature you control. Equip only as a sorcery.)
pub(in crate::card::sets) static TYPE_2_PHASER: CardRecord = CardRecord::new(
    "Type-2 Phaser",
    "c704af22-7c42-45c7-b3a4-b414c4dd8e05",
    "Wero Gallo",
    CardRules::unsupported(),
);

// TRK 272 — U.S.S. Cerritos, Following Up
// Audit: unsupported — Needs an executable declaration for When U.S.S. Cerritos enters, create two 1/1 red Officer
// creature tokens. Station (Tap another creature you control: Put charge counters equal to its
// power on this Spacecraft. Station only as a sorcery. It's an artifact creature at 6+.) 6+ |
// Flying
pub(in crate::card::sets) static U_S_S_CERRITOS_FOLLOWING_UP: CardRecord = CardRecord::new(
    "U.S.S. Cerritos, Following Up",
    "76bf7521-5024-4fdc-9ea1-5e10f2647efb",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// TRK 273 — U.S.S. Enterprise-D, Galaxy-Class
// Audit: unsupported — Needs an executable declaration for Whenever one or more charge counters are put on U.S.S.
// Enterprise-D for the first time each turn, exile the top card of your library. You may play
// that card this turn. Station (Tap another creature you control: Put charge counters equal to
// its power on this Spacecraft. Station only as a sorcery. It's an artifact creature at 7+.)
// 7+ | Flying, vigilance
pub(in crate::card::sets) static U_S_S_ENTERPRISE_D_GALAXY_CLASS: CardRecord = CardRecord::new(
    "U.S.S. Enterprise-D, Galaxy-Class",
    "057a4413-6a17-491e-bfd7-6cd427b1a442",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// TRK 274 — Warp Core
// Audit: unsupported — Needs an executable declaration for When this artifact enters, you may search your library for
// a basic land card, reveal it, put it into your hand, then shuffle. {1}, {T}: Put two charge
// counters on target Spacecraft. Activate only as a sorcery.
pub(in crate::card::sets) static WARP_CORE: CardRecord = CardRecord::new(
    "Warp Core",
    "de8e2bde-b368-4d10-8409-86f5da6997f6",
    "Leon Tukker",
    CardRules::unsupported(),
);

// TRK 275 — Andoria
pub(in crate::card::sets) static ANDORIA: CardRecord = CardRecord::new(
    "Andoria",
    "0e9e72ef-3e34-4d19-aec0-b5e1ba84d244",
    "Josu Hernaiz",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {U} or {R}.",
            AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red]),
        ),
    ]),
);

// TRK 276 — Artificial Anomaly
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {3}{B}, {T}, Sacrifice this land: Each
// opponent loses 2 life and you gain 2 life. Create a tapped Planet land token named New
// Planet with "{T}: Add one mana of any color."
pub(in crate::card::sets) static ARTIFICIAL_ANOMALY: CardRecord = CardRecord::new(
    "Artificial Anomaly",
    "7dfead74-0625-4f6a-b66b-6d5d363a387c",
    "Jonas De Ro",
    CardRules::unsupported(),
);

// TRK 277 — Bajor
pub(in crate::card::sets) static BAJOR: CardRecord = CardRecord::new(
    "Bajor",
    "ff5254a6-4961-4604-b65b-2eb0ce9b1c82",
    "John Di Giovanni",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {R} or {W}.",
            AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::White]),
        ),
    ]),
);

// TRK 278 — Blood Crypt (reprint)
const BLOOD_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dissension::BLOOD_CRYPT,
    "1b7eb998-3ec0-4cbc-a416-5a1e3e5a7316",
    "Jonas De Ro",
);

// TRK 279 — Breeding Pool (reprint)
const BREEDING_POOL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dissension::BREEDING_POOL,
    "63e4dc07-c742-41bd-8301-861637908fd1",
    "Cliff Nielsen",
);

// TRK 280 — Cardassia Prime
pub(in crate::card::sets) static CARDASSIA_PRIME: CardRecord = CardRecord::new(
    "Cardassia Prime",
    "d634424f-9148-46a5-95e3-4965a9226bb4",
    "Alex Negrea",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {W} or {B}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Black]),
        ),
    ]),
);

// TRK 281 — Earth, Sector 001
pub(in crate::card::sets) static EARTH_SECTOR_001: CardRecord = CardRecord::new(
    "Earth, Sector 001",
    "5278d354-97c4-4c21-a902-2f9252b151eb",
    "Colin Searle",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {G} or {U}.",
            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::Blue]),
        ),
    ]),
);

// TRK 282 — Federation Outpost
// Audit: unsupported — Needs an executable declaration for This land enters tapped unless you control two or more
// other lands. When this land enters untapped, choose one — • You gain 2 life. • Surveil 1.
// {T}: Add {W}.
pub(in crate::card::sets) static FEDERATION_OUTPOST: CardRecord = CardRecord::new(
    "Federation Outpost",
    "02e368bd-ece8-4201-8d1a-4c5287d5cfee",
    "Edouard",
    CardRules::unsupported(),
);

// TRK 283 — Ferenginar
pub(in crate::card::sets) static FERENGINAR: CardRecord = CardRecord::new(
    "Ferenginar",
    "17fab75f-7ff9-4c50-a6f7-03a85cd275bf",
    "Colin Searle",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {B} or {G}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green]),
        ),
    ]),
);

// TRK 284 — Gaseous Anomaly
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {5}{U}, {T}, Sacrifice this land: Draw two
// cards. Create a tapped Planet land token named New Planet with "{T}: Add one mana of any
// color."
pub(in crate::card::sets) static GASEOUS_ANOMALY: CardRecord = CardRecord::new(
    "Gaseous Anomaly",
    "e6974579-7e88-4d7f-9b05-93d08167cb17",
    "Racrufi",
    CardRules::unsupported(),
);

// TRK 285 — Godless Shrine (reprint)
const GODLESS_SHRINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::guildpact::GODLESS_SHRINE,
    "8fbd1ae0-3d4c-492a-a1ea-85a95fa3d7b6",
    "John Thacker",
);

// TRK 286 — Hallowed Fountain (reprint)
const HALLOWED_FOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dissension::HALLOWED_FOUNTAIN,
    "b7285986-7e08-4969-86ef-452dc5bfdd9f",
    "Piotr Dura",
);

// TRK 287 — Ni'Var Science Academy
// Audit: unsupported — Needs an executable declaration for This land enters tapped unless you control two or more
// other lands. When this land enters untapped, choose one — • Target creature gains flying
// until end of turn. • Surveil 1. {T}: Add {U}.
pub(in crate::card::sets) static NI_VAR_SCIENCE_ACADEMY: CardRecord = CardRecord::new(
    "Ni'Var Science Academy",
    "a785dcd0-2df5-4d31-a01a-61fa7a1f1806",
    "Raphael Lacoste",
    CardRules::unsupported(),
);

// TRK 288 — Orion
pub(in crate::card::sets) static ORION: CardRecord = CardRecord::new(
    "Orion",
    "dec37ff4-d6e9-4b87-865b-125b17162465",
    "Simon Dominic",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {R} or {G}.",
            AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::Green]),
        ),
    ]),
);

// TRK 289 — Overgrown Tomb (reprint)
const OVERGROWN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ravnica_city_of_guilds::OVERGROWN_TOMB,
    "ad7e18e2-c033-4b6c-86e8-d0e5cc824cfd",
    "Pavel Kolomeyets",
);

// TRK 290 — Q Continuum
// Audit: unsupported — Needs an executable declaration for This land enters tapped unless you control two or more
// other lands. When this land enters untapped, choose one — • Target creature gets +1/+1 and
// gains vigilance until end of turn. • Surveil 1. {T}: Add {G}.
pub(in crate::card::sets) static Q_CONTINUUM: CardRecord = CardRecord::new(
    "Q Continuum",
    "cd424080-e8ad-4a56-a820-20d8534fba31",
    "Jonas De Ro",
    CardRules::unsupported(),
);

// TRK 291 — Qo'noS
pub(in crate::card::sets) static QO_NOS: CardRecord = CardRecord::new(
    "Qo'noS",
    "f1558d93-8a8a-45e0-bcdc-e6cd42f716dc",
    "Rockey Chen",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {B} or {R}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Red]),
        ),
    ]),
);

// TRK 292 — Quark's Bar
// Audit: unsupported — Needs an executable declaration for This land enters tapped unless you control two or more
// other lands. When this land enters untapped, choose one — • This land deals 1 damage to each
// opponent. • Surveil 1. {T}: Add {R}.
pub(in crate::card::sets) static QUARK_S_BAR: CardRecord = CardRecord::new(
    "Quark's Bar",
    "8422ee4a-9845-429d-bda8-74852168dcba",
    "Robin Olausson",
    CardRules::unsupported(),
);

// TRK 293 — Risa
pub(in crate::card::sets) static RISA: CardRecord = CardRecord::new(
    "Risa",
    "c89e7680-9868-4fea-9aaf-6af03bfa0692",
    "Colin Searle",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {G} or {W}.",
            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::White]),
        ),
    ]),
);

// TRK 294 — Romulus
pub(in crate::card::sets) static ROMULUS: CardRecord = CardRecord::new(
    "Romulus",
    "1bce111b-b07c-4589-9b6d-f0b5f320808c",
    "Carlos Dattoli",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {U} or {B}.",
            AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black]),
        ),
    ]),
);

// TRK 295 — Sacred Foundry (reprint)
const SACRED_FOUNDRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ravnica_city_of_guilds::SACRED_FOUNDRY,
    "a7758cc6-4e18-48a5-8720-5f42b5cd9d31",
    "Jessica Rossier",
);

// TRK 296 — Spatial Anomaly
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {4}{R}, {T}, Sacrifice this land: It deals 2
// damage to target creature. Create a tapped Planet land token named New Planet with "{T}: Add
// one mana of any color."
pub(in crate::card::sets) static SPATIAL_ANOMALY: CardRecord = CardRecord::new(
    "Spatial Anomaly",
    "2f683f37-4291-410d-b179-ed6139c06877",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// TRK 297 — Starfleet Academy
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. Federation — {5}, {T}: Choose a color. Add an
// amount of mana of that color equal to the number of creature types among non-Borg creatures
// you control.
pub(in crate::card::sets) static STARFLEET_ACADEMY: CardRecord = CardRecord::new(
    "Starfleet Academy",
    "2da8d3bf-4397-4844-90d6-dd888becc804",
    "Robin Olausson",
    CardRules::unsupported(),
);

// TRK 298 — Steam Vents (reprint)
const STEAM_VENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::guildpact::STEAM_VENTS,
    "a83903c7-fd51-4526-aed2-359e946fea36",
    "Shahab Alizadeh",
);

// TRK 299 — Stomping Ground (reprint)
const STOMPING_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::guildpact::STOMPING_GROUND,
    "1ee8255a-44f6-4faa-9843-15432bf751ea",
    "Arthur Haas",
);

// TRK 300 — Subspace Anomaly
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {4}{W}, {T}, Sacrifice this land: Create two
// 1/1 red Officer creature tokens. Create a tapped Planet land token named New Planet with
// "{T}: Add one mana of any color."
pub(in crate::card::sets) static SUBSPACE_ANOMALY: CardRecord = CardRecord::new(
    "Subspace Anomaly",
    "ed62600f-84ae-421e-970a-fd0682c8f280",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// TRK 301 — Temple Garden (reprint)
const TEMPLE_GARDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ravnica_city_of_guilds::TEMPLE_GARDEN,
    "b9b0589d-f327-46a7-8bac-06b7654c547a",
    "Shahab Alizadeh",
);

// TRK 302 — Temporal Anomaly
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {3}{G}, {T}, Sacrifice this land: Mill four
// cards. You may put a permanent card from among them into your hand. Create a tapped Planet
// land token named New Planet with "{T}: Add one mana of any color."
pub(in crate::card::sets) static TEMPORAL_ANOMALY: CardRecord = CardRecord::new(
    "Temporal Anomaly",
    "d2000219-0a57-48e6-ab0f-d5ad0fb7bcf6",
    "Neil Blevins",
    CardRules::unsupported(),
);

// TRK 303 — Unexplored Territory
// Audit: unsupported — Needs an executable declaration for {T}: Add {C}. {1}, {T}, Sacrifice this land: Search your
// library for a basic land card, put it onto the battlefield tapped, then shuffle.
pub(in crate::card::sets) static UNEXPLORED_TERRITORY: CardRecord = CardRecord::new(
    "Unexplored Territory",
    "ca1e60f1-11a9-4bb8-9f6e-5b652b49c517",
    "Raymond Bonilla",
    CardRules::unsupported(),
);

// TRK 304 — The Unicomplex
// Audit: unsupported — Needs an executable declaration for This land enters tapped unless you control two or more
// other lands. When this land enters untapped, choose one — • Put target creature card from
// your graveyard on top of your library. • Surveil 1. {T}: Add {B}.
pub(in crate::card::sets) static THE_UNICOMPLEX: CardRecord = CardRecord::new(
    "The Unicomplex",
    "4c881efb-d55b-45fe-9e5a-8a61aec6fbbb",
    "JB Casacop",
    CardRules::unsupported(),
);

// TRK 305 — Vulcan
pub(in crate::card::sets) static VULCAN: CardRecord = CardRecord::new(
    "Vulcan",
    "fc3fe0da-8d93-4f1c-9aad-c2fdc9fe34fc",
    "John Di Giovanni",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add {W} or {U}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue]),
        ),
    ]),
);

// TRK 306 — Watery Grave (reprint)
const WATERY_GRAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ravnica_city_of_guilds::WATERY_GRAVE,
    "5525d6a6-e532-4047-9da4-bfae7927fecc",
    "Raymond Bonilla",
);

// TRK 307 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::PLAINS,
    "6b2fba64-b85f-42bc-991e-4bb6201b78fb",
    "Paulius Daščioras",
);

// TRK 308 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::ISLAND,
    "1c5b534a-128f-4235-8f69-646a55ae4d67",
    "Guglielmo Bonis",
);

// TRK 309 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::SWAMP,
    "b4e1965a-dbab-4edb-ada9-43898c57c5fe",
    "Rytis Sabaliauskas",
);

// TRK 310 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::MOUNTAIN,
    "87908c55-4ac4-4b57-a8ee-a28640a18f0f",
    "Rytis Sabaliauskas",
);

// TRK 311 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::FOREST,
    "f026379b-f00f-4897-b03f-dd164d69b925",
    "Bengt Hampus Viklander",
);

// TRK 312 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    1,
    "b741bbf0-40b3-4ee7-9844-cecc64cb0840",
    "Shahab Alizadeh",
);

// TRK 313 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    1,
    "ee2e4482-afad-41ae-9e1a-120e805c4563",
    "Pablo Mendoza",
);

// TRK 314 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    1,
    "e046c0ef-0af3-4f48-8329-7d659e7d1a89",
    "Pablo Mendoza",
);

// TRK 315 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    1,
    "81564344-1c9b-4fac-8a7d-682f57331d61",
    "Pablo Mendoza",
);

// TRK 316 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    1,
    "c7ca6a54-f4e9-45f0-abb0-db14922cf157",
    "Shahab Alizadeh",
);

// TRK 317 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    2,
    "8ab0f4c0-b331-4c57-b68f-2e24bb5ba06c",
    "Bastien Grivet",
);

// TRK 318 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    3,
    "09b3c8bb-10ca-4b9f-8507-db6e8c52a918",
    "Leon Tukker",
);

// TRK 319 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    2,
    "f3cc07cd-cc79-4745-b0b7-eade60175cc3",
    "Bastien Grivet",
);

// TRK 320 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    3,
    "a44efa5f-8fbb-43e0-adc4-d05f4be6b9a6",
    "Leon Tukker",
);

// TRK 321 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    2,
    "b7387103-1df1-4fd0-9e91-1544509792c7",
    "Bastien Grivet",
);

// TRK 322 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    3,
    "214a9e76-b7fd-4cc9-89e7-104a71919a84",
    "Leon Tukker",
);

// TRK 323 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    2,
    "2a844b96-6616-4c39-8f4f-5d14a3b2bd55",
    "Bastien Grivet",
);

// TRK 324 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    3,
    "e5927fd9-dfaf-462e-9128-44b806e45178",
    "Leon Tukker",
);

// TRK 325 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    2,
    "dce15387-4114-4b3e-91aa-5b42b45c44ac",
    "Bastien Grivet",
);

// TRK 326 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    3,
    "4fae200d-5757-4924-8cf4-ab2a5ef2ccc0",
    "Leon Tukker",
);

// TRK 327 — Nyota Uhura, Communications (alternate printing)
const NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NYOTA_UHURA_COMMUNICATIONS,
    1,
    "cc8937ac-15fe-4d7a-90c8-853eb492151a",
    "Darius Zablockis",
);

// TRK 328 — Scotty, Miracle Worker (alternate printing)
const SCOTTY_MIRACLE_WORKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCOTTY_MIRACLE_WORKER,
    1,
    "db7418be-4881-4781-9b41-ea90f59aee8a",
    "Darius Zablockis",
);

// TRK 329 — Dr. Leonard "Bones" McCoy (alternate printing)
const DR_LEONARD_BONES_MCCOY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DR_LEONARD_BONES_MCCOY,
    1,
    "cff4d056-1050-4c08-9b8d-9941e20e974e",
    "Michael MacRae",
);

// TRK 330 — Kirk, Commanding Officer (alternate printing)
const KIRK_COMMANDING_OFFICER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KIRK_COMMANDING_OFFICER,
    1,
    "fdb533f7-a411-4505-978f-0f2e0838b96e",
    "Vlad Petruchik",
);

// TRK 331 — Spock, Always Your Friend (alternate printing)
const SPOCK_ALWAYS_YOUR_FRIEND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPOCK_ALWAYS_YOUR_FRIEND,
    1,
    "018d3844-0fa3-4b3a-805c-e9aeb9fc4185",
    "Rytis Sabaliauskas",
);

// TRK 332 — Federation Outpost (alternate printing)
const FEDERATION_OUTPOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEDERATION_OUTPOST,
    1,
    "85859d4c-c90c-48f4-bac0-75fcccf1b1f4",
    "Paulius Daščioras",
);

// TRK 333 — Ni'Var Science Academy (alternate printing)
const NI_VAR_SCIENCE_ACADEMY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NI_VAR_SCIENCE_ACADEMY,
    1,
    "86ba5cb2-f124-4947-9780-43fc24f37699",
    "Rimas Valeikis",
);

// TRK 334 — Q Continuum (alternate printing)
const Q_CONTINUUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &Q_CONTINUUM,
    1,
    "e9922ffc-bffa-414e-b6e3-de6d7f2ce02d",
    "Vlad Petruchik",
);

// TRK 335 — Quark's Bar (alternate printing)
const QUARK_S_BAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUARK_S_BAR,
    1,
    "b846d3f5-7937-4282-9fa3-95b2f134676b",
    "Vlad Petruchik",
);

// TRK 336 — Starfleet Academy (alternate printing)
const STARFLEET_ACADEMY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFLEET_ACADEMY,
    1,
    "eea2dc59-10e9-481b-b2c3-bbf44253ed56",
    "Rimas Valeikis",
);

// TRK 337 — The Unicomplex (alternate printing)
const THE_UNICOMPLEX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_UNICOMPLEX,
    1,
    "751eedef-1d14-44cf-bb52-99f8ada888b9",
    "Eugene Maslovski",
);

// TRK 338 — Resistance Is Futile (alternate printing)
const RESISTANCE_IS_FUTILE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESISTANCE_IS_FUTILE,
    1,
    "a092e283-24d5-47ec-a9cf-7d589a3121a4",
    "Gal Or",
);

// TRK 339 — Test of Courage (alternate printing)
const TEST_OF_COURAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEST_OF_COURAGE,
    1,
    "6cd8c8e4-a32e-4644-a990-70c649ce8985",
    "Gal Or",
);

// TRK 340 — Captain Kathryn Janeway (alternate printing)
const CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_KATHRYN_JANEWAY,
    1,
    "b16da038-5e35-4cd7-a157-624d4ed6bde3",
    "Gal Or",
);

// TRK 341 — Command Subadjunct (alternate printing)
const COMMAND_SUBADJUNCT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COMMAND_SUBADJUNCT,
    1,
    "f5435570-e78b-43cb-9b77-d80b92e07c2b",
    "Gal Or",
);

// TRK 342 — Conquest Mission (alternate printing)
const CONQUEST_MISSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONQUEST_MISSION,
    1,
    "be8d8f65-211e-47d7-973c-f44537b43826",
    "Gal Or",
);

// TRK 343 — Seven of Nine (alternate printing)
const SEVEN_OF_NINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEVEN_OF_NINE,
    1,
    "8cbe8fea-9c49-4d11-be37-4bc9eb03a406",
    "Gal Or",
);

// TRK 344 — Captain Benjamin Sisko (alternate printing)
const CAPTAIN_BENJAMIN_SISKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_BENJAMIN_SISKO,
    1,
    "c1344601-3f86-4910-b5ad-d7e1e58d851d",
    "Marija Tiurina",
);

// TRK 345 — Dominion Saboteur (alternate printing)
const DOMINION_SABOTEUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOMINION_SABOTEUR,
    1,
    "aabf7d12-f802-422e-a57c-24f5a2858c83",
    "Marija Tiurina",
);

// TRK 346 — The Holodeck (alternate printing)
const THE_HOLODECK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_HOLODECK,
    1,
    "19366f88-8411-428b-9903-8c9527686cac",
    "Marija Tiurina",
);

// TRK 347 — Quark, Humble Proprietor (alternate printing)
const QUARK_HUMBLE_PROPRIETOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUARK_HUMBLE_PROPRIETOR,
    1,
    "c06bc240-4dd2-4b77-be94-aecb882ea734",
    "Marija Tiurina",
);

// TRK 348 — Constable Odo (alternate printing)
const CONSTABLE_ODO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONSTABLE_ODO,
    1,
    "c0d0b4ee-72f6-45b6-8911-e3ef979e40a0",
    "Marija Tiurina",
);

// TRK 349 — Dominion Infiltration (alternate printing)
const DOMINION_INFILTRATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOMINION_INFILTRATION,
    1,
    "48b27529-4307-46ce-9eb4-431b1d6fcbc3",
    "Marija Tiurina",
);

// TRK 350 — Quark's Bar (alternate printing)
const QUARK_S_BAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QUARK_S_BAR,
    2,
    "ac124e83-bb99-4bf0-ac06-364d77771e81",
    "Marija Tiurina",
);

// TRK 351 — Nog, Ambitious Cadet (alternate printing)
const NOG_AMBITIOUS_CADET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOG_AMBITIOUS_CADET,
    1,
    "6cda814c-bdbe-486c-b956-ff2c7872124f",
    "Marija Tiurina",
);

// TRK 352 — Jadzia Dax, Living Legacy (alternate printing)
const JADZIA_DAX_LIVING_LEGACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JADZIA_DAX_LIVING_LEGACY,
    1,
    "6eff742c-9b8a-4458-976e-577d34e9c1d8",
    "Marija Tiurina",
);

// TRK 353 — Mr. Spock, Science Officer (alternate printing)
const MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MR_SPOCK_SCIENCE_OFFICER,
    1,
    "7dd9dec1-5793-4514-8a15-c3bce21a5115",
    "Mark Spears",
);

// TRK 354 — Mirror Universe Intrusion (alternate printing)
const MIRROR_UNIVERSE_INTRUSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRROR_UNIVERSE_INTRUSION,
    1,
    "99d6cd44-bf0d-436f-a398-e3bce9532dc9",
    "Mark Spears",
);

// TRK 355 — Kirk, Commanding Officer (alternate printing)
const KIRK_COMMANDING_OFFICER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KIRK_COMMANDING_OFFICER,
    2,
    "84f6d529-33f7-4bdf-8ee0-43f5c4e8de6a",
    "Mark Spears",
);

// TRK 356 — Sulu, Space Swashbuckler (alternate printing)
const SULU_SPACE_SWASHBUCKLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SULU_SPACE_SWASHBUCKLER,
    1,
    "e62f74b8-b862-448c-bc6e-06af40252212",
    "Mark Spears",
);

// TRK 357 — Transporter Malfunction (alternate printing)
const TRANSPORTER_MALFUNCTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRANSPORTER_MALFUNCTION,
    1,
    "e3ec4467-3a70-4f6b-87ca-957ab21ecd98",
    "Mark Spears",
);

// TRK 358 — Nyota Uhura, Communications (alternate printing)
const NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NYOTA_UHURA_COMMUNICATIONS,
    2,
    "fa9755ba-831d-47a2-a172-d2bfb13edefe",
    "Mark Spears",
);

// TRK 359 — Lieutenant Commander Data (alternate printing)
const LIEUTENANT_COMMANDER_DATA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIEUTENANT_COMMANDER_DATA,
    1,
    "f10ee26b-7ea8-4cd0-83f6-b9a8ac94cb0b",
    "Nicky Barkla",
);

// TRK 360 — Locutus, the One Who Speaks (alternate printing)
const LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOCUTUS_THE_ONE_WHO_SPEAKS,
    1,
    "ee059c0f-eba9-4504-a92e-af762a510ee4",
    "Nicky Barkla",
);

// TRK 361 — Infinite Diversity (alternate printing)
const INFINITE_DIVERSITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INFINITE_DIVERSITY,
    1,
    "2da35dd7-e6b8-47d4-9e92-f5158a4d9d7e",
    "Nicky Barkla",
);

// TRK 362 — Death Is Irrelevant (alternate printing)
const DEATH_IS_IRRELEVANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEATH_IS_IRRELEVANT,
    1,
    "905277a1-ae6b-4876-a724-3d58a84f76cf",
    "Nicky Barkla",
);

// TRK 363 — Captain Jean-Luc Picard (alternate printing)
const CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JEAN_LUC_PICARD,
    1,
    "566c03c3-aedd-4bd0-8932-e1b279e53c68",
    "Erikas Perl",
);

// TRK 364 — Command Subadjunct (alternate printing)
const COMMAND_SUBADJUNCT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COMMAND_SUBADJUNCT,
    2,
    "f2b6ca53-11de-4c63-a0ac-c9fa9a65c43e",
    "Michael MacRae",
);

// TRK 365 — Deanna Troi, Psychic Counsel (alternate printing)
const DEANNA_TROI_PSYCHIC_COUNSEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEANNA_TROI_PSYCHIC_COUNSEL,
    1,
    "4c8ee48d-d0bc-4b5c-ab82-066cd8bbf317",
    "Eglė Mosakaitė",
);

// TRK 366 — Dr. Beverly Crusher (alternate printing)
const DR_BEVERLY_CRUSHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DR_BEVERLY_CRUSHER,
    1,
    "15d800b3-f5ed-4267-b2e7-5de83a207512",
    "Anna Czeszejko",
);

// TRK 367 — Peace in Our Time (alternate printing)
const PEACE_IN_OUR_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PEACE_IN_OUR_TIME,
    1,
    "54da7aa7-3049-4f2a-a92e-62a0a7386de8",
    "Mintautas Šukys",
);

// TRK 368 — Primary Unimatrix Adjunct (alternate printing)
const PRIMARY_UNIMATRIX_ADJUNCT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PRIMARY_UNIMATRIX_ADJUNCT,
    1,
    "59ce5cdb-3e08-4de1-9335-aafeddfd1829",
    "Julia Vasilyeva",
);

// TRK 369 — There's Coffee in that Nebula (alternate printing)
const THERE_S_COFFEE_IN_THAT_NEBULA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THERE_S_COFFEE_IN_THAT_NEBULA,
    1,
    "07b3f211-1ff1-47c3-9063-9a1ac0fd814c",
    "Anna Czeszejko & Rutger van de Steeg",
);

// TRK 370 — Deep Space Scan (alternate printing)
const DEEP_SPACE_SCAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEP_SPACE_SCAN,
    1,
    "c53011f4-e2fb-480b-9eda-660e878a7b6a",
    "Maxim Ruabtsev",
);

// TRK 371 — Lieutenant Commander Data (alternate printing)
const LIEUTENANT_COMMANDER_DATA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LIEUTENANT_COMMANDER_DATA,
    2,
    "4753a94e-5a5c-4246-bcf4-907eb892507b",
    "Mintautas Šukys",
);

// TRK 372 — Lt. Cmdr. Geordi La Forge (alternate printing)
const LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LT_CMDR_GEORDI_LA_FORGE,
    1,
    "df9efa59-ef01-473d-b32a-8328dcf01130",
    "Vlad Petruchik",
);

// TRK 373 — Seven of Nine, Unassimilated (alternate printing)
const SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEVEN_OF_NINE_UNASSIMILATED,
    1,
    "ea6e116e-4086-42be-baf3-48255002d5d1",
    "Anna Czeszejko",
);

// TRK 374 — Wesley Crusher, Starfleet Cadet (alternate printing)
const WESLEY_CRUSHER_STARFLEET_CADET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WESLEY_CRUSHER_STARFLEET_CADET,
    1,
    "647f66af-2445-48ce-997a-3c5e94dcb77c",
    "Rimas Valeikis",
);

// TRK 375 — Death Is Irrelevant (alternate printing)
const DEATH_IS_IRRELEVANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEATH_IS_IRRELEVANT,
    2,
    "835a6118-a234-4db4-95d9-68b7af21ad87",
    "Paulius Daščioras",
);

// TRK 376 — Organic Avulsion Unit (alternate printing)
const ORGANIC_AVULSION_UNIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORGANIC_AVULSION_UNIT,
    1,
    "74029de7-fb85-440d-b149-4b820c41eaa5",
    "Gintas Galvanauskas",
);

// TRK 377 — Overflow Submatrix (alternate printing)
const OVERFLOW_SUBMATRIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERFLOW_SUBMATRIX,
    1,
    "33af9300-c477-4b47-9501-ebe03aa5a806",
    "Viktor Fetsch",
);

// TRK 378 — Resistance Is Futile (alternate printing)
const RESISTANCE_IS_FUTILE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RESISTANCE_IS_FUTILE,
    2,
    "e6707e65-388b-4579-8c8d-77307b8208c1",
    "Jurijus Chitrovas",
);

// TRK 379 — Sela, Tal Shiar Paradox (alternate printing)
const SELA_TAL_SHIAR_PARADOX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SELA_TAL_SHIAR_PARADOX,
    1,
    "71a9b698-e085-4141-80f7-6c57c2ea087f",
    "Jurijus Chitrovas",
);

// TRK 380 — Dominion Infiltration (alternate printing)
const DOMINION_INFILTRATION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DOMINION_INFILTRATION,
    2,
    "09fbeced-90d7-4cfc-80ae-7b6226d95de5",
    "Darius Zablockis",
);

// TRK 381 — Encounter the Unknown (alternate printing)
const ENCOUNTER_THE_UNKNOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENCOUNTER_THE_UNKNOWN,
    1,
    "0b45cf00-f72f-4b21-8d80-124893ed3c7f",
    "Eugene Maslovski",
);

// TRK 382 — Infinite Diversity (alternate printing)
const INFINITE_DIVERSITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &INFINITE_DIVERSITY,
    2,
    "534c939a-6c7f-4402-b405-bf2049989ea9",
    "Eglė Mosakaitė",
);

// TRK 383 — Q, Stacking the Deck (alternate printing)
const Q_STACKING_THE_DECK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &Q_STACKING_THE_DECK,
    1,
    "881693cb-1492-4347-88a5-e96dc907744b",
    "Viktor Fetsch",
);

// TRK 384 — Tasha Yar, Out of Time (alternate printing)
const TASHA_YAR_OUT_OF_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TASHA_YAR_OUT_OF_TIME,
    1,
    "e4575a63-090f-4013-a507-72dc7ea8146d",
    "Vilhelmas Banys",
);

// TRK 385 — Will Riker, First in the Field (alternate printing)
const WILL_RIKER_FIRST_IN_THE_FIELD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILL_RIKER_FIRST_IN_THE_FIELD,
    1,
    "5d6efbad-ea81-42e8-82f1-85ca5e849e69",
    "Vilhelmas Banys",
);

// TRK 386 — Captain Kathryn Janeway (alternate printing)
const CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_KATHRYN_JANEWAY,
    2,
    "ac990bba-7b0c-4235-9726-b944d66952ab",
    "Jurijus Chitrovas",
);

// TRK 387 — Warp Speed (alternate printing)
const WARP_SPEED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARP_SPEED,
    1,
    "ecbad54f-1258-48aa-be7b-a889ed99c1b6",
    "Maxim Ruabtsev",
);

// TRK 388 — Dathon, His Heart Open (alternate printing)
const DATHON_HIS_HEART_OPEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DATHON_HIS_HEART_OPEN,
    1,
    "51602acb-62f6-4d38-8a0e-f73497c8f512",
    "Vilhelmas Banys",
);

// TRK 389 — Locutus, the One Who Speaks (alternate printing)
const LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOCUTUS_THE_ONE_WHO_SPEAKS,
    2,
    "15387ef2-c761-4906-8909-4cf7dddefd9b",
    "Erikas Perl",
);

// TRK 390 — Worf, Honor-Bound Warrior (alternate printing)
const WORF_HONOR_BOUND_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORF_HONOR_BOUND_WARRIOR,
    1,
    "f237dcef-f193-4484-a616-66cb9423c409",
    "Vlad Petruchik",
);

// TRK 391 — Replicator (alternate printing)
const REPLICATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REPLICATOR,
    1,
    "e762c918-1c94-47a4-b141-ac16c5c9e073",
    "Vilhelmas Banys",
);

// TRK 392 — Hallowed Fountain (alternate printing)
const HALLOWED_FOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::HALLOWED_FOUNTAIN,
    1,
    "bbdbd348-49ab-4830-a5db-0e92b2f01b52",
    "Mark Poole",
);

// TRK 393 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::WATERY_GRAVE,
    1,
    "fa84ef4e-44ee-4b34-a454-8a19548ec3cc",
    "Andrew Mar",
);

// TRK 394 — Blood Crypt (alternate printing)
const BLOOD_CRYPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::BLOOD_CRYPT,
    1,
    "746e3a9e-b1b3-4d93-95bc-7ff0e4a2a2a7",
    "Jason A. Engle",
);

// TRK 395 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::STOMPING_GROUND,
    1,
    "e1614245-8a15-4139-acbc-e0e98c9fb9b2",
    "Daniel Romanovsky",
);

// TRK 396 — Temple Garden (alternate printing)
const TEMPLE_GARDEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::TEMPLE_GARDEN,
    1,
    "94c4b369-a997-41da-87df-5cc2faa9a5bd",
    "Jessica Rossier",
);

// TRK 397 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::GODLESS_SHRINE,
    1,
    "5c8a28db-be8b-4b6a-a28a-c5dc091030ed",
    "Raymond Swanland",
);

// TRK 398 — Steam Vents (alternate printing)
const STEAM_VENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::STEAM_VENTS,
    1,
    "e8e9d4cd-f94a-416f-aaa1-0c60bdd58391",
    "Chad Weatherford",
);

// TRK 399 — Overgrown Tomb (alternate printing)
const OVERGROWN_TOMB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::OVERGROWN_TOMB,
    1,
    "732fe13b-eddc-4e75-88f8-e079e30a7dc4",
    "Sam Burley",
);

// TRK 400 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::SACRED_FOUNDRY,
    1,
    "7fb2e855-b0ba-43e0-a4a0-15c9c44622c3",
    "Eliz Roxs",
);

// TRK 401 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::BREEDING_POOL,
    1,
    "e5f775cc-ab3a-4e5d-887f-ef40c45e3560",
    "Josu Solano",
);

// TRK 402 — Dr. McCoy, Surly Surgeon (alternate printing)
const DR_MCCOY_SURLY_SURGEON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DR_MCCOY_SURLY_SURGEON,
    1,
    "e4349c0f-55f8-40d5-a972-4fb655608b6a",
    "Chuck Lukacs",
);

// TRK 403 — Excalbian Gamemaster (alternate printing)
const EXCALBIAN_GAMEMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXCALBIAN_GAMEMASTER,
    1,
    "e6227228-330d-4bf5-a5ca-d5b5839e5874",
    "Chuck Lukacs",
);

// TRK 404 — Federation Resupply (alternate printing)
const FEDERATION_RESUPPLY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEDERATION_RESUPPLY,
    1,
    "d5b47af8-8e96-46b8-a127-0387d0e4ca78",
    "Jason Raish",
);

// TRK 405 — The Needs of the Many (alternate printing)
const THE_NEEDS_OF_THE_MANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_NEEDS_OF_THE_MANY,
    1,
    "a2e52d26-5e0d-436e-ac8c-65c399dc58be",
    "Jack Hughes",
);

// TRK 406 — Saavik, Stoic Student (alternate printing)
const SAAVIK_STOIC_STUDENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAAVIK_STOIC_STUDENT,
    1,
    "af3c4a7f-c7cb-422f-8eab-730de8f5c599",
    "Jack Hughes",
);

// TRK 407 — Sulu, Space Swashbuckler (alternate printing)
const SULU_SPACE_SWASHBUCKLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SULU_SPACE_SWASHBUCKLER,
    2,
    "01a51ffb-6d51-4e73-8cac-49fb32f14c02",
    "Jason Raish",
);

// TRK 408 — Troubling Tribbles (alternate printing)
const TROUBLING_TRIBBLES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TROUBLING_TRIBBLES,
    1,
    "1337f7ed-da1c-4fdd-9ad7-6f33fa5492bf",
    "Jason Raish",
);

// TRK 409 — Ensign Pavel Chekov (alternate printing)
const ENSIGN_PAVEL_CHEKOV_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENSIGN_PAVEL_CHEKOV,
    1,
    "578709ac-771b-410b-bd71-ba4ab8d466a2",
    "Marc Aspinall",
);

// TRK 410 — Kobayashi Maru Scenario (alternate printing)
const KOBAYASHI_MARU_SCENARIO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOBAYASHI_MARU_SCENARIO,
    1,
    "406806fa-213e-40ed-aaee-47ff21f36551",
    "Oliver Barrett",
);

// TRK 411 — Mr. Spock, Science Officer (alternate printing)
const MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MR_SPOCK_SCIENCE_OFFICER,
    2,
    "e3e54e58-327c-4071-b99a-b78c7cc3fe6f",
    "Grant Griffin",
);

// TRK 412 — Nyota Uhura, Communications (alternate printing)
const NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NYOTA_UHURA_COMMUNICATIONS,
    3,
    "2b71b656-a316-4f29-b2bd-44165ad8cebe",
    "Jack Hughes",
);

// TRK 413 — Scotty, Miracle Worker (alternate printing)
const SCOTTY_MIRACLE_WORKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SCOTTY_MIRACLE_WORKER,
    2,
    "d97123d6-8420-4400-8d6c-a3ed5c84ad5d",
    "Marc Aspinall",
);

// TRK 414 — The Destruction of Praxis (alternate printing)
const THE_DESTRUCTION_OF_PRAXIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DESTRUCTION_OF_PRAXIS,
    1,
    "ca2c9cda-1847-405b-a4c1-aee27459bad8",
    "Oliver Barrett",
);

// TRK 415 — General Chang, Cold Warrior (alternate printing)
const GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENERAL_CHANG_COLD_WARRIOR,
    1,
    "502673f3-2794-4b4c-8912-674cc4d2bbd2",
    "Oliver Barrett",
);

// TRK 416 — Khan, Vengeful Heart (alternate printing)
const KHAN_VENGEFUL_HEART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KHAN_VENGEFUL_HEART,
    1,
    "80dcc7dd-203f-4a3b-afe7-52987bbd006b",
    "Grant Griffin",
);

// TRK 417 — Salt Vampire (alternate printing)
const SALT_VAMPIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SALT_VAMPIRE,
    1,
    "5ad670fc-85ef-4f9b-9b5b-bac20f080bf5",
    "Simon Dominic",
);

// TRK 418 — Captain James T. Kirk (alternate printing)
const CAPTAIN_JAMES_T_KIRK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JAMES_T_KIRK,
    1,
    "367aa1e6-ce6c-41a0-ba7c-af00487a4302",
    "Josh Newton",
);

// TRK 419 — Khan, Despot by Design (alternate printing)
const KHAN_DESPOT_BY_DESIGN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KHAN_DESPOT_BY_DESIGN,
    1,
    "0c7fed60-4bec-47a4-81e3-4654e74386d2",
    "Jack Hughes",
);

// TRK 420 — David and Carol Marcus, Creators (alternate printing)
const DAVID_AND_CAROL_MARCUS_CREATORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAVID_AND_CAROL_MARCUS_CREATORS,
    1,
    "43091ed3-a412-48b4-8794-62c0a279e4b2",
    "Jason Raish",
);

// TRK 421 — Dr. Leonard "Bones" McCoy (alternate printing)
const DR_LEONARD_BONES_MCCOY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DR_LEONARD_BONES_MCCOY,
    2,
    "dab837b1-32f9-4a73-a6c4-04a7a30b42d0",
    "Jack Hughes",
);

// TRK 422 — Genesis Device (alternate printing)
const GENESIS_DEVICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENESIS_DEVICE,
    1,
    "dcaa11df-fc56-4dec-ab81-d65661082728",
    "Oliver Barrett",
);

// TRK 423 — Live Long and Prosper (alternate printing)
const LIVE_LONG_AND_PROSPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIVE_LONG_AND_PROSPER,
    1,
    "7de29930-ace7-49ce-9b35-acce4bf876cf",
    "Jack Hughes",
);

// TRK 424 — Mirror Universe Intrusion (alternate printing)
const MIRROR_UNIVERSE_INTRUSION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MIRROR_UNIVERSE_INTRUSION,
    2,
    "c49b5e07-a512-4d08-8fe1-07caefb28eec",
    "Chuck Lukacs",
);

// TRK 425 — Sarek, Federation Ambassador (alternate printing)
const SAREK_FEDERATION_AMBASSADOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAREK_FEDERATION_AMBASSADOR,
    1,
    "25e552e6-43e0-4a1f-b17f-a41569182e11",
    "Oliver Barrett",
);

// TRK 426 — Bat'leth (alternate printing)
const BAT_LETH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BAT_LETH,
    1,
    "bf6c66c7-4795-4ab0-be98-65c771a7ef01",
    "Eugene Maslovski",
);

// TRK 427 — General Chang, Cold Warrior (alternate printing)
const GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GENERAL_CHANG_COLD_WARRIOR,
    2,
    "377bc253-f9a2-45b0-bbf8-e027f2d5dc4d",
    "Julia Vasilyeva",
);

// TRK 428 — A Good Day to Die (alternate printing)
const A_GOOD_DAY_TO_DIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &A_GOOD_DAY_TO_DIE,
    1,
    "ce59648a-dc9d-494d-ba5e-97e8d4636017",
    "Darius Zablockis",
);

// TRK 429 — Storied Veteran (alternate printing)
const STORIED_VETERAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORIED_VETERAN,
    1,
    "32c2bfbf-e2ea-46c3-b369-1e203c8af716",
    "Darius Zablockis",
);

// TRK 430 — Vengeance Mission (alternate printing)
const VENGEANCE_MISSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VENGEANCE_MISSION,
    1,
    "fe71a669-eb73-47e1-944e-f05cb7641814",
    "Viktor Fetsch",
);

// TRK 431 — Worf, Intelligence Contractor (alternate printing)
const WORF_INTELLIGENCE_CONTRACTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORF_INTELLIGENCE_CONTRACTOR,
    1,
    "f177de31-b7f0-46d3-a3a9-a708f8d4a95f",
    "Gintas Galvanauskas",
);

// TRK 432 — The Duras Sisters (alternate printing)
const THE_DURAS_SISTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DURAS_SISTERS,
    1,
    "fc44d954-dc08-4071-8e2b-e65090de65dc",
    "Michael MacRae",
);

// TRK 433 — Gowron, Eager for Battle (alternate printing)
const GOWRON_EAGER_FOR_BATTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOWRON_EAGER_FOR_BATTLE,
    1,
    "1a7bda12-b72b-49e6-a086-644d0560ddc9",
    "Gintas Galvanauskas",
);

// TRK 434 — L'Rell, Fierce Visionary (alternate printing)
const L_RELL_FIERCE_VISIONARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &L_RELL_FIERCE_VISIONARY,
    1,
    "51161805-0955-49b7-97f3-20b002c9b86e",
    "Julia Vasilyeva",
);

// TRK 435 — Worf, Honor-Bound Warrior (alternate printing)
const WORF_HONOR_BOUND_WARRIOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WORF_HONOR_BOUND_WARRIOR,
    2,
    "f2abd2b6-c60f-4667-925b-423c0a135ebc",
    "Rimas Valeikis",
);

// TRK 436 — Bat'leth (alternate printing)
const BAT_LETH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BAT_LETH,
    2,
    "9cece6b4-1257-40c2-adc1-52409cafbf41",
    "Eugene Maslovski",
);

// TRK 437 — General Chang, Cold Warrior (alternate printing)
const GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GENERAL_CHANG_COLD_WARRIOR,
    3,
    "1b1b6249-5c4b-4ccf-a860-22532a0d958b",
    "Julia Vasilyeva",
);

// TRK 438 — A Good Day to Die (alternate printing)
const A_GOOD_DAY_TO_DIE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &A_GOOD_DAY_TO_DIE,
    2,
    "3c2a5428-a557-435e-b226-96159e9f16e7",
    "Darius Zablockis",
);

// TRK 439 — Storied Veteran (alternate printing)
const STORIED_VETERAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STORIED_VETERAN,
    2,
    "c1b5426e-9ba1-40c0-9954-2fc00da8e04f",
    "Darius Zablockis",
);

// TRK 440 — Vengeance Mission (alternate printing)
const VENGEANCE_MISSION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VENGEANCE_MISSION,
    2,
    "3259be03-f5af-4863-9aad-ae40061861d5",
    "Viktor Fetsch",
);

// TRK 441 — Worf, Intelligence Contractor (alternate printing)
const WORF_INTELLIGENCE_CONTRACTOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WORF_INTELLIGENCE_CONTRACTOR,
    2,
    "9af008b2-d11f-4413-85aa-a45c0f6efa8f",
    "Gintas Galvanauskas",
);

// TRK 442 — The Duras Sisters (alternate printing)
const THE_DURAS_SISTERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_DURAS_SISTERS,
    2,
    "56c8d25b-9114-4412-af57-db7f8c2e20c4",
    "Michael MacRae",
);

// TRK 443 — Gowron, Eager for Battle (alternate printing)
const GOWRON_EAGER_FOR_BATTLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOWRON_EAGER_FOR_BATTLE,
    2,
    "20cbf82c-c8c1-4bd3-b701-7e8509a7f56d",
    "Gintas Galvanauskas",
);

// TRK 444 — L'Rell, Fierce Visionary (alternate printing)
const L_RELL_FIERCE_VISIONARY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &L_RELL_FIERCE_VISIONARY,
    2,
    "4b59e50c-21e2-480d-b5b5-cc6c1eeed57a",
    "Julia Vasilyeva",
);

// TRK 445 — Worf, Honor-Bound Warrior (alternate printing)
const WORF_HONOR_BOUND_WARRIOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WORF_HONOR_BOUND_WARRIOR,
    3,
    "7739283d-25e9-4779-9f95-504a03c502d7",
    "Rimas Valeikis",
);

// TRK 446 — Antagonist Incorporator (alternate printing)
const ANTAGONIST_INCORPORATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTAGONIST_INCORPORATOR,
    1,
    "baeaac4a-a004-4a6d-842a-596cb148574b",
    "Miklós Ligeti",
);

// TRK 447 — Miles O'Brien, Transporter Ops (alternate printing)
const MILES_O_BRIEN_TRANSPORTER_OPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MILES_O_BRIEN_TRANSPORTER_OPS,
    1,
    "8adfd9c9-0cbd-479d-bcce-6f115b8a1c38",
    "John Tyler Christopher",
);

// TRK 448 — U.S.S. Enterprise-A, Free Spirit (alternate printing)
const U_S_S_ENTERPRISE_A_FREE_SPIRIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_ENTERPRISE_A_FREE_SPIRIT,
    1,
    "53941fd6-1913-4848-8fa1-7fff7ea3062b",
    "Evan Shipard",
);

// TRK 449 — The Caretaker (alternate printing)
const THE_CARETAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_CARETAKER,
    1,
    "492df5d5-04a2-49d2-a639-61b19f4419df",
    "Sam Kennedy",
);

// TRK 450 — Gwyndala, the Progeny (alternate printing)
const GWYNDALA_THE_PROGENY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GWYNDALA_THE_PROGENY,
    1,
    "2f1e927f-dd27-4af3-8941-64ce80e16e36",
    "Pierre Raveneau",
);

// TRK 451 — H.M.S. Bounty, Unlikely Timeship (alternate printing)
const H_M_S_BOUNTY_UNLIKELY_TIMESHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &H_M_S_BOUNTY_UNLIKELY_TIMESHIP,
    1,
    "15403912-07e8-4f52-9861-c0fc1d2b5a92",
    "Jetfreak-7",
);

// TRK 452 — Romulan Bird-of-Prey (alternate printing)
const ROMULAN_BIRD_OF_PREY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROMULAN_BIRD_OF_PREY,
    1,
    "c54b4e4a-72b3-481c-9eda-647fdcb04ff2",
    "Edouard",
);

// TRK 453 — Subcommander T'Pol (alternate printing)
const SUBCOMMANDER_T_POL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUBCOMMANDER_T_POL,
    1,
    "142b8804-3bfd-4735-998f-3592b10deb80",
    "Kieran Yanner",
);

// TRK 454 — Armus, Skin of Evil (alternate printing)
const ARMUS_SKIN_OF_EVIL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARMUS_SKIN_OF_EVIL,
    1,
    "c709714b-ae51-4006-8873-05f1d71a447c",
    "Camille Alquier",
);

// TRK 455 — Collation Trimatrix (alternate printing)
const COLLATION_TRIMATRIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLLATION_TRIMATRIX,
    1,
    "ec358172-4e73-4499-9032-bb28a7a01e4b",
    "Bryan Sola",
);

// TRK 456 — Elim Garak, Simple Tailor (alternate printing)
const ELIM_GARAK_SIMPLE_TAILOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELIM_GARAK_SIMPLE_TAILOR,
    1,
    "2c881def-1b77-42ed-9fdc-45f0ebbf843d",
    "Alexander Gering",
);

// TRK 457 — Georgiou, Terran Emperor (alternate printing)
const GEORGIOU_TERRAN_EMPEROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GEORGIOU_TERRAN_EMPEROR,
    1,
    "53401f53-aebe-4538-82be-9d6916c01c76",
    "Julia Metzger",
);

// TRK 458 — Grand Nagus Zek (alternate printing)
const GRAND_NAGUS_ZEK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRAND_NAGUS_ZEK,
    1,
    "2d1651f5-7bc9-4d65-9387-f4646a04e816",
    "Wei Guan",
);

// TRK 459 — Gul Dukat, Colonial Prefect (alternate printing)
const GUL_DUKAT_COLONIAL_PREFECT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUL_DUKAT_COLONIAL_PREFECT,
    1,
    "bf9d476e-0a53-49ec-abfe-c8fa1557e8db",
    "Joel Chaim Holtzman",
);

// TRK 460 — Kai Winn, Faithless Leader (alternate printing)
const KAI_WINN_FAITHLESS_LEADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAI_WINN_FAITHLESS_LEADER,
    1,
    "76e49c4e-ebb8-4821-9709-ec6ec1745aa3",
    "Wei Guan",
);

// TRK 461 — U.S.S. Reliant, From Hell's Heart (alternate printing)
const U_S_S_RELIANT_FROM_HELL_S_HEART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_RELIANT_FROM_HELL_S_HEART,
    1,
    "2aad44f7-e3f4-46e9-b633-eb939dfd8e51",
    "Dofresh",
);

// TRK 462 — Brazen Bluff (alternate printing)
const BRAZEN_BLUFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRAZEN_BLUFF,
    1,
    "902c6907-5380-4417-b3ad-9cd693ae9ca2",
    "Filipe Pagliuso",
);

// TRK 463 — D-7 Battle Cruiser (alternate printing)
const D_7_BATTLE_CRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &D_7_BATTLE_CRUISER,
    1,
    "40c41dc2-f67d-4199-a1bb-eb15c776fe8e",
    "Kevin Glint",
);

// TRK 464 — Lwaxana Troi, Charming Guest (alternate printing)
const LWAXANA_TROI_CHARMING_GUEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LWAXANA_TROI_CHARMING_GUEST,
    1,
    "eb65e3e3-eca9-4856-8502-7a47b7dd7216",
    "Marta Nael",
);

// TRK 465 — Pah-Wraith (alternate printing)
const PAH_WRAITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PAH_WRAITH,
    1,
    "9f50fab0-d5a8-403e-b666-5398fbbdd34f",
    "Nino Vecia",
);

// TRK 466 — Branching Timeline (alternate printing)
const BRANCHING_TIMELINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRANCHING_TIMELINE,
    1,
    "b65e91bf-565f-4fb1-b8e6-7a7cd57f84d6",
    "Borja Pindado",
);

// TRK 467 — Captain Jonathan Archer (alternate printing)
const CAPTAIN_JONATHAN_ARCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JONATHAN_ARCHER,
    1,
    "aaaeeb0f-eb6f-41d6-be1f-7f8f14c4e82a",
    "Julia Metzger",
);

// TRK 468 — Captain Pike, Fullness of Time (alternate printing)
const CAPTAIN_PIKE_FULLNESS_OF_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_PIKE_FULLNESS_OF_TIME,
    1,
    "62c9b897-bc43-47f9-8939-13faaa0b7c6b",
    "Joel Chaim Holtzman",
);

// TRK 469 — Dr. Joseph M'Benga (alternate printing)
const DR_JOSEPH_M_BENGA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DR_JOSEPH_M_BENGA,
    1,
    "71f54ed7-1def-4c3f-9be5-11332fce751f",
    "Romana Kendelic",
);

// TRK 470 — Holo-Engineer (alternate printing)
const HOLO_ENGINEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOLO_ENGINEER,
    1,
    "e8a8f6cc-1b50-4ee2-b2b2-6f4b0760b9b3",
    "Josu Hernaiz",
);

// TRK 471 — MACO Strike Team (alternate printing)
const MACO_STRIKE_TEAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MACO_STRIKE_TEAM,
    1,
    "960f4cd1-2fe1-4f8d-9c7a-2547a6a9f46e",
    "Aaron J. Riley",
);

// TRK 472 — Rok-Tahk, Budding Scientist (alternate printing)
const ROK_TAHK_BUDDING_SCIENTIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROK_TAHK_BUDDING_SCIENTIST,
    1,
    "5354636f-a324-401b-bece-85b1c4760c07",
    "Andreas Zafiratos",
);

// TRK 473 — Stable Wormhole (alternate printing)
const STABLE_WORMHOLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STABLE_WORMHOLE,
    1,
    "b18ce9e8-0673-4277-acf0-a109a162f619",
    "Bruce Brenneise",
);

// TRK 474 — U.S.S. Discovery, at Black Alert (alternate printing)
const U_S_S_DISCOVERY_AT_BLACK_ALERT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_DISCOVERY_AT_BLACK_ALERT,
    1,
    "f0a16d25-ab32-4744-8b61-3774dd28a2c1",
    "Kevin Glint",
);

// TRK 475 — Borg Cube (alternate printing)
const BORG_CUBE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BORG_CUBE,
    1,
    "99f2ef04-7134-446a-b1d1-5e54d76adcb8",
    "Alexander Forssberg",
);

// TRK 476 — Hit It! (alternate printing)
const HIT_IT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIT_IT,
    1,
    "2dad81b5-caf4-4d1c-9a29-8b556984d3f1",
    "Aaron J. Riley",
);

// TRK 477 — Kira Nerys, Firebrand (alternate printing)
const KIRA_NERYS_FIREBRAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KIRA_NERYS_FIREBRAND,
    1,
    "f40be112-e41d-456b-919f-14b2dee7e6ee",
    "Pauline Voss",
);

// TRK 478 — Crystalline Entity (alternate printing)
const CRYSTALLINE_ENTITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYSTALLINE_ENTITY,
    1,
    "741798a7-67df-4b8b-b0e9-dd8c2797ccc4",
    "David Álvarez",
);

// TRK 479 — Deep Space 9 (alternate printing)
const DEEP_SPACE_9_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEP_SPACE_9,
    1,
    "473aab2f-cfc2-430b-b785-4b6a491bcd28",
    "Paolo Puggioni",
);

// TRK 480 — Engineered Explosives (alternate printing)
const ENGINEERED_EXPLOSIVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::fifth_dawn::ENGINEERED_EXPLOSIVES,
    1,
    "7e8f0912-a8c5-44ce-b39b-eb0f63da07a4",
    "David Álvarez",
);

// TRK 481 — U.S.S. Enterprise-D, Galaxy-Class (alternate printing)
const U_S_S_ENTERPRISE_D_GALAXY_CLASS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_ENTERPRISE_D_GALAXY_CLASS,
    1,
    "12a2e90a-6776-4213-b696-fd48a4bcf0a2",
    "Pablo Mendoza",
);

// TRK 482 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    4,
    "5b5385bf-54b3-4885-84c9-a383a4b95a5b",
    "Paulius Daščioras",
);

// TRK 483 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    4,
    "a0a6038e-f812-415d-b54a-0229d338e969",
    "Guglielmo Bonis",
);

// TRK 484 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    4,
    "a75fe304-8764-4868-8ee8-d5fb094386c0",
    "Rytis Sabaliauskas",
);

// TRK 485 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    4,
    "95409599-e700-4e16-bea2-bc7b607f9cc4",
    "Rytis Sabaliauskas",
);

// TRK 486 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    4,
    "17923ce1-56db-4f18-b6a7-fda2eca7edbd",
    "Bengt Hampus Viklander",
);

// TRK 487 — Hallowed Fountain (alternate printing)
const HALLOWED_FOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::HALLOWED_FOUNTAIN,
    2,
    "8868a343-2e57-45a8-9e53-971211fe5cc9",
    "Mark Poole",
);

// TRK 488 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::WATERY_GRAVE,
    2,
    "6ccd8ea5-b8c9-4012-b3a0-dbc2b95538c3",
    "Andrew Mar",
);

// TRK 489 — Blood Crypt (alternate printing)
const BLOOD_CRYPT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::BLOOD_CRYPT,
    2,
    "32b61634-d3be-4d78-8e52-357b6be9ad61",
    "Jason A. Engle",
);

// TRK 490 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::STOMPING_GROUND,
    2,
    "64ab23f3-01ae-4c06-967a-f7f1a9a99dec",
    "Daniel Romanovsky",
);

// TRK 491 — Temple Garden (alternate printing)
const TEMPLE_GARDEN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::TEMPLE_GARDEN,
    2,
    "8d003daf-df0b-41d3-bbd1-70b73431bf77",
    "Jessica Rossier",
);

// TRK 492 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::GODLESS_SHRINE,
    2,
    "2a540ef6-ba54-4729-9642-fada37843292",
    "Raymond Swanland",
);

// TRK 493 — Steam Vents (alternate printing)
const STEAM_VENTS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::guildpact::STEAM_VENTS,
    2,
    "a3f14044-9835-4d78-8c1a-adf6e2c67d85",
    "Chad Weatherford",
);

// TRK 494 — Overgrown Tomb (alternate printing)
const OVERGROWN_TOMB_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::OVERGROWN_TOMB,
    2,
    "e97cb934-fc79-4010-829f-bc9bb0ba7635",
    "Sam Burley",
);

// TRK 495 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::ravnica_city_of_guilds::SACRED_FOUNDRY,
    2,
    "1ff8510e-bec2-401b-8e6e-cc350f805574",
    "Eliz Roxs",
);

// TRK 496 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::dissension::BREEDING_POOL,
    2,
    "638731af-512e-42bd-813a-d540a36c8ead",
    "Josu Solano",
);

// TRK 497 — Captain Jean-Luc Picard (alternate printing)
const CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JEAN_LUC_PICARD,
    2,
    "a6eb4655-ea1d-426d-a75b-a0a341362cd3",
    "Erikas Perl",
);

// TRK 498 — Command Subadjunct (alternate printing)
const COMMAND_SUBADJUNCT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COMMAND_SUBADJUNCT,
    3,
    "f0bca1fd-0eef-4563-84c7-05b59ff9e015",
    "Michael MacRae",
);

// TRK 499 — Deanna Troi, Psychic Counsel (alternate printing)
const DEANNA_TROI_PSYCHIC_COUNSEL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEANNA_TROI_PSYCHIC_COUNSEL,
    2,
    "5366830b-323e-40da-a3e8-9cc6e5187e49",
    "Eglė Mosakaitė",
);

// TRK 500 — Dr. Beverly Crusher (alternate printing)
const DR_BEVERLY_CRUSHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DR_BEVERLY_CRUSHER,
    2,
    "179d50b5-cab1-482c-85a6-36f59335b562",
    "Anna Czeszejko",
);

// TRK 501 — Peace in Our Time (alternate printing)
const PEACE_IN_OUR_TIME_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PEACE_IN_OUR_TIME,
    2,
    "f1c86545-0252-46cc-ac35-58d126f55997",
    "Mintautas Šukys",
);

// TRK 502 — Primary Unimatrix Adjunct (alternate printing)
const PRIMARY_UNIMATRIX_ADJUNCT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PRIMARY_UNIMATRIX_ADJUNCT,
    2,
    "ba15432e-64a4-49fe-9f89-86129d332bcc",
    "Julia Vasilyeva",
);

// TRK 503 — There's Coffee in that Nebula (alternate printing)
const THERE_S_COFFEE_IN_THAT_NEBULA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THERE_S_COFFEE_IN_THAT_NEBULA,
    2,
    "1205651a-aa71-4bcb-ade7-0ab101ad95ad",
    "Anna Czeszejko & Rutger van de Steeg",
);

// TRK 504 — Deep Space Scan (alternate printing)
const DEEP_SPACE_SCAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEEP_SPACE_SCAN,
    2,
    "de52d77a-f5af-4392-b53b-60ce79328d3f",
    "Maxim Ruabtsev",
);

// TRK 505 — Lieutenant Commander Data (alternate printing)
const LIEUTENANT_COMMANDER_DATA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LIEUTENANT_COMMANDER_DATA,
    3,
    "191809e6-87f5-414e-a45e-62988f092832",
    "Mintautas Šukys",
);

// TRK 506 — Lt. Cmdr. Geordi La Forge (alternate printing)
const LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LT_CMDR_GEORDI_LA_FORGE,
    2,
    "3c627570-844f-49d6-a788-dda7ef4f399c",
    "Vlad Petruchik",
);

// TRK 507 — Seven of Nine, Unassimilated (alternate printing)
const SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SEVEN_OF_NINE_UNASSIMILATED,
    2,
    "fbe692d2-e8ae-464b-84a9-fe7063e806c0",
    "Anna Czeszejko",
);

// TRK 508 — Wesley Crusher, Starfleet Cadet (alternate printing)
const WESLEY_CRUSHER_STARFLEET_CADET_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WESLEY_CRUSHER_STARFLEET_CADET,
    2,
    "976fcdd6-c300-453e-a424-e0ac6136041d",
    "Rimas Valeikis",
);

// TRK 509 — Death Is Irrelevant (alternate printing)
const DEATH_IS_IRRELEVANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEATH_IS_IRRELEVANT,
    3,
    "423ada94-a251-40f4-ae53-99fbb073d550",
    "Paulius Daščioras",
);

// TRK 510 — Organic Avulsion Unit (alternate printing)
const ORGANIC_AVULSION_UNIT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORGANIC_AVULSION_UNIT,
    2,
    "5e51ded6-4b58-4169-a292-26832918dbef",
    "Gintas Galvanauskas",
);

// TRK 511 — Overflow Submatrix (alternate printing)
const OVERFLOW_SUBMATRIX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERFLOW_SUBMATRIX,
    2,
    "3753f25b-cdee-47f7-86a2-107d6e818be8",
    "Viktor Fetsch",
);

// TRK 512 — Resistance Is Futile (alternate printing)
const RESISTANCE_IS_FUTILE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RESISTANCE_IS_FUTILE,
    3,
    "ea664a07-fac6-4703-8fd2-f3f3d7257b60",
    "Jurijus Chitrovas",
);

// TRK 513 — Sela, Tal Shiar Paradox (alternate printing)
const SELA_TAL_SHIAR_PARADOX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SELA_TAL_SHIAR_PARADOX,
    2,
    "e9c190eb-aff0-42b8-b935-1fabbc9c3425",
    "Jurijus Chitrovas",
);

// TRK 514 — Dominion Infiltration (alternate printing)
const DOMINION_INFILTRATION_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DOMINION_INFILTRATION,
    3,
    "64f326f2-a8d3-4a52-b7b7-b3624d815b64",
    "Darius Zablockis",
);

// TRK 515 — Encounter the Unknown (alternate printing)
const ENCOUNTER_THE_UNKNOWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENCOUNTER_THE_UNKNOWN,
    2,
    "328a5b5b-afeb-4c92-bd78-76feea71b4e2",
    "Eugene Maslovski",
);

// TRK 516 — Infinite Diversity (alternate printing)
const INFINITE_DIVERSITY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &INFINITE_DIVERSITY,
    3,
    "886ab3a2-94ba-4160-9e4f-72f5568253f2",
    "Eglė Mosakaitė",
);

// TRK 517 — Q, Stacking the Deck (alternate printing)
const Q_STACKING_THE_DECK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &Q_STACKING_THE_DECK,
    2,
    "b1126734-d3df-42f0-ba4e-39cbf8f3c4d5",
    "Viktor Fetsch",
);

// TRK 518 — Tasha Yar, Out of Time (alternate printing)
const TASHA_YAR_OUT_OF_TIME_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TASHA_YAR_OUT_OF_TIME,
    2,
    "701dfdf6-c8ea-4476-bf02-6b91ed3b4c57",
    "Vilhelmas Banys",
);

// TRK 519 — Will Riker, First in the Field (alternate printing)
const WILL_RIKER_FIRST_IN_THE_FIELD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WILL_RIKER_FIRST_IN_THE_FIELD,
    2,
    "e81e7191-a039-4b37-9dea-6b3b4ad3e572",
    "Vilhelmas Banys",
);

// TRK 520 — Captain Kathryn Janeway (alternate printing)
const CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_KATHRYN_JANEWAY,
    3,
    "2e462516-fddb-4e76-836b-b590d37e783f",
    "Jurijus Chitrovas",
);

// TRK 521 — Warp Speed (alternate printing)
const WARP_SPEED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WARP_SPEED,
    2,
    "35ccfa21-00da-497b-8e76-631ec4489e72",
    "Maxim Ruabtsev",
);

// TRK 522 — Dathon, His Heart Open (alternate printing)
const DATHON_HIS_HEART_OPEN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DATHON_HIS_HEART_OPEN,
    2,
    "b64323f7-db8c-44e9-bed1-696012f32740",
    "Vilhelmas Banys",
);

// TRK 523 — Locutus, the One Who Speaks (alternate printing)
const LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOCUTUS_THE_ONE_WHO_SPEAKS,
    3,
    "d9d45bf1-9007-4d5d-bc32-fb11a353edc5",
    "Erikas Perl",
);

// TRK 524 — Worf, Honor-Bound Warrior (alternate printing)
const WORF_HONOR_BOUND_WARRIOR_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &WORF_HONOR_BOUND_WARRIOR,
    4,
    "969a7271-30d5-4e6c-aa6b-1c56dfd69e09",
    "Vlad Petruchik",
);

// TRK 525 — Replicator (alternate printing)
const REPLICATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REPLICATOR,
    2,
    "98308586-5ac3-4130-878b-d235b9e1343c",
    "Vilhelmas Banys",
);

// TRK 526 — Dr. McCoy, Surly Surgeon (alternate printing)
const DR_MCCOY_SURLY_SURGEON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DR_MCCOY_SURLY_SURGEON,
    2,
    "00474b9c-620f-44dc-912f-5d1972a1eb22",
    "Chuck Lukacs",
);

// TRK 527 — Excalbian Gamemaster (alternate printing)
const EXCALBIAN_GAMEMASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXCALBIAN_GAMEMASTER,
    2,
    "91f96c5f-e3ce-4c94-9944-ce9b1b5a097a",
    "Chuck Lukacs",
);

// TRK 528 — Federation Resupply (alternate printing)
const FEDERATION_RESUPPLY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FEDERATION_RESUPPLY,
    2,
    "28a6184b-69ee-453e-be58-49180ed72a39",
    "Jason Raish",
);

// TRK 529 — The Needs of the Many (alternate printing)
const THE_NEEDS_OF_THE_MANY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_NEEDS_OF_THE_MANY,
    2,
    "d47beb69-8ec0-4440-a317-77243e2e24b6",
    "Jack Hughes",
);

// TRK 530 — Saavik, Stoic Student (alternate printing)
const SAAVIK_STOIC_STUDENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAAVIK_STOIC_STUDENT,
    2,
    "766eead5-0f5d-4bd5-a3ad-8795b0960bc6",
    "Jack Hughes",
);

// TRK 531 — Sulu, Space Swashbuckler (alternate printing)
const SULU_SPACE_SWASHBUCKLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SULU_SPACE_SWASHBUCKLER,
    3,
    "90965755-2b70-46fa-aecd-8f9cb28767d3",
    "Jason Raish",
);

// TRK 532 — Troubling Tribbles (alternate printing)
const TROUBLING_TRIBBLES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TROUBLING_TRIBBLES,
    2,
    "65d09a80-440a-4686-a0bb-b2be19112f8e",
    "Jason Raish",
);

// TRK 533 — Ensign Pavel Chekov (alternate printing)
const ENSIGN_PAVEL_CHEKOV_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENSIGN_PAVEL_CHEKOV,
    2,
    "f483991b-5291-42a8-89c3-f51629530bb1",
    "Marc Aspinall",
);

// TRK 534 — Kobayashi Maru Scenario (alternate printing)
const KOBAYASHI_MARU_SCENARIO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KOBAYASHI_MARU_SCENARIO,
    2,
    "d557515e-bf1d-4781-aa44-eb2638cfe1ba",
    "Oliver Barrett",
);

// TRK 535 — Mr. Spock, Science Officer (alternate printing)
const MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MR_SPOCK_SCIENCE_OFFICER,
    3,
    "e855f782-9221-4b6f-a182-b738da4b7d48",
    "Grant Griffin",
);

// TRK 536 — Nyota Uhura, Communications (alternate printing)
const NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &NYOTA_UHURA_COMMUNICATIONS,
    4,
    "971cbb33-7374-4b08-8765-8ccb6f5858d1",
    "Jack Hughes",
);

// TRK 537 — Scotty, Miracle Worker (alternate printing)
const SCOTTY_MIRACLE_WORKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SCOTTY_MIRACLE_WORKER,
    3,
    "008db37b-1b2e-4554-b927-8a75ce4ff668",
    "Marc Aspinall",
);

// TRK 538 — The Destruction of Praxis (alternate printing)
const THE_DESTRUCTION_OF_PRAXIS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_DESTRUCTION_OF_PRAXIS,
    2,
    "424c8a03-c9f0-4980-9b6e-9c344090ffb6",
    "Oliver Barrett",
);

// TRK 539 — General Chang, Cold Warrior (alternate printing)
const GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &GENERAL_CHANG_COLD_WARRIOR,
    4,
    "443083e7-dfee-4889-b8ed-82d6876f6402",
    "Oliver Barrett",
);

// TRK 540 — Khan, Vengeful Heart (alternate printing)
const KHAN_VENGEFUL_HEART_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KHAN_VENGEFUL_HEART,
    2,
    "9d0dce51-002b-43b6-ac0e-d78c062b99b7",
    "Grant Griffin",
);

// TRK 541 — Salt Vampire (alternate printing)
const SALT_VAMPIRE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SALT_VAMPIRE,
    2,
    "d633a0bf-bb3a-471f-a286-7686da81bb01",
    "Simon Dominic",
);

// TRK 542 — Captain James T. Kirk (alternate printing)
const CAPTAIN_JAMES_T_KIRK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JAMES_T_KIRK,
    2,
    "a7f991c5-0a55-4d34-8dee-d3f206167be1",
    "Josh Newton",
);

// TRK 543 — Khan, Despot by Design (alternate printing)
const KHAN_DESPOT_BY_DESIGN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KHAN_DESPOT_BY_DESIGN,
    2,
    "e6507344-3e4a-44e8-ae02-5dcfbfc6fb55",
    "Jack Hughes",
);

// TRK 544 — David and Carol Marcus, Creators (alternate printing)
const DAVID_AND_CAROL_MARCUS_CREATORS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DAVID_AND_CAROL_MARCUS_CREATORS,
    2,
    "5b585418-57c6-4319-8ab5-28de8e7f2d88",
    "Jason Raish",
);

// TRK 545 — Dr. Leonard "Bones" McCoy (alternate printing)
const DR_LEONARD_BONES_MCCOY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DR_LEONARD_BONES_MCCOY,
    3,
    "dba2e690-8b55-4c0d-b7d7-747f4b9b382c",
    "Jack Hughes",
);

// TRK 546 — Genesis Device (alternate printing)
const GENESIS_DEVICE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GENESIS_DEVICE,
    2,
    "de57520a-90ea-445b-a878-01413da5d4af",
    "Oliver Barrett",
);

// TRK 547 — Live Long and Prosper (alternate printing)
const LIVE_LONG_AND_PROSPER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LIVE_LONG_AND_PROSPER,
    2,
    "b2f934db-403b-4f6a-9abc-d48eb9140e49",
    "Jack Hughes",
);

// TRK 548 — Mirror Universe Intrusion (alternate printing)
const MIRROR_UNIVERSE_INTRUSION_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MIRROR_UNIVERSE_INTRUSION,
    3,
    "ddfae5a3-7f3f-48f4-bcdf-6c7729504289",
    "Chuck Lukacs",
);

// TRK 549 — Sarek, Federation Ambassador (alternate printing)
const SAREK_FEDERATION_AMBASSADOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAREK_FEDERATION_AMBASSADOR,
    2,
    "b46fdc18-d488-4c8a-af6b-0f28d4db14c2",
    "Oliver Barrett",
);

// TRK 550 — U.S.S. Enterprise-A, Free Spirit (alternate printing)
const U_S_S_ENTERPRISE_A_FREE_SPIRIT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_ENTERPRISE_A_FREE_SPIRIT,
    2,
    "b162c8cd-865d-40a8-9bc1-e11702ac27d2",
    "Scott Okumura",
);

// TRK 551 — H.M.S. Bounty, Unlikely Timeship (alternate printing)
const H_M_S_BOUNTY_UNLIKELY_TIMESHIP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &H_M_S_BOUNTY_UNLIKELY_TIMESHIP,
    2,
    "a58dde3d-74a2-4b31-bbf8-e3bd038b943f",
    "Scott Okumura",
);

// TRK 552 — U.S.S. Reliant, From Hell's Heart (alternate printing)
const U_S_S_RELIANT_FROM_HELL_S_HEART_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_RELIANT_FROM_HELL_S_HEART,
    2,
    "dc9fa6b6-6cb9-4fc9-bda0-2bb9a1fe8313",
    "Scott Okumura",
);

// TRK 553 — D-7 Battle Cruiser (alternate printing)
const D_7_BATTLE_CRUISER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &D_7_BATTLE_CRUISER,
    2,
    "a30e0582-b5de-40f0-a284-3251e4092eda",
    "Scott Okumura",
);

// TRK 554 — U.S.S. Discovery, at Black Alert (alternate printing)
const U_S_S_DISCOVERY_AT_BLACK_ALERT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_DISCOVERY_AT_BLACK_ALERT,
    2,
    "7351dc6f-d162-4854-930d-bdeaf87c684b",
    "Scott Okumura",
);

// TRK 555 — Borg Cube (alternate printing)
const BORG_CUBE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BORG_CUBE,
    2,
    "953e0763-3788-4a97-8e2f-394cc2649e43",
    "Scott Okumura",
);

// TRK 556 — Deep Space 9 (alternate printing)
const DEEP_SPACE_9_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEEP_SPACE_9,
    2,
    "65585609-563d-47e9-92b8-d4b65baa95bd",
    "Scott Okumura",
);

// TRK 557 — U.S.S. Enterprise-D, Galaxy-Class (alternate printing)
const U_S_S_ENTERPRISE_D_GALAXY_CLASS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &U_S_S_ENTERPRISE_D_GALAXY_CLASS,
    2,
    "cb823267-b230-40f3-a1eb-140657df2b1e",
    "Scott Okumura",
);

// TRK 558 — Dr. Leonard "Bones" McCoy (alternate printing)
const DR_LEONARD_BONES_MCCOY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &DR_LEONARD_BONES_MCCOY,
    4,
    "66d618ab-1be6-4049-a384-d50c4bb39c33",
    "Jason Raish",
);

// TRK 559 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    5,
    "4b0ef712-1588-4a64-9434-c7b0cc5299b9",
    "Bastien Grivet",
);

// TRK 560 — Plains (alternate printing)
const PLAINS_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    6,
    "65d70453-4f54-44bc-900a-3877ca6d73c3",
    "Leon Tukker",
);

// TRK 561 — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    5,
    "9a8f79c1-2a34-42b5-9149-6e37f986f9ce",
    "Bastien Grivet",
);

// TRK 562 — Island (alternate printing)
const ISLAND_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    6,
    "b77324fb-e53c-46dd-a73c-88296a022d57",
    "Leon Tukker",
);

// TRK 563 — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    5,
    "c5bcdd40-e749-47d0-ad92-4c4034a6cd65",
    "Bastien Grivet",
);

// TRK 564 — Swamp (alternate printing)
const SWAMP_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    6,
    "30db67a2-129f-4ca9-9c6b-ca345096737b",
    "Leon Tukker",
);

// TRK 565 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    5,
    "2aa7f81c-7263-4ce7-835e-1d7f3595fb8f",
    "Bastien Grivet",
);

// TRK 566 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    6,
    "37a9336b-c9ac-4548-9b40-28b218722732",
    "Leon Tukker",
);

// TRK 567 — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    5,
    "e8dfd50a-65a8-4439-b597-3e13554671eb",
    "Bastien Grivet",
);

// TRK 568 — Forest (alternate printing)
const FOREST_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    6,
    "910b0085-86e8-4d81-be45-ce7bed740143",
    "Leon Tukker",
);

// TRK 569 — Command Subadjunct (alternate printing)
const COMMAND_SUBADJUNCT_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &COMMAND_SUBADJUNCT,
    4,
    "bce0a2eb-20a2-423b-bf17-17029f3bd233",
    "Alex Negrea",
);

// TRK 570 — Conquest Mission (alternate printing)
const CONQUEST_MISSION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CONQUEST_MISSION,
    2,
    "cfb05333-acea-4bf0-973b-293d9e69751a",
    "Mark Spears",
);

// TRK 571 — Malfunctioning Holodeck (alternate printing)
const MALFUNCTIONING_HOLODECK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALFUNCTIONING_HOLODECK,
    1,
    "1f564242-e841-4f70-9d48-36ff410a25c4",
    "Pablo Mendoza",
);

// TRK 572 — Cha'DIch Investigator (alternate printing)
const CHA_DICH_INVESTIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHA_DICH_INVESTIGATOR,
    1,
    "8888dc53-c16f-4894-8d0b-f86a33c41cf1",
    "Javier Charro",
);

// TRK 573 — Hive Mind Coprocessor (alternate printing)
const HIVE_MIND_COPROCESSOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIVE_MIND_COPROCESSOR,
    1,
    "bdfb5e42-1950-4dd8-b1d6-21170ec93159",
    "Andrey Kuzinskiy",
);

// TRK 574 — Cold-Blooded Crew (alternate printing)
const COLD_BLOODED_CREW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLD_BLOODED_CREW,
    1,
    "934612d6-1809-46cf-a917-5a32718bddd2",
    "John Tyler Christopher",
);

// TRK 575 — Syndicate Liquidators (alternate printing)
const SYNDICATE_LIQUIDATORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYNDICATE_LIQUIDATORS,
    1,
    "d15242ea-4460-4804-ba0e-09cb7f24bc50",
    "Carlos C Diaz",
);

// TRK 576 — Locutus, the One Who Speaks (alternate printing)
const LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LOCUTUS_THE_ONE_WHO_SPEAKS,
    4,
    "22af225f-8b59-4e6b-b469-a2272c7d487f",
    "Ariel Orea",
);

// TRK 577 — Earth, Sector 001 (alternate printing)
const EARTH_SECTOR_001_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTH_SECTOR_001,
    1,
    "c1cf4918-f763-4355-abc6-abb402de8224",
    "Colin Searle",
);

// TRK 578 — Risa (alternate printing)
const RISA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RISA,
    1,
    "d7454d76-529f-4217-8490-cae65bec8543",
    "Colin Searle",
);

// TRK 579 — Vulcan (alternate printing)
const VULCAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VULCAN,
    1,
    "4103795c-dbb8-414f-b580-6b0084b8e212",
    "John Di Giovanni",
);

// TRK 1701 — Captain James T. Kirk (alternate printing)
const CAPTAIN_JAMES_T_KIRK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JAMES_T_KIRK,
    3,
    "b1cfb3f1-f1ad-47aa-a7f0-ff3ebd2ffa5e",
    "Chris Rallis",
);

// TRK 1701D — Captain Jean-Luc Picard (alternate printing)
const CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_JEAN_LUC_PICARD,
    3,
    "4d9fcb2a-b4fa-4990-861c-789926052104",
    "Irvin Rodriguez",
);

// TRK 1702 — Mr. Spock, Science Officer (alternate printing)
const MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MR_SPOCK_SCIENCE_OFFICER,
    4,
    "bd0c3372-4cc5-489c-b8fb-ea11e700fa16",
    "Gal Or",
);

// TRK 1703 — Lieutenant Commander Data (alternate printing)
const LIEUTENANT_COMMANDER_DATA_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LIEUTENANT_COMMANDER_DATA,
    4,
    "e5484552-37a3-41ef-b618-ee92fb418c85",
    "Tyler Jacobson",
);

// TRK 1704 — Captain Kathryn Janeway (alternate printing)
const CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_KATHRYN_JANEWAY,
    4,
    "e40fe4b6-81c0-4822-9668-2dac7820dcb9",
    "Lie Setiawan",
);

// TRK 1705 — Seven of Nine, Unassimilated (alternate printing)
const SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SEVEN_OF_NINE_UNASSIMILATED,
    3,
    "4016a92a-b662-4d22-8072-6c1ccb77c2c3",
    "Jack Hughes",
);

// TRK 1706 — Lt. Cmdr. Geordi La Forge (alternate printing)
const LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LT_CMDR_GEORDI_LA_FORGE,
    3,
    "f0028b63-3e33-47ab-88ed-bdbe5705ecf7",
    "Tyler Jacobson",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SEVENTY_THOUSAND_LIGHT_YEARS_FROM_HOME,
    &ANDROMEDAN_AUTOMATONS,
    &ANTAGONIST_INCORPORATOR,
    &ASSEMBLY_SPECIALIST,
    &BRAD_BOIMLER_EAGER_ENSIGN,
    &CAPTAIN_BENJAMIN_SISKO,
    &CAPTAIN_JEAN_LUC_PICARD,
    &COMMAND_DECISION,
    &COMMAND_SUBADJUNCT,
    &CONQUEST_MISSION,
    &DEANNA_TROI_PSYCHIC_COUNSEL,
    &DOT_7_REPAIR_SQUAD,
    &DR_BEVERLY_CRUSHER,
    &DR_MCCOY_SURLY_SURGEON,
    &EMERGENCY_MEDICAL_HOLOGRAM,
    &EXCALBIAN_GAMEMASTER,
    &FEDERATION_FIELD_MEDIC,
    &FEDERATION_RESUPPLY,
    &FREE_BORG_REVOLUTIONARIES,
    &GHOST_OF_ILLYRIA,
    &HOLOGRAM_JANEWAY,
    &MALFUNCTIONING_HOLODECK,
    &THE_MEASURE_OF_A_MAN,
    &MILES_O_BRIEN_TRANSPORTER_OPS,
    &THE_NEEDS_OF_THE_MANY,
    &OBERTH_CLASS_STARSHIP,
    &PEACE_IN_OUR_TIME,
    &PET_SEHLAT,
    &PICARD_LEADING_BY_EXAMPLE,
    &PRIMARY_UNIMATRIX_ADJUNCT,
    &PURE_ENERGY,
    &RUTHERFORD_UPGRADED_ENSIGN,
    &SAAVIK_STOIC_STUDENT,
    &SET_PHASERS_TO,
    &SHIELDS_UP,
    &SHUTTLE_ACE,
    &SULU_SPACE_SWASHBUCKLER,
    &TACTICAL_OFFICER,
    &TEST_OF_CONVICTION,
    &THERE_S_COFFEE_IN_THAT_NEBULA,
    &TROUBLING_TRIBBLES,
    &UNA_CHIN_RILEY_NUMBER_ONE,
    &U_S_S_ENTERPRISE_A_FREE_SPIRIT,
    &U_S_S_EXCELSIOR_CUTTING_EDGE,
    &VEXING_DRONE,
    &ANTHROPOLOGICAL_MISSION,
    &THE_CARETAKER,
    &CETACEAN_OPS,
    &CLOAK_AND_PHASER,
    &CLOISTERED_TELEPATH,
    &CONSIDER_THE_PRIME_DIRECTIVE,
    &DATA_NEW_LIFE,
    &DEEP_SPACE_SCAN,
    &DOMINION_SABOTEUR,
    &EMERGENCY_HOLOGRAPHIC_CREW,
    &ENERGIZE,
    &ENSIGN_PAVEL_CHEKOV,
    &ENTERPRISE_NX_01,
    &FEDERATION_PROBE,
    &GUIDANCE_FAILURE,
    &GWYNDALA_THE_PROGENY,
    &HIGHLY_ILLOGICAL,
    &H_M_S_BOUNTY_UNLIKELY_TIMESHIP,
    &HOSHI_SATO_EXOLINGUIST,
    &HUMPBACK_WHALES,
    &INTERSTELLAR_EFFIGY,
    &JETT_RENO_PROBLEM_SOLVER,
    &KOBAYASHI_MARU_SCENARIO,
    &LIEUTENANT_COMMANDER_DATA,
    &LT_CMDR_GEORDI_LA_FORGE,
    &MAKE_IT_SO,
    &MAROONED,
    &MELKOTIAN_ADJUDICATOR,
    &MR_SPOCK_SCIENCE_OFFICER,
    &NYOTA_UHURA_COMMUNICATIONS,
    &OPEN_COMMUNICATIONS,
    &ROMULAN_BIRD_OF_PREY,
    &RULES_OF_ACQUISITION,
    &SCOTTY_MIRACLE_WORKER,
    &SEVEN_OF_NINE_UNASSIMILATED,
    &STARFLEET_SPANNER_JOCKEY,
    &SUBCOMMANDER_T_POL,
    &SUUROK_CLASS_SCIENCE_VESSEL,
    &SUUS_MAHNA_PRACTITIONER,
    &TEST_OF_PRIORITIES,
    &THOSE_OLD_SCIENTISTS,
    &V_GER_THE_INTRUDER,
    &VULCAN_MIND_MELD,
    &WESLEY_CRUSHER_STARFLEET_CADET,
    &ZEFRAM_COCHRANE_WARP_PIONEER,
    &ARMUS_SKIN_OF_EVIL,
    &ASSAULT_DRONE,
    &AUTOMATED_WARFARE_SYSTEM,
    &BAT_LETH,
    &BATTLE_SCARRED_SURVIVALIST,
    &BLASTONEURON,
    &CHA_DICH_INVESTIGATOR,
    &COLLATION_TRIMATRIX,
    &COLLECTIVE_DRONE,
    &COMMANDER_RAFFI_MUSIKER,
    &D_GHOR_BLOODLETTER,
    &DEATH_IS_IRRELEVANT,
    &THE_DESTRUCTION_OF_PRAXIS,
    &DE_EVOLVE,
    &EJECT_THE_WARP_CORE,
    &ELIM_GARAK_SIMPLE_TAILOR,
    &FUGITIVE_LISTENER,
    &GABRIEL_LORCA_MIRROR_MENACE,
    &GENERAL_CHANG_COLD_WARRIOR,
    &GEORGIOU_TERRAN_EMPEROR,
    &A_GOOD_DAY_TO_DIE,
    &GRAND_NAGUS_ZEK,
    &GUL_DUKAT_COLONIAL_PREFECT,
    &HE_S_DEAD_JIM,
    &HIVE_MIND_COPROCESSOR,
    &IMPOSSIBLE_CHOICE,
    &IN_THE_PALE_MOONLIGHT,
    &KAI_WINN_FAITHLESS_LEADER,
    &KHAN_VENGEFUL_HEART,
    &OBSIDIAN_ORDER_OPERATIVE,
    &ORGANIC_AVULSION_UNIT,
    &OVERFLOW_SUBMATRIX,
    &PATTERN_BUFFER_PRESERVATION,
    &RELENTLESS_DREDNOK,
    &RESISTANCE_IS_FUTILE,
    &ROGUE_ARTIFICIAL_INTELLIGENCE,
    &SALT_VAMPIRE,
    &SELA_TAL_SHIAR_PARADOX,
    &STORIED_VETERAN,
    &STRAY_FROM_HISTORY,
    &TAL_SHIAR_OPERATIVE,
    &TELL_ME_WHAT_YOU_SEE,
    &TENACIOUS_DRONE,
    &TEST_OF_LOYALTY,
    &U_S_S_RELIANT_FROM_HELL_S_HEART,
    &VENGEANCE_MISSION,
    &ACADEMY_INSTRUCTOR,
    &BECKETT_MARINER_IMPETUOUS_ENSIGN,
    &BOARDING_ACTION,
    &BRAZEN_BLUFF,
    &CANTANKEROUS_CAPTAIN,
    &CAPTAIN_JAMES_T_KIRK,
    &CAPTAIN_MICHAEL_BURNHAM,
    &CAPTAIN_S_TUTELAGE,
    &D_7_BATTLE_CRUISER,
    &DAHAR_MASTER,
    &DOMINION_INFILTRATION,
    &ENCOUNTER_THE_UNKNOWN,
    &ERICA_ORTEGAS_PLUCKY_PILOT,
    &EXPLORATORY_MISSION,
    &GENESIS_EFFECT,
    &HORTA,
    &INFINITE_DIVERSITY,
    &KHAAAAAAAAAAAANNN,
    &KHAN_DESPOT_BY_DESIGN,
    &LA_AN_NOONIEN_SINGH_SECURITY,
    &LWAXANA_TROI_CHARMING_GUEST,
    &MUNITIONS_ENTHUSIAST,
    &THE_NAKED_TIME,
    &PAH_WRAITH,
    &PERILS_OF_THE_PAST,
    &PLANETARY_BOMBARDMENT,
    &PLANETARY_PATROL,
    &PLASMA_CASCADE,
    &Q_STACKING_THE_DECK,
    &QUARK_HUMBLE_PROPRIETOR,
    &RAVENOUS_COSMOZOAN,
    &RED_ALERT,
    &SARCOPHAGUS_SHIP_OF_THE_DEAD,
    &SAVVY_PROFITEER,
    &SILICATE_SURVEYOR,
    &STARFLEET_SECURITY,
    &TAL_SHIAR_ANALYST,
    &TARG_WRANGLER,
    &TASHA_YAR_OUT_OF_TIME,
    &TEST_OF_IDENTITY,
    &TRANSPORTER_MALFUNCTION,
    &WILL_RIKER_FIRST_IN_THE_FIELD,
    &WORF_INTELLIGENCE_CONTRACTOR,
    &THE_WRATH_OF_KHAN,
    &ADVERSARY_BY_ALGORITHM,
    &AMOK_TIME,
    &ASTUTE_TERRAFORMER,
    &BRANCHING_TIMELINE,
    &CAPTAIN_JONATHAN_ARCHER,
    &CAPTAIN_KATHRYN_JANEWAY,
    &CAPTAIN_PIKE_FULLNESS_OF_TIME,
    &COLD_BLOODED_CREW,
    &COMMON_GOAL,
    &CULBER_AND_STAMETS_STAR_CROSSED,
    &DATHON_AND_PICARD_AT_EL_ADREL,
    &DAVID_AND_CAROL_MARCUS_CREATORS,
    &DOMINION_ASSAULT_FORCE,
    &DOMINION_SUPERVISOR,
    &DR_JOSEPH_M_BENGA,
    &DR_LEONARD_BONES_MCCOY,
    &FINAL_FRONTIER,
    &FIRST_CONTACT,
    &GENESIS_DEVICE,
    &HOLO_ENGINEER,
    &HOLOGRAM_STALKER,
    &I_M_A_DOCTOR_NOT_A,
    &JADZIA_DAX_LIVING_LEGACY,
    &JAKE_SISKO_WAR_CORRESPONDENT,
    &MACO_STRIKE_TEAM,
    &MOOPSY,
    &MUGATO,
    &NEELIX_MORAL_SUPPORT,
    &PHOTOSYNTHETIC_PHYSICIAN,
    &ROK_TAHK_BUDDING_SCIENTIST,
    &SARU_UNTHREATENED,
    &SAURIAN_EXPLORER,
    &SPECIES_8472,
    &STABLE_WORMHOLE,
    &SUPPORT_MISSION,
    &SYLVIA_TILLY_JOY_OF_DISCOVERY,
    &SYNDICATE_LIQUIDATORS,
    &TEMPORAL_CONQUEROR,
    &TEST_OF_COURAGE,
    &U_S_S_DISCOVERY_AT_BLACK_ALERT,
    &U_S_S_VOYAGER_LONE_WANDERER,
    &WARP_SPEED,
    &XENOBOTANIST,
    &XINDI_SURVEYORS,
    &ALL_GOOD_THINGS,
    &BORG_CUBE,
    &THE_CITY_ON_THE_EDGE_OF_FOREVER,
    &CLEVELAND_BOOKER_EMPATH,
    &CONSTABLE_ODO,
    &DAL_R_EL_AUGMENT_ORPHAN,
    &DANIELS_TEMPORAL_AGENT,
    &DATHON_HIS_HEART_OPEN,
    &THE_DOCTOR_EMH,
    &THE_DURAS_SISTERS,
    &D_VANA_TENDI_OPTIMISTIC_ENSIGN,
    &GOWRON_EAGER_FOR_BATTLE,
    &GRAY_AND_ADIRA_FOREVER_JOINED,
    &HEMMER_FIXING_WHAT_IS_BROKEN,
    &HIT_IT,
    &THE_INNER_LIGHT,
    &KIRA_NERYS_FIREBRAND,
    &KIRK_COMMANDING_OFFICER,
    &LIVE_LONG_AND_PROSPER,
    &LOCUTUS_THE_ONE_WHO_SPEAKS,
    &LORE_POSITRONIC_PROVOCATEUR,
    &L_RELL_FIERCE_VISIONARY,
    &MIRROR_UNIVERSE_INTRUSION,
    &NOG_AMBITIOUS_CADET,
    &PHLOX_UNORTHODOX_DOCTOR,
    &SAREK_FEDERATION_AMBASSADOR,
    &SEVEN_OF_NINE,
    &SPOCK_ALWAYS_YOUR_FRIEND,
    &THE_TRAVELER,
    &VASH_FREELANCE_ARCHAEOLOGIST,
    &WORF_HONOR_BOUND_WARRIOR,
    &YEAR_OF_HELL,
    &ZERO_INCORPOREAL_SENSATE,
    &CLASS_F_SHUTTLECRAFT,
    &COMBADGE,
    &CRYSTALLINE_ENTITY,
    &CYBERNETIC_SPECIALIST,
    &DEEP_SPACE_9,
    &THE_HOLODECK,
    &ISOLINEAR_PROCESSOR,
    &REPLICATOR,
    &SICKBAY,
    &TEA_EARL_GREY_HOT,
    &TRICORDER,
    &TYPE_2_PHASER,
    &U_S_S_CERRITOS_FOLLOWING_UP,
    &U_S_S_ENTERPRISE_D_GALAXY_CLASS,
    &WARP_CORE,
    &ANDORIA,
    &ARTIFICIAL_ANOMALY,
    &BAJOR,
    &CARDASSIA_PRIME,
    &EARTH_SECTOR_001,
    &FEDERATION_OUTPOST,
    &FERENGINAR,
    &GASEOUS_ANOMALY,
    &NI_VAR_SCIENCE_ACADEMY,
    &ORION,
    &Q_CONTINUUM,
    &QO_NOS,
    &QUARK_S_BAR,
    &RISA,
    &ROMULUS,
    &SPATIAL_ANOMALY,
    &STARFLEET_ACADEMY,
    &SUBSPACE_ANOMALY,
    &TEMPORAL_ANOMALY,
    &UNEXPLORED_TERRITORY,
    &THE_UNICOMPLEX,
    &VULCAN,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    RECKLESS_IMPULSE_REPRINT,
    ENGINEERED_EXPLOSIVES_REPRINT,
    BLOOD_CRYPT_REPRINT,
    BREEDING_POOL_REPRINT,
    GODLESS_SHRINE_REPRINT,
    HALLOWED_FOUNTAIN_REPRINT,
    OVERGROWN_TOMB_REPRINT,
    SACRED_FOUNDRY_REPRINT,
    STEAM_VENTS_REPRINT,
    STOMPING_GROUND_REPRINT,
    TEMPLE_GARDEN_REPRINT,
    WATERY_GRAVE_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_1,
    SCOTTY_MIRACLE_WORKER_ALTERNATE_1,
    DR_LEONARD_BONES_MCCOY_ALTERNATE_1,
    KIRK_COMMANDING_OFFICER_ALTERNATE_1,
    SPOCK_ALWAYS_YOUR_FRIEND_ALTERNATE_1,
    FEDERATION_OUTPOST_ALTERNATE_1,
    NI_VAR_SCIENCE_ACADEMY_ALTERNATE_1,
    Q_CONTINUUM_ALTERNATE_1,
    QUARK_S_BAR_ALTERNATE_1,
    STARFLEET_ACADEMY_ALTERNATE_1,
    THE_UNICOMPLEX_ALTERNATE_1,
    RESISTANCE_IS_FUTILE_ALTERNATE_1,
    TEST_OF_COURAGE_ALTERNATE_1,
    CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_1,
    COMMAND_SUBADJUNCT_ALTERNATE_1,
    CONQUEST_MISSION_ALTERNATE_1,
    SEVEN_OF_NINE_ALTERNATE_1,
    CAPTAIN_BENJAMIN_SISKO_ALTERNATE_1,
    DOMINION_SABOTEUR_ALTERNATE_1,
    THE_HOLODECK_ALTERNATE_1,
    QUARK_HUMBLE_PROPRIETOR_ALTERNATE_1,
    CONSTABLE_ODO_ALTERNATE_1,
    DOMINION_INFILTRATION_ALTERNATE_1,
    QUARK_S_BAR_ALTERNATE_2,
    NOG_AMBITIOUS_CADET_ALTERNATE_1,
    JADZIA_DAX_LIVING_LEGACY_ALTERNATE_1,
    MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_1,
    MIRROR_UNIVERSE_INTRUSION_ALTERNATE_1,
    KIRK_COMMANDING_OFFICER_ALTERNATE_2,
    SULU_SPACE_SWASHBUCKLER_ALTERNATE_1,
    TRANSPORTER_MALFUNCTION_ALTERNATE_1,
    NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_2,
    LIEUTENANT_COMMANDER_DATA_ALTERNATE_1,
    LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_1,
    INFINITE_DIVERSITY_ALTERNATE_1,
    DEATH_IS_IRRELEVANT_ALTERNATE_1,
    CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_1,
    COMMAND_SUBADJUNCT_ALTERNATE_2,
    DEANNA_TROI_PSYCHIC_COUNSEL_ALTERNATE_1,
    DR_BEVERLY_CRUSHER_ALTERNATE_1,
    PEACE_IN_OUR_TIME_ALTERNATE_1,
    PRIMARY_UNIMATRIX_ADJUNCT_ALTERNATE_1,
    THERE_S_COFFEE_IN_THAT_NEBULA_ALTERNATE_1,
    DEEP_SPACE_SCAN_ALTERNATE_1,
    LIEUTENANT_COMMANDER_DATA_ALTERNATE_2,
    LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_1,
    SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_1,
    WESLEY_CRUSHER_STARFLEET_CADET_ALTERNATE_1,
    DEATH_IS_IRRELEVANT_ALTERNATE_2,
    ORGANIC_AVULSION_UNIT_ALTERNATE_1,
    OVERFLOW_SUBMATRIX_ALTERNATE_1,
    RESISTANCE_IS_FUTILE_ALTERNATE_2,
    SELA_TAL_SHIAR_PARADOX_ALTERNATE_1,
    DOMINION_INFILTRATION_ALTERNATE_2,
    ENCOUNTER_THE_UNKNOWN_ALTERNATE_1,
    INFINITE_DIVERSITY_ALTERNATE_2,
    Q_STACKING_THE_DECK_ALTERNATE_1,
    TASHA_YAR_OUT_OF_TIME_ALTERNATE_1,
    WILL_RIKER_FIRST_IN_THE_FIELD_ALTERNATE_1,
    CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_2,
    WARP_SPEED_ALTERNATE_1,
    DATHON_HIS_HEART_OPEN_ALTERNATE_1,
    LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_2,
    WORF_HONOR_BOUND_WARRIOR_ALTERNATE_1,
    REPLICATOR_ALTERNATE_1,
    HALLOWED_FOUNTAIN_ALTERNATE_1,
    WATERY_GRAVE_ALTERNATE_1,
    BLOOD_CRYPT_ALTERNATE_1,
    STOMPING_GROUND_ALTERNATE_1,
    TEMPLE_GARDEN_ALTERNATE_1,
    GODLESS_SHRINE_ALTERNATE_1,
    STEAM_VENTS_ALTERNATE_1,
    OVERGROWN_TOMB_ALTERNATE_1,
    SACRED_FOUNDRY_ALTERNATE_1,
    BREEDING_POOL_ALTERNATE_1,
    DR_MCCOY_SURLY_SURGEON_ALTERNATE_1,
    EXCALBIAN_GAMEMASTER_ALTERNATE_1,
    FEDERATION_RESUPPLY_ALTERNATE_1,
    THE_NEEDS_OF_THE_MANY_ALTERNATE_1,
    SAAVIK_STOIC_STUDENT_ALTERNATE_1,
    SULU_SPACE_SWASHBUCKLER_ALTERNATE_2,
    TROUBLING_TRIBBLES_ALTERNATE_1,
    ENSIGN_PAVEL_CHEKOV_ALTERNATE_1,
    KOBAYASHI_MARU_SCENARIO_ALTERNATE_1,
    MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_2,
    NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_3,
    SCOTTY_MIRACLE_WORKER_ALTERNATE_2,
    THE_DESTRUCTION_OF_PRAXIS_ALTERNATE_1,
    GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_1,
    KHAN_VENGEFUL_HEART_ALTERNATE_1,
    SALT_VAMPIRE_ALTERNATE_1,
    CAPTAIN_JAMES_T_KIRK_ALTERNATE_1,
    KHAN_DESPOT_BY_DESIGN_ALTERNATE_1,
    DAVID_AND_CAROL_MARCUS_CREATORS_ALTERNATE_1,
    DR_LEONARD_BONES_MCCOY_ALTERNATE_2,
    GENESIS_DEVICE_ALTERNATE_1,
    LIVE_LONG_AND_PROSPER_ALTERNATE_1,
    MIRROR_UNIVERSE_INTRUSION_ALTERNATE_2,
    SAREK_FEDERATION_AMBASSADOR_ALTERNATE_1,
    BAT_LETH_ALTERNATE_1,
    GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_2,
    A_GOOD_DAY_TO_DIE_ALTERNATE_1,
    STORIED_VETERAN_ALTERNATE_1,
    VENGEANCE_MISSION_ALTERNATE_1,
    WORF_INTELLIGENCE_CONTRACTOR_ALTERNATE_1,
    THE_DURAS_SISTERS_ALTERNATE_1,
    GOWRON_EAGER_FOR_BATTLE_ALTERNATE_1,
    L_RELL_FIERCE_VISIONARY_ALTERNATE_1,
    WORF_HONOR_BOUND_WARRIOR_ALTERNATE_2,
    BAT_LETH_ALTERNATE_2,
    GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_3,
    A_GOOD_DAY_TO_DIE_ALTERNATE_2,
    STORIED_VETERAN_ALTERNATE_2,
    VENGEANCE_MISSION_ALTERNATE_2,
    WORF_INTELLIGENCE_CONTRACTOR_ALTERNATE_2,
    THE_DURAS_SISTERS_ALTERNATE_2,
    GOWRON_EAGER_FOR_BATTLE_ALTERNATE_2,
    L_RELL_FIERCE_VISIONARY_ALTERNATE_2,
    WORF_HONOR_BOUND_WARRIOR_ALTERNATE_3,
    ANTAGONIST_INCORPORATOR_ALTERNATE_1,
    MILES_O_BRIEN_TRANSPORTER_OPS_ALTERNATE_1,
    U_S_S_ENTERPRISE_A_FREE_SPIRIT_ALTERNATE_1,
    THE_CARETAKER_ALTERNATE_1,
    GWYNDALA_THE_PROGENY_ALTERNATE_1,
    H_M_S_BOUNTY_UNLIKELY_TIMESHIP_ALTERNATE_1,
    ROMULAN_BIRD_OF_PREY_ALTERNATE_1,
    SUBCOMMANDER_T_POL_ALTERNATE_1,
    ARMUS_SKIN_OF_EVIL_ALTERNATE_1,
    COLLATION_TRIMATRIX_ALTERNATE_1,
    ELIM_GARAK_SIMPLE_TAILOR_ALTERNATE_1,
    GEORGIOU_TERRAN_EMPEROR_ALTERNATE_1,
    GRAND_NAGUS_ZEK_ALTERNATE_1,
    GUL_DUKAT_COLONIAL_PREFECT_ALTERNATE_1,
    KAI_WINN_FAITHLESS_LEADER_ALTERNATE_1,
    U_S_S_RELIANT_FROM_HELL_S_HEART_ALTERNATE_1,
    BRAZEN_BLUFF_ALTERNATE_1,
    D_7_BATTLE_CRUISER_ALTERNATE_1,
    LWAXANA_TROI_CHARMING_GUEST_ALTERNATE_1,
    PAH_WRAITH_ALTERNATE_1,
    BRANCHING_TIMELINE_ALTERNATE_1,
    CAPTAIN_JONATHAN_ARCHER_ALTERNATE_1,
    CAPTAIN_PIKE_FULLNESS_OF_TIME_ALTERNATE_1,
    DR_JOSEPH_M_BENGA_ALTERNATE_1,
    HOLO_ENGINEER_ALTERNATE_1,
    MACO_STRIKE_TEAM_ALTERNATE_1,
    ROK_TAHK_BUDDING_SCIENTIST_ALTERNATE_1,
    STABLE_WORMHOLE_ALTERNATE_1,
    U_S_S_DISCOVERY_AT_BLACK_ALERT_ALTERNATE_1,
    BORG_CUBE_ALTERNATE_1,
    HIT_IT_ALTERNATE_1,
    KIRA_NERYS_FIREBRAND_ALTERNATE_1,
    CRYSTALLINE_ENTITY_ALTERNATE_1,
    DEEP_SPACE_9_ALTERNATE_1,
    ENGINEERED_EXPLOSIVES_ALTERNATE_1,
    U_S_S_ENTERPRISE_D_GALAXY_CLASS_ALTERNATE_1,
    PLAINS_ALTERNATE_4,
    ISLAND_ALTERNATE_4,
    SWAMP_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_4,
    FOREST_ALTERNATE_4,
    HALLOWED_FOUNTAIN_ALTERNATE_2,
    WATERY_GRAVE_ALTERNATE_2,
    BLOOD_CRYPT_ALTERNATE_2,
    STOMPING_GROUND_ALTERNATE_2,
    TEMPLE_GARDEN_ALTERNATE_2,
    GODLESS_SHRINE_ALTERNATE_2,
    STEAM_VENTS_ALTERNATE_2,
    OVERGROWN_TOMB_ALTERNATE_2,
    SACRED_FOUNDRY_ALTERNATE_2,
    BREEDING_POOL_ALTERNATE_2,
    CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_2,
    COMMAND_SUBADJUNCT_ALTERNATE_3,
    DEANNA_TROI_PSYCHIC_COUNSEL_ALTERNATE_2,
    DR_BEVERLY_CRUSHER_ALTERNATE_2,
    PEACE_IN_OUR_TIME_ALTERNATE_2,
    PRIMARY_UNIMATRIX_ADJUNCT_ALTERNATE_2,
    THERE_S_COFFEE_IN_THAT_NEBULA_ALTERNATE_2,
    DEEP_SPACE_SCAN_ALTERNATE_2,
    LIEUTENANT_COMMANDER_DATA_ALTERNATE_3,
    LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_2,
    SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_2,
    WESLEY_CRUSHER_STARFLEET_CADET_ALTERNATE_2,
    DEATH_IS_IRRELEVANT_ALTERNATE_3,
    ORGANIC_AVULSION_UNIT_ALTERNATE_2,
    OVERFLOW_SUBMATRIX_ALTERNATE_2,
    RESISTANCE_IS_FUTILE_ALTERNATE_3,
    SELA_TAL_SHIAR_PARADOX_ALTERNATE_2,
    DOMINION_INFILTRATION_ALTERNATE_3,
    ENCOUNTER_THE_UNKNOWN_ALTERNATE_2,
    INFINITE_DIVERSITY_ALTERNATE_3,
    Q_STACKING_THE_DECK_ALTERNATE_2,
    TASHA_YAR_OUT_OF_TIME_ALTERNATE_2,
    WILL_RIKER_FIRST_IN_THE_FIELD_ALTERNATE_2,
    CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_3,
    WARP_SPEED_ALTERNATE_2,
    DATHON_HIS_HEART_OPEN_ALTERNATE_2,
    LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_3,
    WORF_HONOR_BOUND_WARRIOR_ALTERNATE_4,
    REPLICATOR_ALTERNATE_2,
    DR_MCCOY_SURLY_SURGEON_ALTERNATE_2,
    EXCALBIAN_GAMEMASTER_ALTERNATE_2,
    FEDERATION_RESUPPLY_ALTERNATE_2,
    THE_NEEDS_OF_THE_MANY_ALTERNATE_2,
    SAAVIK_STOIC_STUDENT_ALTERNATE_2,
    SULU_SPACE_SWASHBUCKLER_ALTERNATE_3,
    TROUBLING_TRIBBLES_ALTERNATE_2,
    ENSIGN_PAVEL_CHEKOV_ALTERNATE_2,
    KOBAYASHI_MARU_SCENARIO_ALTERNATE_2,
    MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_3,
    NYOTA_UHURA_COMMUNICATIONS_ALTERNATE_4,
    SCOTTY_MIRACLE_WORKER_ALTERNATE_3,
    THE_DESTRUCTION_OF_PRAXIS_ALTERNATE_2,
    GENERAL_CHANG_COLD_WARRIOR_ALTERNATE_4,
    KHAN_VENGEFUL_HEART_ALTERNATE_2,
    SALT_VAMPIRE_ALTERNATE_2,
    CAPTAIN_JAMES_T_KIRK_ALTERNATE_2,
    KHAN_DESPOT_BY_DESIGN_ALTERNATE_2,
    DAVID_AND_CAROL_MARCUS_CREATORS_ALTERNATE_2,
    DR_LEONARD_BONES_MCCOY_ALTERNATE_3,
    GENESIS_DEVICE_ALTERNATE_2,
    LIVE_LONG_AND_PROSPER_ALTERNATE_2,
    MIRROR_UNIVERSE_INTRUSION_ALTERNATE_3,
    SAREK_FEDERATION_AMBASSADOR_ALTERNATE_2,
    U_S_S_ENTERPRISE_A_FREE_SPIRIT_ALTERNATE_2,
    H_M_S_BOUNTY_UNLIKELY_TIMESHIP_ALTERNATE_2,
    U_S_S_RELIANT_FROM_HELL_S_HEART_ALTERNATE_2,
    D_7_BATTLE_CRUISER_ALTERNATE_2,
    U_S_S_DISCOVERY_AT_BLACK_ALERT_ALTERNATE_2,
    BORG_CUBE_ALTERNATE_2,
    DEEP_SPACE_9_ALTERNATE_2,
    U_S_S_ENTERPRISE_D_GALAXY_CLASS_ALTERNATE_2,
    DR_LEONARD_BONES_MCCOY_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    PLAINS_ALTERNATE_6,
    ISLAND_ALTERNATE_5,
    ISLAND_ALTERNATE_6,
    SWAMP_ALTERNATE_5,
    SWAMP_ALTERNATE_6,
    MOUNTAIN_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_6,
    FOREST_ALTERNATE_5,
    FOREST_ALTERNATE_6,
    COMMAND_SUBADJUNCT_ALTERNATE_4,
    CONQUEST_MISSION_ALTERNATE_2,
    MALFUNCTIONING_HOLODECK_ALTERNATE_1,
    CHA_DICH_INVESTIGATOR_ALTERNATE_1,
    HIVE_MIND_COPROCESSOR_ALTERNATE_1,
    COLD_BLOODED_CREW_ALTERNATE_1,
    SYNDICATE_LIQUIDATORS_ALTERNATE_1,
    LOCUTUS_THE_ONE_WHO_SPEAKS_ALTERNATE_4,
    EARTH_SECTOR_001_ALTERNATE_1,
    RISA_ALTERNATE_1,
    VULCAN_ALTERNATE_1,
    CAPTAIN_JAMES_T_KIRK_ALTERNATE_3,
    CAPTAIN_JEAN_LUC_PICARD_ALTERNATE_3,
    MR_SPOCK_SCIENCE_OFFICER_ALTERNATE_4,
    LIEUTENANT_COMMANDER_DATA_ALTERNATE_4,
    CAPTAIN_KATHRYN_JANEWAY_ALTERNATE_4,
    SEVEN_OF_NINE_UNASSIMILATED_ALTERNATE_3,
    LT_CMDR_GEORDI_LA_FORGE_ALTERNATE_3,
];
