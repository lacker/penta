//! Thundering Falls: a tapped dual that looks at the top of your library on
//! the way in and lets you bin what you find.

use super::*;

/// Player One with a Thundering Falls in hand and a known card on top.
fn staged(top: CardDefinitionId) -> (Game, GameObjectId) {
    staged_with(cards::THUNDERING_FALLS, top)
}

/// The same, for any land in the cycle.
fn staged_with(land: CardDefinitionId, top: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    let card = game
        .build_zone(PlayerId::One, &[top])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].library.push(card);
    let land = game
        .build_zone(PlayerId::One, &[land])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let land_id = land.id;
    game.players[0].hand.push(land);
    (game, land_id)
}

/// Plays the land and answers the surveil, keeping the card on top when
/// `bin` is false.
fn play_and_surveil(game: &mut Game, land: GameObjectId, bin: bool) {
    game.priority = PlayerId::One;
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == land))
        .expect("a land drop is available");
    game.apply(PlayerId::One, play)
        .expect("the land is playable");

    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Choosing the card puts it in the graveyard; choosing nothing
            // leaves it where it was.
            let options = if bin {
                decision
                    .options
                    .first()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("surveil accepts either answer");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn the_land(game: &Game) -> Option<&Permanent> {
    the_land_named(game, cards::THUNDERING_FALLS)
}

fn the_land_named(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
}

/// Every colour of mana this permanent will make.
fn colors_of(game: &Game, id: GameObjectId) -> Vec<ManaColor> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
            _ => None,
        })
        .collect()
}

/// The check every member of the cycle gets: play it, watch it arrive
/// tapped, bin what the surveil turned up, and confirm the two colours its
/// basic types make. The cycle is one card printed six ways, so what is
/// worth asserting per member is which two.
fn cycle_member_makes(land: CardDefinitionId, first: ManaColor, second: ManaColor) {
    let (mut game, card) = staged_with(land, cards::LIGHTNING_BOLT);
    play_and_surveil(&mut game, card, true);
    let permanent = the_land_named(&game, land).expect("it is on the battlefield");
    let id = permanent.card.id;
    assert!(permanent.tapped, "tapped on arrival");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and its surveil bins what it was told to",
    );
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
    {
        permanent.tapped = false;
    }

    let colors = colors_of(&game, id);
    assert!(colors.contains(&first));
    assert!(colors.contains(&second));
    assert_eq!(colors.len(), 2, "and nothing else");
}

/// It arrives tapped, whatever you do with the surveil.
#[test]
fn it_enters_tapped() {
    let (mut game, land) = staged(cards::LIGHTNING_BOLT);
    play_and_surveil(&mut game, land, false);

    assert!(the_land(&game).is_some_and(|permanent| permanent.tapped));
}

/// Binning the card puts it in the graveyard and empties the library.
#[test]
fn surveil_may_put_the_card_in_the_graveyard() {
    let (mut game, land) = staged(cards::LIGHTNING_BOLT);
    play_and_surveil(&mut game, land, true);

    assert!(game.players[0].library.is_empty(), "it left the top");
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::LIGHTNING_BOLT)
            .count(),
        1,
    );
}

/// Declining leaves it on top, which is the other half of the choice.
#[test]
fn surveil_may_leave_the_card_on_top() {
    let (mut game, land) = staged(cards::LIGHTNING_BOLT);
    play_and_surveil(&mut game, land, false);

    assert_eq!(game.players[0].library.len(), 1, "still on top");
    assert!(
        game.players[0].graveyard.is_empty(),
        "and nothing was binned",
    );
}

/// The mana abilities come from the basic land types rather than a printed
/// clause, so both colours are on offer.
#[test]
fn it_taps_for_either_colour() {
    let (mut game, land) = staged(cards::LIGHTNING_BOLT);
    play_and_surveil(&mut game, land, false);
    let id = the_land(&game).expect("it is on the battlefield").card.id;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
    {
        permanent.tapped = false;
    }

    let colors = colors_of(&game, id);
    assert!(colors.contains(&ManaColor::Blue), "Island");
    assert!(colors.contains(&ManaColor::Red), "Mountain");
}

/// The basic types are on the land itself rather than granted, which is
/// what makes the mana abilities appear at all.
#[test]
fn the_sewers_bring_their_own_two_basic_types() {
    let (mut game, land) = staged_with(cards::UNDERCITY_SEWERS, cards::MOUNTAIN);
    play_and_surveil(&mut game, land, false);

    let sewers = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::UNDERCITY_SEWERS)
        .expect("it is on the battlefield");
    let subtypes = game.effective_subtypes(sewers);

    assert!(subtypes.contains(crate::card::Subtype::named("Island")));
    assert!(subtypes.contains(crate::card::Subtype::named("Swamp")));
    assert!(sewers.tapped, "and it still arrives tapped");
}

