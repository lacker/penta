// Constructors for the keywords that attach one permanent to another, and for
// soulbond, which pairs two without attaching either.
//
// Split out of the reusable clause library next door only to keep one file
// readable; these are ordinary members of `abilities`. Included textually, so
// the imports here are the parent module's.

/// The target an "Enchant creature" Aura spell chooses.
pub static ENCHANT_CREATURE_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

/// The target an "Enchant artifact" Aura spell chooses.
pub static ENCHANT_ARTIFACT_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Artifact),
    )];

/// The target an "Enchant enchantment" Aura spell chooses.
pub static ENCHANT_ENCHANTMENT_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Enchantment),
    )];

/// The target an "Enchant land" Aura spell chooses.
pub static ENCHANT_LAND_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Land),
)];

/// The target an "Enchant creature you control" Aura spell chooses.
pub static ENCHANT_YOUR_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// An Aura's own spell clause: it targets what it will enchant, and attaching
/// is what the spell does when it resolves. Every Aura prints one, so it
/// belongs here rather than once per set module.
#[must_use]
pub const fn aura_spell(text: &'static str, targets: &'static [AbilityTargetDef]) -> AbilityDef {
    AbilityDef::spell_with_targets(
        text,
        targets,
        EffectDef::Attachment(AttachmentDef::Attach{
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        }),
    )
}

/// The overwhelmingly common Aura spell clause: "Enchant creature."
#[must_use]
pub const fn enchant_creature() -> AbilityDef {
    aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET)
}

/// The common Aura spell clause: "Enchant artifact."
#[must_use]
pub const fn enchant_artifact() -> AbilityDef {
    aura_spell("Enchant artifact", &ENCHANT_ARTIFACT_TARGET)
}

/// The common Aura spell clause: "Enchant enchantment."
#[must_use]
pub const fn enchant_enchantment() -> AbilityDef {
    aura_spell("Enchant enchantment", &ENCHANT_ENCHANTMENT_TARGET)
}

/// The common Aura spell clause: "Enchant land."
#[must_use]
pub const fn enchant_land() -> AbilityDef {
    aura_spell("Enchant land", &ENCHANT_LAND_TARGET)
}

/// The recurring narrower Aura spell clause: "Enchant creature you control."
#[must_use]
pub const fn enchant_creature_you_control() -> AbilityDef {
    aura_spell(
        "Enchant creature you control",
        &ENCHANT_YOUR_CREATURE_TARGET,
    )
}

/// The target an "Enchant player" Aura spell chooses.
pub static ENCHANT_PLAYER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

/// The common Curse and player-Aura spell clause: "Enchant player."
#[must_use]
pub const fn enchant_player() -> AbilityDef {
    aura_spell("Enchant player", &ENCHANT_PLAYER_TARGET)
}

/// An Aura's "at the beginning of the upkeep of enchanted <thing>'s
/// controller" trigger. The host's controller is the one whose upkeep this
/// watches, which is not the Aura's controller once a host changes hands.
#[must_use]
pub const fn enchanted_controller_upkeep(text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::ControllerOfAttachedPermanent,
        },
        effect,
    )
}

/// An Aura's "at the beginning of enchanted player's upkeep" trigger.
#[must_use]
pub const fn enchanted_player_upkeep(text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::EnchantedPlayer,
        },
        effect,
    )
}

/// The target an equip ability chooses: a creature its controller controls.
static EQUIP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// The target a fortify ability chooses: a land its controller controls.
static FORTIFY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Land),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// The optional target in a reconfigure activation. Choosing none is the
/// unattach branch, which the action generator offers only while attached.
static RECONFIGURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
    1,
)];

/// Equip. Attaching is what the ability does, and it is sorcery-speed, which
/// is the whole difference between this and an Aura arriving from the stack.
/// The complete printed cost is supplied as one ordered list: mana-only Equip
/// abilities use a one-item slice, while alternative Equip costs can mix mana
/// and any supported nonmana costs without changing helpers.
#[must_use]
pub const fn equip(costs: &'static [AbilityCostDef], text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::borrowed(costs),
        &EQUIP_TARGET,
        EffectDef::Attachment(AttachmentDef::Attach{
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        }),
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// The creature soulbond may pair with: another unpaired creature its
/// controller controls. Excluding the source is what makes "another" true
/// even before the pair exists.
static SOULBOND_PARTNER: ObjectSetDef = ObjectSetDef::Query(ObjectQueryDef::controlled_by(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Unpaired,
    ]),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
));

