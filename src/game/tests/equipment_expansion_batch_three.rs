//! The third ISD–M14 Equipment completion batch: a granted ability that
//! sacrifices and attributes damage to its granting Equipment, and an attack
//! trigger that reveals/mills the defending player's library into a bound
//! same-resolution count.

use super::*;
use crate::{CardArt, CardPrintingId};

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn attached_board(
    equipment: CardDefinitionId,
    host: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let host = creature(10_100, host, PlayerId::One);
    let host_id = host.card.id;
    let mut equipment = creature(10_000, equipment, PlayerId::One);
    let equipment_id = equipment.card.id;
    equipment.attached_to = Some(host_id);
    game.battlefield.extend([equipment, host]);
    (game, equipment_id, host_id)
}

#[test]
fn blazing_torch_sacrifices_the_exact_granter_and_keeps_it_as_damage_source() {
    let (mut game, torch_id, host_id) = attached_board(cards::BLAZING_TORCH, cards::GRIZZLY_BEARS);
    let mut spare = creature(10_001, cards::BLAZING_TORCH, PlayerId::One);
    let spare_id = spare.card.id;
    spare.attached_to = Some(host_id);
    let victim = creature(10_200, cards::SERRA_ANGEL, PlayerId::Two);
    let victim_id = victim.card.id;
    // Put the other granting copy first so neither battlefield order nor
    // identical printed text can accidentally choose the cost's object.
    game.battlefield.insert(0, spare);
    game.battlefield.push(victim);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Granted {
                        source: granting_source,
                        ..
                    },
                    targets,
                    cost_objects,
                    ..
                } if *source == host_id
                    && *granting_source == torch_id
                    && cost_objects.is_empty()
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim_id))
            )
        })
        .expect("the equipped creature receives this Torch's targeted activation");

    game.apply(PlayerId::One, activation)
        .expect("the granted activation is legal");
    assert!(permanent(&game, host_id).tapped, "the creature pays {{T}}");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != torch_id),
        "the exact Torch that granted the ability is sacrificed",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == spare_id),
        "another Torch cannot be substituted for the named cost",
    );

    drain_pending(&mut game);
    let victim = permanent(&game, victim_id);
    assert_eq!(victim.damage, 2);
    assert_eq!(
        victim.damage_sources,
        vec![torch_id],
        "damage attribution uses the sacrificed Torch's retired identity, not the creature carrying the ability",
    );
    assert!(game.events.iter().any(|event| {
        matches!(
            event,
            GameEvent::AbilityActivated { source, .. } if *source == host_id
        )
    }));
}

#[test]
fn blazing_torch_evasion_and_split_control_read_the_live_attachment() {
    let (mut game, torch_id, host_id) = attached_board(cards::BLAZING_TORCH, cards::GRIZZLY_BEARS);
    let vampire = creature(10_200, cards::STROMKIRK_NOBLE, PlayerId::Two);
    let zombie = creature(10_201, cards::DIREGRAF_GHOUL, PlayerId::Two);
    let bear = creature(10_202, cards::GRIZZLY_BEARS, PlayerId::Two);
    assert!(game.blocking_is_prevented(permanent(&game, host_id), &vampire));
    assert!(game.blocking_is_prevented(permanent(&game, host_id), &zombie));
    assert!(!game.blocking_is_prevented(permanent(&game, host_id), &bear));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == torch_id)
        .expect("the Torch remains attached")
        .controller = PlayerId::Two;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == host_id)
        }),
        "the creature's controller cannot sacrifice an Equipment they do not control",
    );
}

