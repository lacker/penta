//! Landwalk as one keyword parameterized by land type.
//!
//! CR 702.14 is a single rule: the creature cannot be blocked as long as the
//! defending player controls a land of the named type. The engine used to
//! carry Mountainwalk and Forestwalk as separate keywords with the blocking
//! rule spelled out once per variant, which is why the other three could not
//! be printed. These tests drive the rule through the blocker list a seat is
//! actually offered.

use super::*;
use crate::card::BasicLandType;

fn walk_game(walker: CardDefinitionId, defender_land: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, walker, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    // Something that could block if the walk did not apply.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, defender_land, PlayerId::Two));
    (game, attacker_id)
}

fn can_be_blocked(game: &Game, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::DeclareBlocker { attacker: a, .. } if *a == attacker),
    )
}

/// Each printed variant reads its own land type, and only that type.
#[test]
fn every_printed_landwalk_variant_is_stopped_only_by_its_own_land() {
    for (walker, matching, other) in [
        (cards::BOG_WRAITH, cards::SWAMP, cards::ISLAND),
        (cards::RIGHTEOUS_AVENGERS, cards::PLAINS, cards::SWAMP),
        (cards::DEVOURING_DEEP, cards::ISLAND, cards::MOUNTAIN),
        (cards::SEGOVIAN_LEVIATHAN, cards::ISLAND, cards::FOREST),
        (cards::LOST_SOUL, cards::SWAMP, cards::PLAINS),
        (cards::MARSH_GOBLINS, cards::SWAMP, cards::FOREST),
    ] {
        game_declares_blockers_only_without_the_land(walker, matching, other);
    }
}

fn game_declares_blockers_only_without_the_land(
    walker: CardDefinitionId,
    matching: CardDefinitionId,
    other: CardDefinitionId,
) {
    let (blocked, attacker) = walk_game(walker, other);
    assert!(
        can_be_blocked(&blocked, attacker),
        "an unrelated land should not turn on landwalk",
    );

    let (unblockable, attacker) = walk_game(walker, matching);
    assert!(
        !can_be_blocked(&unblockable, attacker),
        "the named land should make the attacker unblockable",
    );
}

/// The rule reads the land's current types rather than its printed name, so a
/// dual land turns on the walk that matches either half.
#[test]
fn landwalk_reads_effective_land_types_rather_than_card_names() {
    let (mut game, attacker) = walk_game(cards::BOG_WRAITH, cards::MOUNTAIN);
    assert!(can_be_blocked(&game, attacker));

    // Badlands is a Swamp Mountain, so the same board now stops blocking.
    game.battlefield
        .retain(|permanent| permanent.card.definition != cards::MOUNTAIN);
    game.battlefield
        .push(creature(10_003, cards::BADLANDS, PlayerId::Two));
    assert!(
        !can_be_blocked(&game, attacker),
        "a dual land counts as both of its types"
    );
}

/// One creature can carry more than one landwalk, and any single match is
/// enough. The old shape could only express this by repeating the rule.
#[test]
fn several_landwalks_on_one_creature_each_stand_on_their_own() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BOG_WRAITH, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    attacker
        .temporary_keywords
        .push(KeywordAbility::Landwalk(BasicLandType::Island));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::ISLAND, PlayerId::Two));

    assert!(
        !can_be_blocked(&game, attacker_id),
        "the granted islandwalk applies even though the printed walk does not"
    );
}

/// A lord grants the walk to everything it names, and the grant behaves like
/// a printed one: the same blocking rule reads it.
#[test]
fn a_granted_landwalk_makes_the_recipient_unblockable() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.battlefield
        .push(creature(10_000, cards::LORD_OF_ATLANTIS, PlayerId::One));
    let mut merfolk = creature(10_001, cards::LORD_OF_ATLANTIS, PlayerId::One);
    merfolk.attacking = true;
    merfolk.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let merfolk_id = merfolk.card.id;
    game.battlefield.push(merfolk);
    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two));

    game.battlefield
        .push(creature(10_003, cards::MOUNTAIN, PlayerId::Two));
    assert!(
        can_be_blocked(&game, merfolk_id),
        "without an Island the granted islandwalk does nothing"
    );

    game.battlefield
        .retain(|permanent| permanent.card.definition != cards::MOUNTAIN);
    game.battlefield
        .push(creature(10_004, cards::ISLAND, PlayerId::Two));
    assert!(
        !can_be_blocked(&game, merfolk_id),
        "the other Merfolk has islandwalk from the lord"
    );
}

