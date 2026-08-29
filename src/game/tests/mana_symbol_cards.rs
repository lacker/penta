//! Printed cards that exercise flexible mana symbols and the untap symbol.

use super::*;
use crate::{FlexibleManaSymbol, TokenCharacteristics};

fn tamiyos_notebook(catalog: &CardCatalog) -> TokenCharacteristics {
    let ability = catalog
        .get(cards::TAMIYO_COMPLEATED_SAGE)
        .expect("Tamiyo is cataloged")
        .rules
        .ability_clauses()
        .iter()
        .find(|ability| ability.text.starts_with("−7"))
        .expect("Tamiyo has her ultimate");
    let Some(EffectDef::CreateToken { token, .. }) = ability.declarative_effect() else {
        panic!("Tamiyo's ultimate directly creates her Notebook");
    };
    token
}

fn resolve_stack(game: &mut Game) {
    for _ in 0..16 {
        if !game.pending_decisions.is_empty()
            || (game.stack.is_empty() && game.pending_triggers.is_empty())
        {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn assert_flexible_card_printings(catalog: &CardCatalog) {
    for (id, name, set, cost, scryfall_id, artist) in [
        (
            cards::GUT_SHOT,
            "Gut Shot",
            CardSet::NewPhyrexia,
            mana_cost!("{R/P}"),
            "a54a2a30-b96a-49c7-9151-1f4b0d4a4413",
            "Greg Staples",
        ),
        (
            cards::BARKSHELL_BLESSING,
            "Barkshell Blessing",
            CardSet::Shadowmoor,
            mana_cost!("{G/W}"),
            "cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69",
            "Steven Belledin",
        ),
        (
            cards::BESEECH_THE_QUEEN,
            "Beseech the Queen",
            CardSet::Shadowmoor,
            mana_cost!("{2/B}{2/B}{2/B}"),
            "64ee0a93-0f6d-42be-bdca-1de5422d8d54",
            "Jason Chan",
        ),
        (
            cards::ULALEK_FUSED_ATROCITY,
            "Ulalek, Fused Atrocity",
            CardSet::ModernHorizons3Commander,
            mana_cost!("{C/W}{C/U}{C/B}{C/R}{C/G}"),
            "fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c",
            "Alex Konstad",
        ),
        (
            cards::TAMIYO_COMPLEATED_SAGE,
            "Tamiyo, Compleated Sage",
            CardSet::KamigawaNeonDynasty,
            mana_cost!("{2}{G}{G/U/P}{U}"),
            "222a736e-d819-452d-aeda-eb848c4b2302",
            "Chris Rahn",
        ),
        (
            cards::FARMSTEAD_GLEANER,
            "Farmstead Gleaner",
            CardSet::ModernHorizons1,
            mana_cost!("{3}"),
            "edafd52f-2dda-4981-baee-404f47ee8969",
            "Josh Hass",
        ),
    ] {
        let definition = catalog.get(id).expect("requested card is cataloged");
        assert_eq!(definition.name, name);
        assert_eq!(definition.debut_set, set);
        assert_eq!(definition.rules.mana_cost(), Some(cost));
        let art = definition
            .art
            .expect("printed card carries exact art metadata");
        assert_eq!(art.scryfall_id, scryfall_id);
        assert_eq!(art.artist, artist);
    }
}

#[test]
fn flexible_symbol_cards_keep_their_exact_catalog_metadata() {
    let catalog = poc::catalog().expect("catalog builds");
    assert_flexible_card_printings(&catalog);

    let ulalek = catalog
        .get(cards::ULALEK_FUSED_ATROCITY)
        .expect("cataloged");
    assert!(
        ulalek.rules.colors().iter().all(|present| !present),
        "devoid is represented as colorless printed metadata",
    );
    let tamiyo = catalog
        .get(cards::TAMIYO_COMPLEATED_SAGE)
        .expect("cataloged");
    assert_eq!(
        tamiyo.rules.ability_clauses()[0].text,
        "Compleated ({G/U/P} can be paid with {G}, {U}, or 2 life. If life was paid, this planeswalker enters with two fewer loyalty counters.)",
        "Tamiyo keeps her printing-specific Compleated reminder text",
    );
}

#[test]
fn ulalek_is_catalog_metadata_not_a_castable_vanilla_body() {
    let mut game = ready_game();
    let ulalek = card(90_000, cards::ULALEK_FUSED_ATROCITY, PlayerId::One);
    let ulalek_id = ulalek.id;
    game.players[PlayerId::One.index()].hand.push(ulalek);
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 5);
    }

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if card == ulalek_id))
    );
}

