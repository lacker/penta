//! Built-in card records, grouped by release year and set.
//!
//! Each canonical card is defined in one set module. Records default to a
//! complete implementation and explicitly carry a reason when they are partial
//! or metadata-only. Reprints and alternate-art variants point back to that
//! canonical record from their own set module.

mod tokens;
mod y1993;
mod y1994;
mod y2007;
mod y2011;
mod y2012;
mod y2013;

use super::record::{CardRecord, PrintingRecord};
use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardRules, CardSet};

static UNSUPPORTED_RULES: CardRules = CardRules::unsupported();

struct SetModule {
    set: CardSet,
    cards: &'static [&'static CardRecord],
    additional_printings: &'static [PrintingRecord],
}

impl SetModule {
    const fn new(
        set: CardSet,
        cards: &'static [&'static CardRecord],
        additional_printings: &'static [PrintingRecord],
    ) -> Self {
        Self {
            set,
            cards,
            additional_printings,
        }
    }
}

/// Every cataloged set has one source module. `cards` contains definitions
/// introduced by that module; `additional_printings` contains reprints and
/// further variants of definitions introduced elsewhere.
const SET_MODULES: &[SetModule] = &[
    SetModule::new(CardSet::Token, tokens::CARDS, tokens::ADDITIONAL_PRINTINGS),
    SetModule::new(
        CardSet::Alpha,
        y1993::alpha::CARDS,
        y1993::alpha::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Beta,
        y1993::beta::CARDS,
        y1993::beta::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Unlimited,
        y1993::unlimited::CARDS,
        y1993::unlimited::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::CollectorsEdition,
        y1993::collectors_edition::CARDS,
        y1993::collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::InternationalCollectorsEdition,
        y1993::international_collectors_edition::CARDS,
        y1993::international_collectors_edition::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ArabianNights,
        y1993::arabian_nights::CARDS,
        y1993::arabian_nights::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Antiquities,
        y1994::antiquities::CARDS,
        y1994::antiquities::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Revised,
        y1994::revised::CARDS,
        y1994::revised::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Legends,
        y1994::legends::CARDS,
        y1994::legends::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::TheDark,
        y1994::the_dark::CARDS,
        y1994::the_dark::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FallenEmpires,
        y1994::fallen_empires::CARDS,
        y1994::fallen_empires::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Promo1994,
        y1994::promo_1994::CARDS,
        y1994::promo_1994::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::FutureSight,
        y2007::future_sight::CARDS,
        y2007::future_sight::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Innistrad,
        y2011::innistrad::CARDS,
        y2011::innistrad::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DarkAscension,
        y2012::dark_ascension::CARDS,
        y2012::dark_ascension::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::AvacynRestored,
        y2012::avacyn_restored::CARDS,
        y2012::avacyn_restored::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2013,
        y2012::magic_2013::CARDS,
        y2012::magic_2013::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::ReturnToRavnica,
        y2012::return_to_ravnica::CARDS,
        y2012::return_to_ravnica::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Gatecrash,
        y2013::gatecrash::CARDS,
        y2013::gatecrash::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::DragonsMaze,
        y2013::dragons_maze::CARDS,
        y2013::dragons_maze::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Magic2014,
        y2013::magic_2014::CARDS,
        y2013::magic_2014::ADDITIONAL_PRINTINGS,
    ),
    SetModule::new(
        CardSet::Theros,
        y2013::theros::CARDS,
        y2013::theros::ADDITIONAL_PRINTINGS,
    ),
];

pub(super) fn definitions() -> Vec<CardDefinition> {
    let mut definitions = Vec::with_capacity(253);
    for module in SET_MODULES {
        definitions.extend(module.cards.iter().map(|record| record.definition()));
    }
    definitions
}