/// An Aura grants the walk to what it enchants, which is the same grant
/// mechanism read through an attachment rather than a lord's query.
#[test]
fn an_aura_grants_landwalk_to_the_creature_it_enchants() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::ISLAND, PlayerId::Two));
    assert!(
        can_be_blocked(&game, attacker_id),
        "the unenchanted attacker is blockable"
    );

    let mut oil = creature(10_003, cards::FISHLIVER_OIL, PlayerId::One);
    oil.attached_to = Some(attacker_id);
    game.battlefield.push(oil);
    assert!(
        !can_be_blocked(&game, attacker_id),
        "the Aura's islandwalk applies to the creature it enchants"
    );
}

/// The Legends cycle turns one landwalk off for blocking. The keyword itself
/// is untouched, so this is a blocking rule rather than ability removal.
#[test]
fn a_negating_enchantment_lets_that_one_walk_be_blocked() {
    let (mut game, attacker) = walk_game(cards::BOG_WRAITH, cards::SWAMP);
    assert!(!can_be_blocked(&game, attacker), "swampwalk applies");

    // Crevasse names mountainwalk, so a swampwalker is unaffected.
    game.battlefield
        .push(creature(10_010, cards::CREVASSE, PlayerId::Two));
    assert!(
        !can_be_blocked(&game, attacker),
        "a different walk is not the one turned off"
    );

    game.battlefield
        .push(creature(10_011, cards::QUAGMIRE, PlayerId::Two));
    assert!(
        can_be_blocked(&game, attacker),
        "Quagmire names swampwalk, so it can be blocked anyway"
    );
}

/// It works from either side of the table, and from a creature as well as an
/// enchantment: Ur-Drago carries the same clause Quagmire does.
#[test]
fn the_negation_is_read_from_any_permanent_on_the_battlefield() {
    let (mut game, attacker) = walk_game(cards::BOG_WRAITH, cards::SWAMP);
    game.battlefield
        .push(creature(10_010, cards::UR_DRAGO, PlayerId::One));
    assert!(
        can_be_blocked(&game, attacker),
        "the attacking player's own Ur-Drago still turns swampwalk off"
    );
}

#[test]
fn losing_the_negation_ability_restores_landwalk() {
    let (mut game, attacker) = walk_game(cards::BOG_WRAITH, cards::SWAMP);
    let ur_drago = creature(10_010, cards::UR_DRAGO, PlayerId::One);
    let ur_drago_id = ur_drago.card.id;
    game.battlefield.push(ur_drago);
    assert!(
        can_be_blocked(&game, attacker),
        "Ur-Drago initially turns swampwalk off"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        ur_drago_id,
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );

    assert!(
        !can_be_blocked(&game, attacker),
        "removing Ur-Drago's abilities restores swampwalk"
    );
}

/// The keyword survives; only blocking ignores it.
#[test]
fn negating_a_walk_does_not_remove_the_keyword() {
    let (mut game, _) = walk_game(cards::BOG_WRAITH, cards::SWAMP);
    game.battlefield
        .push(creature(10_010, cards::QUAGMIRE, PlayerId::Two));
    let wraith = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BOG_WRAITH)
        .expect("the Wraith is on the battlefield");
    assert!(
        game.permanent_has_executable_keyword(
            wraith,
            KeywordAbility::Landwalk(BasicLandType::Swamp)
        ),
        "the creature still has swampwalk for everything that reads it"
    );
}

#[test]
fn legendary_landwalk_reads_the_land_supertype() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut livonya = creature(10_000, cards::LIVONYA_SILONE, PlayerId::One);
    livonya.attacking = true;
    livonya.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let livonya_id = livonya.card.id;
    game.battlefield.push(livonya);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));

    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::Two));
    assert!(
        can_be_blocked(&game, livonya_id),
        "an ordinary land is not legendary"
    );

    game.battlefield
        .push(creature(10_003, cards::KARAKAS, PlayerId::Two));
    assert!(
        !can_be_blocked(&game, livonya_id),
        "a legendary land turns the walk on"
    );
}

