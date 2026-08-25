// The clauses more than one card prints word for word.
//
// Not keywords and not a cycle: what belongs here is a run of instructions
// that several unrelated cards happen to say identically, so that the second
// card to say it does not re-derive it. Included textually into
// `abilities.rs`, so the imports here are the parent module's.

static RETURN_EXILE_AT_NEXT_END_STEP: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, return the exiled card to the battlefield under its owner's control.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
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

static RETURN_EXILE_AT_NEXT_END_STEP_UNDER_YOUR_CONTROL: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, return the exiled card to the battlefield under your control.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: Some(PlayerRelation::You),
        transformed: false,
    },
);

/// Exile one object and install the delayed trigger that returns it under its
/// owner's control at the next end step. Linking, exile, and trigger
/// installation are deliberately hidden from the caller.
#[must_use]
pub const fn exile_until_next_end_step(object: EffectRecipientDef) -> EffectDef {
    EffectDef::ExileUntilNextEndStep {
        object,
        return_ability: &RETURN_EXILE_AT_NEXT_END_STEP,
    }
}

/// Venser's variant of [`exile_until_next_end_step`], which returns the card
/// under the effect controller's control instead of its owner's.
#[must_use]
pub const fn exile_until_next_end_step_under_your_control(
    object: EffectRecipientDef,
) -> EffectDef {
    EffectDef::ExileUntilNextEndStep {
        object,
        return_ability: &RETURN_EXILE_AT_NEXT_END_STEP_UNDER_YOUR_CONTROL,
    }
}

/// The outright win that replaces an empty-library draw. Keeping this as an
/// ordinary effect inside the replacement program gives it the same outcome
/// and reporting as every other effect that says its controller wins.
static EMPTY_LIBRARY_DRAW_WIN: EffectDef = EffectDef::WinTheGame {
    player: EffectRecipientDef::Controller,
};

static EMPTY_LIBRARY_DRAW_REPLACEMENT: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::ReplaceEventWithNothing,
    ReplacementEffectDef::Perform(&EMPTY_LIBRARY_DRAW_WIN),
];

/// "If you would draw a card while your library has no cards in it, you win
/// the game instead." Shared by Laboratory Maniac and Jace, Wielder of
/// Mysteries as a true replacement so it competes correctly with other draw
/// replacements.
#[must_use]
pub const fn empty_library_draw_wins() -> AbilityDef {
    AbilityDef::defined_replacement(
        "If you would draw a card while your library has no cards in it, you win the game instead.",
        ReplacementAbilityDef::new()
            .with_event(ReplacementEventDef::WouldDraw {
                player: PlayerRelation::You,
                during_own_draw_step: false,
            })
            .with_condition(ReplacementConditionDef::ControllerLibraryEmpty),
        ReplacementEffectDef::Sequence(&EMPTY_LIBRARY_DRAW_REPLACEMENT),
    )
}

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
    then: None,
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
        from: None,
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
            tapped: false,
},
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::EachPlayer,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::EachPlayer,
        amount: ValueDef::Constant(7),
    },
];
