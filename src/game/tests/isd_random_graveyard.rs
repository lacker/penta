//! Replay-stable random graveyard returns from Innistrad.

use super::*;

fn ghoulraiser_result(seed: u64) -> CardDefinitionId {
    let mut game = ready_game_with_seed(seed);
    game.players[0].graveyard = vec![
        card(90_001, cards::DIREGRAF_GHOUL, PlayerId::One),
        card(90_002, cards::WALKING_CORPSE, PlayerId::One),
        card(90_003, cards::GRIZZLY_BEARS, PlayerId::One),
    ];

    game.put_onto_battlefield(PlayerId::One, cards::GHOULRAISER)
        .expect("Ghoulraiser is cataloged");
    drain_pending(&mut game);

    assert!(
        game.pending_decisions.is_empty(),
        "the RNG, not a player, chooses"
    );
    assert_eq!(game.players[0].hand.len(), 1, "exactly one Zombie returned");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "a non-Zombie is not eligible",
    );
    game.players[0].hand[0].definition
}

fn woodland_sleuth_result(seed: u64) -> CardDefinitionId {
    let mut game = ready_game_with_seed(seed);
    game.players[0].graveyard = vec![
        card(90_011, cards::GRIZZLY_BEARS, PlayerId::One),
        card(90_012, cards::SAVANNAH_LIONS, PlayerId::One),
        card(90_013, cards::LIGHTNING_BOLT, PlayerId::One),
    ];
    let fodder = creature(90_014, cards::SEDGE_TROLL, PlayerId::Two);
    let fodder_id = fodder.card.id;
    game.battlefield.push(fodder);
    game.destroy_permanent(fodder_id);
    drain_pending(&mut game);

    game.put_onto_battlefield(PlayerId::One, cards::WOODLAND_SLEUTH)
        .expect("Woodland Sleuth is cataloged");
    drain_pending(&mut game);

    assert!(
        game.pending_decisions.is_empty(),
        "the RNG, not a player, chooses"
    );
    assert_eq!(game.players[0].hand.len(), 1, "one creature returned");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "a noncreature card is not eligible",
    );
    game.players[0].hand[0].definition
}

#[test]
fn ghoulraiser_returns_only_a_zombie_and_replays_the_same_seed() {
    let first = ghoulraiser_result(0x0047_584f_554c);
    let replay = ghoulraiser_result(0x0047_584f_554c);

    assert_eq!(
        first, replay,
        "the recorded seed reproduces the random return"
    );
    assert!(matches!(
        first,
        cards::DIREGRAF_GHOUL | cards::WALKING_CORPSE
    ));
}

#[test]
fn ghoulraiser_does_nothing_when_no_zombie_card_is_in_the_graveyard() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(90_010, cards::GRIZZLY_BEARS, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::GHOULRAISER)
        .expect("Ghoulraiser is cataloged");
    drain_pending(&mut game);

    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn woodland_sleuth_returns_only_a_creature_and_replays_the_same_seed() {
    let first = woodland_sleuth_result(0x0053_4c45_5554);
    let replay = woodland_sleuth_result(0x0053_4c45_5554);

    assert_eq!(first, replay, "the recorded seed reproduces the return");
    assert!(matches!(
        first,
        cards::GRIZZLY_BEARS | cards::SAVANNAH_LIONS
    ));
}

#[test]
fn woodland_sleuth_does_not_choose_on_a_quiet_turn() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(90_015, cards::GRIZZLY_BEARS, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::WOODLAND_SLEUTH)
        .expect("Woodland Sleuth is cataloged");
    drain_pending(&mut game);

    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn charmbreaker_upkeep_returns_only_an_instant_or_sorcery() {
    let mut game = ready_game_with_seed(0x0043_4841_524d);
    game.players[0].graveyard = vec![
        card(90_020, cards::LIGHTNING_BOLT, PlayerId::One),
        card(90_021, cards::DEMONIC_TUTOR, PlayerId::One),
        card(90_022, cards::GRIZZLY_BEARS, PlayerId::One),
    ];
    game.put_onto_battlefield(PlayerId::One, cards::CHARMBREAKER_DEVILS)
        .expect("Charmbreaker Devils is cataloged");
    drain_pending(&mut game);

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 1);
    assert!(matches!(
        game.players[0].hand[0].definition,
        cards::LIGHTNING_BOLT | cards::DEMONIC_TUTOR
    ));
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "a creature card is not eligible",
    );
}

