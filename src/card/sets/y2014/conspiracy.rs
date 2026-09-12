//! Conspiracy cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CNS",
    slug: "conspiracy",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CNS 16 — Council's Judgment
pub(in crate::card::sets) static COUNCILS_JUDGMENT: CardRecord = CardRecord::new(
    "Council's Judgment",
    "17f28b16-da65-41a8-ba4f-f1c5e104aad6",
    "Kev Walker",
// Exiling without targeting is what it is played for: shroud, hexproof,
    // and protection are all no answer at all. Two players usually means two
    // permanents, since a disagreement ties.
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "Will of the council — Starting with you, each player votes for a nonland permanent you don't control. Exile each permanent with the most votes or tied for most votes.",
        EffectDef::VoteForPermanentToExile {
            // "A nonland permanent you don't control" is read against the spell's
            // controller for every voter, so both players choose from the same ballot.
            // The vote machinery supplies the "you don't control" half.
            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        },
    )),
);

// CNS 18 — Custodi Squire
// Audit: unsupported — Needs a will-of-the-council vote over graveyard cards that returns the winners. The only vote effect is VoteForPermanentToExile, which votes over battlefield permanents and exiles them; this votes over cards in your graveyard and returns every card tied for most votes to your hand.
pub(in crate::card::sets) static CUSTODI_SQUIRE: CardRecord = CardRecord::new(
    "Custodi Squire",
    "a9151422-8df1-409c-a686-0cd89247eb43",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// CNS 36 — Treasonous Ogre
pub(in crate::card::sets) static TREASONOUS_OGRE_36: CardRecord = CardRecord::new(
    "Treasonous Ogre",
    "ae48c31d-6fd9-457f-adb8-37f367724ba1",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Ogre", "Shaman"], 2, 3).with_abilities(&[
AbilityDef::triggered("Dethrone (Whenever this creature attacks the player with the most life or tied for most life, put a +1/+1 counter on it.)", TriggerEventDef::While { event: &TriggerEventDef::attacks_a_player(ObjectPredicateDef::Source), condition: &TriggerConditionDef::PlayerHasMostLife(PlayerRelation::Opponent) }, EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::activated_mana("Pay 3 life: Add {R}.", &[CostDef::PayLife(3)], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)))
]),
);

// CNS 42 — Dack Fayden
pub(in crate::card::sets) static DACK_FAYDEN: CardRecord = CardRecord::new(
    "Dack Fayden",
    "3fcb7810-1054-4001-855c-6e17939b3d3f",
    "Eric Deschamps",
// The greatest thief in the multiverse, and in a cube full of Moxen the
    // minus is what he is actually here for.
    CardRules::new_planeswalker(mana_cost!("{1}{U}{R}"), &["Dack"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Target player draws two then discards two cards.",
                &[CostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                // Two for two is a wash against most decks and a windmill against a graveyard
                // one, which is the whole reason to point it at yourself.
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            // Nothing is holding the theft and no cleanup ends it: a control change
            // with no stated duration lasts indefinitely (CR 611.2b).
            AbilityDef::activated_with_targets(
                "−2: Gain control of target artifact.",
                &[CostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::Indefinitely,
                ),
            ),
            AbilityDef::activated(
                "−6: You get an emblem with \"Whenever you cast a spell that targets one or more \
                 permanents, gain control of those permanents.\"",
                &[CostDef::Loyalty(-6)],
                EffectDef::create_emblem("Dack Fayden emblem", &[AbilityDef::triggered(
                    "Whenever you cast a spell that targets one or more permanents, gain control of those \
                         permanents.",
                    TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::Any),
                    ])),
                    EffectDef::gain_control(
                        EffectRecipientDef::objects(ObjectSetDef::PermanentsTargetedBy(
                            ObjectRefDef::TriggeringObject,
                        )),
                        PlayerRefDef::EffectController,
                        ControlDurationDef::Indefinitely,
                    ),
                )]),
            ),
        ]),
);

// CNS 51 — Selvala, Explorer Returned
// Audit: unsupported — The activated-mana runtime requires a plannable AddMana effect. It cannot reveal both libraries, compute production from those hidden cards, gain life, and draw cards within the same immediate mana resolution.
pub(in crate::card::sets) static SELVALA_EXPLORER_RETURNED_51: CardRecord = CardRecord::new(
    "Selvala, Explorer Returned",
    "89d4786c-e022-4ae5-9ef3-75886db51f49",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COUNCILS_JUDGMENT,
    &CUSTODI_SQUIRE,
    &TREASONOUS_OGRE_36,
    &DACK_FAYDEN,
    &SELVALA_EXPLORER_RETURNED_51,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
