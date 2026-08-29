//! Regeneration as a declarative effect.
//!
//! The shield machinery and the destroy-event replacement already existed and
//! are exercised elsewhere; what is new is an `EffectDef` that arms a shield,
//! so a printed "{cost}: Regenerate this creature" is an ordinary activated
//! ability instead of an engine-level card branch. These tests drive it the
//! way a player would: find the ability in the legal-action list, pay for it,
//! and let the shield meet a real destruction.

use super::*;

/// Sedge Troll is the card that used to reach regeneration through a
/// card-identity escape valve, so it is the one that proves the declarative
/// path replaced it rather than joining it.
fn troll_game() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    game.players[PlayerId::One.index()].mana_pool.black = 4;
    (game, troll_id)
}

fn regenerate_actions(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .collect()
}

fn arm_shield(game: &mut Game, source: GameObjectId) {
    let actions = regenerate_actions(game, source);
    assert_eq!(
        actions.len(),
        1,
        "the regeneration ability must be offered exactly once, not once per path"
    );
    game.apply(PlayerId::One, actions[0].clone())
        .expect("the regeneration ability activates");
    pass_priority_pair(game);
}

fn troll(game: &Game, id: GameObjectId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
}

#[test]
fn a_declarative_regeneration_ability_arms_one_shield() {
    let (mut game, troll_id) = troll_game();
    assert_eq!(
        troll(&game, troll_id)
            .expect("the Troll is on the battlefield")
            .regeneration_shields,
        0,
    );

    arm_shield(&mut game, troll_id);

    let shielded = troll(&game, troll_id).expect("regenerating does nothing to the creature yet");
    assert_eq!(shielded.regeneration_shields, 1);
    assert!(
        !shielded.tapped,
        "a shield waits for a destruction rather than tapping now"
    );
}

/// CR 701.15: regeneration replaces destruction with tapping, removing from
/// combat, and removing all damage. The shield is spent doing it.
#[test]
fn an_armed_shield_replaces_lethal_damage_and_is_spent() {
    let (mut game, troll_id) = troll_game();
    arm_shield(&mut game, troll_id);

    {
        let shielded = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("the Troll is on the battlefield");
        shielded.damage = 99;
        shielded.attacking = true;
    }
    game.check_state_based_actions();

    let survivor = troll(&game, troll_id).expect("the shield replaced the destruction");
    assert_eq!(survivor.damage, 0, "regeneration removes all damage");
    assert!(survivor.tapped, "regeneration taps the permanent");
    assert!(!survivor.attacking, "regeneration removes it from combat");
    assert_eq!(survivor.regeneration_shields, 0, "the shield was spent");
}

#[test]
fn a_spent_shield_does_not_save_the_creature_twice() {
    let (mut game, troll_id) = troll_game();
    arm_shield(&mut game, troll_id);

    for _ in 0..2 {
        if let Some(permanent) = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == troll_id)
        {
            permanent.damage = 99;
        }
        game.check_state_based_actions();
    }

    assert!(
        troll(&game, troll_id).is_none(),
        "the second lethal damage had no shield left to replace it"
    );
}

/// A shield is a promise about this turn only. Two activations stack, and
/// whatever is left over is discarded rather than carried forward.
#[test]
fn shields_stack_within_a_turn_and_do_not_survive_cleanup() {
    let (mut game, troll_id) = troll_game();
    arm_shield(&mut game, troll_id);
    arm_shield(&mut game, troll_id);
    assert_eq!(
        troll(&game, troll_id)
            .expect("the Troll is on the battlefield")
            .regeneration_shields,
        2,
        "each activation arms its own shield"
    );

    game.finish_cleanup();

    assert_eq!(
        troll(&game, troll_id)
            .expect("the Troll is on the battlefield")
            .regeneration_shields,
        0,
        "unused shields do not carry to the next turn"
    );
}

