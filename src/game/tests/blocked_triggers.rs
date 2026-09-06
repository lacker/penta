//! What happens the moment a block is declared, before any damage. Four of
//! these read the blocker itself and one reads only how many there were, so
//! together they cover both halves of the becomes-blocked event: the
//! triggering object a clause can act on, and the count rampage was written
//! against -- here running the other way, as a penalty for gang-blocking.

use super::*;

/// `attacker` attacking, blocked by each of `blockers`.
fn blocked_by(attacker: CardDefinitionId, blockers: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.step = Step::DeclareBlockers;
    let mut threat = creature(10_000, attacker, PlayerId::One);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(threat);
    for (index, definition) in blockers.iter().enumerate() {
        let mut blocker = creature(
            11_000 + u32::try_from(index).expect("blocker index fits"),
            *definition,
            PlayerId::Two,
        );
        blocker.blocking = vec![GameObjectId(10_000)];
        game.battlefield.push(blocker);
    }
    game
}

fn finish(game: &mut Game) {
    game.finish_declaring_blockers();
    game.finish_rules_procedure();
    for _ in 0..8 {
        if game.stack.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while the block trigger resolves");
    }
    drain_pending(game);
}

fn attacker_stats(game: &Game) -> (i16, i16) {
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("the attacker is on the battlefield");
    let stats = game.creature_stats(attacker).expect("a creature");
    (stats.power, stats.toughness)
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == ObjectKind::Card(definition))
}

/// Rampage read backwards. One blocker is still "the first", so the Wurm is
/// only punished for the help that came after it.
#[test]
fn jungle_wurm_shrinks_for_every_blocker_past_the_first() {
    let mut alone = blocked_by(cards::JUNGLE_WURM, &[cards::SAVANNAH_LIONS]);
    finish(&mut alone);
    assert_eq!(
        attacker_stats(&alone),
        (5, 5),
        "one blocker costs the Wurm nothing"
    );

    let mut ganged = blocked_by(
        cards::JUNGLE_WURM,
        &[
            cards::SAVANNAH_LIONS,
            cards::SAVANNAH_LIONS,
            cards::SAVANNAH_LIONS,
        ],
    );
    finish(&mut ganged);
    assert_eq!(
        attacker_stats(&ganged),
        (3, 3),
        "two blockers beyond the first, at -1/-1 each"
    );
}

/// The Reaper reads the blocker's colour, so the same block is fatal or free
/// depending on who threw themselves in front of it.
#[test]
fn the_phyrexian_reaper_eats_a_green_blocker_and_spares_the_rest() {
    let mut green = blocked_by(cards::PHYREXIAN_REAPER, &[cards::GRIZZLY_BEARS]);
    finish(&mut green);
    assert!(
        !on_battlefield(&green, cards::GRIZZLY_BEARS),
        "a green creature that blocks the Reaper is destroyed"
    );

    let mut white = blocked_by(cards::PHYREXIAN_REAPER, &[cards::SAVANNAH_LIONS]);
    finish(&mut white);
    assert!(
        on_battlefield(&white, cards::SAVANNAH_LIONS),
        "and a white one is left alone"
    );
}

/// The Warhounds do not kill the blocker; they charge the defender their next
/// draw for it, which means the card is still in the deck afterwards.
#[test]
fn elven_warhounds_put_the_blocker_back_on_top_of_the_library() {
    let mut game = blocked_by(cards::ELVEN_WARHOUNDS, &[cards::GRIZZLY_BEARS]);
    finish(&mut game);
    assert!(
        !on_battlefield(&game, cards::GRIZZLY_BEARS),
        "the blocker left the battlefield"
    );
    let library = &game.players[PlayerId::Two.index()].library;
    assert_eq!(
        library
            .last()
            .map(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        Some(true),
        "and it is the next card its owner draws"
    );
    assert!(
        !game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        "not destroyed on the way"
    );
}

/// The Lamprey leaves the counter on whichever creature blocked it, and the
/// counter outlives the combat.
#[test]
fn the_quagmire_lamprey_marks_its_blocker_with_a_counter() {
    let mut game = blocked_by(cards::QUAGMIRE_LAMPREY, &[cards::GRIZZLY_BEARS]);
    finish(&mut game);
    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS))
        .expect("a 2/2 survives a -1/-1 counter");
    let stats = game.creature_stats(bear).expect("a creature");
    assert_eq!(
        (stats.power, stats.toughness),
        (1, 1),
        "the Bears is a 1/1 for the rest of the game"
    );
}

/// The Gargadon's rent is charged for blocking as well as attacking, and the
/// half nobody remembers is the one tested here: it pays even on defence.
#[test]
fn lesser_gargadon_pays_a_land_for_blocking() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.step = Step::DeclareBlockers;
    let mut threat = creature(12_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(threat);
    let mut gargadon = creature(12_001, cards::LESSER_GARGADON, PlayerId::One);
    gargadon.entered_controller_turn = 0;
    gargadon.blocking = vec![GameObjectId(12_000)];
    game.battlefield.push(gargadon);
    for index in 0..2 {
        game.battlefield
            .push(creature(12_010 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.active_player = PlayerId::Two;

    let lands = |game: &Game| {
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == ObjectKind::Card(cards::MOUNTAIN))
            .count()
    };
    assert_eq!(lands(&game), 2, "two Mountains before the block");
    finish(&mut game);
    assert_eq!(
        lands(&game),
        1,
        "blocking costs a land just as attacking does"
    );
}