#[test]
fn ulalek_cannot_be_reanimated_as_a_vanilla_body() {
    let mut game = ready_game();
    let ulalek = card(90_001, cards::ULALEK_FUSED_ATROCITY, PlayerId::One);
    let ulalek_id = ulalek.id;
    game.players[PlayerId::One.index()].graveyard.push(ulalek);
    let reanimate = card(90_002, cards::REANIMATE, PlayerId::One);
    let reanimate_id = reanimate.id;
    game.players[PlayerId::One.index()].hand.push(reanimate);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);

    assert!(game.legal_actions(PlayerId::One).into_iter().all(|action| {
        !matches!(action, Action::CastSpell { card, choices, .. }
            if card == reanimate_id
                && choices.iter_targets().any(|target| *target == Target::Card(ulalek_id)))
    }));

    assert_eq!(
        game.move_target_to_zone(
            Target::Card(ulalek_id),
            ZoneKind::Battlefield,
            ZoneMoveCause::Effect {
                controller: PlayerId::One,
            },
            Some(BattlefieldArrival::under(PlayerId::One)),
            ZonePlacement::Top,
        ),
        None,
        "even a non-targeted zone move cannot materialize Ulalek's metadata as a vanilla body",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.id == ulalek_id),
        "a blocked metadata-only move leaves the card in its source zone",
    );
}

#[test]
fn gut_shot_can_pay_life_and_damage_a_player() {
    let mut game = ready_game();
    let shot = card(90_000, cards::GUT_SHOT, PlayerId::One);
    let shot_id = shot.id;
    game.players[PlayerId::One.index()].hand.push(shot);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let opponent_casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == shot_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .collect::<Vec<_>>();
    assert!(
        opponent_casts
            .iter()
            .any(|action| matches!(action, Action::CastSpell { choices, .. } if choices.mana_payment().alternatives().is_empty())),
        "red mana is one legal payment",
    );
    assert!(
        opponent_casts
            .iter()
            .any(|action| matches!(action, Action::CastSpell { choices, .. } if !choices.mana_payment().alternatives().is_empty())),
        "two life is a distinct legal payment",
    );

    game.players[PlayerId::One.index()].life = 1;
    let low_life_casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == shot_id
                && choices.iter_targets().any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .collect::<Vec<_>>();
    assert!(
        low_life_casts
            .iter()
            .any(|action| matches!(action, Action::CastSpell { choices, .. } if choices.mana_payment().alternatives().is_empty())),
        "red mana remains legal at one life",
    );
    assert!(
        low_life_casts
            .iter()
            .all(|action| matches!(action, Action::CastSpell { choices, .. } if choices.mana_payment().alternatives().is_empty())),
        "a player at one life cannot pay two life",
    );

    game.players[PlayerId::One.index()].life = 20;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == shot_id
                    && !choices.mana_payment().alternatives().is_empty()
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("two life casts Gut Shot with no mana");
    game.apply(PlayerId::One, action).expect("Gut Shot is cast");
    resolve_stack(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 18);
    assert_eq!(game.players[PlayerId::Two.index()].life, 19);
}

#[test]
fn gut_shot_life_action_does_not_preview_or_tap_a_red_source() {
    let mut game = ready_game();
    let mountain = game
        .put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("Mountain is cataloged");
    let shot = card(90_001, cards::GUT_SHOT, PlayerId::One);
    let shot_id = shot.id;
    game.players[PlayerId::One.index()].hand.push(shot);

    let life_cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == shot_id
                && !choices.mana_payment().alternatives().is_empty()
                && choices.iter_targets().any(|target| {
                    *target == Target::Player(PlayerId::Two)
                }))
        })
        .expect("Gut Shot can be paid with life while red mana is available");
    assert!(
        game.mana_sources_for_action(PlayerId::One, &life_cast)
            .is_empty(),
        "the life action's preview owes no mana",
    );
    game.apply(PlayerId::One, life_cast)
        .expect("the life action remains authoritative");
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain)
            .expect("the Mountain remains")
            .tapped,
        "paying life does not tap the available Mountain",
    );
}

#[test]
fn variable_cost_spells_can_choose_x_after_selecting_phyrexian_life() {
    let definition_id = CardDefinitionId::new(10_105);
    let mut definition = CardDefinition::new(
        definition_id,
        "Variable Phyrexian cost test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_instant(mana_cost!("{X}{R/P}")).with_ability(AbilityDef::spell(
            "Draw no cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(0),
            },
        ));
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the fixture is valid");
    let spell = card(90_002, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    assert!(game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(action, Action::CastSpell { card, choices, .. }
            if card == spell_id
                && choices.x() == 3
                && matches!(choices.mana_payment().alternatives(), [payment]
                    if payment.symbol() == FlexibleManaSymbol::RedPhyrexian
                        && payment.count() == 1))
    }));
}