/// The point of the primitive is the cards it unblocks, so one of them is
/// played here rather than merely counted: cast it, activate it, and let the
/// shield meet a destruction.
#[test]
fn a_newly_unblocked_regenerator_casts_activates_and_survives() {
    let mut game = ready_game();
    let troll = card(11_000, cards::UTHDEN_TROLL, PlayerId::One);
    let troll_card_id = troll.id;
    game.players[PlayerId::One.index()].hand.push(troll);
    game.players[PlayerId::One.index()].mana_pool = ManaPool {
        red: 4,
        colorless: 4,
        ..ManaPool::default()
    };

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: troll_card_id,
            choices: CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Uthden Troll is castable");
    pass_priority_pair(&mut game);

    let troll_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::UTHDEN_TROLL)
        .expect("the Troll resolved onto the battlefield")
        .card
        .id;

    arm_shield(&mut game, troll_id);
    game.destroy_permanent(troll_id);

    let survivor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("the shield replaced the destruction");
    assert!(survivor.tapped);
    assert_eq!(survivor.regeneration_shields, 0);
}

fn attached_aura(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    host: GameObjectId,
) -> Permanent {
    let mut aura = creature(id, definition, controller);
    aura.attached_to = Some(host);
    aura
}

#[test]
fn sacrificing_an_aura_as_the_cost_still_regenerates_its_former_host() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let host_id = host.card.id;
    let aura = attached_aura(10_001, cards::THRULL_RETAINER, PlayerId::One, host_id);
    let aura_id = aura.card.id;
    game.battlefield.extend([host, aura]);

    arm_shield(&mut game, aura_id);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura_id),
        "Thrull Retainer was sacrificed as the activation cost",
    );
    assert_eq!(
        troll(&game, host_id)
            .expect("the enchanted creature remains")
            .regeneration_shields,
        1,
        "the stack ability uses the Aura's last-known attachment",
    );
    game.destroy_permanent(host_id);
    assert!(troll(&game, host_id).is_some());
}

#[test]
fn removing_an_aura_in_response_does_not_erase_its_activated_abilitys_host() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let host_id = host.card.id;
    let aura = attached_aura(10_001, cards::REGENERATION, PlayerId::One, host_id);
    let aura_id = aura.card.id;
    game.battlefield.extend([host, aura]);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = regenerate_actions(&game, aura_id)
        .into_iter()
        .next()
        .expect("Regeneration's activation is legal");
    game.apply(PlayerId::One, action)
        .expect("the Aura's ability activates");
    game.destroy_permanent(aura_id);
    pass_priority_pair(&mut game);

    assert_eq!(
        troll(&game, host_id)
            .expect("the former enchanted creature remains")
            .regeneration_shields,
        1,
    );
}

#[test]
fn regeneration_preserves_damage_source_history_for_later_death_triggers() {
    let mut game = ready_game();
    let axelrod = creature(10_000, cards::AXELROD_GUNNARSON, PlayerId::One);
    let axelrod_id = axelrod.card.id;
    let victim = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([axelrod, victim]);

    assert_eq!(
        game.damage_target_from(Some(axelrod_id), Some(Target::Permanent(victim_id)), 1),
        1,
    );
    game.add_regeneration_shield(victim_id);
    game.destroy_permanent(victim_id);
    assert!(
        troll(&game, victim_id)
            .expect("the first destruction was regenerated")
            .damage_sources
            .contains(&axelrod_id),
        "regeneration removes marked damage but not the fact it was dealt",
    );

    game.destroy_permanent(victim_id);
    assert!(game.pending_triggers.iter().any(|trigger| {
        trigger.source.object == axelrod_id
            && trigger
                .text
                .starts_with("Whenever a creature dealt damage by Axelrod")
    }));
}

fn retired_regeneration_shields(game: &Game, id: GameObjectId) -> Option<u8> {
    match game.retired_objects.get(&id) {
        Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.regeneration_shields),
        Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
    }
}

