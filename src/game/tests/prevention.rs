//! Unified damage-prevention matching, spending, and expiration.

use super::super::prevention_state::{
    ResolvedDamagePrevention, ResolvedDamagePreventionCapacity, ResolvedDamagePreventionCoverage,
    ResolvedDamageRecipientMatcher, ResolvedDamageSourceMatcher,
};
use super::*;
use crate::{
    DamageEventMatcherDef, DamagePreventionDef, DamageRecipientMatcherDef,
    ResolvedEffectDurationDef,
};

fn install_prevention(
    game: &mut Game,
    source: ResolvedDamageSourceMatcher,
    recipient: ResolvedDamageRecipientMatcher,
    combat_only: bool,
    capacity: ResolvedDamagePreventionCapacity,
    coverage: ResolvedDamagePreventionCoverage,
    gain_life: Option<PlayerId>,
) {
    let timestamp = game.allocate_continuous_effect_timestamp();
    game.damage_preventions.push(ResolvedDamagePrevention {
        source,
        recipient,
        combat_only,
        capacity,
        coverage,
        gain_life,
        source_ability: AbilitySourceRef {
            object: GameObjectId(20_000),
            ability: AbilityOrigin::Printed {
                definition: cards::FOG,
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            },
        },
        timestamp,
        expiration: ContinuousEffectExpiration::EndOfTurn,
    });
}

fn install_fog(game: &mut Game) {
    install_prevention(
        game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Any,
        true,
        ResolvedDamagePreventionCapacity::Unlimited,
        ResolvedDamagePreventionCoverage::All,
        None,
    );
}

fn fogged_combat(cast_fog: bool) -> Game {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SEA_SERPENT, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);
    let mut blocker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = vec![GameObjectId(10_000)];
    game.battlefield.push(blocker);
    if cast_fog {
        install_fog(&mut game);
    }
    game
}

fn resolve_combat_damage(game: &mut Game) {
    game.finish_declaring_blockers();
    game.start_combat_damage();
    game.finish_rules_procedure();
}

#[test]
fn combat_damage_lands_without_a_fog() {
    let mut game = fogged_combat(false);
    resolve_combat_damage(&mut game);
    let serpent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000));
    assert!(
        serpent.is_some_and(|permanent| permanent.damage > 0),
        "the blocker's damage is marked"
    );
}

#[test]
fn a_fog_prevents_damage_in_both_directions() {
    let mut game = fogged_combat(true);
    resolve_combat_damage(&mut game);
    for id in [GameObjectId(10_000), GameObjectId(10_001)] {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("both combatants survive a Fog");
        assert_eq!(permanent.damage, 0, "{id:?} took no combat damage");
    }
}

/// The shield covers what the attacker would have dealt to the player too.
#[test]
fn a_fog_prevents_damage_to_the_defending_player() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SEA_SERPENT, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);
    install_fog(&mut game);
    let before = game.players[PlayerId::Two.index()].life;

    resolve_combat_damage(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        before,
        "an unblocked attacker deals nothing through a Fog"
    );
}

/// It is a turn-scoped shield, not a permanent one.
#[test]
fn a_fog_does_not_survive_cleanup() {
    let mut game = fogged_combat(true);
    game.finish_cleanup();
    assert!(
        game.damage_preventions.is_empty(),
        "the rule expires with the turn"
    );
}

#[test]
fn fog_uses_the_central_damage_pipeline_and_only_matches_combat() {
    let mut game = ready_game();
    install_fog(&mut game);
    let before = game.players[PlayerId::Two.index()].life;

    assert_eq!(
        game.damage_target_from_kind(None, Some(Target::Player(PlayerId::Two)), 3, true),
        0,
    );
    assert_eq!(
        game.damage_target_from(None, Some(Target::Player(PlayerId::Two)), 2),
        2,
    );
    assert_eq!(game.players[PlayerId::Two.index()].life, before - 2);
}