#[test]
fn beseech_the_queen_exposes_every_two_brid_payment_mix() {
    let mut game = ready_game();
    let beseech = card(90_005, cards::BESEECH_THE_QUEEN, PlayerId::One);
    let beseech_id = beseech.id;
    game.players[PlayerId::One.index()].hand.push(beseech);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);

    let mut generic_halves = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == beseech_id => {
                let payment = choices.mana_payment().alternatives();
                match payment {
                    [] => Some(0),
                    [payment] if payment.symbol() == FlexibleManaSymbol::TwoBlack => {
                        Some(payment.count())
                    }
                    _ => None,
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    generic_halves.sort_unstable();
    assert_eq!(generic_halves, vec![0, 1, 2, 3]);
}

#[test]
fn generic_reductions_apply_after_beseechs_two_brid_choice() {
    let mut game = ready_game();
    let notebook = tamiyos_notebook(&game.catalog);
    game.create_token(PlayerId::One, notebook);
    drain_pending(&mut game);
    let beseech = card(90_006, cards::BESEECH_THE_QUEEN, PlayerId::One);
    let beseech_id = beseech.id;
    game.players[PlayerId::One.index()].hand.push(beseech);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    let all_generic = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == beseech_id
                    && matches!(choices.mana_payment().alternatives(), [payment]
                        if payment.symbol() == FlexibleManaSymbol::TwoBlack
                            && payment.count() == 3))
        })
        .expect("the announced six-generic equivalent is reduced to four");
    game.apply(PlayerId::One, all_generic)
        .expect("the reduced all-generic cast is legal");
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "the Notebook reduces the chosen generic equivalent by two",
    );
}

#[test]
fn barkshell_blessing_executes_its_pump_without_conspire() {
    let mut game = ready_game();
    let creature = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let blessing = card(90_010, cards::BARKSHELL_BLESSING, PlayerId::One);
    let blessing_id = blessing.id;
    game.players[PlayerId::One.index()].hand.push(blessing);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == blessing_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(creature))
            }
            _ => false,
        })
        .expect("the creature is a legal target");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    resolve_stack(&mut game);

    let creature = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == creature)
        .expect("the creature remains");
    assert_eq!(
        (game.power(creature), game.toughness(creature)),
        (Some(4), Some(4))
    );
}

#[test]
fn beseech_the_queen_limits_its_search_to_the_lands_you_control() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    game.players[PlayerId::One.index()].library.extend([
        card(90_020, cards::SERRA_ANGEL, PlayerId::One),
        card(90_021, cards::LIGHTNING_BOLT, PlayerId::One),
    ]);
    let beseech = card(90_022, cards::BESEECH_THE_QUEEN, PlayerId::One);
    let beseech_id = beseech.id;
    game.players[PlayerId::One.index()].hand.push(beseech);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == beseech_id))
        .expect("three black mana casts Beseech the Queen");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    resolve_stack(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the search waits");
    let offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    assert_eq!(offered, vec![cards::LIGHTNING_BOLT]);
    let bolt = decision
        .options
        .iter()
        .find(|option| option.card.is_some())
        .expect("the Bolt is offered");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![bolt.id],
        },
    )
    .expect("the search is answered");
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
    );
}

#[test]
fn tamiyos_plus_one_skips_an_untap_and_minus_seven_makes_the_notebook() {
    let mut game = ready_game();
    game.battlefield.clear();
    let tamiyo = game
        .put_onto_battlefield(PlayerId::One, cards::TAMIYO_COMPLEATED_SAGE)
        .expect("cataloged");
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);

    let plus_one = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == tamiyo
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|candidate| *candidate == Target::Permanent(target))
            }
            _ => false,
        })
        .expect("Tamiyo can target the creature");
    game.apply(PlayerId::One, plus_one)
        .expect("the +1 activates");
    resolve_stack(&mut game);
    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains");
    assert!(target.tapped);
    assert_eq!(target.skipped_untap_steps, 1);

    let mut game = ready_game();
    game.battlefield.clear();
    let tamiyo = game
        .put_onto_battlefield(PlayerId::One, cards::TAMIYO_COMPLEATED_SAGE)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tamiyo)
        .expect("Tamiyo is present")
        .set_counters(CounterKind::Loyalty, 7);
    drain_pending(&mut game);
    let minus_seven = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == tamiyo
                    && game
                        .ability_for_origin(tamiyo, *ability)
                        .is_some_and(|ability| ability.text.starts_with("−7")))
        })
        .expect("the -7 is available");
    game.apply(PlayerId::One, minus_seven)
        .expect("the -7 activates");
    resolve_stack(&mut game);
    let notebook = tamiyos_notebook(&game.catalog);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.token_characteristics == Some(notebook) && permanent.controller == PlayerId::One
    }));
}