/// The rest of the cycle, one line each: the same land with a different
/// pair of basic types on it.
#[test]
fn the_white_blue_land_taps_for_its_own_two() {
    cycle_member_makes(cards::METICULOUS_ARCHIVE, ManaColor::White, ManaColor::Blue);
}

#[test]
fn the_white_black_land_taps_for_its_own_two() {
    cycle_member_makes(
        cards::SHADOWY_BACKSTREET,
        ManaColor::White,
        ManaColor::Black,
    );
}

#[test]
fn the_blue_black_land_taps_for_its_own_two() {
    cycle_member_makes(cards::UNDERCITY_SEWERS, ManaColor::Blue, ManaColor::Black);
}

#[test]
fn the_black_red_land_taps_for_its_own_two() {
    cycle_member_makes(cards::RAUCOUS_THEATER, ManaColor::Black, ManaColor::Red);
}

#[test]
fn the_red_green_land_taps_for_its_own_two() {
    cycle_member_makes(cards::COMMERCIAL_DISTRICT, ManaColor::Red, ManaColor::Green);
}

#[test]
fn the_green_white_land_taps_for_its_own_two() {
    cycle_member_makes(cards::LUSH_PORTICO, ManaColor::Green, ManaColor::White);
}

#[test]
fn the_green_blue_land_taps_for_its_own_two() {
    cycle_member_makes(cards::HEDGE_MAZE, ManaColor::Green, ManaColor::Blue);
}

#[test]
fn the_black_green_land_taps_for_its_own_two() {
    cycle_member_makes(
        cards::UNDERGROUND_MORTUARY,
        ManaColor::Black,
        ManaColor::Green,
    );
}

/// The basic types are what a fetchland reads: a Misty Rainforest looks for
/// a Forest or an Island and the Hedge Maze is both. What it fetches still
/// arrives tapped and still surveils, which is the whole cost of playing
/// them over the originals.
#[test]
fn a_fetchland_finds_it_and_it_still_arrives_tapped() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].library.clear();
    for definition in [cards::MOUNTAIN, cards::HEDGE_MAZE, cards::GRIZZLY_BEARS] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let fetch = game
        .put_onto_battlefield(PlayerId::One, cards::MISTY_RAINFOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let crack = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == fetch))
        .expect("a life and a sacrifice");
    game.apply(PlayerId::One, crack).expect("it activates");
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // The search names the Maze; the surveil that follows bins
            // whatever it turned up.
            let options = decision
                .options
                .iter()
                .find(|option| {
                    matches!(
                        option.card,
                        Some((_, ObjectCharacteristics::Card { definition, .. }))
                            if definition == cards::HEDGE_MAZE
                    )
                })
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let maze = the_land_named(&game, cards::HEDGE_MAZE).expect("the Maze was found");
    assert!(maze.tapped, "and it arrives tapped, fetched or played");
    assert_eq!(
        colors_of(&game, maze.card.id),
        Vec::new(),
        "so it makes nothing this turn",
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        2,
        "the fetchland it sacrificed and the card its surveil binned",
    );
    assert_eq!(game.players[0].life, 19, "the fetch cost a life");
    assert_eq!(
        game.players[0].library.len(),
        1,
        "one card left: the Maze came out and the surveil binned the next one",
    );
}

/// A Blood Moon takes the printed text with the type line: the Theater is a
/// Mountain, which taps for red alone -- and, since "this land enters
/// tapped" and the surveil are printed abilities rather than anything the
/// basic types carry, it arrives untapped and looks at nothing.
#[test]
fn a_blood_moon_leaves_it_a_plain_untapped_mountain() {
    let (mut game, land) = staged_with(cards::RAUCOUS_THEATER, cards::GRIZZLY_BEARS);
    game.put_onto_battlefield(PlayerId::One, cards::BLOOD_MOON)
        .expect("cataloged");
    drain_pending(&mut game);

    play_and_surveil(&mut game, land, true);

    let theater = the_land_named(&game, cards::RAUCOUS_THEATER).expect("it was played");
    assert!(!theater.tapped, "the clause that taps it is gone");
    assert_eq!(
        colors_of(&game, theater.card.id),
        vec![ManaColor::Red],
        "and a Mountain makes red",
    );
    assert_eq!(
        game.players[0].library.len(),
        1,
        "nothing was surveilled: the trigger went with the rest of the text",
    );
    assert!(game.players[0].graveyard.is_empty());
}