fn shielded_creature(game: &mut Game) -> GameObjectId {
    let creature = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let id = creature.card.id;
    game.battlefield.push(creature);
    id
}

/// A shield waits for damage rather than acting now, and is spent by the
/// damage it covers.
#[test]
fn a_shield_absorbs_up_to_its_amount_and_is_then_gone() {
    let mut game = ready_game();
    let target = shielded_creature(&mut game);
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Permanent(target)),
        false,
        ResolvedDamagePreventionCapacity::Amount(2),
        ResolvedDamagePreventionCoverage::All,
        None,
    );

    game.damage_target(Some(Target::Permanent(target)), 1);
    let marked = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .map_or(0, |permanent| permanent.damage)
    };
    assert_eq!(marked(&game), 0, "the first point is prevented");

    game.damage_target(Some(Target::Permanent(target)), 3);
    assert_eq!(
        marked(&game),
        2,
        "one point of the shield was left, so two of the three land"
    );
    assert!(
        game.damage_preventions.is_empty(),
        "a spent promise is gone"
    );
}

/// "Prevent all damage" is never spent, so it holds for the whole turn.
#[test]
fn a_prevent_all_shield_is_not_consumed() {
    let mut game = ready_game();
    let target = shielded_creature(&mut game);
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Permanent(target)),
        false,
        ResolvedDamagePreventionCapacity::Unlimited,
        ResolvedDamagePreventionCoverage::All,
        None,
    );

    for _ in 0..3 {
        game.damage_target(Some(Target::Permanent(target)), 5);
    }
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target)
        .expect("the creature survives");
    assert_eq!(permanent.damage, 0, "every point was prevented");
    assert_eq!(game.damage_preventions.len(), 1, "the rule still holds");
}

#[test]
fn a_shield_only_covers_the_recipient_it_names() {
    let mut game = ready_game();
    let shielded = shielded_creature(&mut game);
    let other = creature(20_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let other_id = other.card.id;
    game.battlefield.push(other);
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Permanent(shielded)),
        false,
        ResolvedDamagePreventionCapacity::Amount(5),
        ResolvedDamagePreventionCoverage::All,
        None,
    );

    game.damage_target(Some(Target::Permanent(other_id)), 1);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == other_id)
        .expect("the other creature is on the battlefield");
    assert_eq!(permanent.damage, 1, "an unshielded creature takes damage");
}

/// Shields cover players too, which is what "any target" means.
#[test]
fn a_shield_can_cover_a_player() {
    let mut game = ready_game();
    let before = game.players[PlayerId::Two.index()].life;
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::Two)),
        false,
        ResolvedDamagePreventionCapacity::Amount(3),
        ResolvedDamagePreventionCoverage::All,
        None,
    );

    game.damage_target(Some(Target::Player(PlayerId::Two)), 2);
    assert_eq!(game.players[PlayerId::Two.index()].life, before);
}

#[test]
fn shields_do_not_survive_cleanup() {
    let mut game = ready_game();
    let target = shielded_creature(&mut game);
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Permanent(target)),
        false,
        ResolvedDamagePreventionCapacity::Unlimited,
        ResolvedDamagePreventionCoverage::All,
        None,
    );
    game.finish_cleanup();
    assert!(game.damage_preventions.is_empty());
}

fn resolving_prevention_object(controller: PlayerId) -> StackObject {
    spell(20_000, cards::LIGHTNING_BOLT, controller, 0)
}

#[test]
fn static_source_predicates_use_retired_creature_last_known_information() {
    let mut game = ready_game();
    let uncle = creature(10_001, cards::UNCLE_ISTVAN, PlayerId::One);
    let uncle_id = uncle.card.id;
    let source = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let source_id = source.card.id;
    game.battlefield.extend([uncle, source]);
    game.move_permanents_to_graveyard(&[source_id]);

    assert_eq!(
        game.damage_target_from(Some(source_id), Some(Target::Permanent(uncle_id)), 3),
        0,
        "the live static ability still recognizes the departed creature source",
    );
}