#[test]
fn tamiyo_enters_with_less_loyalty_only_when_phyrexian_life_was_paid() {
    let mut mana_game = ready_game();
    let tamiyo = card(90_035, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    mana_game.players[PlayerId::One.index()].hand.push(tamiyo);
    mana_game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    mana_game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    mana_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let mana_cast = mana_game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == tamiyo_id && choices.mana_payment().alternatives().is_empty())
        })
        .expect("Tamiyo can pay the Phyrexian hybrid symbol with mana");
    mana_game
        .apply(PlayerId::One, mana_cast)
        .expect("Tamiyo is cast with mana");
    resolve_stack(&mut mana_game);
    assert_eq!(
        mana_game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::TAMIYO_COMPLEATED_SAGE)
            .expect("Tamiyo resolves")
            .counters(CounterKind::Loyalty),
        5,
    );

    let mut life_game = ready_game();
    let tamiyo = card(90_036, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    life_game.players[PlayerId::One.index()].hand.push(tamiyo);
    life_game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    life_game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    life_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let life_cast = life_game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == tamiyo_id
                && matches!(choices.mana_payment().alternatives(), [payment]
                    if payment.symbol() == FlexibleManaSymbol::BlueGreenPhyrexian
                        && payment.count() == 1))
        })
        .expect("Tamiyo can pay two life for the Phyrexian hybrid symbol");
    life_game
        .apply(PlayerId::One, life_cast)
        .expect("Tamiyo is cast with life");
    resolve_stack(&mut life_game);
    assert_eq!(life_game.players[PlayerId::One.index()].life, 18);
    assert_eq!(
        life_game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::TAMIYO_COMPLEATED_SAGE)
            .expect("Tamiyo resolves")
            .counters(CounterKind::Loyalty),
        3,
    );
}

fn life_paid_tamiyo_cast(game: &Game, tamiyo: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == tamiyo
                    && matches!(choices.mana_payment().alternatives(), [payment]
                        if payment.symbol() == FlexibleManaSymbol::BlueGreenPhyrexian
                            && payment.count() == 1))
        })
}

fn blue_pain_source_definition() -> (CardDefinitionId, CardDefinition) {
    static BLUE_PAIN_COSTS: [AbilityCostDef; 2] =
        [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];
    static BLUE_PAIN_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add {U}.",
        &BLUE_PAIN_COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
    )];
    let blue_pain_id = CardDefinitionId::new(10_107);
    let mut blue_pain = CardDefinition::new(
        blue_pain_id,
        "Blue pain source test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    blue_pain.rules = CardRules::new_land(&[]).with_abilities(&BLUE_PAIN_ABILITIES);
    synchronize_single_part_definition(&mut blue_pain);
    (blue_pain_id, blue_pain)
}

