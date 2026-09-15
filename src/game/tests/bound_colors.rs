use super::color_layers::{cast_target, put_with_colors};
use super::*;

fn choose_pair(game: &mut Game, indices: [usize; 2]) {
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: indices.map(|i| decision.options[i].id).to_vec(),
        },
    )
    .unwrap();
}

#[test]
fn bound_colors_independent_entry_bindings_survive_a_pending_choice_checkpoint() {
    const FIRST: crate::Binding = crate::Binding!("first_colors");
    const SECOND: crate::Binding = crate::Binding!("second_colors");
    const RULES: CardRules =
        CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::as_enters(
            "Choose two color sets.",
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: FIRST,
                    effect: &crate::card::ReplacementEffectDef::Choose(
                        crate::card::ReplacementChoiceDef::Colors(2),
                    ),
                },
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: SECOND,
                    effect: &crate::card::ReplacementEffectDef::Choose(
                        crate::card::ReplacementChoiceDef::Colors(2),
                    ),
                },
            ]),
        )]);
    for prepared in [false, true] {
        let mut game = ready_game();
        let mut definitions = game
            .catalog
            .definitions()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        let definition = definitions
            .iter_mut()
            .find(|d| d.id == cards::TABLET_OF_THE_GUILDS)
            .unwrap();
        definition.rules = RULES;
        synchronize_single_part_definition(definition);
        game.catalog = CardCatalog::new(definitions).unwrap();
        game.prepared_engine = crate::prepared_engine::PreparedEngine::compile(&game.catalog);
        game.set_prepared_engine_enabled(prepared);
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::TABLET_OF_THE_GUILDS)
            .unwrap();
        choose_pair(&mut game, [0, 1]);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        choose_pair(&mut game, [2, 3]);
        let first = ColorSet::from_colors(&[ManaColor::White, ManaColor::Blue]);
        let second = ColorSet::from_colors(&[ManaColor::Black, ManaColor::Red]);
        for (binding, expected) in [(FIRST, first), (SECOND, second)] {
            assert_eq!(
                game.color_set_value(crate::card::ColorSetDef::Binding(binding), |_| Some(source)),
                expected
            );
        }
        assert!(
            game.color_set_value(
                crate::card::ColorSetDef::Binding(crate::Binding!("absent")),
                |_| Some(source)
            )
            .is_colorless()
        );
        let shown = game
            .observe(PlayerId::One)
            .battlefield
            .into_iter()
            .find(|p| p.id == source)
            .unwrap();
        assert_eq!(shown.chosen_colors.len(), 2);
        let (mut wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let permanent = wire["battlefield"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|p| p["chosenColors"].as_object().is_some_and(|m| m.len() == 2))
            .unwrap();
        permanent["chosenColors"]["first_colors"] = serde_json::json!(["Black", "Red"]);
        assert!(
            Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                42
            )
            .is_err()
        );
    }
}

#[test]
fn bound_colors_tablet_trigger_uses_its_old_source_after_removal_and_reentry() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let old = put_with_colors(&mut game, cards::TABLET_OF_THE_GUILDS, [2, 3]);
        let before = game.players[0].life;
        cast_target(
            &mut game,
            cards::LIGHTNING_BOLT,
            Some(Target::Player(PlayerId::Two)),
        );
        game.return_permanent_to_hand(old);
        // A second incarnation's different choice must not overwrite the trigger's link.
        let index = game.players[0]
            .hand
            .iter()
            .position(|card| card.definition == cards::TABLET_OF_THE_GUILDS)
            .unwrap();
        let card = game.players[0].hand.remove(index);
        assert_ne!(old, card.id);
        game.put_card_onto_battlefield_from(
            card,
            ZoneKind::Hand,
            crate::game::BattlefieldArrival::under(PlayerId::One),
            None,
        );
        choose_pair(&mut game, [0, 1]);
        let new = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::TABLET_OF_THE_GUILDS)
            .unwrap()
            .card
            .id;
        assert_ne!(old, new);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, before + 1);
        assert_eq!(
            game.color_set_value(
                crate::card::ColorSetDef::Binding(crate::Binding!("guild_colors")),
                |_| Some(old)
            ),
            ColorSet::from_colors(&[ManaColor::Black, ManaColor::Red])
        );
    }
}