#[test]
fn safe_passage_tracks_later_controlled_creatures_but_not_planeswalkers() {
    let mut game = ready_game();
    let object = resolving_prevention_object(PlayerId::One);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(
                DamageEventMatcherDef::to_player_and_creatures_controlled_by(
                    PlayerRefDef::EffectController,
                ),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        }),
        &object,
        TriggerContext::empty(),
    );

    let protected = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    let protected_id = protected.card.id;
    let unprotected = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    let unprotected_id = unprotected.card.id;
    let planeswalker = creature(10_003, cards::VRASKA_THE_UNSEEN, PlayerId::One);
    let planeswalker_id = planeswalker.card.id;
    game.battlefield
        .extend([protected, unprotected, planeswalker]);

    assert_eq!(
        game.damage_target_from(None, Some(Target::Player(PlayerId::One)), 2),
        0,
    );
    assert_eq!(
        game.damage_target_from(None, Some(Target::Permanent(protected_id)), 2),
        0,
        "a creature entering after resolution is protected",
    );
    assert_eq!(
        game.damage_target_from(None, Some(Target::Permanent(unprotected_id)), 2),
        2,
    );
    assert_eq!(
        game.damage_target_from(None, Some(Target::Permanent(planeswalker_id)), 2),
        2,
        "the player-wide wording does not include their planeswalkers",
    );
}

#[test]
fn terrifying_presence_preserves_only_the_chosen_sources_combat_damage() {
    let mut game = ready_game();
    let chosen = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    let chosen_id = chosen.card.id;
    let other = creature(10_002, cards::SERRA_ANGEL, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.extend([chosen, other]);

    let mut object = resolving_prevention_object(PlayerId::One);
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(vec![Target::Permanent(chosen_id)], 0),
    ));
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_except(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        }),
        &object,
        TriggerContext::empty(),
    );

    assert_eq!(
        game.damage_target_from_kind(
            Some(chosen_id),
            Some(Target::Player(PlayerId::Two)),
            3,
            true,
        ),
        3,
    );
    assert_eq!(
        game.damage_target_from_kind(Some(other_id), Some(Target::Player(PlayerId::Two)), 3, true,),
        0,
    );
    assert_eq!(
        game.damage_target_from(Some(other_id), Some(Target::Player(PlayerId::Two)), 1),
        1,
        "the effect applies only to combat damage",
    );
}

#[test]
fn drain_life_gains_only_the_damage_left_after_prevention() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].life = 10;
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::Two)),
        false,
        ResolvedDamagePreventionCapacity::Amount(2),
        ResolvedDamagePreventionCoverage::All,
        None,
    );
    let object = resolving_prevention_object(PlayerId::One);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::DrainLife {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::Constant(3),
        }),
        &object,
        TriggerContext::empty(),
    );

    assert_eq!(game.players[PlayerId::One.index()].life, 11);
    assert_eq!(game.players[PlayerId::Two.index()].life, 19);
}

#[test]
fn a_life_gain_shield_precedes_overlapping_relational_prevention() {
    let mut game = ready_game();
    let source = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
    let source_id = source.card.id;
    game.battlefield.push(source);
    // Install the unlimited rule first to prove consumable prevention still
    // receives the event before it.
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(PlayerId::One),
        false,
        ResolvedDamagePreventionCapacity::Unlimited,
        ResolvedDamagePreventionCoverage::All,
        None,
    );
    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Exact(source_id),
        ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::One)),
        false,
        ResolvedDamagePreventionCapacity::Events(1),
        ResolvedDamagePreventionCoverage::All,
        Some(PlayerId::One),
    );
    let starting_life = game.players[PlayerId::One.index()].life;

    assert_eq!(
        game.damage_target_from(Some(source_id), Some(Target::Player(PlayerId::One)), 3),
        0,
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        starting_life + 3,
        "the existing Reverse Damage-style shield keeps its prevention rider",
    );
    assert_eq!(
        game.damage_preventions.len(),
        1,
        "only the unlimited Safe Passage-style rule remains",
    );
}