/// Two identities whose audit lines named landwalk long after the keyword
/// landed. Neither needed engine work; what they needed was for someone to
/// re-read the line. The tests below drive what each one actually asks of the
/// keyword: reading it off another creature, and granting it for a turn.
mod follow_up {
    use super::*;

    fn assassin_game() -> (Game, GameObjectId) {
        let mut game = ready_game();
        let assassin = creature(10_000, cards::MERFOLK_ASSASSIN, PlayerId::One);
        let assassin_id = assassin.card.id;
        game.battlefield.push(assassin);
        (game, assassin_id)
    }

    fn destroy_targets(game: &Game, source: GameObjectId) -> Vec<GameObjectId> {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source: actual,
                    targets,
                    ..
                } if actual == source => targets
                    .iter()
                    .flat_map(crate::casting::TargetSelection::targets)
                    .find_map(|target| match target {
                        Target::Permanent(id) => Some(*id),
                        _ => None,
                    }),
                _ => None,
            })
            .collect()
    }

    /// The predicate is "with islandwalk", not "is a Merfolk" and not "is
    /// blue": a printed islandwalker is a legal target and a creature without
    /// the keyword is never offered as one.
    #[test]
    fn merfolk_assassin_only_targets_creatures_that_have_islandwalk() {
        let (mut game, assassin_id) = assassin_game();
        let walker = creature(10_001, cards::DEVOURING_DEEP, PlayerId::Two);
        let walker_id = walker.card.id;
        game.battlefield.push(walker);
        game.battlefield
            .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two));

        assert_eq!(
            destroy_targets(&game, assassin_id),
            vec![walker_id],
            "the Fish walks on islands; the Lions do not"
        );

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == assassin_id)
            })
            .expect("the ability is offered");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == walker_id),
            "the islandwalker was destroyed"
        );
    }

    /// A Merfolk wearing a Lord of Atlantis grant is unblockable across an
    /// Island, and Merfolk Assassin can target it. The blocking rules and
    /// target legality read one ability set, so a static grant cannot make a
    /// creature unblockable and untargetable at the same time.
    #[test]
    fn merfolk_assassin_targets_islandwalk_a_lord_handed_out() {
        let (mut game, assassin_id) = assassin_game();
        let merfolk = creature(10_001, cards::MERFOLK_ASSASSIN, PlayerId::Two);
        let merfolk_id = merfolk.card.id;
        game.battlefield.push(merfolk);
        game.battlefield
            .push(creature(10_002, cards::LORD_OF_ATLANTIS, PlayerId::Two));

        let granted = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == merfolk_id)
            .expect("the Merfolk is on the battlefield");
        assert!(
            game.permanent_has_executable_keyword(
                granted,
                KeywordAbility::Landwalk(BasicLandType::Island)
            ),
            "the blocking rules see the Lord's grant"
        );
        assert_eq!(
            destroy_targets(&game, assassin_id),
            vec![assassin_id, merfolk_id],
            "and so does target legality, for every Merfolk the Lord hands a walk to"
        );
    }

    /// Wormwood Treefolk grants itself a walk for the turn and pays two damage
    /// on top of the mana. Both halves land in one resolution.
    #[test]
    fn wormwood_treefolk_buys_a_walk_and_takes_the_damage() {
        let mut game = ready_game();
        let treefolk = creature(10_000, cards::WORMWOOD_TREEFOLK, PlayerId::One);
        let treefolk_id = treefolk.card.id;
        game.battlefield.push(treefolk);
        game.players[PlayerId::One.index()].mana_pool.green = 2;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == treefolk_id)
            })
            .expect("the green clause is affordable");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        let walker = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == treefolk_id)
            .expect("the Treefolk survived its own two damage");
        assert!(
            game.has_forestwalk(walker),
            "the clause granted forestwalk for the turn"
        );
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 2,
            "and charged its controller two damage for it"
        );
    }
}
