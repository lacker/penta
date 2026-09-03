//! Effective legendary status, copy-process exceptions, and conditional
//! ability grants used by the legendary-card audit.

use super::*;

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is cataloged"))
}

fn copied_legend(id: u32, base: CardDefinitionId, name: &str, controller: PlayerId) -> Permanent {
    let mut permanent = creature(id, base, controller);
    let mut copy = copied_characteristics(base);
    copy.name = Some(name.to_owned());
    copy.added_supertypes[CardSupertype::Legendary.index()] = true;
    permanent.copy_effect = Some(copy);
    permanent
}

#[test]
fn legend_rule_compares_effective_names_and_supertypes() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.extend([
        copied_legend(90_100, cards::SAVANNAH_LIONS, "Sakashima", PlayerId::One),
        copied_legend(
            90_101,
            cards::SAVANNAH_LIONS,
            "Not Sakashima",
            PlayerId::One,
        ),
    ]);

    game.check_state_based_actions();
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "different effective names do not collide merely because the copies share one rules source",
    );

    game.battlefield[1]
        .copy_effect
        .as_mut()
        .expect("the test permanent is a copy")
        .name = Some("Sakashima".to_owned());
    game.check_state_based_actions();
    assert!(
        game.observe(PlayerId::One).decision.is_some(),
        "same-named effective legends invoke the legend rule",
    );
}

#[test]
fn mirror_gallery_suspends_the_legend_rule_for_every_player() {
    let mut game = ready_game();
    game.battlefield.clear();
    let gallery = definition(&game, "Mirror Gallery");
    game.put_onto_battlefield(PlayerId::One, gallery)
        .expect("Mirror Gallery is cataloged");
    game.battlefield.extend([
        copied_legend(90_102, cards::SAVANNAH_LIONS, "Lazav", PlayerId::One),
        copied_legend(90_103, cards::SAVANNAH_LIONS, "Lazav", PlayerId::One),
    ]);

    game.check_state_based_actions();

    assert!(game.observe(PlayerId::One).decision.is_none());
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                game.effective_permanent_name(permanent).as_deref() == Some("Lazav")
            })
            .count(),
        2,
    );
}

#[test]
fn cadric_exempts_tokens_without_hiding_a_nontoken_legend_group() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, definition(&game, "Cadric, Soul Kindler"))
        .expect("Cadric is cataloged");
    drain_pending(&mut game);

    let mut token_copy = copied_characteristics(cards::SAVANNAH_LIONS);
    token_copy.name = Some("Shared Legend".to_owned());
    token_copy.added_supertypes[CardSupertype::Legendary.index()] = true;
    game.create_token_copy(PlayerId::One, token_copy, None, CardPartId::PRIMARY);
    drain_pending(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the token copy entered")
        .card
        .id;

    game.battlefield.extend([
        copied_legend(
            90_104,
            cards::SAVANNAH_LIONS,
            "Shared Legend",
            PlayerId::One,
        ),
        copied_legend(
            90_105,
            cards::SAVANNAH_LIONS,
            "Shared Legend",
            PlayerId::One,
        ),
    ]);
    game.check_state_based_actions();

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the two nonexempt nontokens still invoke the legend rule");
    let candidates = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(object, _)| object))
        .collect::<Vec<_>>();
    assert_eq!(candidates.len(), 2);
    assert!(!candidates.contains(&token));
}

#[test]
fn cadric_copies_an_arriving_legend_and_grants_the_token_haste() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, definition(&game, "Cadric, Soul Kindler"))
        .expect("Cadric is cataloged");
    drain_pending(&mut game);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let livonya = definition(&game, "Livonya Silone");
    game.put_onto_battlefield(PlayerId::One, livonya)
        .expect("Livonya is cataloged");

    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pay = decision
                .options
                .iter()
                .find(|option| option.label != "Decline")
                .expect("Cadric offers the payment")
                .id;
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![pay],
                },
            )
            .expect("the payment is accepted");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority advances the trigger");
    }

    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition.is_token()
                && game.effective_permanent_name(permanent).as_deref() == Some("Livonya Silone")
        })
        .expect("Cadric made the copy token");
    assert!(game.permanent_has_executable_keyword(token, KeywordAbility::Haste));
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                game.effective_permanent_name(permanent).as_deref() == Some("Livonya Silone")
            })
            .count(),
        2,
        "the token exemption lets the original and copy coexist",
    );
}

#[test]
fn council_of_reeds_exempts_creatures_but_not_other_permanents() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, definition(&game, "Council of Reeds"))
        .expect("Council of Reeds is cataloged");
    drain_pending(&mut game);
    game.battlefield.extend([
        copied_legend(90_106, cards::SAVANNAH_LIONS, "Council Pair", PlayerId::One),
        copied_legend(90_107, cards::SAVANNAH_LIONS, "Council Pair", PlayerId::One),
    ]);

    game.check_state_based_actions();
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "same-named legendary creatures are exempt",
    );

    game.battlefield.extend([
        copied_legend(90_108, cards::BLACK_LOTUS, "Relic Pair", PlayerId::One),
        copied_legend(90_109, cards::BLACK_LOTUS, "Relic Pair", PlayerId::One),
    ]);
    game.check_state_based_actions();
    assert!(
        game.observe(PlayerId::One).decision.is_some(),
        "same-named legendary noncreatures still invoke the legend rule",
    );
}

