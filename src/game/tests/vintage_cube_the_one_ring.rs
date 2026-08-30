//! The One Ring: one turn nothing can touch you, then a card every turn for
//! a life total that drains faster each time.

use super::*;

/// The Ring in hand with four mana up, a library to draw from, and a Bolt in
/// the other player's hand to aim at things.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    for index in 0..12 {
        game.players[0]
            .library
            .push(card(97_000 + index, cards::ISLAND, PlayerId::One));
    }
    let ring = game
        .build_zone(PlayerId::One, &[cards::THE_ONE_RING])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let ring_id = ring.id;
    game.players[0].hand.push(ring);
    game.players[1]
        .hand
        .push(card(97_500, cards::LIGHTNING_BOLT, PlayerId::Two));
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    (game, ring_id)
}

/// Passes priority until the stack and the trigger queue are empty.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if !game.pending_decisions.is_empty() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn cast_the_ring(game: &mut Game, ring: GameObjectId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ring))
        .expect("four mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(game);
}

fn burden(game: &Game) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::THE_ONE_RING)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::named("burden"))
        })
}

fn ring_activations(game: &Game) -> Vec<Action> {
    let ring = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::THE_ONE_RING)
        .expect("the Ring is on the battlefield")
        .card
        .id;
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == ring),
        )
        .collect()
}

/// Whether the other player's Bolt can be aimed at the Ring's controller.
/// Asked on a copy holding priority, so the answer is about who may be
/// targeted rather than about whose turn it is.
fn bolt_can_hit_player_one(game: &Game) -> bool {
    let mut game = game.clone();
    game.priority = PlayerId::Two;
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::CastSpell { card, ref choices, .. }
                if game.players[1].hand.iter().any(|held| held.id == card)
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::One))
        )
    })
}

/// Whether the other player's Curse can be aimed at the Ring's controller.
fn curse_can_enchant_player_one(game: &Game) -> bool {
    // A Curse is a sorcery-speed spell, so the copy asks during their own
    // main phase.
    let mut game = game.clone();
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::CastSpell { card, ref choices, .. }
                if game.players[1].hand.iter().any(|held| {
                    held.id == card && held.definition == cards::CURSE_OF_THE_BLOODY_TOME
                }) && choices
                    .iter_targets()
                    .any(|target| *target == Target::Player(PlayerId::One))
        )
    })
}

/// Cast, and nothing the other player has can be pointed at you.
#[test]
fn casting_it_grants_protection_from_everything() {
    let (mut game, ring) = staged();
    assert!(
        bolt_can_hit_player_one(&game),
        "before the Ring the Bolt has a face to aim at",
    );

    cast_the_ring(&mut game, ring);

    assert!(
        !bolt_can_hit_player_one(&game),
        "protection from everything takes the player off the target list",
    );
}