#[test]
fn combat_player_trigger_uses_only_damage_left_after_prevention() {
    let mut game = ready_game();
    let courier = creature(10_001, cards::CROSSTOWN_COURIER, PlayerId::One);
    let courier_id = courier.card.id;
    game.battlefield.push(courier);
    let starting_life = game.players[PlayerId::Two.index()].life;

    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::Two)),
        false,
        ResolvedDamagePreventionCapacity::Amount(2),
        ResolvedDamagePreventionCoverage::All,
        None,
    );
    game.deal_combat_damage_to_player(courier_id, PlayerId::Two, 2);
    assert!(
        game.pending_triggers.is_empty(),
        "fully prevented combat damage does not trigger the Courier",
    );
    assert_eq!(game.players[PlayerId::Two.index()].life, starting_life);

    install_prevention(
        &mut game,
        ResolvedDamageSourceMatcher::Any,
        ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::Two)),
        false,
        ResolvedDamagePreventionCapacity::Amount(1),
        ResolvedDamagePreventionCoverage::All,
        None,
    );
    game.deal_combat_damage_to_player(courier_id, PlayerId::Two, 2);
    let trigger = game
        .pending_triggers
        .iter()
        .find(|trigger| trigger.source.object == courier_id)
        .expect("the Courier sees the one point actually dealt");
    assert_eq!(trigger.context.trigger.amount, Some(1));
    assert_eq!(game.players[PlayerId::Two.index()].life, starting_life - 1);
}

/// A second sweep, prompted by the shields having outlived their audit lines.
/// Seven identities were blocked on "a duration-scoped replacement/prevention
/// effect" that had already been built; the two shapes below are the ones the
/// first sweep never drove -- a shield aimed at a player, and prevention of
/// only the combat damage a creature deals.
mod follow_up {
    use super::*;

    /// Conservator shields its controller, not a permanent. The shield has to
    /// find a player recipient and spend itself on damage aimed there.
    #[test]
    fn conservator_shields_its_controller_and_spends_the_shield() {
        let mut game = ready_game();
        let conservator = creature(10_000, cards::CONSERVATOR, PlayerId::One);
        let conservator_id = conservator.card.id;
        game.battlefield.push(conservator);
        game.players[PlayerId::One.index()].mana_pool.colorless = 3;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == conservator_id)
            })
            .expect("the ability is affordable");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(
            game.damage_preventions.len(),
            1,
            "one prevention rule, aimed at a player"
        );
        game.damage_target(Some(Target::Player(PlayerId::One)), 3);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 1,
            "two of the three damage was prevented"
        );
        assert!(
            game.damage_preventions.is_empty(),
            "and the rule was spent doing it"
        );
    }

    /// Horn of Deafening stops what the creature deals without touching what
    /// is dealt to it, which is the distinction between the two combat-damage
    /// prevention effects.
    #[test]
    fn horn_of_deafening_silences_one_attacker_in_one_direction() {
        let mut game = ready_game();
        let horn = creature(10_000, cards::HORN_OF_DEAFENING, PlayerId::One);
        let horn_id = horn.card.id;
        game.battlefield.push(horn);
        game.players[PlayerId::One.index()].mana_pool.colorless = 2;
        let ogre = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        let ogre_id = ogre.card.id;
        game.battlefield.push(ogre);

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == horn_id)
            })
            .expect("the ability is affordable");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(
            game.damage_target_from_kind(
                Some(ogre_id),
                Some(Target::Player(PlayerId::One)),
                3,
                true,
            ),
            0,
            "the creature deals no combat damage this turn",
        );
        assert_eq!(
            game.damage_target_from_kind(Some(horn_id), Some(Target::Permanent(ogre_id)), 1, true,),
            1,
            "but combat damage dealt to it is untouched",
        );
    }
}

