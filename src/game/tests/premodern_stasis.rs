//! The cards the Premodern Stasis list needed.

use super::*;

/// Forsaken City is a land that stays tapped unless you feed it, which is
/// what makes it playable only in a deck holding cards it will not cast.
#[test]
fn forsaken_city_stays_tapped_until_a_card_is_exiled_for_it() {
    let mut game = ready_game();
    let mut city = creature(10_000, cards::FORSAKEN_CITY, PlayerId::One);
    city.tapped = true;
    let city_id = city.card.id;
    game.battlefield.push(city);
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()]
        .hand
        .push(card(10_001, cards::COUNTERSPELL, PlayerId::One));

    game.turn += 1;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    // The trigger reaches the stack and then resolves; the choice comes with
    // its resolution.
    let decision = advance_to_prompt(
        &mut game,
        PlayerId::One,
        "At the beginning of your upkeep, you may exile a card from your hand. If you do, untap this land.",
    );
    let yes = decision
        .options
        .iter()
        .find(|option| option.id != 0)
        .expect("accepting is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![yes],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == city_id)
            .expect("still there")
            .tapped,
        "paying a card untapped it",
    );
    assert!(
        game.players[PlayerId::One.index()].hand.is_empty(),
        "and the card is gone",
    );
}

/// Treva's Ruins pays for its colours with the land drop before it.
#[test]
fn trevas_ruins_returns_a_land_or_sacrifices_itself() {
    let played = |pay: bool| {
        let mut game = ready_game();
        let island = creature(10_001, cards::ISLAND, PlayerId::One);
        let island_id = island.card.id;
        game.battlefield.push(island);
        let ruins = card(10_000, cards::TREVAS_RUINS, PlayerId::One);
        let ruins_card = ruins.id;
        game.players[PlayerId::One.index()].hand.push(ruins);
        game.priority = PlayerId::One;
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: ruins_card,
                option: PlayOptionId::DEFAULT,
            },
        )
        .expect("the Lair can be played");
        pass_priority_pair(&mut game);

        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the Lair asks for its land");
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            10_002,
        )
        .expect("the matching-permanent move payment reconstructs");
        let option = if pay {
            decision
                .options
                .iter()
                .find(|option| option.card.is_some_and(|(card, _)| card == island_id))
                .expect("the Island can pay")
                .id
        } else {
            0
        };
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .unwrap();
        drain_pending(&mut game);
        game
    };

    let paid = played(true);
    assert!(
        paid.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::TREVAS_RUINS),
        "the Lair stayed",
    );
    assert!(
        paid.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::ISLAND),
        "and the Island went back to hand",
    );

    let declined = played(false);
    assert!(
        !declined
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::TREVAS_RUINS),
        "declining sacrificed it",
    );
    assert!(
        declined
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ISLAND),
        "and the Island stayed put",
    );
}

/// Arcane Denial's gift is delayed a turn, which is the whole reason a
/// control deck can afford to give it.
#[test]
fn arcane_denial_counters_now_and_pays_at_the_next_upkeep() {
    let mut game = ready_game();
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(bolt_id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .expect("the Bolt is cast");
    let spell = game.stack.last().expect("the Bolt is on the stack").id;

    let denial = card(10_000, cards::ARCANE_DENIAL, PlayerId::One);
    let denial_id = denial.id;
    game.players[PlayerId::One.index()].hand.push(denial);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    let before = game.players[PlayerId::One.index()].hand.len();

    game.apply(
        PlayerId::One,
        cast_action(denial_id, vec![Target::Spell(spell)], Vec::new(), 0),
    )
    .expect("the Denial answers it");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        20,
        "the Bolt was countered",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before - 1,
        "and nothing is drawn yet",
    );

    game.turn += 1;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before,
        "the caster's own card arrives at the next upkeep",
    );
}

/// Chain of Vapor hands the chain to the player it bounced: paying a land
/// buys them the copy.
#[test]
fn chain_of_vapor_bounces_and_offers_the_chain_onward() {
    let mut game = ready_game();
    let vise = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let vise_id = vise.card.id;
    game.battlefield.push(vise);
    game.battlefield
        .push(creature(10_002, cards::ISLAND, PlayerId::Two));

    let chain = card(10_000, cards::CHAIN_OF_VAPOR, PlayerId::One);
    let chain_id = chain.id;
    game.players[PlayerId::One.index()].hand.push(chain);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Permanent(vise_id)], Vec::new(), 0),
    )
    .expect("the Chain can bounce a nonland permanent");
    pass_until_decision(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == vise_id),
        "the Vise went back to hand",
    );
    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("its controller is offered the land sacrifice");
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label.starts_with("Sacrifice")),
        "the sacrifice names the land it would spend",
    );
}

/// Storm counts what came before it, not itself. Each copy is targeted
/// separately, so the total is what this checks rather than which library
/// each one emptied.
#[test]
fn brain_freeze_copies_itself_once_per_earlier_spell() {
    let mut game = ready_game();
    // Two cheap spells first, so the Freeze arrives as the third.
    for (index, definition) in [cards::OPT, cards::OPT].into_iter().enumerate() {
        let id = 20_000 + u32::try_from(index).expect("two spells fit");
        let spell = card(id, definition, PlayerId::One);
        let spell_id = spell.id;
        game.players[PlayerId::One.index()].hand.push(spell);
        game.players[PlayerId::One.index()].mana_pool.blue = 1;
        game.priority = PlayerId::One;
        game.apply(
            PlayerId::One,
            cast_action(spell_id, Vec::new(), Vec::new(), 0),
        )
        .expect("a cantrip is castable");
        drain_pending(&mut game);
    }

    let freeze = card(10_000, cards::BRAIN_FREEZE, PlayerId::One);
    let freeze_id = freeze.id;
    game.players[PlayerId::One.index()].hand.push(freeze);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    let library_before = game.players[PlayerId::Two.index()].library.len();
    let own_before = game.players[PlayerId::One.index()].library.len();

    game.apply(
        PlayerId::One,
        cast_action(
            freeze_id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("the Freeze is cast");
    drain_pending(&mut game);

    let theirs = library_before - game.players[PlayerId::Two.index()].library.len();
    let mine = own_before - game.players[PlayerId::One.index()].library.len();
    assert_eq!(
        theirs + mine,
        9,
        "the original and its two copies milled three each (theirs {theirs}, mine {mine})",
    );
}
