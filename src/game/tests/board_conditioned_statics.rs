//! Four statics that read something other than a color.
//!
//! Guildscorn Ward's quality is a color *count*, Fog Bank's shield points
//! both ways at once, and Night Revelers and Angelic Overseer read opposite
//! sides of the battlefield continuously rather than once.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
}

/// Two colors are stopped; one of those same colors alone is not.
#[test]
fn the_ward_reads_the_color_count_not_the_colors() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut ward = creature(10_001, cards::GUILDSCORN_WARD, PlayerId::One);
    ward.attached_to = Some(bear_id);
    game.battlefield.push(ward);

    let gold = creature(10_100, cards::SPIKE_JESTER, PlayerId::Two);
    let gold_id = gold.card.id;
    game.battlefield.push(gold);
    game.damage_target_from(Some(gold_id), Some(Target::Permanent(bear_id)), 1);
    assert_eq!(
        permanent(&game, bear_id).damage,
        0,
        "a black-red creature is multicolored",
    );

    // Mono-black, one of the very colors the Jester is made of.
    let mono = creature(10_101, cards::DAGGERDROME_IMP, PlayerId::Two);
    let mono_id = mono.card.id;
    game.battlefield.push(mono);
    game.damage_target_from(Some(mono_id), Some(Target::Permanent(bear_id)), 1);
    assert_eq!(
        permanent(&game, bear_id).damage,
        1,
        "one colour is not multicolored, whichever colour it is",
    );
}

/// Both directions of Fog Bank's clause, in one combat.
#[test]
fn the_fog_bank_neither_deals_nor_takes_combat_damage() {
    let mut game = ready();
    let bank = creature(10_000, cards::FOG_BANK, PlayerId::One);
    let bank_id = bank.card.id;
    game.battlefield.push(bank);
    let attacker = creature(10_100, cards::AIR_ELEMENTAL, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    game.damage_target_from_kind(Some(attacker_id), Some(Target::Permanent(bank_id)), 4, true);
    assert_eq!(permanent(&game, bank_id).damage, 0, "nothing lands on it");

    game.damage_target_from_kind(Some(bank_id), Some(Target::Permanent(attacker_id)), 4, true);
    assert_eq!(
        permanent(&game, attacker_id).damage,
        0,
        "and nothing it deals lands either",
    );
}

/// Combat damage only, so a burn spell still gets through.
#[test]
fn the_fog_bank_still_takes_noncombat_damage() {
    let mut game = ready();
    let bank = creature(10_000, cards::FOG_BANK, PlayerId::One);
    let bank_id = bank.card.id;
    game.battlefield.push(bank);

    game.damage_target_from(None, Some(Target::Permanent(bank_id)), 1);
    assert_eq!(
        permanent(&game, bank_id).damage,
        1,
        "the clause names combat damage",
    );
}

/// "As long as", so the haste follows the opponent's board in both directions.
#[test]
fn the_revelers_take_haste_from_the_opponents_humans() {
    let mut game = ready();
    let revelers = creature(10_000, cards::NIGHT_REVELERS, PlayerId::One);
    let revelers_id = revelers.card.id;
    game.battlefield.push(revelers);

    let hasty = |game: &Game| {
        game.permanent_has_executable_keyword(permanent(game, revelers_id), KeywordAbility::Haste)
    };
    assert!(!hasty(&game), "no Humans anywhere yet");

    // A Human of your own is not an opponent's.
    let mine = creature(10_100, cards::ELITE_INQUISITOR, PlayerId::One);
    game.battlefield.push(mine);
    assert!(!hasty(&game), "the clause names an opponent's Human");

    let theirs = creature(10_101, cards::ELITE_INQUISITOR, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    assert!(hasty(&game), "now one of theirs is out");

    game.battlefield
        .retain(|permanent| permanent.card.id != theirs_id);
    assert!(!hasty(&game), "and it lapses when theirs leaves");
}

/// Both keywords follow your Humans onto and off the battlefield together.
#[test]
fn the_overseer_is_protected_only_while_you_control_a_human() {
    let mut game = ready();
    let overseer = creature(10_000, cards::ANGELIC_OVERSEER, PlayerId::One);
    let overseer_id = overseer.card.id;
    game.battlefield.push(overseer);

    let has_keyword = |game: &Game, keyword| {
        game.permanent_has_executable_keyword(permanent(game, overseer_id), keyword)
    };
    for keyword in [KeywordAbility::Hexproof, KeywordAbility::Indestructible] {
        assert!(
            !has_keyword(&game, keyword),
            "no Human means no {keyword:?}"
        );
    }

    game.battlefield
        .push(creature(10_100, cards::ELITE_INQUISITOR, PlayerId::Two));
    for keyword in [KeywordAbility::Hexproof, KeywordAbility::Indestructible] {
        assert!(
            !has_keyword(&game, keyword),
            "an opponent's Human does not grant {keyword:?}",
        );
    }

    let human = creature(10_101, cards::ELITE_INQUISITOR, PlayerId::One);
    let human_id = human.card.id;
    game.battlefield.push(human);
    for keyword in [KeywordAbility::Hexproof, KeywordAbility::Indestructible] {
        assert!(has_keyword(&game, keyword), "your Human grants {keyword:?}");
    }

    game.battlefield
        .retain(|permanent| permanent.card.id != human_id);
    for keyword in [KeywordAbility::Hexproof, KeywordAbility::Indestructible] {
        assert!(
            !has_keyword(&game, keyword),
            "losing your last Human removes {keyword:?}",
        );
    }
}
