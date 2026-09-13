//! Phantasmal Image: the best creature on the board for two mana, until
//! somebody points at it.

use super::*;

/// Player One holding an Image with two mana up, and `theirs` on the
/// battlefield under Player Two.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in theirs {
        game.put_onto_battlefield(PlayerId::Two, *definition)
            .expect("cataloged");
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::PHANTASMAL_IMAGE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let image = card.id;
    game.players[0].hand.push(card);
    drain_pending(&mut game);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, image)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Casts the Image and answers the entry choice: the permanent named by
/// `copied`, or "Enter as itself" when nothing is named.
fn cast_copying(game: &mut Game, image: GameObjectId, copied: Option<GameObjectId>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == image))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
    let seat = deciding(game).expect("entering asks what to copy");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = match copied {
        Some(id) => decision
            .options
            .iter()
            .find(|option| option.card.is_some_and(|(found, _)| found == id))
            .expect("the permanent is on the menu"),
        None => decision
            .options
            .iter()
            .find(|option| option.label == "Enter as itself")
            .expect("that is always on the menu"),
    }
    .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

fn the_image(game: &Game) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PHANTASMAL_IMAGE)
}

/// It copies the creature wholesale: power, toughness, and keywords.
#[test]
fn it_enters_as_a_copy_of_a_creature() {
    let (mut game, image) = staged(&[cards::SERRA_ANGEL]);
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("it is here")
        .card
        .id;

    cast_copying(&mut game, image, Some(angel));

    let copy = the_image(&game).expect("it resolved");
    assert_eq!(game.power(copy), Some(4), "a 4/4");
    assert_eq!(game.toughness(copy), Some(4));
    assert!(
        game.permanent_has_executable_keyword(copy, KeywordAbility::Flying),
        "with the Angel's flying",
    );
    assert_eq!(
        copy.controller,
        PlayerId::One,
        "under the Image's controller, not the original's",
    );
}

/// "It's an Illusion in addition to its other types": the copy keeps the
/// Image's own printed subtype beside the ones it copied.
#[test]
fn the_copy_is_still_an_illusion() {
    let (mut game, image) = staged(&[cards::SERRA_ANGEL]);
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("it is here")
        .card
        .id;

    cast_copying(&mut game, image, Some(angel));

    let subtypes = game.effective_subtypes(the_image(&game).expect("it resolved"));
    assert!(
        subtypes.contains(crate::card::Subtype::named("Angel")),
        "an Angel: {subtypes:?}"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Illusion")),
        "and an Illusion too"
    );
}

/// "It has 'When this creature becomes the target of a spell or ability,
/// sacrifice it.'" -- the clause survives the copy.
#[test]
fn targeting_the_copy_kills_it() {
    let (mut game, image) = staged(&[cards::SERRA_ANGEL]);
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("it is here")
        .card
        .id;
    cast_copying(&mut game, image, Some(angel));
    let copy = the_image(&game).expect("it resolved").card.id;

    // Their own Bolt, pointed at the copy: the trigger is what kills it, and
    // three damage would not have.
    let bolt = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(copy))
            }
            _ => false,
        })
        .expect("the copy is a legal target");
    game.apply(PlayerId::Two, cast).expect("it casts");
    settle(&mut game);

    assert!(
        the_image(&game).is_none(),
        "the Image sacrificed itself on being targeted",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::PHANTASMAL_IMAGE),
        "into its owner's graveyard as the card it really is",
    );
}

/// The original is not the copy: pointing at the Angel does nothing to it.
#[test]
fn the_clause_belongs_to_the_copy_alone() {
    let (mut game, image) = staged(&[cards::SERRA_ANGEL]);
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("it is here")
        .card
        .id;
    cast_copying(&mut game, image, Some(angel));

    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.priority = PlayerId::One;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(angel))
            }
            _ => false,
        })
        .expect("the Angel is a legal target");
    game.apply(PlayerId::One, cast).expect("it casts");
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "a 4/4 shrugs off three damage and has no such clause",
    );
    assert!(the_image(&game).is_some(), "and the copy is untouched");
}

/// "You may": entering as itself is always on the menu, and what arrives is
/// a 0/0 that dies to itself.
#[test]
fn entering_as_itself_is_a_zero_zero_that_dies() {
    let (mut game, image) = staged(&[cards::SERRA_ANGEL]);

    cast_copying(&mut game, image, None);

    assert!(
        the_image(&game).is_none(),
        "a 0/0 with nothing on it is put into a graveyard",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::PHANTASMAL_IMAGE),
        "as a state-based action",
    );
}

/// Nothing to copy is not a decision: with an empty board it simply enters.
#[test]
fn an_empty_board_offers_no_copy_at_all() {
    let (mut game, image) = staged(&[]);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == image))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(&mut game);

    assert!(deciding(&game).is_none(), "nobody was asked anything");
}