pub(super) fn additional_printings() -> Vec<CardPrinting> {
    SET_MODULES
        .iter()
        .flat_map(|module| {
            module
                .additional_printings
                .iter()
                .map(|record| record.printing(module.set))
        })
        .collect()
}

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
#[allow(clippy::too_many_lines)]
pub(super) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::Atog => &y1994::antiquities::ATOG.rules,
        CardBehavior::AugurOfBolas => &y2012::magic_2013::AUGUR_OF_BOLAS.rules,
        CardBehavior::BlackVise => &y1993::alpha::BLACK_VISE.rules,
        CardBehavior::BloodBaronOfVizkopa => &y2013::dragons_maze::BLOOD_BARON_OF_VIZKOPA.rules,
        CardBehavior::BlueElementalBlast => &y1993::alpha::BLUE_ELEMENTAL_BLAST.rules,
        CardBehavior::BloodMoon => &y1994::the_dark::BLOOD_MOON.rules,
        CardBehavior::ChainLightning => &y1994::legends::CHAIN_LIGHTNING.rules,
        CardBehavior::Detonate => &y1994::antiquities::DETONATE.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::Fork => &y1993::alpha::FORK.rules,
        CardBehavior::GlassesOfUrza => &y1993::alpha::GLASSES_OF_URZA.rules,
        CardBehavior::LightningBolt => &y1993::alpha::LIGHTNING_BOLT.rules,
        CardBehavior::Smoke => &y1993::alpha::SMOKE.rules,
        CardBehavior::StoneGiant => &y1993::alpha::STONE_GIANT.rules,
        CardBehavior::WinterOrb => &y1993::alpha::WINTER_ORB.rules,
        CardBehavior::ChaosOrb => &y1993::alpha::CHAOS_ORB.rules,
        CardBehavior::DragonWhelp => &y1993::alpha::DRAGON_WHELP.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::IronclawOrcs => &y1993::alpha::IRONCLAW_ORCS.rules,
        CardBehavior::MishrasFactory => &y1994::antiquities::MISHRA_S_FACTORY.rules,
        CardBehavior::OrcishMechanics => &y1994::antiquities::ORCISH_MECHANICS.rules,
        CardBehavior::RedElementalBlast => &y1993::alpha::RED_ELEMENTAL_BLAST.rules,
        CardBehavior::WheelOfFortune => &y1993::alpha::WHEEL_OF_FORTUNE.rules,
        CardBehavior::Juggernaut => &y1993::alpha::JUGGERNAUT.rules,
        CardBehavior::ManaVault => &y1993::alpha::MANA_VAULT.rules,
        CardBehavior::Triskelion => &y1994::antiquities::TRISKELION.rules,
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::SwordsToPlowshares => &y1993::alpha::SWORDS_TO_PLOWSHARES.rules,
        CardBehavior::TimeWalk => &y1993::alpha::TIME_WALK.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::Channel => &y1993::alpha::CHANNEL.rules,
        CardBehavior::Crusade => &y1993::alpha::CRUSADE.rules,
        CardBehavior::DemonicTutor => &y1993::alpha::DEMONIC_TUTOR.rules,
        CardBehavior::DivineOffering => &y1994::legends::DIVINE_OFFERING.rules,
        CardBehavior::Dispel => &y2012::return_to_ravnica::DISPEL.rules,
        CardBehavior::Dissipate => &y2011::innistrad::DISSIPATE.rules,
        CardBehavior::DoomBlade => &y2013::magic_2014::DOOM_BLADE.rules,
        CardBehavior::DrainLife => &y1993::alpha::DRAIN_LIFE.rules,
        CardBehavior::Duress => &y2012::magic_2013::DURESS.rules,
        CardBehavior::Earthquake => &y1993::alpha::EARTHQUAKE.rules,
        CardBehavior::ErhnamDjinn => &y1993::arabian_nights::ERHNAM_DJINN.rules,
        CardBehavior::EssenceScatter => &y2012::magic_2013::ESSENCE_SCATTER.rules,
        CardBehavior::HymnToTourach => &y1994::fallen_empires::HYMN_TO_TOURACH.rules,
        CardBehavior::HypnoticSpecter => &y1993::alpha::HYPNOTIC_SPECTER.rules,
        CardBehavior::IcatianJavelineers => &y1994::fallen_empires::ICATIAN_JAVELINEERS.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::LifebaneZombie => &y2013::magic_2014::LIFEBANE_ZOMBIE.rules,
        CardBehavior::ManaDrain => &y1994::legends::MANA_DRAIN.rules,
        CardBehavior::MazeOfIth => &y1994::the_dark::MAZE_OF_ITH.rules,
        CardBehavior::MindTwist => &y1993::alpha::MIND_TWIST.rules,
        CardBehavior::NevinyrralsDisk => &y1993::alpha::NEVINYRRALS_DISK.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::Regrowth => &y1993::alpha::REGROWTH.rules,
        CardBehavior::SylvanLibrary => &y1994::legends::SYLVAN_LIBRARY.rules,
        CardBehavior::Terror => &y1993::alpha::TERROR.rules,
        CardBehavior::TimeVault => &y1993::alpha::TIME_VAULT.rules,
        CardBehavior::Timetwister => &y1993::alpha::TIMETWISTER.rules,
        CardBehavior::WhirlingDervish => &y1994::legends::WHIRLING_DERVISH.rules,
        CardBehavior::ArgothianPixies => &y1994::antiquities::ARGOTHIAN_PIXIES.rules,
        CardBehavior::Berserk => &y1993::alpha::BERSERK.rules,
        CardBehavior::CityInABottle => &y1993::arabian_nights::CITY_IN_A_BOTTLE.rules,
        CardBehavior::CopyArtifact => &y1993::alpha::COPY_ARTIFACT.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::GiantGrowth => &y1993::alpha::GIANT_GROWTH.rules,
        CardBehavior::GrislySalvage => &y2012::return_to_ravnica::GRISLY_SALVAGE.rules,
        CardBehavior::HurkylsRecall => &y1994::antiquities::HURKYLS_RECALL.rules,
        CardBehavior::IcyManipulator => &y1993::alpha::ICY_MANIPULATOR.rules,
        CardBehavior::KirdApe => &y1993::arabian_nights::KIRD_APE.rules,
        CardBehavior::Moat => &y1994::legends::MOAT.rules,
        CardBehavior::Mulch => &y2011::innistrad::MULCH.rules,
        CardBehavior::Negate => &y2012::magic_2013::NEGATE.rules,
        CardBehavior::Pendelhaven => &y1994::legends::PENDELHAVEN.rules,
        CardBehavior::PillarOfFlame => &y2012::avacyn_restored::PILLAR_OF_FLAME.rules,
        CardBehavior::Putrefy => &y2013::dragons_maze::PUTREFY.rules,
        CardBehavior::SedgeTroll => &y1993::alpha::SEDGE_TROLL.rules,
        CardBehavior::SinCollector => &y2013::dragons_maze::SIN_COLLECTOR.rules,
        CardBehavior::SphinxsRevelation => &y2012::return_to_ravnica::SPHINXS_REVELATION.rules,
        CardBehavior::SupremeVerdict => &y2012::return_to_ravnica::SUPREME_VERDICT.rules,
        CardBehavior::Tetravus => &y1994::antiquities::TETRAVUS.rules,
        CardBehavior::TheAbyss => &y1994::legends::THE_ABYSS.rules,
        CardBehavior::UltimatePrice => &y2012::return_to_ravnica::ULTIMATE_PRICE.rules,
        CardBehavior::WarleadersHelix => &y2013::dragons_maze::WARLEADERS_HELIX.rules,
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{CardRecord, SET_MODULES, y1993, y1994, y2011, y2012, y2013};
    use crate::card::{
        AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef,
        AlternativeCastKindDef, AppliedEffectDef, BasicLandType, CardPrinting, CardPrintingId,
        CardStructure, CardSupertype, DeclarativeAbilityDef, DoubleFacedKind, EffectDef,
        EffectDurationDef, EffectRecipientDef, ImplementationStatus, KeywordAbility, ManaColor,
        ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ObjectPredicateDef,
        PlayActionKind, PlayRestriction, PlayerRelation, ReplacementEventDef, SpellForm,
        TargetPredicate, TriggerConditionDef, TriggerEventDef, TurnStepDef, ZoneKind,
        ZoneMoveCauseDef, cards,
    };
    use crate::{
        AbilityId, CardDefinitionId, CardPartId, CardSet, Format, ManaCost, ModeId, PlayOptionId,
    };

    fn standard_records() -> Vec<&'static CardRecord> {
        let mut records = SET_MODULES
            .iter()
            .filter(|module| Format::IsdRtrStandard.allows_set(module.set))
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        records.sort_unstable_by_key(|record| record.id);
        records
    }

    fn is_uuid(value: &str) -> bool {
        value.len() == 36
            && value.bytes().enumerate().all(|(index, byte)| match index {
                8 | 13 | 18 | 23 => byte == b'-',
                _ => byte.is_ascii_hexdigit(),
            })
    }

    fn printings_for_set(set: CardSet) -> Vec<CardPrinting> {
        let module = SET_MODULES.iter().find(|module| module.set == set).unwrap();
        module
            .cards
            .iter()
            .map(|record| CardPrinting::new(record.id, set))
            .chain(
                module
                    .additional_printings
                    .iter()
                    .map(|record| record.printing(set)),
            )
            .collect()
    }

    fn shared_object_predicate(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates.iter().copied().all(shared_object_predicate)
            }
            ObjectPredicateDef::Not(predicate) => shared_object_predicate(*predicate),
            ObjectPredicateDef::Special(_) => false,
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::HasType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::Supertype(_)
            | ObjectPredicateDef::SharesNameWithSource
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::Attacking => true,
        }
    }

    fn shared_effect_recipient(recipient: EffectRecipientDef) -> bool {
        match recipient {
            EffectRecipientDef::MatchingObjects { object, zones, .. } => {
                !zones.is_empty()
                    && zones.iter().all(|zone| {
                        matches!(
                            zone,
                            ZoneKind::Battlefield
                                | ZoneKind::Stack
                                | ZoneKind::Library
                                | ZoneKind::Hand
                                | ZoneKind::Graveyard
                                | ZoneKind::Exile
                                | ZoneKind::Command
                        )
                    })
                    && shared_object_predicate(object)
            }
            EffectRecipientDef::ObjectsSharingNameWithTarget(_)
            | EffectRecipientDef::Source
            | EffectRecipientDef::AttachedPermanent
            | EffectRecipientDef::Controller
            | EffectRecipientDef::Opponent
            | EffectRecipientDef::Target(_)
            | EffectRecipientDef::TriggeringObject
            | EffectRecipientDef::ControllerOfTriggeringObject
            | EffectRecipientDef::EventPlayer => true,
        }
    }

    fn shared_keyword(keyword: KeywordAbility) -> bool {
        matches!(
            keyword,
            KeywordAbility::Flying
                | KeywordAbility::Trample
                | KeywordAbility::Haste
                | KeywordAbility::FirstStrike
                | KeywordAbility::DoubleStrike
                | KeywordAbility::Vigilance
                | KeywordAbility::Defender
                | KeywordAbility::Deathtouch
                | KeywordAbility::Lifelink
                | KeywordAbility::Reach
                | KeywordAbility::Flash
                | KeywordAbility::Hexproof
                | KeywordAbility::Intimidate
                | KeywordAbility::Undying
                | KeywordAbility::Mountainwalk
                | KeywordAbility::ProtectionFrom(_)
        )
    }

    fn shared_zone_move_cause(cause: ZoneMoveCauseDef) -> bool {
        matches!(
            cause,
            ZoneMoveCauseDef::Any
                | ZoneMoveCauseDef::EffectControlledBy(
                    PlayerRelation::Any
                        | PlayerRelation::You
                        | PlayerRelation::Opponent
                        | PlayerRelation::ActivePlayer
                        | PlayerRelation::NonactivePlayer
                )
        )
    }

    fn shared_cannot_be_countered_effect(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                !effects.is_empty()
                    && effects
                        .iter()
                        .copied()
                        .all(shared_cannot_be_countered_effect)
            }
            AppliedEffectDef::CannotBeCountered => true,
            AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::GrantAbility(_)
            | AppliedEffectDef::Special(_) => false,
        }
    }

    fn shared_mana_effect(effect: EffectDef, choices_are_supported: bool) -> bool {
        let EffectDef::AddMana(mana) = effect else {
            return false;
        };
        let selection_is_supported = match mana.mana {
            ManaSelectionDef::One(_) => true,
            ManaSelectionDef::Choice(colors) => choices_are_supported && !colors.is_empty(),
        };
        selection_is_supported
            && mana.amount > 0
            && mana
                .restrictions
                .iter()
                .copied()
                .all(|restriction| match restriction {
                    ManaRestrictionDef::CastSpell(object) => shared_object_predicate(object),
                    ManaRestrictionDef::CastCreatureSpellOfChosenType => true,
                    ManaRestrictionDef::ActivateAbility(_) | ManaRestrictionDef::Special(_) => {
                        false
                    }
                })
            && mana.spend_effects.iter().copied().all(|effect| {
                let ManaSpendEffectDef::ApplyToPaidSpell(effect) = effect else {
                    return false;
                };
                shared_cannot_be_countered_effect(effect)
            })
    }

    fn shared_resolving_apply(
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
    ) -> bool {
        if duration != EffectDurationDef::UntilEndOfTurn || !shared_effect_recipient(recipient) {
            return false;
        }
        shared_resolving_applied_effect(effect)
    }

    fn shared_resolving_applied_effect(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                !effects.is_empty() && effects.iter().copied().all(shared_resolving_applied_effect)
            }
            AppliedEffectDef::ModifyPowerToughness { .. } => true,
            AppliedEffectDef::GrantAbility(ability) => {
                ability.implementation == AbilityImplementationDef::Definition
                    && match ability.definition {
                        DeclarativeAbilityDef::Keyword(keyword) => shared_keyword(keyword),
                        DeclarativeAbilityDef::AlternativeCast(definition) => {
                            definition.kind == AlternativeCastKindDef::Flashback
                                && ability.effect == EffectDef::None
                        }
                        _ => false,
                    }
            }
            // A blocking restriction is continuous, not an until-end-of-turn
            // rider a spell hands out.
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::Special(_) => false,
        }
    }

    fn shared_stack_effect(effect: EffectDef) -> bool {
        shared_stack_effect_at_position(effect, true)
    }

    /// A queued decision returns control to the decision procedure instead of
    /// suspending its caller. It is therefore safe at the root of a resolving
    /// effect (and may wrap a whole sequence), but not as one component of a
    /// sequence whose remaining components would otherwise resolve first.
    fn shared_stack_effect_at_position(effect: EffectDef, deferred_decision_allowed: bool) -> bool {
        match effect {
            EffectDef::Sequence(effects) => {
                !effects.is_empty()
                    && effects
                        .iter()
                        .copied()
                        .all(|effect| shared_stack_effect_at_position(effect, false))
            }
            EffectDef::AddMana(_) => shared_mana_effect(effect, false),
            EffectDef::DealDamage { recipient, .. }
            | EffectDef::GainLife { recipient, .. }
            | EffectDef::DrawCards { recipient, .. }
            | EffectDef::DiscardCards { recipient, .. }
            | EffectDef::LoseLife { recipient, .. } => shared_effect_recipient(recipient),
            // The chooser is a player, and the choices are their own
            // battlefield, so only the predicate needs checking.
            EffectDef::SacrificeOfChoice { player, object } => {
                shared_effect_recipient(player) && shared_object_predicate(object)
            }
            // Only the two destinations the return path knows.
            EffectDef::ReturnLinkedExiles { zone, .. } => {
                matches!(zone, ZoneKind::Battlefield | ZoneKind::Hand)
            }
            EffectDef::May(inner) => {
                deferred_decision_allowed && shared_stack_effect_at_position(*inner, true)
            }
            EffectDef::Tap { object }
            | EffectDef::Untap { object }
            | EffectDef::Destroy { object, .. }
            | EffectDef::Sacrifice { object }
            | EffectDef::Counter { object }
            | EffectDef::ExileLinkedToSource { object }
            | EffectDef::MakeUnblockableThisTurn { object }
            | EffectDef::AddCounters { object, .. }
            | EffectDef::Attach { object }
            | EffectDef::ChangeTextBasicLandType { object }
            | EffectDef::BecomeCopyOf { object, .. } => shared_effect_recipient(object),
            // Only the two destinations counter_spell_into knows.
            EffectDef::CounterUnlessPaid { object, zone, .. } => {
                matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile)
                    && shared_effect_recipient(object)
            }
            // Neither needs a recipient: a token is created under the
            // resolving object's controller, and the flash grant is about its
            // controller's next spell.
            EffectDef::CreateToken { .. } | EffectDef::GrantFlashToNextSorcery => true,
            EffectDef::OptionalManaPayment { effect, .. } => {
                deferred_decision_allowed && shared_stack_effect_at_position(*effect, true)
            }
            // Scheduling creates a fresh resolution boundary. A decision may
            // therefore be the delayed effect's root even when scheduling it
            // is itself one component of a sequence.
            EffectDef::AtNextStep { effect, .. } => shared_stack_effect_at_position(*effect, true),
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => shared_resolving_apply(recipient, effect, duration),
            // Only the moves the runtime actually performs are inside the
            // boundary. A move to the stack or command zone is still a seam.
            EffectDef::MoveToZone { object, zone } => {
                matches!(
                    zone,
                    ZoneKind::Battlefield
                        | ZoneKind::Hand
                        | ZoneKind::Graveyard
                        | ZoneKind::Exile
                        | ZoneKind::Library
                ) && shared_effect_recipient(object)
            }
            EffectDef::None
            | EffectDef::EntersTapped
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => false,
        }
    }

    fn shared_trigger_event(event: TriggerEventDef) -> bool {
        match event {
            TriggerEventDef::ZoneChanged { object, from, to } => {
                const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 5] = [
                    (ZoneKind::Hand, ZoneKind::Battlefield),
                    (ZoneKind::Stack, ZoneKind::Battlefield),
                    (ZoneKind::Battlefield, ZoneKind::Graveyard),
                    (ZoneKind::Battlefield, ZoneKind::Exile),
                    (ZoneKind::Battlefield, ZoneKind::Hand),
                ];
                shared_object_predicate(object)
                    && COMMITTED_TRANSITIONS
                        .iter()
                        .any(|(actual_from, actual_to)| {
                            from.is_none_or(|expected| expected == *actual_from)
                                && to.is_none_or(|expected| expected == *actual_to)
                        })
            }
            TriggerEventDef::BecomesTapped(object)
            | TriggerEventDef::Attacks(object)
            | TriggerEventDef::TappedForMana(object)
            | TriggerEventDef::SpellCast(object) => shared_object_predicate(object),
            TriggerEventDef::StepBegins { .. }
            | TriggerEventDef::LifeGained(_)
            | TriggerEventDef::DamagedCreatureDied => true,
            // Only "whenever this creature is dealt damage" is committed; a
            // wider recipient has no event behind it yet.
            TriggerEventDef::DamageDealt { source, recipient } => {
                recipient == EffectRecipientDef::Source && source == ObjectPredicateDef::Any
            }
            TriggerEventDef::AbilityActivated(_)
            | TriggerEventDef::ManaAdded(_)
            | TriggerEventDef::Special(_) => false,
        }
    }

    fn shared_activated_costs(source_zones: &[ZoneKind], costs: &[AbilityCostDef]) -> bool {
        let battlefield = source_zones == [ZoneKind::Battlefield];
        let hand = source_zones == [ZoneKind::Hand];
        let sacrifice_choices = costs
            .iter()
            .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
            .count();
        sacrifice_choices <= 1
            && costs.iter().all(|cost| match cost {
                // A variable X is offered one activation per affordable
                // value. More than one X in the same cost is not: nothing
                // enumerates a cost that charges X twice.
                AbilityCostDef::Mana(cost) => cost.x_multiplier <= 1,
                AbilityCostDef::SacrificePermanent { object, .. } => {
                    battlefield && shared_object_predicate(*object)
                }
                AbilityCostDef::TapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::PayLife(_) => battlefield,
                AbilityCostDef::DiscardSource => hand,
                AbilityCostDef::UntapSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Special(_) => false,
            })
    }

    fn shared_static_effect(source_zones: &[ZoneKind], effect: EffectDef) -> bool {
        match effect {
            // A prohibition applies to the source's controller, and only
            // while the source is on the battlefield to say so.
            EffectDef::CannotBeForcedToSacrifice => battlefield_only(source_zones),
            // A cost reduction is read while the card is being cast from
            // hand, and only counts what the runtime can count.
            EffectDef::ReduceGenericCostBy(value) => {
                source_zones == [ZoneKind::Hand]
                    && matches!(
                        value,
                        crate::card::ValueDef::Constant(_)
                            | crate::card::ValueDef::CountMatchingObjects(_)
                    )
            }
            EffectDef::Sequence(effects) => {
                !effects.is_empty()
                    && effects
                        .iter()
                        .copied()
                        .all(|effect| shared_static_effect(source_zones, effect))
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => {
                let battlefield_recipient_is_supported = match recipient {
                    EffectRecipientDef::Source | EffectRecipientDef::AttachedPermanent => true,
                    EffectRecipientDef::MatchingObjects { object, zones, .. } => {
                        zones == [ZoneKind::Battlefield] && shared_object_predicate(object)
                    }
                    EffectRecipientDef::Controller
                    | EffectRecipientDef::Opponent
                    | EffectRecipientDef::Target(_)
                    | EffectRecipientDef::ObjectsSharingNameWithTarget(_)
                    | EffectRecipientDef::TriggeringObject
                    | EffectRecipientDef::ControllerOfTriggeringObject
                    | EffectRecipientDef::EventPlayer => false,
                };
                let battlefield_effect_is_supported =
                    shared_static_applied_effect(recipient, effect);
                let battlefield_effect = battlefield_only(source_zones)
                    && battlefield_recipient_is_supported
                    && battlefield_effect_is_supported
                    && matches!(
                        duration,
                        EffectDurationDef::WhileSourceRemainsInZone
                            | EffectDurationDef::UntilSourceLeavesZone
                    );
                let stack_source_effect = source_zones == [ZoneKind::Stack]
                    && recipient == EffectRecipientDef::Source
                    && shared_cannot_be_countered_effect(effect)
                    && duration == EffectDurationDef::WhileSourceRemainsInZone;
                battlefield_effect || stack_source_effect
            }
            // None of these is a static ability; all execute from the stack.
            EffectDef::GrantFlashToNextSorcery
            | EffectDef::May(_)
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::EntersTapped
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => false,
        }
    }

    fn shared_static_applied_effect(
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
    ) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                !effects.is_empty()
                    && effects
                        .iter()
                        .copied()
                        .all(|effect| shared_static_applied_effect(recipient, effect))
            }
            AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
                let supported = |value| {
                    matches!(
                        value,
                        crate::card::ValueDef::Constant(_)
                            | crate::card::ValueDef::AnyMatchingObject(_)
                    )
                };
                supported(power) && supported(toughness)
            }
            AppliedEffectDef::AddLandTypes(land_types) => !land_types.is_empty(),
            AppliedEffectDef::GrantAbility(ability) => shared_definition_ability(ability),
            AppliedEffectDef::CannotBeBlockedBy(predicate) => {
                recipient == EffectRecipientDef::Source && shared_object_predicate(predicate)
            }
            AppliedEffectDef::CannotBeCountered => true,
            AppliedEffectDef::Special(_) => false,
        }
    }

    fn shared_trigger_condition(condition: TriggerConditionDef) -> bool {
        match condition {
            TriggerConditionDef::ObjectCount { query, .. } => shared_object_predicate(query.object),
        }
    }

    fn battlefield_only(zones: &[ZoneKind]) -> bool {
        zones == [ZoneKind::Battlefield]
    }

    #[allow(clippy::too_many_lines)]
    fn shared_definition_ability(ability: &AbilityDef) -> bool {
        if ability.implementation != AbilityImplementationDef::Definition {
            return false;
        }
        match ability.definition {
            DeclarativeAbilityDef::Spell(definition) => {
                if let Some(modal) = definition.modal() {
                    modal.modes.iter().all(|mode| {
                        mode.implementation != AbilityImplementationDef::Definition
                            || shared_definition_ability(mode)
                    })
                } else {
                    shared_stack_effect(ability.effect)
                }
            }
            DeclarativeAbilityDef::ActivatedMana(definition) => {
                battlefield_only(definition.source_zones)
                    && definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource | AbilityCostDef::SacrificeSource
                        )
                    })
                    && definition.costs.iter().all(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource
                                | AbilityCostDef::SacrificeSource
                                | AbilityCostDef::PayLife(_)
                        )
                    })
                    && shared_mana_effect(ability.effect, true)
            }
            DeclarativeAbilityDef::TriggeredMana(definition) => {
                fn immediate_mana_effect(effect: EffectDef) -> bool {
                    match effect {
                        EffectDef::Sequence(effects) => {
                            !effects.is_empty()
                                && effects.iter().copied().all(immediate_mana_effect)
                        }
                        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
                        EffectDef::May(_)
                        | EffectDef::None
                        | EffectDef::DealDamage { .. }
                        | EffectDef::GainLife { .. }
                        | EffectDef::DrawCards { .. }
                        | EffectDef::DiscardCards { .. }
                        | EffectDef::LoseLife { .. }
                        | EffectDef::Tap { .. }
                        | EffectDef::Untap { .. }
                        | EffectDef::Attach { .. }
                        | EffectDef::CreateToken { .. }
                        | EffectDef::Destroy { .. }
                        | EffectDef::Sacrifice { .. }
                        | EffectDef::SacrificeOfChoice { .. }
                        | EffectDef::Counter { .. }
                        | EffectDef::CounterUnlessPaid { .. }
                        | EffectDef::AddCounters { .. }
                        | EffectDef::ChangeTextBasicLandType { .. }
                        | EffectDef::BecomeCopyOf { .. }
                        | EffectDef::OptionalManaPayment { .. }
                        | EffectDef::EntersTapped
                        | EffectDef::CannotBeForcedToSacrifice
                        | EffectDef::GrantFlashToNextSorcery
                        | EffectDef::ExileLinkedToSource { .. }
                        | EffectDef::ReturnLinkedExiles { .. }
                        | EffectDef::MakeUnblockableThisTurn { .. }
                        | EffectDef::AtNextStep { .. }
                        | EffectDef::ReduceGenericCostBy(_)
                        | EffectDef::MultiplyEventAmount(_)
                        | EffectDef::MoveToZone { .. }
                        | EffectDef::ChooseCreatureType { .. }
                        | EffectDef::Apply { .. }
                        | EffectDef::Special(_) => false,
                    }
                }
                battlefield_only(definition.source_zones)
                    && shared_trigger_event(definition.event)
                    && immediate_mana_effect(ability.effect)
            }
            DeclarativeAbilityDef::Activated(definition) => {
                matches!(
                    definition.source_zones,
                    [ZoneKind::Battlefield | ZoneKind::Hand]
                ) && shared_activated_costs(definition.source_zones, definition.costs.as_slice())
                    && shared_stack_effect(ability.effect)
            }
            DeclarativeAbilityDef::Triggered(definition) => {
                battlefield_only(definition.source_zones)
                    && shared_trigger_event(definition.event)
                    && definition
                        .condition
                        .is_none_or(|condition| shared_trigger_condition(*condition))
                    && shared_stack_effect(ability.effect)
            }
            DeclarativeAbilityDef::Static(definition) => {
                shared_static_effect(definition.source_zones, ability.effect)
            }
            DeclarativeAbilityDef::Replacement(definition) => match definition.event {
                ReplacementEventDef::EntersBattlefield => {
                    battlefield_only(definition.source_zones)
                        && matches!(
                            ability.effect,
                            EffectDef::EntersTapped
                                | EffectDef::ChooseCreatureType {
                                    object: EffectRecipientDef::Source,
                                }
                        )
                }
                ReplacementEventDef::WouldMove { from, to, cause } => {
                    definition.source_zones == [from]
                        && from == ZoneKind::Hand
                        && to == ZoneKind::Graveyard
                        && shared_zone_move_cause(cause)
                        && ability.effect
                            == EffectDef::MoveToZone {
                                object: EffectRecipientDef::Source,
                                zone: ZoneKind::Battlefield,
                            }
                }
                ReplacementEventDef::WouldGainLife(_) => {
                    battlefield_only(definition.source_zones)
                        && matches!(ability.effect, EffectDef::MultiplyEventAmount(_))
                }
                ReplacementEventDef::Special(_) => false,
            },
            DeclarativeAbilityDef::AlternativeCast(definition) => match definition.kind {
                AlternativeCastKindDef::Flashback => ability.effect == EffectDef::None,
                AlternativeCastKindDef::Overload => shared_stack_effect(ability.effect),
            },
            DeclarativeAbilityDef::Keyword(keyword) => shared_keyword(keyword),
            DeclarativeAbilityDef::SpecialAction(_) | DeclarativeAbilityDef::Legacy => false,
        }
    }

    fn assert_nested_definition_abilities(card_name: &str, effect: EffectDef) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    assert_nested_definition_abilities(card_name, *effect);
                }
            }
            EffectDef::OptionalManaPayment { effect, .. }
            | EffectDef::May(effect)
            | EffectDef::AtNextStep { effect, .. } => {
                assert_nested_definition_abilities(card_name, *effect);
            }
            EffectDef::Apply { effect, .. } => {
                assert_nested_definition_applied_effect(card_name, effect);
            }
            EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::EntersTapped
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => {}
        }
    }

    fn assert_nested_definition_applied_effect(card_name: &str, effect: AppliedEffectDef) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    assert_nested_definition_applied_effect(card_name, *effect);
                }
            }
            AppliedEffectDef::GrantAbility(ability) => {
                if ability.implementation == AbilityImplementationDef::Definition {
                    assert!(
                        shared_definition_ability(ability),
                        "{card_name} contains a nested Definition ability outside the shared runtime boundary: {ability:?}",
                    );
                }
                assert_nested_definition_abilities(card_name, ability.effect);
            }
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::Special(_) => {}
        }
    }

    #[test]
    fn every_cataloged_set_has_one_matching_module() {
        let format_sets = Format::OldSchool9394
            .rules()
            .allowed_sets
            .iter()
            .chain(Format::IsdRtrStandard.rules().allowed_sets)
            .copied()
            .collect::<Vec<_>>();
        // Tokens are registered like a set so a client can resolve one by
        // definition, but they are deliberately in no format's card pool, so
        // they are not part of this correspondence.
        let registered_sets = SET_MODULES
            .iter()
            .map(|module| module.set)
            .filter(|set| *set != CardSet::Token)
            .collect::<Vec<_>>();
        for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
            assert!(
                !format.rules().allowed_sets.contains(&CardSet::Token),
                "no format may allow the token set"
            );
        }

        assert!(
            format_sets.iter().all(|set| registered_sets.contains(set)),
            "every format-supported set must be cataloged",
        );
        for testbed_set in [CardSet::FutureSight, CardSet::Theros] {
            assert!(registered_sets.contains(&testbed_set));
            assert!(!Format::OldSchool9394.allows_set(testbed_set));
            assert!(!Format::IsdRtrStandard.allows_set(testbed_set));
        }
        assert_eq!(registered_sets.len(), 22);
        assert_eq!(
            registered_sets
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            22
        );

        for module in SET_MODULES {
            for record in module.cards {
                assert_eq!(
                    record.debut_set, module.set,
                    "{} is registered in the wrong set",
                    record.name
                );
            }
        }
    }

    #[test]
    fn built_in_records_keep_stable_dense_ids_and_unique_identity() {
        let records = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 253);

        let mut ids = records.iter().map(|record| record.id).collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(
            ids.iter().map(|id| id.0).collect::<Vec<_>>(),
            (1..=253).collect::<Vec<_>>()
        );
        assert_eq!(
            records
                .iter()
                .map(|record| record.name)
                .collect::<HashSet<_>>()
                .len(),
            records.len()
        );
    }

    #[test]
    fn built_in_catalog_indexes_definitions_and_printings_separately() {
        let catalog = crate::card::catalog().unwrap();
        let printing_count = (1..=253)
            .filter(|id| {
                *id != cards::BEAST_TOKEN_3_3_GREEN.0
                    && *id != cards::KNIGHT_TOKEN_2_2_WHITE.0
                    && *id != cards::SOLDIER_TOKEN_1_1_RED_WHITE.0
                    && *id != cards::DEMON_TOKEN_5_5_BLACK.0
            })
            .map(|id| catalog.printings_for(CardDefinitionId(id)).len())
            .sum::<usize>();

        assert_eq!(printing_count, 629);
        for variant in 0..3 {
            assert!(
                catalog
                    .get_printing(CardPrintingId::with_variant(
                        cards::PLAINS,
                        CardSet::Beta,
                        variant,
                    ))
                    .is_some()
            );
        }
        assert_eq!(catalog.find_by_name("Plains"), Some(cards::PLAINS));
    }

    #[test]
    fn every_non_declarative_clause_explains_its_implementation() {
        let records = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 253);

        for record in records {
            let definition = record.definition();
            for part in &definition.parts {
                for ability in part.rules.ability_clauses() {
                    if !matches!(ability.implementation, AbilityImplementationDef::Definition) {
                        assert!(
                            ability
                                .implementation
                                .explanation()
                                .is_some_and(|explanation| !explanation.trim().is_empty()),
                            "{} has a non-declarative clause without an explanation: {}",
                            record.name,
                            ability.text
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn standard_records_cover_the_top_eight_pool_with_stable_unique_ids() {
        let records = standard_records();
        assert_eq!(records.len(), 117);

        let expected_ids = (129..=244).chain([251]).collect::<Vec<_>>();
        assert_eq!(
            records.iter().map(|record| record.id.0).collect::<Vec<_>>(),
            expected_ids,
        );

        let mut names = HashSet::new();
        for record in records {
            assert!(names.insert(record.name));
            assert!(!record.rules.has_supertype(CardSupertype::Basic));
            assert!(Format::IsdRtrStandard.allows_set(record.debut_set));
            if let Some(behavior) = record.rules.special_behavior() {
                assert_eq!(behavior.rules(), &record.rules);
            }
        }

        assert!(!names.contains("Celestial Purge"));
        assert!(names.contains("Celestial Flare"));
        assert!(names.contains("Thespian's Stage"));
    }

    #[test]
    fn standard_records_have_complete_unique_scryfall_metadata() {
        let records = standard_records();
        let mut scryfall_ids = HashSet::new();

        for record in records {
            let scryfall_id = record.art.scryfall_id;
            assert!(
                is_uuid(scryfall_id),
                "{} has an invalid Scryfall ID: {scryfall_id}",
                record.name
            );
            assert!(
                scryfall_ids.insert(scryfall_id),
                "{} repeats Scryfall ID {scryfall_id}",
                record.name
            );
            assert!(
                !record.art.artist.trim().is_empty(),
                "{} is missing its artist",
                record.name
            );
        }

        assert_eq!(scryfall_ids.len(), 117);
    }

    #[test]
    fn structured_records_expose_parts_and_play_options_without_losing_primary_rules() {
        let garruk = y2011::innistrad::GARRUK_RELENTLESS.definition();
        assert_eq!(garruk.name, "Garruk Relentless");
        assert_eq!(garruk.rules, garruk.primary_part().unwrap().rules);
        assert_eq!(garruk.parts.len(), 2);
        assert_eq!(garruk.parts[1].name, "Garruk, the Veil-Cursed");
        assert_eq!(garruk.parts[1].rules.mana_cost(), None);
        assert_eq!(
            garruk.parts[1].rules.colors(),
            [false, false, true, false, true]
        );
        assert!(matches!(
            garruk.structure,
            CardStructure::DoubleFaced {
                front: CardPartId(0),
                back: CardPartId(1),
                kind: DoubleFacedKind::Transforming,
            }
        ));

        let huntmaster = y2012::dark_ascension::HUNTMASTER_OF_THE_FELLS.definition();
        assert_eq!(huntmaster.rules, huntmaster.primary_part().unwrap().rules);
        assert_eq!(huntmaster.parts.len(), 2);
        assert_eq!(huntmaster.parts[1].name, "Ravager of the Fells");
        assert_eq!(huntmaster.parts[1].rules.mana_cost(), None);
        assert_eq!(huntmaster.parts[1].rules.creature_stats().unwrap().power, 4);
        assert!(
            huntmaster.parts[1]
                .rules
                .has_executable_keyword(KeywordAbility::Trample)
        );

        let turn_burn = y2013::dragons_maze::TURN_BURN.definition();
        assert_eq!(turn_burn.name, "Turn // Burn");
        assert_eq!(turn_burn.rules, turn_burn.parts[0].rules);
        assert_eq!(turn_burn.parts.len(), 2);
        assert_eq!(turn_burn.parts[0].name, "Turn");
        assert_eq!(turn_burn.parts[1].name, "Burn");
        assert_eq!(
            turn_burn.parts[1].rules.colors(),
            [false, false, false, true, false]
        );
        assert!(matches!(
            turn_burn.structure,
            CardStructure::Split {
                ref parts,
                fused: Some(PlayOptionId(2)),
            } if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(turn_burn.play_options.len(), 3);
        assert!(matches!(
            turn_burn.play_options[2].form,
            SpellForm::Combined(ref parts) if parts == &[CardPartId(0), CardPartId(1)]
        ));
        assert_eq!(
            turn_burn.play_options[2].restriction,
            PlayRestriction::FromHandOnly
        );
        assert_eq!(turn_burn.play_options[2].targets.len(), 2);

        let charm = y2012::return_to_ravnica::IZZET_CHARM.definition();
        assert_eq!(charm.parts.len(), 1);
        assert_eq!(charm.play_options.len(), 1);
        let modes = charm.play_options[0].modes.as_ref().unwrap();
        assert_eq!(
            (modes.minimum, modes.maximum, modes.may_repeat),
            (1, 1, false)
        );
        assert_eq!(modes.modes.len(), 3);
        assert_eq!(modes.modes[0].id, ModeId(0));
        assert_eq!(
            modes.modes[0].targets[0].predicate,
            TargetPredicate::NoncreatureSpell
        );
        assert_eq!(
            modes.modes[1].targets[0].predicate,
            TargetPredicate::CreaturePermanent
        );
        assert!(modes.modes[2].targets.is_empty());
        assert_eq!(charm.play_options[0].action, PlayActionKind::CastSpell);
    }

    #[test]
    fn ordinary_records_synthesize_one_primary_part_and_play_option() {
        let bolt = y1993::alpha::LIGHTNING_BOLT.definition();
        assert_eq!(bolt.parts.len(), 1);
        assert_eq!(bolt.primary_part_id(), CardPartId::PRIMARY);
        assert_eq!(bolt.primary_part().unwrap().rules, bolt.rules);
        assert!(matches!(
            bolt.structure,
            CardStructure::Single {
                main: CardPartId::PRIMARY,
            }
        ));
        assert_eq!(bolt.play_options.len(), 1);
        assert_eq!(bolt.play_options[0].id, PlayOptionId::DEFAULT);
        assert_eq!(
            bolt.play_options[0].form,
            SpellForm::Part(CardPartId::PRIMARY)
        );

        let mountain = y1993::alpha::MOUNTAIN.definition();
        assert_eq!(mountain.parts[0].rules.mana_cost(), None);
        assert_eq!(mountain.play_options[0].action, PlayActionKind::PlayLand);
        assert_eq!(mountain.play_options[0].mana_cost, None);
    }

    #[test]
    fn cavern_records_both_mana_abilities_and_the_colored_mana_riders() {
        let abilities = y2012::avacyn_restored::CAVERN_OF_SOULS
            .rules
            .ability_clauses();
        assert_eq!(abilities.len(), 3);
        assert!(matches!(
            abilities[1].definition,
            DeclarativeAbilityDef::ActivatedMana(_)
        ));
        assert!(matches!(
            abilities[1].effect,
            EffectDef::AddMana(mana)
                if mana.mana == ManaSelectionDef::One(ManaColor::Colorless)
                    && mana.amount == 1
                    && mana.restrictions.is_empty()
                    && mana.spend_effects.is_empty()
        ));
        assert!(matches!(
            abilities[2].definition,
            DeclarativeAbilityDef::ActivatedMana(_)
        ));
        assert!(matches!(
            abilities[2].effect,
            EffectDef::AddMana(mana)
                if mana.mana == ManaSelectionDef::Choice(&[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ])
                    && mana.amount == 1
                    && mana.restrictions
                        == [ManaRestrictionDef::CastCreatureSpellOfChosenType]
                    && mana.spend_effects
                        == [ManaSpendEffectDef::ApplyToPaidSpell(
                            AppliedEffectDef::CannotBeCountered,
                        )]
        ));
    }

    #[test]
    fn every_builtin_mana_land_has_a_printed_or_intrinsic_source() {
        let lands = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .filter(|record| record.rules.has_type(crate::card::CardType::Land))
            .collect::<Vec<_>>();
        assert_eq!(lands.len(), 47);

        let lands_without_mana = lands
            .iter()
            .filter(|record| {
                let has_intrinsic_source = BasicLandType::ALL
                    .into_iter()
                    .any(|land_type| record.rules.has_subtype(land_type.subtype()));
                let has_printed_source = record.rules.ability_clauses().iter().any(|ability| {
                    ability.implementation.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                });
                !has_intrinsic_source && !has_printed_source
            })
            .map(|record| record.name)
            .collect::<Vec<_>>();
        assert_eq!(lands_without_mana, ["Maze of Ith"]);
    }

    #[test]
    fn basic_land_subtypes_do_not_repeat_intrinsic_mana_as_printed_clauses() {
        let lands = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
            .filter(|record| record.rules.has_type(crate::card::CardType::Land))
            .filter(|record| {
                BasicLandType::ALL
                    .into_iter()
                    .any(|land_type| record.rules.has_subtype(land_type.subtype()))
            })
            .collect::<Vec<_>>();
        assert_eq!(lands.len(), 23);

        let mut intrinsic_types = 0;
        for land in lands {
            assert_eq!(
                land.rules.implementation_status(),
                ImplementationStatus::Complete,
                "{} should be complete once basic-land mana is derived intrinsically",
                land.name,
            );
            let land_types = BasicLandType::ALL
                .into_iter()
                .filter(|land_type| land.rules.has_subtype(land_type.subtype()))
                .count();
            intrinsic_types += land_types;
            assert!(
                !land.rules.ability_clauses().iter().any(|ability| matches!(
                    ability.definition,
                    DeclarativeAbilityDef::ActivatedMana(_)
                )),
                "{} should rely on its basic land subtypes for mana",
                land.name,
            );
        }
        assert_eq!(intrinsic_types, 40);
    }

    #[test]
    fn every_nonland_mana_permanent_has_an_activated_mana_clause() {
        let records = [
            &y1993::alpha::BLACK_LOTUS,
            &y1993::alpha::MOX_EMERALD,
            &y1993::alpha::MOX_JET,
            &y1993::alpha::MOX_PEARL,
            &y1993::alpha::MOX_RUBY,
            &y1993::alpha::MOX_SAPPHIRE,
            &y1993::alpha::SOL_RING,
            &y1993::alpha::MANA_VAULT,
            &y1993::alpha::BIRDS_OF_PARADISE,
            &y1993::alpha::LLANOWAR_ELVES,
            &y1994::the_dark::FELLWAR_STONE,
            &y2011::innistrad::AVACYNS_PILGRIM,
            &y2013::magic_2014::ELVISH_MYSTIC,
        ];
        assert_eq!(records.len(), 13);
        for record in records {
            assert!(
                record.rules.ability_clauses().iter().any(|ability| {
                    matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                }),
                "{} is missing its activated mana clause",
                record.name
            );
        }
    }

    #[test]
    fn activation_presentation_lives_on_the_exact_activated_clause() {
        let cases = [
            (
                &y1993::alpha::GLASSES_OF_URZA,
                AbilityId::PRIMARY,
                "Look at {}'s hand with Glasses of Urza",
                "Look at a player's hand",
            ),
            (
                &y1993::alpha::STONE_GIANT,
                AbilityId::PRIMARY,
                "Give {} flying with Stone Giant",
                "Give a smaller creature flying",
            ),
            (
                &y1993::alpha::CHAOS_ORB,
                AbilityId::PRIMARY,
                "Flip Chaos Orb onto {}",
                "Flip Chaos Orb onto a permanent",
            ),
            (
                &y1993::alpha::ICY_MANIPULATOR,
                AbilityId::PRIMARY,
                "Tap {} with Icy Manipulator",
                "Tap an artifact, creature, or land",
            ),
            (
                &y1994::antiquities::MISHRA_S_FACTORY,
                AbilityId(2),
                "Give {} +1/+1 with Mishra's Factory",
                "Give an Assembly-Worker +1/+1",
            ),
            (
                &y1994::antiquities::ORCISH_MECHANICS,
                AbilityId::PRIMARY,
                "Deal 2 damage to {} with Orcish Mechanics",
                "Deal 2 damage",
            ),
            (
                &y1994::antiquities::STRIP_MINE,
                AbilityId(1),
                "Destroy {} with Strip Mine",
                "Destroy a land",
            ),
            (
                &y1994::antiquities::TRISKELION,
                AbilityId(1),
                "Deal 1 damage to {} with Triskelion",
                "Deal 1 damage",
            ),
            (
                &y1994::fallen_empires::ICATIAN_JAVELINEERS,
                AbilityId(1),
                "Deal 1 damage to {} with Icatian Javelineers",
                "Deal 1 damage",
            ),
            (
                &y1994::legends::PENDELHAVEN,
                AbilityId(1),
                "Give {} +1/+2 with Pendelhaven",
                "Give a 1/1 creature +1/+2",
            ),
            (
                &y1994::legends::RELIC_BARRIER,
                AbilityId::PRIMARY,
                "Tap {} with Relic Barrier",
                "Tap an artifact",
            ),
            (
                &y1994::the_dark::MAZE_OF_ITH,
                AbilityId::PRIMARY,
                "Untap {} and take it out of combat",
                "Take an attacker out of combat",
            ),
        ];

        for (record, ability_id, targeted, summary) in cases {
            let ability = record
                .rules
                .ability(ability_id)
                .unwrap_or_else(|| panic!("{} is missing ability {ability_id:?}", record.name));
            assert!(matches!(
                ability.definition,
                DeclarativeAbilityDef::Activated(_)
            ));
            let presentation = ability.activation_text.unwrap();
            assert_eq!(presentation.targeted, targeted);
            assert_eq!(presentation.summary, summary);
        }

        let presentation_count = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter())
            .flat_map(|record| record.rules.ability_clauses())
            .filter(|ability| ability.activation_text.is_some())
            .count();
        assert_eq!(presentation_count, cases.len());
    }

    #[test]
    fn migrated_activated_cards_preserve_their_derived_implementation_status() {
        let partial = [
            &y1993::alpha::GLASSES_OF_URZA,
            &y1993::alpha::STONE_GIANT,
            &y1993::alpha::CHAOS_ORB,
            &y1994::antiquities::MISHRA_S_FACTORY,
            &y1994::antiquities::TRISKELION,
            &y1994::fallen_empires::ICATIAN_JAVELINEERS,
            &y1994::legends::PENDELHAVEN,
            &y1994::the_dark::MAZE_OF_ITH,
        ];
        let complete = [
            &y1993::alpha::ICY_MANIPULATOR,
            &y1994::antiquities::ORCISH_MECHANICS,
            &y1994::antiquities::STRIP_MINE,
            &y1994::legends::RELIC_BARRIER,
        ];

        for record in partial {
            assert_eq!(
                record.rules.implementation_status(),
                ImplementationStatus::Partial,
                "{} should remain partially implemented",
                record.name
            );
        }
        for record in complete {
            assert_eq!(
                record.rules.implementation_status(),
                ImplementationStatus::Complete,
                "{} should remain completely implemented",
                record.name
            );
        }
    }

    #[test]
    fn early_core_sets_reuse_definitions_without_duplicating_identity() {
        let all_definition_ids = SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().map(|record| record.id))
            .collect::<HashSet<_>>();
        let basics = [
            cards::PLAINS,
            cards::ISLAND,
            cards::SWAMP,
            cards::MOUNTAIN,
            cards::FOREST,
        ];

        let early_sets = [
            (CardSet::Alpha, 85, 90, 2_u16),
            (CardSet::Beta, 84, 94, 3_u16),
            (CardSet::Unlimited, 84, 94, 3_u16),
            (CardSet::CollectorsEdition, 84, 94, 3_u16),
            (CardSet::InternationalCollectorsEdition, 84, 94, 3_u16),
        ];

        let mut printing_ids = HashSet::new();
        for (set, expected_cards, expected_printings, expected_basic_variants) in early_sets {
            let printings = printings_for_set(set);
            assert_eq!(printings.len(), expected_printings);
            assert_eq!(
                printings
                    .iter()
                    .map(|printing| printing.id.definition)
                    .collect::<HashSet<_>>()
                    .len(),
                expected_cards
            );

            for printing in &printings {
                assert!(all_definition_ids.contains(&printing.id.definition));
                assert_eq!(printing.id.set, set);
                assert!(printing_ids.insert(printing.id));
            }
            for basic in basics {
                let variants = printings
                    .iter()
                    .filter(|printing| printing.id.definition == basic)
                    .map(|printing| printing.id.variant)
                    .collect::<HashSet<_>>();
                assert_eq!(variants.len(), usize::from(expected_basic_variants));
                assert_eq!(variants, (0..expected_basic_variants).collect());
            }
        }

        assert_eq!(y1993::beta::VOLCANIC_ISLAND.id, cards::VOLCANIC_ISLAND);
        assert_eq!(y1993::beta::VOLCANIC_ISLAND.debut_set, CardSet::Beta);
    }

    #[test]
    fn activated_cost_boundary_is_specific_to_the_source_zone() {
        let mana = AbilityCostDef::Mana(ManaCost::colored(0, 0, 0, 0, 1, 1));
        assert!(shared_activated_costs(
            &[ZoneKind::Hand],
            &[mana, AbilityCostDef::DiscardSource],
        ));
        assert!(!shared_activated_costs(
            &[ZoneKind::Hand],
            &[AbilityCostDef::PayLife(1)],
        ));
        assert!(shared_activated_costs(
            &[ZoneKind::Battlefield],
            &[mana, AbilityCostDef::TapSource],
        ));
        assert!(!shared_activated_costs(
            &[ZoneKind::Battlefield],
            &[AbilityCostDef::DiscardSource],
        ));
    }

    #[test]
    fn decision_effects_stay_at_the_stack_effect_root() {
        static TAP: EffectDef = EffectDef::Tap {
            object: EffectRecipientDef::Source,
        };
        static UNTAP: EffectDef = EffectDef::Untap {
            object: EffectRecipientDef::Source,
        };
        static PLAIN_SEQUENCE_COMPONENTS: [EffectDef; 2] = [TAP, UNTAP];
        static PLAIN_SEQUENCE: EffectDef = EffectDef::Sequence(&PLAIN_SEQUENCE_COMPONENTS);
        static MAY_TAP: EffectDef = EffectDef::May(&TAP);
        static OPTIONAL_TAP: EffectDef = EffectDef::OptionalManaPayment {
            cost: ManaCost::new(1, 0),
            effect: &TAP,
        };
        static DELAYED_MAY: EffectDef = EffectDef::AtNextStep {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
            effect: &MAY_TAP,
        };
        static SEQUENCE_WITH_MAY: [EffectDef; 2] = [MAY_TAP, UNTAP];
        static SEQUENCE_WITH_PAYMENT: [EffectDef; 2] = [OPTIONAL_TAP, UNTAP];
        static SEQUENCE_WITH_DELAYED_MAY: [EffectDef; 2] = [DELAYED_MAY, UNTAP];

        assert!(shared_stack_effect(MAY_TAP));
        assert!(shared_stack_effect(EffectDef::May(&PLAIN_SEQUENCE)));
        assert!(shared_stack_effect(OPTIONAL_TAP));
        assert!(!shared_stack_effect(EffectDef::Sequence(
            &SEQUENCE_WITH_MAY,
        )));
        assert!(!shared_stack_effect(EffectDef::Sequence(
            &SEQUENCE_WITH_PAYMENT,
        )));
        assert!(shared_stack_effect(EffectDef::Sequence(
            &SEQUENCE_WITH_DELAYED_MAY,
        )));
    }

    #[test]
    fn composite_uncounterability_stays_within_the_shared_runtime_boundary() {
        static CANNOT_BE_COUNTERED: [AppliedEffectDef; 1] = [AppliedEffectDef::CannotBeCountered];
        static MIXED: [AppliedEffectDef; 2] = [
            AppliedEffectDef::CannotBeCountered,
            AppliedEffectDef::Special("unsupported"),
        ];
        static RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
            AppliedEffectDef::Composite(&CANNOT_BE_COUNTERED),
        )];
        static MIXED_RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
            AppliedEffectDef::Composite(&MIXED),
        )];

        let stack_effect = |effect| EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        };
        assert!(shared_static_effect(
            &[ZoneKind::Stack],
            stack_effect(AppliedEffectDef::Composite(&CANNOT_BE_COUNTERED)),
        ));
        assert!(!shared_static_effect(
            &[ZoneKind::Stack],
            stack_effect(AppliedEffectDef::Composite(&MIXED)),
        ));
        assert!(!shared_static_effect(
            &[ZoneKind::Stack],
            stack_effect(AppliedEffectDef::Composite(&[])),
        ));

        assert!(shared_mana_effect(
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless).with_spend_effects(&RIDERS),
            ),
            false,
        ));
        assert!(!shared_mana_effect(
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless).with_spend_effects(&MIXED_RIDERS),
            ),
            false,
        ));
    }

    #[test]
    fn fully_declarative_clauses_stay_within_the_shared_runtime_boundary() {
        for record in SET_MODULES
            .iter()
            .flat_map(|module| module.cards.iter().copied())
        {
            let definition = record.definition();
            for part in &definition.parts {
                for attached in part.rules.indexed_abilities() {
                    let ability_id = attached.id;
                    let ability = attached.definition;
                    assert!(
                        !matches!(
                            (ability.definition, ability.implementation),
                            (
                                DeclarativeAbilityDef::Legacy,
                                AbilityImplementationDef::CustomFull { behavior: None, .. }
                            )
                        ),
                        "{} {:?} ability {:?} is legacy text claiming full implementation without an executable behavior: {ability:?}",
                        definition.name,
                        part.id,
                        ability_id,
                    );
                    if ability.implementation == AbilityImplementationDef::Definition {
                        assert!(
                            shared_definition_ability(&ability),
                            "{} {:?} ability {:?} claims Definition outside the shared runtime boundary: {ability:?}",
                            definition.name,
                            part.id,
                            ability_id,
                        );
                    }
                    assert_nested_definition_abilities(&definition.name, ability.effect);
                    if let DeclarativeAbilityDef::Spell(spell) = ability.definition
                        && let Some(modal) = spell.modal()
                    {
                        for mode in modal.modes {
                            if mode.implementation == AbilityImplementationDef::Definition {
                                assert!(
                                    shared_definition_ability(mode),
                                    "{} {:?} ability {:?} contains a modal Definition branch outside the shared runtime boundary: {mode:?}",
                                    definition.name,
                                    part.id,
                                    ability_id,
                                );
                            }
                            assert_nested_definition_abilities(&definition.name, mode.effect);
                        }
                    }
                }
            }
        }
    }
}
