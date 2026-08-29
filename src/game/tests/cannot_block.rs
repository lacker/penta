//! The blocker's own prohibition.
//!
//! The vocabulary had only the attacker-side restriction, so "this creature
//! can't block" and "target creature can't block this turn" had no shape at
//! all. These drive the prohibition through the blocker list a seat is
//! offered, both as a printed static and as a rider a spell hands out.

use super::*;

fn combat(defender: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let blocker = creature(10_001, defender, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    (game, attacker_id, blocker_id)
}

fn can_block(game: &Game, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker: actual, .. } if *actual == blocker),
    )
}

#[test]
fn a_creature_that_cannot_block_is_not_offered_as_a_blocker() {
    let (game, _attacker_id, blocker_id) = combat(cards::VAMPIRE_INTERLOPER);
    assert!(
        !can_block(&game, blocker_id),
        "the printed restriction keeps it out of the blocker list"
    );
}

/// The same board with an ordinary creature, so the test above is measuring
/// the restriction rather than something else about the setup.
#[test]
fn an_ordinary_creature_is_offered() {
    let (game, _attacker_id, blocker_id) = combat(cards::SAVANNAH_LIONS);
    assert!(can_block(&game, blocker_id));
}

/// A spell hands the same prohibition out for the turn, which is the shape
/// the runtime boundary had to be widened for: every other blocking
/// restriction is continuous.
#[test]
fn a_spell_can_hand_out_the_prohibition_for_the_turn() {
    let (mut game, _attacker_id, blocker_id) = combat(cards::SAVANNAH_LIONS);
    assert!(can_block(&game, blocker_id));

    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    let clutches = card(10_002, cards::NIGHTBIRDS_CLUTCHES, PlayerId::One);
    let clutches_id = clutches.id;
    game.players[PlayerId::One.index()].hand.push(clutches);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == clutches_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(blocker_id))
            }
            _ => false,
        })
        .expect("the Clutches can name that creature");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    game.step = Step::DeclareBlockers;
    assert!(
        !can_block(&game, blocker_id),
        "the spell took its blocking away for the turn"
    );
}

/// The Ghoul's restriction names a creature type rather than the whole
/// blocker list, so it blocks anything that is not a Human.
#[test]
fn the_ghoul_blocks_everything_except_humans() {
    let (game, _attacker, ghoul) = combat(cards::HUNTED_GHOUL);
    assert!(can_block(&game, ghoul), "a Sedge Troll is no Human");
}

/// A creature the Cathar pointed at sits the combat out. Its trigger picks
/// the target as it goes on the stack, not as the Cathar is cast.
#[test]
fn the_cathar_sits_one_blocker_down() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let victim = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    let cathar = card(10_001, cards::FERVENT_CATHAR, PlayerId::One);
    let cathar_id = cathar.id;
    game.players[PlayerId::One.index()].hand.push(cathar);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.red = 1;
    pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == cathar_id))
        .expect("the Cathar is castable");
    game.apply(PlayerId::One, action)
        .expect("the Cathar is cast");
    drain_pending(&mut game);

    // The Lions are the only creature the trigger could have named.
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);

    assert!(!can_block(&game, victim_id), "it was told to stand down");
}

/// The Tern's restriction reads a keyword rather than a type, and reads it
/// off the attacker as it is now.
#[test]
fn the_tern_blocks_only_fliers() {
    let (game, _attacker, tern) = combat(cards::WELKIN_TERN);
    assert!(!can_block(&game, tern), "a Sedge Troll is earthbound");

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut flier = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    flier.attacking = true;
    flier.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(flier);
    let tern = creature(10_001, cards::WELKIN_TERN, PlayerId::Two);
    let tern_id = tern.card.id;
    game.battlefield.push(tern);

    assert!(can_block(&game, tern_id), "and an Angel is not");
}

/// Pacifism bars both declarations, which is how a card that says "can't
/// attack or block" is expressed: two prohibitions, not one combat ban.
#[test]
fn pacifism_bars_both_declarations() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    assert!(
        game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == bear_id)
                .expect("the Bears are there")
        )
    );

    let mut aura = creature(10_001, cards::PACIFISM, PlayerId::Two);
    aura.attached_to = Some(bear_id);
    game.battlefield.push(aura);

    assert!(
        !game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == bear_id)
                .expect("the Bears are there")
        ),
        "it cannot attack"
    );

    // And the same creature on the other side of a combat cannot block.
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(attacker);
    let blocks = game.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker, .. } if blocker == bear_id),
    );
    assert!(!blocks, "and it cannot block");
}

/// Crippling Blight shrinks and silences at once; Tormented Soul does the
/// same to both sides of a block.
#[test]
fn the_other_two_prohibitions_land_as_well() {
    let (game, _attacker, soul) = combat(cards::TORMENTED_SOUL);
    assert!(!can_block(&game, soul));

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let blight = creature(10_001, cards::CRIPPLING_BLIGHT, PlayerId::One);
    let blight_id = blight.card.id;
    game.battlefield.push(blight);
    let victim = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    assert!(can_block(&game, victim_id), "free before the Aura");

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == blight_id)
        .expect("the Blight is there")
        .attached_to = Some(victim_id);

    assert!(!can_block(&game, victim_id));
    assert_eq!(
        game.power(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == victim_id)
                .expect("the Bears are there")
        ),
        Some(1),
        "and it is a point smaller"
    );
    assert_ne!(attacker_id, victim_id);
}

/// The attacker's side, printed as a static rather than handed out for a
/// turn. Both forms had to exist for the same reason the blocker's side did:
/// the turn-scoped one is a resolving rider and this one holds while its
/// source does.
mod cannot_be_blocked {
    use super::*;

    fn attacking(definition: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
        let mut game = ready_game();
        game.step = Step::DeclareBlockers;
        game.attackers_declared = true;
        let mut attacker = creature(10_000, definition, PlayerId::One);
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        let attacker_id = attacker.card.id;
        game.battlefield.push(attacker);
        let blocker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        let blocker_id = blocker.card.id;
        game.battlefield.push(blocker);
        (game, attacker_id, blocker_id)
    }

    fn offers_block(game: &Game, attacker: GameObjectId) -> bool {
        game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::DeclareBlocker { attacker: a, .. } if *a == attacker),
        )
    }

    #[test]
    fn nothing_is_offered_as_a_blocker_for_it() {
        let (game, attacker_id, _blocker_id) = attacking(cards::ELUSIVE_KRASIS);
        assert!(
            !offers_block(&game, attacker_id),
            "a printed unblockable attacker takes no blockers"
        );
    }

    /// The same board with an ordinary attacker, so the test above measures
    /// the restriction rather than the setup.
    #[test]
    fn an_ordinary_attacker_can_be_blocked() {
        let (game, attacker_id, _blocker_id) = attacking(cards::SAVANNAH_LIONS);
        assert!(offers_block(&game, attacker_id));
    }
}
