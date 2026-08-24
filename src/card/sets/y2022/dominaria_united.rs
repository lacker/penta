//! Dominaria United cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet,
    CardSupertype, CardType, DrawEventMatcherDef, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ObjectPredicateDef, PlayerRelation, StackTargetKindDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// DMU 2 — Anointed Peacekeeper
pub(in crate::card::sets) static ANOINTED_PEACEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b8127b5-3a65-411a-84bc-54e5c1be1477"),
    "Anointed Peacekeeper",
    crate::card::CardArt::new("5b8127b5-3a65-411a-84bc-54e5c1be1477", "Tia Masic"),
    crate::card::CardSet::DominariaUnited,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::look_at_opponent_hand_then_choose_card_name_as_enters(
            "As this creature enters, look at an opponent's hand, then choose any card name.",
        ),
        abilities::chosen_name_spell_cost_increase(
            "Spells your opponents cast with the chosen name cost {2} more to cast.",
            PlayerRelation::Opponent,
            mana_cost!("{2}"),
        ),
        abilities::chosen_name_ability_cost_increase(
            "Activated abilities of sources with the chosen name cost {2} more to activate unless they're mana abilities.",
            mana_cost!("{2}"),
        ),
    ]),
);

// DMU 24 — Leyline Binding
/// "Until this enchantment leaves the battlefield" is one printed clause, so
/// the return rides on a delayed trigger rather than appearing as a second
/// ability the card does not print.
static BINDING_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "When this enchantment leaves the battlefield, return the exiled card to the battlefield \
     under its owner's control.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        None,
    ),
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
);

static A_NONLAND_PERMANENT_THEY_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

static BINDING_EXILES_IT: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&BINDING_RETURNS_IT)),
];

pub(in crate::card::sets) static LEYLINE_BINDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c3ac3dd-35db-447f-8674-37b4680a1ef7"),
    "Leyline Binding",
    CardArt::new("3c3ac3dd-35db-447f-8674-37b4680a1ef7", "Cristi Balanescu"),
    CardSet::DominariaUnited,
    // Six mana on paper and one in a deck with every basic land type, cast
    // at instant speed: the whole card is the mana base it asks for.
    CardRules::new_enchantment(mana_cost!("{5}{W}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::static_ability(
            "Domain — This spell costs {1} less to cast for each basic land type among lands you \
             control.",
            EffectDef::ReduceGenericCostBy(ValueDef::BasicLandTypesControlled(PlayerRelation::You)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile target nonland permanent an opponent controls \
             until this enchantment leaves the battlefield.",
            &A_NONLAND_PERMANENT_THEY_CONTROL,
            EffectDef::Sequence(&BINDING_EXILES_IT),
        ),
    ]),
);

// DMU 72 — Tolarian Terror
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_TERROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42f01cba-43d4-46ad-b7a5-d7631b0e1347"),
    "Tolarian Terror",
    crate::card::CardArt::new(
        "42f01cba-43d4-46ad-b7a5-d7631b0e1347",
        "Vincent Christiaens",
    ),
    crate::card::CardSet::DominariaUnited,
    crate::card::CardRules::unsupported(),
);

// DMU 89 — Cut Down
/// "Total power and toughness 5 or less" is read live, so a creature that
/// was in range stops being a legal target the moment anything pumps it.
static CUT_DOWN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::TotalPowerAndToughnessAtMost(5),
    ]),
)];

pub(in crate::card::sets) static CUT_DOWN: CardRecord = CardRecord::new_with_legacy_id(
    2204,
    "Cut Down",
    CardArt::new("753db072-5d6a-4f37-8f7d-255572ecd3bd", "Dominik Mayer"),
    CardSet::DominariaUnited,
    // One black mana answers most of what an aggressive deck plays and
    // nothing of what a big one does, which is the whole design.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with total power and toughness 5 or less.",
        &CUT_DOWN_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// DMU 107 — Sheoldred, the Apocalypse
/// Two clauses rather than one symmetrical one, because they are not
/// symmetrical: yours gains and theirs loses, and a card that made both
/// players lose would read very differently.
static SHEOLDRED_ABILITIES: [AbilityDef; 3] = [
    abilities::deathtouch(),
    AbilityDef::triggered(
        "Whenever you draw a card, you gain 2 life.",
        TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ),
    AbilityDef::triggered(
        "Whenever an opponent draws a card, they lose 2 life.",
        TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(2),
        },
    ),
];

pub(in crate::card::sets) static SHEOLDRED_THE_APOCALYPSE: CardRecord =
    CardRecord::new_with_legacy_id(
        2180,
        "Sheoldred, the Apocalypse",
        CardArt::new("d67be074-cdd4-41d9-ac89-0a0456c4e4b2", "Chris Rahn"),
        CardSet::DominariaUnited,
        // A four-mana 4/5 deathtouch would be playable on its own. The draw
        // clauses are what make it unanswerable: the opponent's own draw step
        // pays for it, every turn it survives.
        CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Praetor"], 4, 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&SHEOLDRED_ABILITIES),
    );