/// Regeneration takes any recipient, so "regenerate target creature" and
/// "regenerate enchanted creature" were expressible from the moment the
/// effect existed. These cards were blocked on an audit line rather than on
/// a capability, so the test drives one of each shape rather than trusting
/// the coverage status alone.
#[test]
fn a_spell_can_regenerate_a_creature_it_targets() {
    let mut game = ready_game();
    let lions = creature(12_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    let ward = card(11_000, cards::DEATH_WARD, PlayerId::One);
    let ward_id = ward.id;
    game.players[PlayerId::One.index()].hand.push(ward);
    for color in [
        ManaColor::White,
        ManaColor::Green,
        ManaColor::Red,
        ManaColor::Black,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == ward_id
                        && choices.iter_targets().copied().eq([Target::Permanent(lions_id)])
            )
        })
        .expect("Death Ward can target the Lions");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    let shielded = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions_id)
        .expect("the Lions are on the battlefield");
    assert_eq!(shielded.regeneration_shields, 1, "the target was shielded");
}

/// An Aura regenerates what it is attached to, which is the same effect
/// reading a different recipient.
#[test]
fn an_aura_regenerates_the_creature_it_enchants() {
    let mut game = ready_game();
    let lions = creature(12_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    let mut aura = creature(12_001, cards::REGENERATION, PlayerId::One);
    aura.attached_to = Some(lions_id);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    for color in [
        ManaColor::White,
        ManaColor::Green,
        ManaColor::Red,
        ManaColor::Black,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == aura_id),
        )
        .expect("the Aura offers its regeneration ability");
    game.apply(PlayerId::One, action).expect("it activates");
    pass_priority_pair(&mut game);

    let shielded = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions_id)
        .expect("the enchanted creature is on the battlefield");
    assert_eq!(
        shielded.regeneration_shields, 1,
        "the enchanted creature was shielded, not the Aura"
    );
}

/// The follow-up sweep declared identities whose printed regeneration pays
/// something other than mana or shields something other than the source. Each
/// combination below is a cost or recipient the earlier tests never drove.
mod follow_up {
    use super::*;

    fn shields(game: &Game, id: GameObjectId) -> u8 {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the permanent is still on the battlefield")
            .regeneration_shields
    }