fn make_a_wish_result(seed: u64) -> Vec<CardDefinitionId> {
    let mut game = ready_game_with_seed(seed);
    game.players[0].graveyard = vec![
        card(90_040, cards::GRIZZLY_BEARS, PlayerId::One),
        card(90_041, cards::SAVANNAH_LIONS, PlayerId::One),
        card(90_042, cards::LIGHTNING_BOLT, PlayerId::One),
    ];
    let wish = card(90_043, cards::MAKE_A_WISH, PlayerId::One);
    let wish_id = wish.id;
    game.players[0].hand.push(wish);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;

    cast_from_hand(&mut game, wish_id);

    assert!(
        game.pending_decisions.is_empty(),
        "the RNG, not a player, chooses"
    );
    assert_eq!(game.players[0].hand.len(), 2, "exactly two cards returned");
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|card| card.definition != cards::MAKE_A_WISH)
            .count(),
        1,
        "one of the three original graveyard cards remains",
    );
    game.players[0]
        .hand
        .iter()
        .map(|card| card.definition)
        .collect()
}

#[test]
fn make_a_wish_returns_two_distinct_cards_and_replays_the_same_seed() {
    let first = make_a_wish_result(0x004d_414b_4557);
    let replay = make_a_wish_result(0x004d_414b_4557);

    assert_eq!(first, replay, "the recorded seed reproduces both returns");
    assert_ne!(first[0], first[1], "selection is without replacement");
}

#[test]
fn make_a_wish_returns_the_only_available_card() {
    let mut game = ready_game();
    let graveyard_card = card(90_044, cards::GRIZZLY_BEARS, PlayerId::One);
    game.players[0].graveyard.push(graveyard_card);
    let wish = card(90_045, cards::MAKE_A_WISH, PlayerId::One);
    let wish_id = wish.id;
    game.players[0].hand.push(wish);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;

    cast_from_hand(&mut game, wish_id);

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::GRIZZLY_BEARS);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(game.players[0].graveyard[0].definition, cards::MAKE_A_WISH);
}

fn cast_from_hand(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("the spell is castable");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

fn power(game: &Game, id: GameObjectId) -> Option<i16> {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("Charmbreaker Devils remains on the battlefield");
    game.power(permanent)
}

#[test]
fn charmbreaker_gets_four_power_per_instant_or_sorcery_cast_until_end_of_turn() {
    let mut game = ready_game();
    let devils = game
        .put_onto_battlefield(PlayerId::One, cards::CHARMBREAKER_DEVILS)
        .expect("Charmbreaker Devils is cataloged");
    drain_pending(&mut game);

    let bear = card(90_030, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_ritual = card(90_031, cards::DARK_RITUAL, PlayerId::One);
    let second_ritual = card(90_032, cards::DARK_RITUAL, PlayerId::One);
    let (bear_id, first_id, second_id) = (bear.id, first_ritual.id, second_ritual.id);
    game.players[0].hand = vec![bear, first_ritual, second_ritual];
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.black = 1;

    cast_from_hand(&mut game, bear_id);
    assert_eq!(
        power(&game, devils),
        Some(4),
        "a creature spell does not trigger it"
    );

    cast_from_hand(&mut game, first_id);
    assert_eq!(power(&game, devils), Some(8));
    cast_from_hand(&mut game, second_id);
    assert_eq!(
        power(&game, devils),
        Some(12),
        "the two triggers accumulate"
    );

    game.finish_cleanup();
    assert_eq!(power(&game, devils), Some(4), "both boosts expire together");
}

#[test]
fn random_returns_compose_selection_with_a_zone_move() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::GHOULRAISER,
        cards::CHARMBREAKER_DEVILS,
        cards::WOODLAND_SLEUTH,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        let effect = card.rules.ability_clauses()[0]
            .declarative_effect()
            .expect("the random return is declarative");
        let EffectDef::Sequence([selection, movement]) = effect else {
            panic!("{} should compose selection and movement", card.name);
        };
        let EffectDef::BindOutput { effect, binding } = *selection else {
            panic!("{} should bind the random selection", card.name);
        };
        let EffectDef::SelectAtRandomFromZone { source, .. } = *effect else {
            panic!("{} should select randomly before moving", card.name);
        };
        assert_eq!(source, ZoneKind::Graveyard);
        assert_eq!(
            binding,
            crate::card::EffectOutputBindingDef::Objects("random_graveyard_cards")
        );
        let EffectDef::MoveToZone { object, zone, .. } = *movement else {
            panic!("{} should use an ordinary zone move", card.name);
        };
        assert_eq!(zone, ZoneKind::Hand);
        assert_eq!(
            object,
            EffectRecipientDef::objects(ObjectSetDef::NamedBinding(
                &crate::card::EffectBindingLabelDef("random_graveyard_cards"),
            ))
        );
    }
}
