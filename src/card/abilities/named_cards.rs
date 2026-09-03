// Shared card-name choices and the rules that consume explicit name values.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

use super::model::{
    BattlefieldEntryScalarChoiceDef, CardNameSetDef, PlayActionMatcherDef, PlayRestrictionDef,
    ReplacementChoiceDef,
};

/// Chooses one catalog-derived card name as a permanent enters.
///
/// Wrap this producer in [`ReplacementEffectDef::BindOutput`] to record the
/// result under an authored binding on the entering permanent.
#[must_use]
pub const fn choose_card_name_as_enters(
    names: CardNameSetDef,
) -> ReplacementEffectDef {
    ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
        BattlefieldEntryScalarChoiceDef::card_name(names),
    ))
}

/// No player can cast spells with the resolved name.
#[must_use]
pub const fn cannot_cast_spells_with_name(
    text: &'static str,
    name: CardNameDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                ObjectPredicateDef::NameEquals(name),
            ))),
        },
    )
}

/// No player can activate nonmana abilities of sources with the resolved name.
#[must_use]
pub const fn cannot_activate_nonmana_abilities_with_name(
    text: &'static str,
    name: CardNameDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::ActivateNonManaAbility,
                ObjectPredicateDef::NameEquals(name),
            ))),
        },
    )
}

/// No player can activate any ability of a source with the resolved name,
/// including mana abilities and sources outside the battlefield.
#[must_use]
pub const fn cannot_activate_abilities_with_name(
    text: &'static str,
    name: CardNameDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::ActivateAbility,
                ObjectPredicateDef::NameEquals(name),
            ))),
        },
    )
}

/// Spells cast by the named relation with the resolved name cost this much more.
#[must_use]
pub const fn spell_cost_increase_for_name(
    text: &'static str,
    name: CardNameDef,
    caster: PlayerRelation,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::increase_spell(
            ObjectPredicateDef::NameEquals(name),
            caster,
            amount,
        )),
    )
}

/// Nonmana activated abilities of sources with the resolved name cost this
/// much more. Mana abilities remain exempt.
#[must_use]
pub const fn ability_cost_increase_for_name(
    text: &'static str,
    name: CardNameDef,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::SourceAbilityIncrease {
            source: ObjectPredicateDef::NameEquals(name),
            amount,
        }),
    )
}

const SEARCH_AND_EXILE_HAND_BINDING: Binding = Binding!("search_and_exile_hand");
const SEARCH_AND_EXILE_HAND_MOVE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(SEARCH_AND_EXILE_HAND_BINDING)),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};
const SEARCH_AND_EXILE_LIBRARY_BINDING: Binding = Binding!("search_and_exile_library");
const SEARCH_AND_EXILE_LIBRARY_MOVE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(SEARCH_AND_EXILE_LIBRARY_BINDING)),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};
const SEARCH_AND_EXILE_GRAVEYARD_BINDING: Binding = Binding!("search_and_exile_graveyard");
const SEARCH_AND_EXILE_GRAVEYARD_MOVE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
        SEARCH_AND_EXILE_GRAVEYARD_BINDING,
    )),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};

/// Searches one player's graveyard, hand, or library for matching cards and
/// exiles the cards found. Public graveyards contribute every match; hidden
/// zones let the effect controller fail to find any number of matches.
#[must_use]
pub const fn search_and_exile(
    zone: ZoneKind,
    binding: Binding,
) -> EffectDef {
    search_named_cards_and_exile(
        zone,
        PlayerRefDef::OwnerOf(ObjectRefDef::Binding(binding)),
        binding,
        false,
    )
}

/// Searches the target owner's named zone and lets the effect controller exile
/// any number of matches, including zero cards from a public graveyard.
#[must_use]
pub const fn search_and_exile_any_number(zone: ZoneKind, binding: Binding) -> EffectDef {
    search_named_cards_and_exile(
        zone,
        PlayerRefDef::OwnerOf(ObjectRefDef::Binding(binding)),
        binding,
        true,
    )
}

/// Counterbore's corresponding search follows the countered spell's
/// controller rather than the physical card's owner.
#[must_use]
pub const fn search_controllers_zone_and_exile(zone: ZoneKind, binding: Binding) -> EffectDef {
    search_named_cards_and_exile(
        zone,
        PlayerRefDef::ControllerOf(ObjectRefDef::Binding(binding)),
        binding,
        false,
    )
}

const fn search_named_cards_and_exile(
    zone: ZoneKind,
    player: PlayerRefDef,
    binding: Binding,
    choose_from_graveyard: bool,
) -> EffectDef {
    let candidates = ObjectSetDef::Query(ObjectQueryDef::owned_by(
        ObjectPredicateDef::NameEquals(CardNameDef::NameOf(ObjectRefDef::Binding(binding))),
        match zone {
            ZoneKind::Graveyard => &[ZoneKind::Graveyard],
            ZoneKind::Hand => &[ZoneKind::Hand],
            ZoneKind::Library => &[ZoneKind::Library],
            _ => panic!("search_and_exile supports graveyard, hand, and library"),
        },
        PlayerSetDef::One(player),
    ));
    match zone {
        ZoneKind::Graveyard if choose_from_graveyard => EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(SEARCH_AND_EXILE_GRAVEYARD_BINDING),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates,
            exclude: None,
            minimum: 0,
            maximum: usize::MAX,
            visibility: ChoiceVisibilityDef::Public,
            then: &SEARCH_AND_EXILE_GRAVEYARD_MOVE,
        }),
        ZoneKind::Graveyard => EffectDef::MoveToZone {
            object: EffectRecipientDef::objects(candidates),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
        },
        ZoneKind::Hand => EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(SEARCH_AND_EXILE_HAND_BINDING),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates,
            exclude: None,
            minimum: 0,
            maximum: usize::MAX,
            visibility: ChoiceVisibilityDef::Private,
            then: &SEARCH_AND_EXILE_HAND_MOVE,
        }),
        ZoneKind::Library => EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(SEARCH_AND_EXILE_LIBRARY_BINDING),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates,
            exclude: None,
            minimum: 0,
            maximum: usize::MAX,
            visibility: ChoiceVisibilityDef::Private,
            then: &SEARCH_AND_EXILE_LIBRARY_MOVE,
        }),
        _ => panic!("search_and_exile supports graveyard, hand, and library"),
    }
}