/// A continuous combat-damage prevention, which is what an Aura needs and
/// what the turn-scoped effects could not give it. The flags those set are
/// written once and cleared at cleanup; this is asked afresh every time
/// combat damage is dealt, so the Aura leaving mid-combat stops applying.
mod gaseous_form {
    use super::*;

    fn form_game() -> (Game, GameObjectId, GameObjectId) {
        let mut game = ready_game();
        game.step = Step::DeclareBlockers;
        let mut attacker = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        let attacker_id = attacker.card.id;
        game.battlefield.push(attacker);

        let mut aura = creature(10_001, cards::GASEOUS_FORM, PlayerId::One);
        aura.attached_to = Some(attacker_id);
        let aura_id = aura.card.id;
        game.battlefield.push(aura);
        (game, attacker_id, aura_id)
    }

    #[test]
    fn an_enchanted_attacker_deals_no_combat_damage() {
        let (mut game, _attacker_id, _aura_id) = form_game();
        game.finish_declaring_blockers();
        game.deal_combat_damage();

        assert_eq!(
            game.players[PlayerId::Two.index()].life,
            i16::from(rules::STARTING_LIFE),
            "the enchanted creature's combat damage was prevented"
        );
    }

    #[test]
    fn the_composite_prevents_combat_damage_both_to_and_by_the_creature() {
        let (mut game, attacker_id, _aura_id) = form_game();
        let mut blocker = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
        let blocker_id = blocker.card.id;
        blocker.blocking = vec![attacker_id];
        game.battlefield.push(blocker);

        resolve_combat_damage(&mut game);

        for id in [attacker_id, blocker_id] {
            let permanent = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .expect("both creatures survive the prevented exchange");
            assert_eq!(permanent.damage, 0, "{id:?} takes no combat damage");
        }
    }

    /// The same creature, once the Aura is gone, hits for its printed power.
    /// This is the half a turn-scoped flag would get wrong.
    #[test]
    fn removing_the_aura_restores_the_damage_immediately() {
        let (mut game, attacker_id, aura_id) = form_game();
        game.battlefield
            .retain(|permanent| permanent.card.id != aura_id);
        let power = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .and_then(|permanent| game.power(permanent))
            .expect("the attacker has power");

        game.finish_declaring_blockers();
        game.deal_combat_damage();

        assert_eq!(
            game.players[PlayerId::Two.index()].life,
            i16::from(rules::STARTING_LIFE) - power,
            "with the Aura gone nothing is prevented"
        );
    }
}

/// A shield keyed to one chosen source, which is what a Circle of Protection
/// needs and what a recipient-keyed shield cannot express. The distinction
/// only shows when more than one thing is dealing damage, so that is what
/// these drive.
mod circle_of_protection {
    use super::*;
    use crate::ParentBinding;

    fn circle_game() -> (Game, GameObjectId) {
        let mut game = ready_game();
        let circle = creature(10_000, cards::CIRCLE_OF_PROTECTION_RED, PlayerId::One);
        let circle_id = circle.card.id;
        game.battlefield.push(circle);
        game.players[PlayerId::One.index()].mana_pool.colorless = 4;
        (game, circle_id)
    }