    /// Goblin Chirurgeon pays by sacrificing a Goblin -- which may be any
    /// Goblin, including itself -- and shields a chosen creature rather than
    /// its own body. Both halves are new: a predicate-matched sacrifice cost
    /// and a targeted recipient.
    #[test]
    fn goblin_chirurgeon_sacrifices_a_goblin_to_shield_the_creature_it_targets() {
        let mut game = ready_game();
        let chirurgeon = creature(10_000, cards::GOBLIN_CHIRURGEON, PlayerId::One);
        let chirurgeon_id = chirurgeon.card.id;
        game.battlefield.push(chirurgeon);
        let fodder = creature(10_001, cards::GOBLIN_CHIRURGEON, PlayerId::One);
        let fodder_id = fodder.card.id;
        game.battlefield.push(fodder);
        // Kobolds are not Goblins, so the sacrifice cost must refuse them even
        // though they are creatures their controller could otherwise give up.
        let kobolds = creature(10_002, cards::KOBOLDS_OF_KHER_KEEP, PlayerId::One);
        let kobolds_id = kobolds.card.id;
        game.battlefield.push(kobolds);
        let patient = creature(10_003, cards::SEDGE_TROLL, PlayerId::One);
        let patient_id = patient.card.id;
        game.battlefield.push(patient);

        assert!(
            !game.legal_actions(PlayerId::One).iter().any(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { cost_objects, .. } if cost_objects.as_slice() == [kobolds_id]
                )
            }),
            "no offered activation pays with something that is not a Goblin"
        );

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::ActivateAbility {
                    source,
                    cost_objects,
                    targets,
                    ..
                } => {
                    *source == chirurgeon_id
                        && cost_objects.as_slice() == [fodder_id]
                        && targets
                            .iter()
                            .flat_map(crate::casting::TargetSelection::targets)
                            .copied()
                            .eq(std::iter::once(Target::Permanent(patient_id)))
                }
                _ => false,
            })
            .expect("sacrificing the Kobolds to shield the Troll is one of the offered plays");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(shields(&game, patient_id), 1, "the target took the shield");
        assert_eq!(
            shields(&game, chirurgeon_id),
            0,
            "and the source kept none for itself"
        );
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == fodder_id),
            "the sacrificed Goblin left the battlefield"
        );
    }

    /// Marrow Bats pays life. Nothing taps, nothing is sacrificed, and the
    /// payment is visible on the life total rather than on the board.
    #[test]
    fn marrow_bats_buys_its_shield_with_life() {
        let mut game = ready_game();
        let bats = creature(10_000, cards::MARROW_BATS, PlayerId::One);
        let bats_id = bats.card.id;
        game.battlefield.push(bats);

        arm_shield(&mut game, bats_id);

        assert_eq!(shields(&game, bats_id), 1);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 4,
            "four life is what the shield cost"
        );
    }

    /// Necrobite is the first card whose single resolution both grants a
    /// keyword and arms a shield, so it proves a sequenced effect keeps one
    /// target across both halves.
    #[test]
    fn necrobite_grants_deathtouch_and_a_shield_to_the_same_target() {
        let mut game = ready_game();
        let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
        let troll_id = troll.card.id;
        game.battlefield.push(troll);
        let necrobite = card(10_001, cards::NECROBITE, PlayerId::One);
        game.players[PlayerId::One.index()]
            .hand
            .push(necrobite.clone());
        game.players[PlayerId::One.index()].mana_pool.black = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 2;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == necrobite.id
                        && choices
                            .iter_targets()
                            .copied()
                            .eq(std::iter::once(Target::Permanent(troll_id)))
                }
                _ => false,
            })
            .expect("Necrobite can be cast at the Troll");
        game.apply(PlayerId::One, action)
            .expect("the spell is cast");
        pass_priority_pair(&mut game);

        let shielded = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("the Troll is still there");
        assert_eq!(shielded.regeneration_shields, 1);
        assert!(
            game.permanent_has_executable_keyword(shielded, KeywordAbility::Deathtouch),
            "the same resolution left deathtouch behind"
        );
    }
}

/// "Can't be regenerated" as a standalone effect rather than a property of a
/// destroy. CR 701.19c: shields are not removed, they stop applying, and
/// resolving regeneration effects may still create them.
mod cannot_be_regenerated {
    use super::*;

    fn jackal_game() -> (Game, GameObjectId, GameObjectId) {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let jackal = creature(10_000, cards::HURR_JACKAL, PlayerId::One);
        let jackal_id = jackal.card.id;
        game.battlefield.push(jackal);
        let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        let troll_id = troll.card.id;
        game.battlefield.push(troll);
        game.players[PlayerId::Two.index()].mana_pool.black = 4;
        (game, jackal_id, troll_id)
    }