fn attack_with_blade(defending_library: Vec<CardInstance>) -> (Game, GameObjectId, GameObjectId) {
    let (mut game, _blade_id, attacker_id) =
        attached_board(cards::TREPANATION_BLADE, cards::GRIZZLY_BEARS);
    game.players[PlayerId::One.index()].library =
        vec![card(30_000, cards::SERRA_ANGEL, PlayerId::One)];
    game.players[PlayerId::Two.index()].library = defending_library;

    let other = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let other_id = other.card.id;
    let mut walker = creature(10_200, cards::DOMRI_RADE, PlayerId::Two);
    let walker_id = walker.card.id;
    walker.add_counters(CounterKind::Loyalty, 3);
    game.battlefield.extend([other, walker]);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(attacker_id, AttackDefender::Planeswalker(walker_id));
    game.declare_attacker(other_id, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "only the equipped attacker creates a Blade trigger",
    );
    assert_eq!(
        game.pending_triggers[0].context.trigger.event_player,
        Some(PlayerId::Two),
        "the attack freezes the chosen planeswalker's controller as defending player",
    );
    drain_pending(&mut game);
    (game, attacker_id, other_id)
}

#[test]
fn trepanation_blade_reveals_and_mills_defending_player_then_uses_bound_count() {
    // Library storage is bottom-to-top, so these are revealed Bear, Bolt,
    // Forest. The matching land is included in both the mill and the count.
    let library = vec![
        card(20_002, cards::FOREST, PlayerId::Two),
        card(20_001, cards::LIGHTNING_BOLT, PlayerId::Two),
        card(20_000, cards::GRIZZLY_BEARS, PlayerId::Two),
    ];
    let (game, attacker_id, other_id) = attack_with_blade(library);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "the Equipment controller's library is untouched",
    );
    assert!(game.players[PlayerId::Two.index()].library.is_empty());
    assert_eq!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::GRIZZLY_BEARS, cards::LIGHTNING_BOLT, cards::FOREST],
        "all three revealed cards, including the land, are milled in reveal order",
    );
    assert_eq!(game.power(permanent(&game, attacker_id)), Some(5));
    assert_eq!(
        game.power(permanent(&game, other_id)),
        Some(2),
        "the bound-count continuation pumps only the triggering creature",
    );
    assert_eq!(
        game.events
            .iter()
            .filter_map(|event| match event {
                GameEvent::CardRevealed {
                    player: PlayerId::Two,
                    definition,
                    ..
                } => Some(*definition),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![cards::GRIZZLY_BEARS, cards::LIGHTNING_BOLT, cards::FOREST],
        "every passed and matching card emits one public reveal event",
    );
}

#[test]
fn trepanation_blade_counts_every_card_when_no_land_exists_and_zero_when_empty() {
    for (library, expected_bonus, expected_reveals) in [
        (
            vec![
                card(21_001, cards::LIGHTNING_BOLT, PlayerId::Two),
                card(21_000, cards::GRIZZLY_BEARS, PlayerId::Two),
            ],
            2,
            2,
        ),
        (Vec::new(), 0, 0),
    ] {
        let (game, attacker_id, _) = attack_with_blade(library);
        assert_eq!(
            game.power(permanent(&game, attacker_id)),
            Some(2 + expected_bonus),
        );
        assert_eq!(
            game.players[PlayerId::Two.index()].graveyard.len(),
            expected_reveals,
        );
        assert_eq!(
            game.events
                .iter()
                .filter(|event| {
                    matches!(
                        event,
                        GameEvent::CardRevealed {
                            player: PlayerId::Two,
                            ..
                        }
                    )
                })
                .count(),
            expected_reveals,
        );
    }
}

#[test]
fn blazing_torch_keeps_its_printing_identity() {
    let catalog = poc::catalog().expect("catalog builds");
    let torch = catalog
        .get(cards::BLAZING_TORCH)
        .expect("Blazing Torch is cataloged");
    assert_eq!(torch.debut_set, CardSet::Zendikar);
    assert_eq!(
        torch.art,
        Some(CardArt::new(
            "1e9d1ff2-9ce3-4737-af1d-9fc82e4dffe6",
            "Vance Kovacs",
        )),
    );
    assert!(
        catalog
            .get_printing(CardPrintingId::new(
                cards::BLAZING_TORCH,
                CardSet::Innistrad,
            ))
            .is_some(),
        "ISD 216 remains an indexed reprint of the ZEN identity",
    );
}
