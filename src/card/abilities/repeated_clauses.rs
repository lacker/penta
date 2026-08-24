// The clauses more than one card prints word for word.
//
// Not keywords and not a cycle: what belongs here is a run of instructions
// that several unrelated cards happen to say identically, so that the second
// card to say it does not re-derive it. Included textually into
// `abilities.rs`, so the imports here are the parent module's.

/// Brainstorm's clause, which Jace, the Mind Sculptor prints again word for
/// word as his zero: draw three cards, then put two cards from your hand on
/// top of your library in any order.
///
/// The arrangement is the order the two are named in: each is placed on top
/// of the last, so the card named second is the one drawn first.
#[must_use]
pub const fn brainstorm() -> EffectDef {
    EffectDef::Sequence(&BRAINSTORM_STEPS)
}

static BRAINSTORM_STEPS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
    EffectDef::ChooseCards {
        player: EffectRecipientDef::Controller,
        sources: &BRAINSTORM_HAND,
        object: ObjectPredicateDef::Any,
        minimum: 2,
        maximum: 2,
        reveal: false,
        destination: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
    },
];

static BRAINSTORM_HAND: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

const END_OF_COMBAT_DESTROY_BINDING: ObjectSetBindingIndex = ObjectSetBindingIndex::PRIMARY;
static DESTROY_AT_END_OF_COMBAT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
        END_OF_COMBAT_DESTROY_BINDING,
    )),
    can_regenerate: true,
};
static END_OF_COMBAT_DESTROY_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At end of combat, destroy that creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::EndOfCombat,
        player: PlayerRelation::Any,
    },
    DESTROY_AT_END_OF_COMBAT,
);
static INSTALL_END_OF_COMBAT_DESTROY_TRIGGER: EffectDef =
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&END_OF_COMBAT_DESTROY_TRIGGER));

/// Remember the creature from a blocking trigger, then create the ordinary
/// one-shot delayed trigger that destroys it at end of combat.
///
/// The binding is what keeps "that creature" tied to the original combat
/// event after the later step-begin event replaces the trigger context.
#[must_use]
pub const fn destroy_triggering_object_at_end_of_combat() -> EffectDef {
    EffectDef::BindMatching {
        objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
        binding: END_OF_COMBAT_DESTROY_BINDING,
        then: &INSTALL_END_OF_COMBAT_DESTROY_TRIGGER,
    }
}

/// The wheel: each player shuffles their hand and graveyard into their
/// library, then draws seven cards. Timetwister, Echo of Eons, and Time
/// Spiral print the same three steps word for word.
///
/// The spell doing it is on the stack while this resolves, so it is not
/// among the cards that go back; where it lands afterwards is the card's own
/// resolution destination.
#[must_use]
pub const fn shuffle_back_and_draw_seven() -> EffectDef {
    EffectDef::Sequence(&WHEEL_STEPS)
}

static WHEEL_STEPS: [EffectDef; 3] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Any,
            &[ZoneKind::Hand, ZoneKind::Graveyard],
            PlayerRelation::Any,
        ),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::EachPlayer,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::EachPlayer,
        amount: ValueDef::Constant(7),
    },
];