    fn activate(game: &mut Game, circle: GameObjectId) {
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == circle)
            })
            .expect("the Circle is affordable");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(game);
    }

    /// With one red source there is nothing to choose, so the shield is
    /// installed without a decision and answers that source.
    #[test]
    fn a_single_red_source_needs_no_decision_and_is_shielded_against() {
        let (mut game, circle_id) = circle_game();
        let dragon = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let dragon_id = dragon.card.id;
        game.battlefield.push(dragon);

        activate(&mut game, circle_id);
        assert!(
            game.pending_decisions.is_empty(),
            "one candidate is not a choice"
        );

        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 4);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE),
            "all of the chosen source's damage was prevented, not just one point"
        );
    }

    /// The shield answers its own source and nothing else. A second red
    /// creature the player did not name still connects -- this is the whole
    /// difference from a shield that answers any source.
    #[test]
    fn the_shield_ignores_a_red_source_it_did_not_name() {
        let (mut game, circle_id) = circle_game();
        let named = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let named_id = named.card.id;
        game.battlefield.push(named);

        activate(&mut game, circle_id);

        let other = creature(10_002, cards::DRAGON_WHELP, PlayerId::Two);
        let other_id = other.card.id;
        game.battlefield.push(other);

        game.damage_target_from(Some(other_id), Some(Target::Player(PlayerId::One)), 3);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 3,
            "the unnamed source is not covered"
        );

        game.damage_target_from(Some(named_id), Some(Target::Player(PlayerId::One)), 3);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 3,
            "and the named one still is"
        );
    }

    /// One activation is one prevention. The shield is spent by the first
    /// damage its source deals, however much that was.
    #[test]
    fn the_shield_is_spent_by_the_first_damage_from_its_source() {
        let (mut game, circle_id) = circle_game();
        let dragon = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let dragon_id = dragon.card.id;
        game.battlefield.push(dragon);

        activate(&mut game, circle_id);
        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 2);
        assert!(game.damage_preventions.is_empty(), "the rule was spent");

        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 2);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 2,
            "the second hit from the same source is not covered"
        );
    }

    /// Two red sources is a real choice, and the player is asked. This is the
    /// decision an ordinary prevention shield never has to make.
    #[test]
    fn two_red_sources_ask_the_controller_which_one() {
        let (mut game, circle_id) = circle_game();
        game.battlefield
            .push(creature(10_001, cards::DRAGON_WHELP, PlayerId::Two));
        game.battlefield
            .push(creature(10_002, cards::DRAGON_WHELP, PlayerId::Two));

        activate(&mut game, circle_id);

        let decision = game
            .pending_decisions
            .first()
            .expect("the controller is asked which source");
        assert_eq!(decision.observation.player, PlayerId::One);
        assert_eq!(decision.observation.options.len(), 2);
    }

    /// Object choices retain their actual zone kind. A spell chosen on the
    /// stack must remain a spell reference when the nested shield consumes
    /// the binding; treating every choice as a permanent silently drops it.
    #[test]
    fn a_stack_spell_can_be_the_chosen_damage_source() {
        static SHIELD: EffectDef = EffectDef::PreventDamage {
            prevention: DamagePreventionDef::events(
                DamageEventMatcherDef {
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::Controller,
                    ),
                    ..DamageEventMatcherDef::from(ObjectRefDef::Binding(ParentBinding))
                },
                1,
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        };

        let mut game = ready_game();
        let bolt = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
        let bolt_id = bolt.id;
        game.stack.push(bolt);
        let resolving = resolving_prevention_object(PlayerId::One);

        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    &[ZoneKind::Battlefield, ZoneKind::Stack],
                )),
                exclude: Some(ObjectRefDef::ResolvingObject),
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &SHIELD,
            })),
            &resolving,
            TriggerContext::empty(),
        );

        assert!(game.pending_decisions.is_empty(), "there is one red source");
        assert_eq!(game.damage_preventions.len(), 1);
        assert_eq!(
            game.damage_preventions[0].source,
            ResolvedDamageSourceMatcher::Exact(bolt_id),
        );
        assert_eq!(
            game.damage_target_from(Some(bolt_id), Some(Target::Player(PlayerId::One)), 3),
            0,
        );
    }
}

// Two printed cards make the shield stop something other than the whole hit,
// or pay a rider when it fires. The arithmetic is the point: an odd point of
// damage gets through Dark Sphere, and Reverse Damage gains exactly what it
// stopped rather than what was aimed.
include!("prevention/shield_coverage.rs");