/// Surveil 1 with nothing to look at: no question is asked, and the land is
/// on the battlefield tapped all the same.
#[test]
fn an_empty_library_is_surveilled_over() {
    let (mut game, land) = staged(cards::LIGHTNING_BOLT);
    game.players[0].library.clear();

    game.priority = PlayerId::One;
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == land))
        .expect("a land drop is available");
    game.apply(PlayerId::One, play)
        .expect("the land is playable");
    drain_pending(&mut game);

    assert!(
        game.pending_decisions.is_empty(),
        "an empty library is nothing to look at",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and nothing was binned",
    );
    let falls = the_land(&game).expect("it arrived");
    assert!(falls.tapped, "tapped, as it always is");
    assert_eq!(
        colors_of(&game, falls.card.id),
        Vec::new(),
        "and tapped means no mana out of it yet",
    );
}

/// Surveil is a look and not a draw: a Sheoldred watching the table takes
/// nothing for it and pays nothing for it, whichever way the card goes.
#[test]
fn surveil_is_not_a_draw() {
    for bin in [false, true] {
        let (mut game, land) = staged_with(cards::LUSH_PORTICO, cards::LIGHTNING_BOLT);
        game.battlefield.push(creature(
            94_000,
            cards::SHEOLDRED_THE_APOCALYPSE,
            PlayerId::One,
        ));
        let life = game.players[PlayerId::One.index()].life;
        let hand = game.players[PlayerId::One.index()].hand.len();

        play_and_surveil(&mut game, land, bin);

        assert_eq!(
            game.players[PlayerId::One.index()].life,
            life,
            "looking at the top card is not drawing it, bin={bin}",
        );
        assert_eq!(
            game.players[PlayerId::One.index()].hand.len(),
            hand - 1,
            "and the only card that left hand is the land itself",
        );
    }
}

/// A Wooded Foothills reads "Mountain or Forest card", and the Commercial
/// District is both. Cracking it on their turn is the line the tapped clause
/// is supposed to punish and does not: the land arrives tapped for a turn
/// that was theirs anyway, and the surveil happens at instant speed.
#[test]
fn a_red_green_fetch_finds_it_on_their_turn() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].library.clear();
    for definition in [cards::COMMERCIAL_DISTRICT, cards::LIGHTNING_BOLT] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let fetch = game
        .put_onto_battlefield(PlayerId::One, cards::WOODED_FOOTHILLS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let crack = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == fetch))
        .expect("a fetchland is cracked at instant speed");
    game.apply(PlayerId::One, crack).expect("it activates");
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| {
                    matches!(
                        option.card,
                        Some((_, ObjectCharacteristics::Card { definition, .. }))
                            if definition == cards::COMMERCIAL_DISTRICT
                    )
                })
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let district =
        the_land_named(&game, cards::COMMERCIAL_DISTRICT).expect("a Mountain Forest was found");
    let subtypes = game.effective_subtypes(district);
    assert!(
        subtypes.contains(crate::card::Subtype::named("Mountain")),
        "the half the Foothills read"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Forest")),
        "and its other half"
    );
    assert!(district.tapped, "fetched onto their turn, and still tapped");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and the surveil resolved on their turn, binning the Bolt",
    );
    assert_eq!(game.players[0].life, 19, "the fetch cost its life");
}

/// The surveil is a triggered ability, so the look happens where the trigger
/// resolves rather than where the land arrives: a top card that changes
/// while the trigger waits is the one the Maze sees.
#[test]
fn the_look_happens_when_the_trigger_resolves() {
    let (mut game, land) = staged_with(cards::HEDGE_MAZE, cards::LIGHTNING_BOLT);

    game.priority = PlayerId::One;
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == land))
        .expect("a land drop is available");
    game.apply(PlayerId::One, play)
        .expect("the land is playable");
    assert!(
        game.pending_decisions.is_empty(),
        "nothing is asked while the trigger is still on the stack",
    );

    // In response: whatever put a new card on top, the trigger has not
    // looked yet.
    let angel = game
        .build_zone(PlayerId::One, &[cards::SERRA_ANGEL])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].library.push(angel);

    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the surveil asks about the top card");
    assert_eq!(
        decision
            .options
            .iter()
            .filter_map(|option| option
                .card
                .and_then(|(_, characteristics)| { characteristics.card_definition() }))
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "the card it looks at is the one on top now, not the one on top then",
    );

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision.options.iter().map(|option| option.id).collect(),
        },
    )
    .expect("binning it is an answer");
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "and that is the card it bins",
    );
    assert!(
        game.players[0]
            .library
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt is still in the library, one card further down",
    );
}