#[test]
fn phyrexian_life_is_reserved_from_channel() {
    let mut channel_game = ready_game();
    channel_game.players[PlayerId::One.index()].life = 2;
    resolve_channel(&mut channel_game);
    channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let tamiyo = card(90_038, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    channel_game.players[PlayerId::One.index()]
        .hand
        .push(tamiyo);
    assert!(
        life_paid_tamiyo_cast(&channel_game, tamiyo_id).is_none(),
        "the same two life cannot pay Phyrexian mana and become Channel mana",
    );

    let mut exact_channel_game = ready_game();
    exact_channel_game.players[PlayerId::One.index()].life = 4;
    resolve_channel(&mut exact_channel_game);
    exact_channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    exact_channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    exact_channel_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let tamiyo = card(90_043, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    exact_channel_game.players[PlayerId::One.index()]
        .hand
        .push(tamiyo);
    let exact_channel_cast = life_paid_tamiyo_cast(&exact_channel_game, tamiyo_id)
        .expect("Channel can be activated without spending the last life");
    exact_channel_game
        .apply(PlayerId::One, exact_channel_cast)
        .expect("the offered Channel and Phyrexian-life cast remains payable");
    assert_eq!(exact_channel_game.players[PlayerId::One.index()].life, 1);
}

#[test]
fn phyrexian_life_is_reserved_from_life_cost_mana_sources() {
    let mut lands_game = ready_game();
    lands_game.players[PlayerId::One.index()].life = 2;
    for definition in [
        cards::FOREST,
        cards::ISLAND,
        cards::SUNBAKED_CANYON,
        cards::SUNBAKED_CANYON,
    ] {
        lands_game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("the mana source is cataloged");
    }
    let tamiyo = card(90_039, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    lands_game.players[PlayerId::One.index()].hand.push(tamiyo);
    assert!(
        life_paid_tamiyo_cast(&lands_game, tamiyo_id).is_none(),
        "mana abilities that each cost life share the reserved life budget",
    );

    let (blue_pain_id, blue_pain) = blue_pain_source_definition();
    let mut mixed_game = ready_game();
    mixed_game.players[PlayerId::One.index()].life = 5;
    resolve_channel(&mut mixed_game);
    let mut definitions = mixed_game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(blue_pain);
    mixed_game.catalog = CardCatalog::new(definitions).expect("the fixture is valid");
    for definition in [cards::FOREST, cards::PLAINS] {
        mixed_game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("the mana source is cataloged");
    }
    let blue_pain = mixed_game
        .put_onto_battlefield(PlayerId::One, blue_pain_id)
        .expect("the blue pain source is cataloged");
    let tamiyo = card(90_041, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    mixed_game.players[PlayerId::One.index()].hand.push(tamiyo);
    let mixed_cast = life_paid_tamiyo_cast(&mixed_game, tamiyo_id)
        .expect("Channel and a life-cost mana source can share the remaining life");
    mixed_game
        .apply(PlayerId::One, mixed_cast)
        .expect("Channel is activated before the mana source spends the last life");
    assert_eq!(mixed_game.players[PlayerId::One.index()].life, 1);
    assert!(
        mixed_game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == blue_pain)
            .is_some_and(|permanent| permanent.tapped),
        "the colored pain source, rather than a second Channel activation, supplies blue",
    );
}

#[test]
fn phyrexian_life_can_be_paid_down_to_exactly_zero() {
    let mut exact_game = ready_game();
    exact_game.players[PlayerId::One.index()].life = 2;
    exact_game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    exact_game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    exact_game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let tamiyo = card(90_040, cards::TAMIYO_COMPLEATED_SAGE, PlayerId::One);
    let tamiyo_id = tamiyo.id;
    exact_game.players[PlayerId::One.index()].hand.push(tamiyo);
    let exact_cast = life_paid_tamiyo_cast(&exact_game, tamiyo_id)
        .expect("paying life down to exactly zero is legal when mana is available");
    exact_game
        .apply(PlayerId::One, exact_cast)
        .expect("an offered exact-life payment remains payable");
    assert_eq!(exact_game.players[PlayerId::One.index()].life, 0);
}

#[test]
fn compleated_reduces_loyalty_for_each_phyrexian_symbol_paid_with_life() {
    let definition_id = CardDefinitionId::new(10_104);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multiple Compleated symbols test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_planeswalker(mana_cost!("{R/P}{R/P}"), &["Test"], 5)
        .with_ability(abilities::compleated("Compleated"));
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the fixture is valid");
    let walker = card(90_037, definition_id, PlayerId::One);
    let walker_id = walker.id;
    game.players[PlayerId::One.index()].hand.push(walker);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == walker_id
                    && matches!(choices.mana_payment().alternatives(), [payment]
                        if payment.symbol() == FlexibleManaSymbol::RedPhyrexian
                            && payment.count() == 2))
        })
        .expect("both symbols may be paid with life");
    game.apply(PlayerId::One, cast)
        .expect("the life-paid walker is cast");
    resolve_stack(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 16);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition_id)
            .expect("the walker resolves")
            .counters(CounterKind::Loyalty),
        1,
    );
}

#[test]
fn farmstead_gleaner_stays_tapped_then_untaps_to_grow() {
    let mut game = ready_game();
    game.battlefield.clear();
    let gleaner = game
        .put_onto_battlefield(PlayerId::One, cards::FARMSTEAD_GLEANER)
        .expect("cataloged");
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == gleaner)
        .expect("the Gleaner is present");
    permanent.tapped = true;
    permanent.entered_controller_turn = 0;
    game.turns_started[PlayerId::One.index()] = 1;

    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == gleaner)
            .expect("the Gleaner remains")
            .tapped,
        "its static ability keeps it tapped",
    );

    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == gleaner),
        )
        .expect("the untap-symbol ability is available while tapped");
    game.apply(PlayerId::One, activation)
        .expect("the ability activates");
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == gleaner)
            .expect("the Gleaner remains")
            .tapped,
        "untapping pays {{Q}}",
    );
    resolve_stack(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == gleaner)
            .expect("the Gleaner remains")
            .counters(CounterKind::PlusOnePlusOne),
        1,
    );
}
