//! Ivora, Insatiable Heir and the Blood token her shared trigger creates.

use super::*;

/// Ivora's two clauses feed each other: the Blood she makes is spent by
/// discarding, and the discard is what grows her.
#[test]
fn ivora_makes_blood_on_arrival_and_grows_on_any_discard() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    let ivora = game
        .put_onto_battlefield(PlayerId::One, cards::IVORA_INSATIABLE_HEIR)
        .expect("cataloged");
    drain_pending(&mut game);

    let blood = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::blood()))
        .expect("entering made a Blood token")
        .card
        .id;
    let size = |game: &Game| {
        let ivora = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == ivora)
            .expect("she is still there");
        (game.power(ivora), game.toughness(ivora))
    };
    assert_eq!(size(&game), (Some(1), Some(1)));

    // Spending the Blood discards a card, and that discard is a discard.
    game.players[PlayerId::One.index()]
        .hand
        .push(card(69_000, cards::FOREST, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == blood))
        .expect("the Blood token can be spent");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(
        size(&game),
        (Some(2), Some(2)),
        "the discard paid as a cost still grows her",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blood),
        "and the token sacrificed itself to do it",
    );
}

/// Combat damage is the other way in, and it is the same printed ability --
/// which is why it has to be combat damage rather than any damage at all.
#[test]
fn ivora_makes_a_second_blood_only_when_she_connects_in_combat() {
    let mut game = ready_game();
    game.battlefield.clear();
    let ivora = creature(69_100, cards::IVORA_INSATIABLE_HEIR, PlayerId::One);
    let ivora_id = ivora.card.id;
    game.battlefield.push(ivora);

    let bloods = |game: &Game| {
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(permanent, tokens::blood()))
            .count()
    };

    game.damage_target_from(Some(ivora_id), Some(Target::Player(PlayerId::Two)), 1);
    drain_pending(&mut game);
    assert_eq!(
        bloods(&game),
        0,
        "damage that is not combat damage does nothing"
    );

    game.step = Step::DeclareAttackers;
    game.declare_attacker(ivora_id, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);
    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(bloods(&game), 1, "connecting in combat makes another Blood");
}