static SOULBOND_PAIR: EffectDef = EffectDef::Attachment(AttachmentDef::PairWithSource{
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
});

/// The optional pairing choice both halves of soulbond offer. Zero is a legal
/// number to choose, which is how "you may" is expressed.
static SOULBOND_CHOICE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: SOULBOND_PARTNER,
    exclude: Some(ObjectRefDef::Source),
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &SOULBOND_PAIR,
});

/// The half that fires when the soulbond creature itself arrives.
static SOULBOND_ENTERS: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::Source,
    None,
    Some(ZoneKind::Battlefield),
);

/// The other half: another creature arriving beside an unpaired one.
static SOULBOND_OTHER_ENTERS: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    None,
    Some(ZoneKind::Battlefield),
);

/// Soulbond. CR 702.94 is two triggered abilities rather than one: the
/// creature offers a pairing as it arrives, and offers one again whenever
/// another creature arrives while it is still unpaired.
#[must_use]
pub const fn soulbond() -> [AbilityDef; 2] {
    [
        AbilityDef::triggered(
            "Soulbond (You may pair this creature with another unpaired creature when either \
             enters. They remain paired for as long as you control both of them.)",
            SOULBOND_ENTERS,
            SOULBOND_CHOICE,
        ),
        AbilityDef::triggered_if(
            "You may pair this creature with another unpaired creature when that creature enters.",
            SOULBOND_OTHER_ENTERS,
            &SOURCE_IS_UNPAIRED,
            SOULBOND_CHOICE,
        ),
    ]
}

/// The intervening-if on soulbond's second half: an already-paired creature
/// offers nothing when a third creature arrives.
static SOURCE_IS_UNPAIRED: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Unpaired,
};

/// Fortify. Like equip, this is a sorcery-speed attachment activation; the
/// shared attachment relation supplies Fortification's distinct land-host
/// legality and state-based unattach behavior.
#[must_use]
pub const fn fortify(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &FORTIFY_TARGET,
        EffectDef::Attachment(AttachmentDef::Attach{
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        }),
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// The rules-defined 0/0 black Phyrexian Germ created by every instance of
/// living weapon. Its illustration is the earliest indexed printing of the
/// current Phyrexian Germ token identity.
const GERM: crate::TokenCharacteristics = crate::TokenCharacteristics::creature(
    &["Phyrexian", "Germ"],
    &[ManaColor::Black],
    0,
    0,
)
.with_art(crate::card::CardArt::new(
    "b53e0681-603e-4180-bc86-3dadf214e61a",
    "Igor Kieryluk",
));

/// Living weapon's enter-the-battlefield trigger. The effect's entry
/// continuation attaches the rules-defined Germ to the Equipment before
/// state-based actions are checked.
#[must_use]
pub const fn living_weapon() -> AbilityDef {
    enters_trigger(
        "Living weapon (When this Equipment enters, create a 0/0 black Phyrexian Germ creature token, then attach this to it.)",
        EffectDef::CreateAttachedToken {
            token: GERM,
            host: None,
        },
    )
}

/// The Mirran resistance's 2/2, the same one every "For Mirrodin!"
/// Equipment brings with it.
const REBEL: crate::TokenCharacteristics =
    crate::TokenCharacteristics::creature(&["Rebel"], &[ManaColor::Red], 2, 2)
        .with_art(crate::card::CardArt::new(
            "a41eb9df-d8b4-4697-a759-886faf16754d",
            "Bram Sels",
        ));

/// "For Mirrodin!" -- living weapon with the resistance's own token. The
/// mechanism is identical; what differs is that the Rebel arrives able to
/// attack on its own, where a Germ without its Equipment is a 0/0 that dies
/// where it stands.
#[must_use]
pub const fn for_mirrodin() -> AbilityDef {
    enters_trigger(
        "For Mirrodin! (When this Equipment enters, create a 2/2 red Rebel creature token, then attach this to it.)",
        EffectDef::CreateAttachedToken {
            token: REBEL,
            host: None,
        },
    )
}

/// Reconfigure's paired sorcery-speed attachment procedures.
#[must_use]
pub const fn reconfigure(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &RECONFIGURE_TARGET,
        EffectDef::Attachment(AttachmentDef::Reconfigure{
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        }),
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}
