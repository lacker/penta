//! Ragavan, Nimble Pilferer: a one-mana 2/1 that pays for itself the first
//! time it connects, and a dash cost for the turns it would only die.

use super::*;

/// Ragavan in hand, with `mana` red available and an opponent's library
/// stocked.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].library.clear();
    for index in 0..3 {
        game.players[1]
            .library
            .push(card(101_000 + index, cards::LIGHTNING_BOLT, PlayerId::Two));
    }
    let ragavan = game
        .build_zone(PlayerId::One, &[cards::RAGAVAN_NIMBLE_PILFERER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let ragavan_id = ragavan.id;
    game.players[0].hand.push(ragavan);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, mana);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, ragavan_id)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Casts it, dashed or not, and returns the permanent it became.
fn cast(game: &mut Game, ragavan: GameObjectId, dashed: bool) -> GameObjectId {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == ragavan && choices.costs().alternative().is_some() == dashed)
        })
        .unwrap_or_else(|| panic!("it is castable (dashed: {dashed})"));
    game.apply(PlayerId::One, action).expect("it is cast");
    resolve(game);
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::RAGAVAN_NIMBLE_PILFERER)
        .expect("it resolved onto the battlefield")
        .card
        .id
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Hard cast, it is an ordinary 2/1 with no haste.
#[test]
fn cast_for_one_it_has_no_haste() {
    let (mut game, ragavan) = staged(1);
    let monkey = cast(&mut game, ragavan, false);

    let permanent = permanent(&game, monkey);
    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(2), Some(1))
    );
    assert!(!game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste));
}

/// Dashed, it has haste and goes home at the end of the turn.
#[test]
fn dashed_it_has_haste_and_goes_home() {
    let (mut game, ragavan) = staged(2);
    let monkey = cast(&mut game, ragavan, true);

    assert!(game.permanent_has_executable_keyword(permanent(&game, monkey), KeywordAbility::Haste));

    game.step = Step::End;
    game.begin_step_triggers();
    resolve(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::RAGAVAN_NIMBLE_PILFERER),
        "it is returned at the beginning of the next end step",
    );
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::RAGAVAN_NIMBLE_PILFERER],
        "to its owner's hand",
    );
}

/// Hard cast, nothing returns it.
#[test]
fn a_hard_cast_one_stays() {
    let (mut game, ragavan) = staged(1);
    cast(&mut game, ragavan, false);

    game.step = Step::End;
    game.begin_step_triggers();
    resolve(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::RAGAVAN_NIMBLE_PILFERER),
    );
}

/// Connecting makes a Treasure and steals the top card of their library.
#[test]
fn connecting_steals_a_card_and_makes_treasure() {
    let (mut game, ragavan) = staged(1);
    let monkey = cast(&mut game, ragavan, false);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(monkey, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    resolve(&mut game);

    assert!(
        game.battlefield.iter().any(|permanent| game
            .effective_subtypes(permanent)
            .contains(crate::card::Subtype::named("Treasure"))),
        "a Treasure for the trouble",
    );
    assert_eq!(game.players[1].library.len(), 2, "their top card is gone");
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "and sitting in exile where you can cast it",
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { .. })),
        "which is yours to cast this turn",
    );
}

/// "If you exile a land card, you can't play that card." The permission is
/// to cast, and a land is played rather than cast.
#[test]
fn a_stolen_land_stays_in_exile() {
    let (mut game, ragavan) = staged(1);
    game.players[1].library.clear();
    game.players[1]
        .library
        .push(card(101_100, cards::MOUNTAIN, PlayerId::Two));
    let monkey = cast(&mut game, ragavan, false);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(monkey, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    resolve(&mut game);

    let stolen = game.players[1]
        .exile
        .iter()
        .find(|card| card.definition == cards::MOUNTAIN)
        .expect("the Mountain was exiled")
        .id;
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 0;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::PlayLand { card, .. } | Action::CastSpell { card, .. }
                if *card == stolen)
        }),
        "a land is played, and what the trigger gave you is a cast",
    );
}

/// "You'll create a Treasure token even if that player has no cards left in
/// their library to exile."
#[test]
fn an_empty_library_still_pays_a_treasure() {
    let (mut game, ragavan) = staged(1);
    game.players[1].library.clear();
    let monkey = cast(&mut game, ragavan, false);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(monkey, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    resolve(&mut game);

    assert!(
        game.battlefield.iter().any(|permanent| game
            .effective_subtypes(permanent)
            .contains(crate::card::Subtype::named("Treasure"))),
        "the Treasure comes whether or not there was anything to steal",
    );
    assert!(
        game.players[1].exile.is_empty(),
        "and nothing was exiled from an empty library",
    );
}