/// An Aura is the third thing protection stops: a Curse cannot be aimed at
/// a protected player, so it cannot enchant them either.
#[test]
fn a_curse_cannot_enchant_a_protected_player() {
    let (mut game, ring) = staged();
    game.players[1]
        .hand
        .push(card(97_501, cards::CURSE_OF_THE_BLOODY_TOME, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 3);
    assert!(
        curse_can_enchant_player_one(&game),
        "an unprotected player is an ordinary Curse target",
    );

    cast_the_ring(&mut game, ring);

    assert!(
        !curse_can_enchant_player_one(&game),
        "protection from everything refuses the Aura as well",
    );
}

/// Protection prevents the damage as well as the targeting, whatever the
/// source is.
#[test]
fn damage_from_anything_is_prevented() {
    let (mut game, ring) = staged();
    cast_the_ring(&mut game, ring);
    let before = game.players[0].life;
    let attacker = creature(97_600, cards::GRIZZLY_BEARS, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let dealt = game.damage_target_from_kind(
        Some(attacker_id),
        Some(Target::Player(PlayerId::One)),
        2,
        true,
    );

    assert_eq!(dealt, 0, "no damage from a matching source reaches them");
    assert_eq!(
        game.players[0].life, before,
        "so the life total is untouched"
    );
}

/// "Until your next turn" ends where it says: the turn after the one it was
/// cast on.
#[test]
fn the_protection_ends_on_your_next_turn() {
    let (mut game, ring) = staged();
    cast_the_ring(&mut game, ring);
    assert!(!bolt_can_hit_player_one(&game));

    game.start_next_turn();
    assert!(
        !bolt_can_hit_player_one(&game),
        "their turn is inside the window",
    );

    game.start_next_turn();

    assert!(
        bolt_can_hit_player_one(&game),
        "your own next turn beginning ends it",
    );
}

/// "If you cast it" is an intervening-if: a Ring put onto the battlefield
/// protects nobody, and never puts the trigger on the stack.
#[test]
fn a_ring_that_was_not_cast_protects_nobody() {
    let (mut game, _ring) = staged();
    game.players[0].hand.clear();

    game.put_onto_battlefield(PlayerId::One, cards::THE_ONE_RING)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(game.stack.is_empty(), "the trigger never went on the stack");
    assert!(
        bolt_can_hit_player_one(&game),
        "and nothing protects its controller",
    );
}

#[test]
fn a_copy_of_the_ring_spell_was_not_cast() {
    let (mut game, ring) = staged();
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ring))
        .expect("the Ring is castable");
    game.apply(PlayerId::One, cast).expect("the Ring is cast");
    let original = game.stack.last().expect("the Ring spell exists").clone();
    game.push_copy_with_colors(original, PlayerId::One, Vec::new(), None);

    pass_priority_pair(&mut game);

    let copy = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the spell copy resolved as a token copy");
    assert!(
        copy.cast.as_ref().is_some_and(|cast| !cast.was_cast()),
        "copied casting choices do not turn a spell copy into a cast spell",
    );
    assert_eq!(game.stack.len(), 1, "only the original Ring spell remains");
    assert!(
        game.pending_triggers.is_empty(),
        "the copy created no protection trigger"
    );
}

/// The counter goes on before the draw is counted, so the first activation
/// draws one and the second draws two.
#[test]
fn each_activation_draws_one_more_than_the_last() {
    let (mut game, ring) = staged();
    cast_the_ring(&mut game, ring);
    let hand = game.players[0].hand.len();

    let first = ring_activations(&game)
        .into_iter()
        .next()
        .expect("tapping it is free");
    game.apply(PlayerId::One, first).expect("it activates");
    settle(&mut game);

    assert_eq!(burden(&game), 1, "one burden counter");
    assert_eq!(game.players[0].hand.len(), hand + 1, "and one card");

    for permanent in &mut game.battlefield {
        permanent.tapped = false;
    }
    let second = ring_activations(&game)
        .into_iter()
        .next()
        .expect("untapped, it goes again");
    game.apply(PlayerId::One, second).expect("it activates");
    settle(&mut game);

    assert_eq!(burden(&game), 2, "two burden counters");
    assert_eq!(
        game.players[0].hand.len(),
        hand + 3,
        "and two more cards for the second activation",
    );
}

/// The upkeep charges one life for every burden counter.
#[test]
fn the_upkeep_charges_one_life_for_each_burden_counter() {
    let (mut game, ring) = staged();
    cast_the_ring(&mut game, ring);
    for permanent in &mut game.battlefield {
        if permanent.card.definition == cards::THE_ONE_RING {
            permanent.add_counters(CounterKind::named("burden"), 3);
        }
    }
    let before = game.players[0].life;

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].life,
        before - 3,
        "one life for each of the three counters",
    );
}

/// Indestructible: the Bolt that would kill it does nothing.
#[test]
fn it_is_indestructible() {
    let (mut game, ring) = staged();
    cast_the_ring(&mut game, ring);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::THE_ONE_RING)
        .expect("it resolved");

    assert!(
        game.has_indestructible(permanent),
        "nothing destroys The One Ring",
    );
}
