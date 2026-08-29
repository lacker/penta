//! Preventing what one creature would deal for a turn.
//!
//! Subdue stops its combat damage; Kry Shield stops every kind. The pair is
//! what makes the difference visible: the same creature under one still
//! pings with an ability, and under the other does not. Both hand back
//! toughness equal to the creature's mana value, which is read off the
//! target as the spell resolves.

use super::*;

/// An Orcish Artillery for player one -- a creature that can deal damage
/// without attacking -- and the given shield already on the battlefield.
fn artillery_and(shield: Option<CardDefinitionId>) -> (Game, GameObjectId, Option<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let orc = creature(10_000, cards::ORCISH_ARTILLERY, PlayerId::One);
    let orc_id = orc.card.id;
    game.battlefield.push(orc);
    let shield_id = shield.map(|definition| {
        let permanent = creature(10_001, definition, PlayerId::One);
        let id = permanent.card.id;
        game.battlefield.push(permanent);
        id
    });
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    (game, orc_id, shield_id)
}

/// Aims `source`'s ability at the Orc and resolves it.
fn point_at(game: &mut Game, source: GameObjectId, orc: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: from,
                targets,
                ..
            } => {
                *from == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(orc))
            }
            _ => false,
        })
        .expect("the shield can name the Orc");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

fn toughness(game: &Game, permanent: GameObjectId) -> Option<i16> {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .and_then(|candidate| game.toughness(candidate))
}

#[test]
fn kry_shield_silences_every_kind_of_damage() {
    let (mut game, orc, shield) = artillery_and(Some(cards::KRY_SHIELD));
    let shield = shield.expect("the shield is there");
    point_at(&mut game, shield, orc);

    // Orcish Artillery is a 1/3 costing {2}{R}, so it gains three toughness.
    assert_eq!(toughness(&game, orc), Some(6));

    let victim = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.damage_target_from(Some(orc), Some(Target::Permanent(victim_id)), 2);

    let victim = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == victim_id)
        .expect("still there");
    assert_eq!(victim.damage, 0, "even its ability's damage is prevented");
}

/// The same creature under Subdue still pings: Subdue names combat damage,
/// and an ability is not combat damage.
#[test]
fn subdue_leaves_an_ability_alone() {
    let mut game = ready_game();
    let orc = creature(10_000, cards::ORCISH_ARTILLERY, PlayerId::One);
    let orc_id = orc.card.id;
    game.battlefield.push(orc);
    let spell = card(10_001, cards::SUBDUE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(orc_id))
            }
            _ => false,
        })
        .expect("Subdue can name the Orc");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(toughness(&game, orc_id), Some(6));

    let victim = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.damage_target_from(Some(orc_id), Some(Target::Permanent(victim_id)), 2);

    let victim = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == victim_id)
        .expect("still there");
    assert_eq!(victim.damage, 2, "Subdue names combat damage and no other");
}

/// Both stop combat damage, which is the half they share.
#[test]
fn a_silenced_attacker_deals_no_combat_damage() {
    let (mut game, orc, shield) = artillery_and(Some(cards::KRY_SHIELD));
    let shield = shield.expect("the shield is there");
    point_at(&mut game, shield, orc);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == orc)
        .expect("still there")
        .attacking = true;
    let before = game.players[PlayerId::Two.index()].life;
    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, before);
}
