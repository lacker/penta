// Shared Convoke and Buyback constructors.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// Convoke (CR 702.51): creatures become one-unit payment sources while the
/// total cost of this spell is being paid. The payment planner executes the
/// tap; this clause only marks which spells use that shared procedure.
#[must_use]
pub const fn convoke() -> AbilityDef {
    keyword(
        "Convoke (Your creatures can help cast this spell. Each creature you tap while casting \
         this spell pays for {1} or one mana of that creature's color.)",
        KeywordAbility::Convoke,
    )
}

/// Delve (CR 702.66): graveyard cards become generic-only payment sources.
#[must_use]
pub const fn delve() -> AbilityDef {
    keyword(
        "Delve (Each card you exile from your graveyard while casting this spell pays for {1}.)",
        KeywordAbility::Delve,
    )
}

/// Improvise (CR 702.126): untapped artifacts become generic-only payment
/// sources after mana abilities have been activated.
#[must_use]
pub const fn improvise() -> AbilityDef {
    keyword(
        "Improvise (Your artifacts can help cast this spell. Each artifact you tap after you're \
         done activating mana abilities pays for {1}.)",
        KeywordAbility::Improvise,
    )
}

/// Buyback with a mana surcharge. It is an optional additional cost, so it
/// composes with flashback and every other casting permission.
#[must_use]
pub const fn buyback(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Buyback.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Buyback,
            mana_cost: Some(mana_cost),
            additional_cost: None,
            resolution_destination: SpellResolutionDestinationDef::Hand,
        },
    )
}

/// Replicate (CR 702.55): an optional additional cost payable any number of
/// times. The copies are not this ability's business -- the card prints a
/// cast trigger beside it that counts the payments -- so all this says is
/// what one payment costs and that it may be made again.
#[must_use]
pub const fn replicate(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Replicate.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Replicate,
            mana_cost: Some(mana_cost),
            additional_cost: None,
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
        },
    )
}

static STORM_COPY: CopyStackObjectDef = CopyStackObjectDef {
    object: EffectRecipientDef::Source,
    controller: PlayerRefDef::EffectController,
    count: ValueDef::SpellsCastBeforeThisTurn,
    retarget: true,
    colors: None,
};

/// Storm (CR 702.40): casting the spell triggers one copy for every spell
/// cast before it this turn. Each copy gets its own target choice.
#[must_use]
pub const fn storm() -> AbilityDef {
    AbilityDef::triggered(
        "Storm (When you cast this spell, copy it for each spell cast before it this turn. You may choose new targets for the copies.)",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
        EffectDef::CopyStackObject(&STORM_COPY),
    )
}

/// Multikicker: an additional cost the caster may pay any number of times,
/// with nothing else attached. What it buys is printed separately, as a
/// clause that reads how many times it was paid.
#[must_use]
pub const fn multikicker(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        OptionalAdditionalCostKindDef::Multikicker.label(),
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Multikicker,
            mana_cost: Some(mana_cost),
            additional_cost: None,
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
        },
    )
}

/// Buyback paid with a selected nonmana object, such as sacrificing a land.
#[must_use]
pub const fn buyback_with_additional_cost(
    text: &'static str,
    cost: &'static SpellAdditionalCostDef,
) -> AbilityDef {
    AbilityDef::optional_additional_cost(
        text,
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Buyback,
            mana_cost: None,
            additional_cost: Some(*cost),
            resolution_destination: SpellResolutionDestinationDef::Hand,
        },
    )
}

/// The delayed half of dash: the creature goes home at the beginning of the
/// next end step, whoever's it is.
static DASH_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, return this creature to its owner's hand.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Source,
        from: None,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
            tapped: false,
},
);

static WAS_DASHED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Dash);

static DASH_HASTE: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&HASTE),
};

static HASTE: AbilityDef = haste();

/// Dash (CR 702.109a): an ordinary cast from hand for a different price.
/// What it buys is stated by the two clauses below, the way evoke's
/// sacrifice is stated beside its own alternative cost.
#[must_use]
pub const fn dash(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Dash,
        Some(text),
        EffectDef::None,
    )
}

/// "If you do, it gains haste": read live off how the permanent was cast,
/// so a creature that arrived some other way has nothing.
#[must_use]
pub const fn dashed_haste() -> AbilityDef {
    AbilityDef::static_ability(
        "This creature has haste as long as it was dashed.",
        EffectDef::IfCondition {
            condition: &WAS_DASHED,
            then: &DASH_HASTE,
        },
    )
}

/// "And it's returned from the battlefield to its owner's hand at the
/// beginning of the next end step." A delayed trigger set up as the dashed
/// creature arrives, so a second dash later sets up a second return.
#[must_use]
pub const fn dashed_return() -> AbilityDef {
    AbilityDef::triggered_if(
        "When this creature enters, if it was dashed, return it to its owner's hand at the \
         beginning of the next end step.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &WAS_DASHED,
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&DASH_RETURNS_IT)),
    )
}

/// The delayed half of warp: the permanent is exiled at the beginning of
/// the next end step, and its owner may cast it from there afterwards.
static WARP_EXILES_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, exile this permanent. You may cast it from exile on \
     a later turn.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::ExileGrantingOwnerPlay {
        object: EffectRecipientDef::Source,
        // Nothing on top: the card is simply castable from where it now
        // sits.
        surcharge: ManaCost::new(0, 0),
    },
);

static WAS_WARPED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Warp);

/// Warp: an ordinary cast from hand for a different price, with what it
/// buys stated by the clause below.
#[must_use]
pub const fn warp(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Warp,
        Some(text),
        EffectDef::None,
    )
}

/// "Exile it at the beginning of the next end step, then you may cast it
/// from exile on a later turn." A delayed trigger set up as the warped
/// permanent arrives.
#[must_use]
pub const fn warped_exile() -> AbilityDef {
    AbilityDef::triggered_if(
        "When this permanent enters, if it was warped, exile it at the beginning of the next end \
         step. You may cast it from exile on a later turn.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &WAS_WARPED,
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&WARP_EXILES_IT)),
    )
}