#[test]
fn sakashima_entry_copy_keeps_its_name_legendary_status_and_return_ability() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lion = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("Savannah Lions is cataloged");
    drain_pending(&mut game);
    let sakashima = game
        .put_onto_battlefield(PlayerId::One, definition(&game, "Sakashima the Impostor"))
        .expect("Sakashima is cataloged");
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Sakashima offers its entry copy choice");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == lion))
        .expect("the Lion is a copy choice")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the copy choice resolves");
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sakashima)
        .expect("Sakashima entered");
    assert_eq!(
        game.effective_permanent_name(permanent).as_deref(),
        Some("Sakashima the Impostor"),
    );
    assert!(
        game.permanent_supertypes(permanent)
            .is_some_and(|supertypes| supertypes.contains(CardSupertype::Legendary))
    );
    assert!(
        game.find_effective_ability(permanent, |effective| {
            effective
                .ability
                .rules_text()
                .starts_with("{2}{U}{U}: Return Sakashima the Impostor")
        })
        .is_some()
    );
}

#[test]
fn champion_helm_grants_hexproof_from_effective_legendary_status() {
    let mut game = ready_game();
    game.battlefield.clear();
    let creature = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("Savannah Lions is cataloged");
    let helm_definition = definition(&game, "Champion's Helm");
    let helm = game
        .put_onto_battlefield(PlayerId::One, helm_definition)
        .expect("Champion's Helm is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == helm)
        .expect("the Equipment is on the battlefield")
        .attached_to = Some(creature);

    let has_hexproof = |game: &Game| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature)
            .expect("the equipped creature is on the battlefield");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof)
    };
    assert!(!has_hexproof(&game));

    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == creature)
        .expect("the equipped creature is on the battlefield");
    let mut copy = copied_characteristics(cards::SAVANNAH_LIONS);
    copy.added_supertypes[CardSupertype::Legendary.index()] = true;
    permanent.copy_effect = Some(copy);

    assert!(has_hexproof(&game));
}

#[test]
fn aeve_spell_copies_enter_as_nonlegendary_tokens() {
    let mut game = ready_game();
    game.battlefield.clear();
    let aeve = definition(&game, "Aeve, Progenitor Ooze");
    game.create_token_copy(
        PlayerId::One,
        copied_characteristics(aeve),
        None,
        CardPartId::PRIMARY,
    );
    drain_pending(&mut game);

    let token = game.battlefield.last().expect("the Aeve copy entered");
    assert!(token.card.definition.is_token());
    assert!(
        !game
            .permanent_supertypes(token)
            .is_some_and(|supertypes| supertypes.contains(CardSupertype::Legendary))
    );
}

#[test]
fn helm_of_the_host_removes_legendary_copiably_but_grants_haste_afterward() {
    let game = ready_game();
    let helm = game
        .catalog
        .get(definition(&game, "Helm of the Host"))
        .expect("Helm of the Host is cataloged");
    let ability = helm
        .rules
        .ability_clauses()
        .first()
        .expect("the combat trigger is printed first");
    let Some(EffectDef::CreateToken {
        copy: Some(copy),
        created: Some(created),
        ..
    }) = ability.declarative_effect()
    else {
        panic!("the combat trigger creates a copy token and continues with it");
    };
    assert!(
        copy.exceptions
            .removed_supertypes
            .contains(&CardSupertype::Legendary),
    );
    assert!(copy.exceptions.added_abilities.is_empty());
    assert!(matches!(
        *created.then,
        EffectDef::Apply {
            effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )),
            ..
        } if ability == &abilities::haste()
    ));
}

#[test]
fn audited_cards_are_complete_declarative_definitions() {
    let game = ready_game();
    for name in [
        "Lazav, Dimir Mastermind",
        "Sakashima the Impostor",
        "Mirror Gallery",
        "Livonya Silone",
        "Minamo, School at Water's Edge",
        "Mountain Stronghold",
        "Aeve, Progenitor Ooze",
        "Arena of the Ancients",
        "Champion's Helm",
        "Helm of the Host",
        "Heroes' Podium",
        "Hammerheim",
        "Cadric, Soul Kindler",
        "Council of Reeds",
    ] {
        let card = game
            .catalog
            .get(definition(&game, name))
            .expect("cataloged");
        assert_eq!(
            card.implementation_status(),
            ImplementationStatus::Complete,
            "{name}",
        );
    }
}

#[test]
fn audited_blockers_remain_whole_card_unsupported() {
    let game = ready_game();
    for name in [
        "Mirror Box",
        "Sakashima of a Thousand Faces",
        "Brothers Yamazaki",
        "Sliver Gravemother",
        "The Master, Multiplied",
        "Spider-Verse",
    ] {
        let card = game
            .catalog
            .get(definition(&game, name))
            .expect("cataloged");
        assert_eq!(
            card.implementation_status(),
            ImplementationStatus::Unsupported,
            "{name}",
        );
    }
}