/// The trigger belongs to whoever played the land: their Falls looks at
/// their library and asks them, and your own library is no part of it.
#[test]
fn their_copy_surveils_their_own_library() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    game.players[1].library.clear();
    game.players[1].hand.clear();
    game.players[0]
        .library
        .push(card(97_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.players[1]
        .library
        .push(card(97_001, cards::SERRA_ANGEL, PlayerId::Two));
    let falls = game
        .build_zone(PlayerId::Two, &[cards::THUNDERING_FALLS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let falls_id = falls.id;
    game.players[1].hand.push(falls);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game.players[1].lands_played_this_turn = 0;

    let play = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == falls_id))
        .expect("their land drop");
    game.apply(PlayerId::Two, play).expect("it is played");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the surveil asks somebody");
    assert_eq!(decision.player, PlayerId::Two, "and it asks them");
    assert_eq!(
        decision
            .options
            .iter()
            .filter_map(|option| option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition()))
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "about the top of their own library",
    );

    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision.options.iter().map(|option| option.id).collect(),
        },
    )
    .expect("binning it is an answer");
    drain_pending(&mut game);

    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "what they binned is in their graveyard",
    );
    assert_eq!(
        game.players[0].library.len(),
        1,
        "and your library was never touched",
    );
}

/// Two basic land types and no basic supertype: a search for a basic land
/// card walks past the Mortuary, and a Wasteland may name it where it may
/// not name the Swamp beside it.
#[test]
fn it_carries_the_types_without_the_supertype() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for (index, definition) in [cards::UNDERGROUND_MORTUARY, cards::SWAMP]
        .into_iter()
        .enumerate()
    {
        game.players[0].library.push(card(
            97_500 + u32::try_from(index).expect("two cards"),
            definition,
            PlayerId::One,
        ));
    }
    let growth = game
        .build_zone(PlayerId::One, &[cards::RAMPANT_GROWTH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let growth_id = growth.id;
    game.players[0].hand.push(growth);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == growth_id))
        .expect("two mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the search asks");
    let offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![cards::SWAMP],
        "the Swamp is a basic land card and the Mortuary is not",
    );
}

/// The cost the cycle actually charges: the turn it comes down it makes no
/// mana at all, and it is the untap step of your next turn that frees it.
/// The per-member checks untap it by hand to read its colours, so this is
/// where the tapped clause is paid.
#[test]
fn the_turn_it_arrives_it_makes_no_mana_and_the_next_one_it_does() {
    let (mut game, mortuary) = staged_with(cards::UNDERGROUND_MORTUARY, cards::LIGHTNING_BOLT);

    play_and_surveil(&mut game, mortuary, true);

    let permanent =
        the_land_named(&game, cards::UNDERGROUND_MORTUARY).expect("it is on the battlefield");
    let id = permanent.card.id;
    assert!(permanent.tapped, "tapped on arrival");
    assert!(
        colors_of(&game, id).is_empty(),
        "and a tapped land offers nothing to tap",
    );

    game.start_next_turn();
    game.start_next_turn();

    assert!(
        !the_land_named(&game, cards::UNDERGROUND_MORTUARY)
            .expect("still there")
            .tapped,
        "your own untap step is what frees it",
    );
    let colors = colors_of(&game, id);
    assert!(colors.contains(&ManaColor::Black), "a Swamp");
    assert!(colors.contains(&ManaColor::Green), "and a Forest");
}

/// A basic land type on the battlefield is read by more than a fetchland's
/// search: a Flinthoof Boar asks whether you control a Mountain, and a
/// Commercial District is one while it sits there -- tapped, as it arrived.
/// The Underground Mortuary beside it is a Swamp Forest and answers nothing,
/// which is what makes it the type doing the work rather than the land.
#[test]
fn the_district_is_a_mountain_for_whatever_asks() {
    let mut game = ready_game();
    game.battlefield.clear();
    let boar = game
        .put_onto_battlefield(PlayerId::One, cards::FLINTHOOF_BOAR)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::UNDERGROUND_MORTUARY)
        .expect("cataloged");
    drain_pending(&mut game);

    let size = |game: &Game| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == boar)
            .expect("the Boar is out");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(
        size(&game),
        (Some(2), Some(2)),
        "a Swamp Forest is no Mountain",
    );

    game.put_onto_battlefield(PlayerId::One, cards::COMMERCIAL_DISTRICT)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        size(&game),
        (Some(3), Some(3)),
        "and the Mountain Forest is",
    );
}
