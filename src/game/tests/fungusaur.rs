use super::*;
use crate::ImplementationStatus;

#[test]
fn simultaneous_combat_damage_gives_fungusaur_one_counter() {
    let mut game = ready_game();
    let mut fungusaur = creature(10_000, cards::FUNGUSAUR, PlayerId::One);
    fungusaur.add_counters(CounterKind::PlusOnePlusOne, 1);
    let fungusaur_id = fungusaur.card.id;

    let first = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::Two);
    let first_id = first.card.id;
    let second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield = vec![fungusaur, first, second];

    game.deal_damage_simultaneously(vec![
        DamageAssignment {
            source: Some(first_id),
            target: Some(Target::Permanent(fungusaur_id)),
            amount: 1,
            combat: true,
        },
        DamageAssignment {
            source: Some(second_id),
            target: Some(Target::Permanent(fungusaur_id)),
            amount: 1,
            combat: true,
        },
    ]);

    let triggers = game
        .pending_triggers
        .iter()
        .filter(|trigger| trigger.source.object == fungusaur_id)
        .collect::<Vec<_>>();
    assert_eq!(
        triggers.len(),
        1,
        "one damage event produces one Fungusaur trigger even with two sources",
    );
    assert_eq!(
        triggers[0].context.trigger.amount,
        Some(2),
        "the recipient-facing occurrence carries the event's total damage",
    );

    drain_pending(&mut game);

    let fungusaur = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == fungusaur_id)
        .expect("the initial counter lets Fungusaur survive two damage");
    assert_eq!(fungusaur.counters(CounterKind::PlusOnePlusOne), 2);
    assert_eq!(
        game.catalog
            .get(cards::FUNGUSAUR)
            .expect("Fungusaur is cataloged")
            .rules
            .implementation_status(),
        ImplementationStatus::Complete,
    );
}