// DMU 137 — Lightning Strike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbb03f2e-2b92-4aa1-afae-301ed5d151d3"),
    "Lightning Strike",
    crate::card::CardArt::new("7d541125-bfb8-4f88-8bf3-ad7b6af7ad1d", "Marta Nael"),
    crate::card::CardSet::DominariaUnited,
    crate::card::CardRules::unsupported(),
);

/// What two mana buys, on either side of the board.
static AN_ARTIFACT_OR_ENCHANTMENT: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
    )];

/// What four buys instead. "Instead" is the point: the kicked spell targets
/// once, not twice, and what it may name is wider rather than longer.
static A_NONLAND_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
)];

static TEAR_ASUNDER_EXILES: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
    controller: None,
};

// DMU 183 — Tear Asunder
pub(in crate::card::sets) static TEAR_ASUNDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("629aa907-9533-4681-9bf2-9e56450a4cc2"),
    "Tear Asunder",
    CardArt::new("629aa907-9533-4681-9bf2-9e56450a4cc2", "Dave Kendall"),
    CardSet::DominariaUnited,
    // Two mana for the artifact or enchantment the deck was worried about,
    // or four for anything at all -- and exile rather than destruction,
    // which is what the extra mana is really paying for.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Kicker {1}{B} (You may pay an additional {1}{B} as you cast this spell.)\nExile \
             target artifact or enchantment. If this spell was kicked, exile target nonland \
             permanent instead.",
            &AN_ARTIFACT_OR_ENCHANTMENT,
            TEAR_ASUNDER_EXILES,
        ),
        abilities::kicker(
            mana_cost!("{2}{G}{B}"),
            "Exile target nonland permanent.",
            &A_NONLAND_PERMANENT,
            TEAR_ASUNDER_EXILES,
        ),
    ]),
);

// DMU 339 — Ertai Resurrected
/// "Spell, activated ability, or triggered ability" is every stack object
/// there is: mana abilities never use the stack, so the wider slot needs no
/// clause excluding them.
static A_SPELL_OR_ABILITY: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::StackObject {
        object: ObjectPredicateDef::Any,
        controller: None,
        kind: StackTargetKindDef::SpellOrAbility,
    },
)];

/// "Another" is the exclusion; Ertai himself has just arrived, so without it
/// he would be a legal answer to his own trigger.
static ANOTHER_CREATURE_OR_PLANESWALKER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// Both modes pay the same compensation, and both read it off the target
/// after that target is gone: the countered or destroyed object is retired
/// with its controller recorded, which is what "its controller" wants.
static ERTAI_MODES: [AbilityDef; 2] = [
    AbilityDef::spell_with_targets(
        "Counter target spell, activated ability, or triggered ability. Its controller draws a \
         card.",
        &A_SPELL_OR_ABILITY,
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ]),
    ),
    AbilityDef::spell_with_targets(
        "Destroy another target creature or planeswalker. Its controller draws a card.",
        &ANOTHER_CREATURE_OR_PLANESWALKER,
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ]),
    ),
];

pub(in crate::card::sets) static ERTAI_RESURRECTED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c46a2ca-27fd-44d4-80d0-7c83ed0a564e"),
    "Ertai Resurrected",
    CardArt::new(
        "2c46a2ca-27fd-44d4-80d0-7c83ed0a564e",
        "Justin Hernandez & Alexis Hernandez",
    ),
    CardSet::DominariaUnited,
    // A flash body that answers something on the way in, and pays for the
    // privilege with the card its victim's controller draws.
    CardRules::new_creature(
        mana_cost!("{2}{U}{B}"),
        &["Phyrexian", "Human", "Wizard"],
        3,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flash(),
        AbilityDef::modal_triggered_up_to_one(
            "When this creature enters, choose up to one —\n• Counter target spell, activated \
             ability, or triggered ability. Its controller draws a card.\n• Destroy another \
             target creature or planeswalker. Its controller draws a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &ERTAI_MODES,
        ),
    ]),
);

// DMU 387 — Leyline Binding (alternate printing)

// DMU 388 — Serra Paragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_PARAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69284b53-f712-418c-94a0-4e5638117256"),
    "Serra Paragon",
    crate::card::CardArt::new("69284b53-f712-418c-94a0-4e5638117256", "Heonhwa"),
    crate::card::CardSet::DominariaUnited,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANOINTED_PEACEKEEPER,
    &LEYLINE_BINDING,
    &TOLARIAN_TERROR,
    &CUT_DOWN,
    &SHEOLDRED_THE_APOCALYPSE,
    &LIGHTNING_STRIKE,
    &TEAR_ASUNDER,
    &ERTAI_RESURRECTED,
    &SERRA_PARAGON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&LEYLINE_BINDING, 1), // DMU 387
];
