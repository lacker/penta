//! The first ISD–M14 Equipment completion batch: a nonmana equip cost,
//! attachment-scoped characteristic changes, and triggers that watch the
//! creature currently carrying an Equipment.

use super::*;

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

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = permanent(game, id);
    (game.power(permanent), game.toughness(permanent))
}

fn attached_board(
    equipment: crate::ids::CardDefinitionId,
    host: crate::ids::CardDefinitionId,
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

fn attached_token_board(
    equipment: crate::ids::CardDefinitionId,
    host: TokenCharacteristics,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let host = token_permanent(10_100, host, PlayerId::One);
    let host_id = host.card.id;
    let mut equipment = creature(10_000, equipment, PlayerId::One);
    let equipment_id = equipment.card.id;
    equipment.attached_to = Some(host_id);
    game.battlefield.extend([equipment, host]);
    (game, equipment_id, host_id)
}

#[test]
fn demonmail_hauberk_sacrifices_a_creature_to_equip() {
    let mut game = ready();
    let hauberk = creature(10_000, cards::DEMONMAIL_HAUBERK, PlayerId::One);
    let hauberk_id = hauberk.card.id;
    let host = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let fodder = creature(10_200, cards::SAVANNAH_LIONS, PlayerId::One);
    let fodder_id = fodder.card.id;
    game.battlefield.extend([hauberk, host, fodder]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    cost_objects,
                    ..
                } if *source == hauberk_id
                    && cost_objects.as_slice() == [fodder_id]
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host_id))
            )
        })
        .expect("sacrificing the Lions can pay for equipping the Bears");

    game.apply(PlayerId::One, action)
        .expect("the equip activation is legal");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != fodder_id),
        "the chosen creature is sacrificed as the activation is paid",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the sacrificed nontoken reaches its owner's graveyard",
    );
    assert_eq!(
        permanent(&game, hauberk_id).attached_to,
        None,
        "equip still uses the stack",
    );

    drain_pending(&mut game);
    assert_eq!(permanent(&game, hauberk_id).attached_to, Some(host_id));
    assert_eq!(stats(&game, host_id), (Some(6), Some(4)));
}

#[test]
fn avacyns_collar_watches_only_its_equipped_human_die() {
    let (mut game, _collar, human_id) = attached_token_board(
        cards::AVACYNS_COLLAR,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
    );
    assert_eq!(stats(&game, human_id), (Some(2), Some(1)));
    assert!(
        game.permanent_has_executable_keyword(
            permanent(&game, human_id),
            KeywordAbility::Vigilance,
        ),
        "the equipped Human receives vigilance",
    );

    let bystander = token_permanent(
        10_200,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
        PlayerId::One,
    );
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);
    game.destroy_permanent(bystander_id);
    game.check_state_based_actions();
    drain_pending(&mut game);
    assert!(
        game.battlefield.iter().all(|permanent| !is_token_with(
            permanent,
            token_with_flying(tokens::creature(&["Spirit"], &[ManaColor::White], 1, 1))
        )),
        "an unequipped Human dying does not trigger the Collar",
    );

    game.destroy_permanent(human_id);
    game.check_state_based_actions();
    drain_pending(&mut game);
    let spirits = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_flying(tokens::creature(&["Spirit"], &[ManaColor::White], 1, 1)),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        spirits.len(),
        1,
        "the equipped Human's death creates one Spirit"
    );
    assert!(game.permanent_has_executable_keyword(spirits[0], KeywordAbility::Flying));
}

#[test]
fn angelic_armaments_adds_white_and_angel_without_replacing_characteristics() {
    let (mut game, armaments_id, bear_id) =
        attached_board(cards::ANGELIC_ARMAMENTS, cards::GRIZZLY_BEARS);
    let bear = permanent(&game, bear_id);
    assert_eq!((game.power(bear), game.toughness(bear)), (Some(4), Some(4)));
    assert!(game.permanent_has_executable_keyword(bear, KeywordAbility::Flying));
    let colors = game.permanent_colors(bear);
    assert!(
        colors[ManaColor::White.color_index().expect("white is a color")],
        "white is added",
    );
    assert!(
        colors[ManaColor::Green.color_index().expect("green is a color")],
        "the Bear remains green",
    );
    let subtypes = game.object_subtypes(bear_id);
    assert!(subtypes.contains(&"Angel"));
    assert!(subtypes.contains(&"Bear"));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == armaments_id)
        .expect("the Armaments remains")
        .attached_to = None;
    assert_eq!(stats(&game, bear_id), (Some(2), Some(2)));
    assert!(!game.object_subtypes(bear_id).contains(&"Angel"));
    assert!(
        !game.permanent_colors(permanent(&game, bear_id))
            [ManaColor::White.color_index().expect("white is a color")],
        "the added color leaves with the attachment",
    );
}

#[test]
fn moonsilver_spear_triggers_only_for_the_equipped_attacker() {
    let (mut game, _spear, equipped_id) =
        attached_board(cards::MOONSILVER_SPEAR, cards::GRIZZLY_BEARS);
    assert!(game.permanent_has_executable_keyword(
        permanent(&game, equipped_id),
        KeywordAbility::FirstStrike,
    ));
    let other = creature(10_200, cards::SAVANNAH_LIONS, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(equipped_id, AttackDefender::Player(PlayerId::Two));
    game.declare_attacker(other_id, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    let angels = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_flying(tokens::creature(&["Angel"], &[ManaColor::White], 4, 4)),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        angels.len(),
        1,
        "only the equipped attack raises the trigger"
    );
    assert!(!angels[0].attacking, "the created Angel is not attacking");
    assert!(game.permanent_has_executable_keyword(angels[0], KeywordAbility::Flying));
}