    fn point_at(game: &mut Game, jackal: GameObjectId, victim: GameObjectId) {
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    *source == jackal
                        && targets
                            .iter()
                            .flat_map(crate::casting::TargetSelection::targets)
                            .any(|target| *target == Target::Permanent(victim))
                }
                _ => false,
            })
            .expect("the Jackal can point at that creature");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(game);
    }

    #[test]
    fn a_shield_armed_afterwards_is_kept_but_does_not_apply() {
        let (mut game, jackal_id, troll_id) = jackal_game();
        point_at(&mut game, jackal_id, troll_id);

        game.add_regeneration_shield(troll_id);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == troll_id)
                .expect("still there")
                .regeneration_shields,
            1,
            "the prohibition stops application, not shield creation"
        );

        game.destroy_permanent(troll_id);
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == troll_id),
        );
        assert_eq!(
            retired_regeneration_shields(&game, troll_id),
            Some(1),
            "the prohibited shield was not consumed",
        );
    }

    /// A shield armed *before* the prohibition is not removed by it -- it
    /// simply stops applying, which is the distinction CR 701.19c draws.
    #[test]
    fn a_shield_armed_beforehand_is_kept_but_does_not_apply() {
        let (mut game, jackal_id, troll_id) = jackal_game();
        game.add_regeneration_shield(troll_id);
        point_at(&mut game, jackal_id, troll_id);

        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == troll_id)
                .expect("still there")
                .regeneration_shields,
            1,
            "the shield is still there"
        );

        game.destroy_permanent(troll_id);
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == troll_id),
            "and it did not save the creature"
        );
    }

    #[test]
    fn lethal_damage_cannot_apply_a_shield_while_the_prohibition_holds() {
        let (mut game, jackal_id, troll_id) = jackal_game();
        game.add_regeneration_shield(troll_id);
        point_at(&mut game, jackal_id, troll_id);
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("the Troll is still there")
            .damage = 2;

        game.check_state_based_actions();

        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == troll_id),
            "lethal damage uses the same prohibition as explicit destruction",
        );
        assert_eq!(retired_regeneration_shields(&game, troll_id), Some(1));
    }

    /// The prohibition is for the turn, so a creature pointed at survives the
    /// next turn's destruction on a fresh shield.
    #[test]
    fn the_prohibition_ends_with_the_turn() {
        let (mut game, jackal_id, troll_id) = jackal_game();
        point_at(&mut game, jackal_id, troll_id);
        game.finish_cleanup();

        game.add_regeneration_shield(troll_id);
        game.destroy_permanent(troll_id);

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.id == troll_id),
            "a new turn is a new shield"
        );
    }
}

/// The Premodern half of the same sweep. Neither card needed engine work;
/// what they add is a sacrifice cost that refuses the source itself, and a
/// granted regeneration ability riding on an Aura.
mod premodern {
    use super::*;

    #[test]
    fn vampire_warlord_eats_another_creature_but_not_itself() {
        let mut game = ready_game();
        let warlord = creature(10_000, cards::VAMPIRE_WARLORD, PlayerId::One);
        let warlord_id = warlord.card.id;
        game.battlefield.push(warlord);
        let fodder = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
        let fodder_id = fodder.card.id;
        game.battlefield.push(fodder);

        assert!(
            !game.legal_actions(PlayerId::One).iter().any(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, cost_objects, .. }
                        if *source == warlord_id && cost_objects.as_slice() == [warlord_id]
                )
            }),
            "\"another creature\" excludes the Warlord itself"
        );

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, cost_objects, .. }
                        if *source == warlord_id && cost_objects.as_slice() == [fodder_id]
                )
            })
            .expect("the other creature can pay");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == warlord_id)
                .expect("still there")
                .regeneration_shields,
            1,
        );
    }

    /// Trollhide grants its host an activated ability, so the shield is armed
    /// by the creature rather than by the Aura.
    #[test]
    fn trollhide_grants_its_host_a_regeneration_ability() {
        let mut game = ready_game();
        let troll = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
        let troll_id = troll.card.id;
        game.battlefield.push(troll);
        let mut hide = creature(10_001, cards::TROLLHIDE, PlayerId::One);
        hide.attached_to = Some(troll_id);
        game.battlefield.push(hide);
        game.players[PlayerId::One.index()].mana_pool.green = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 1;

        let host = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("the host is on the battlefield");
        assert_eq!(game.power(host), Some(4), "a 2/1 wearing +2/+2");

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == troll_id)
            })
            .expect("the granted ability is offered on the creature");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == troll_id)
                .expect("still there")
                .regeneration_shields,
            1,
        );
    }
}
