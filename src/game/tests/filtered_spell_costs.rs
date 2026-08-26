//! Filtered battlefield and stack cost modifications.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn castable(game: &Game, player: PlayerId, spell: GameObjectId) -> bool {
    game.legal_actions(player)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

#[test]
fn sphere_and_thorn_share_the_filterable_tax() {
    let mut sphere = ready();
    sphere
        .battlefield
        .push(creature(10_000, cards::SPHERE_OF_RESISTANCE, PlayerId::One));
    let bolt = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    sphere.players[PlayerId::One.index()].hand.push(bolt);
    sphere.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(!castable(&sphere, PlayerId::One, bolt_id));
    sphere.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(castable(&sphere, PlayerId::One, bolt_id));

    let mut thorn = ready();
    thorn
        .battlefield
        .push(creature(10_001, cards::THORN_OF_AMETHYST, PlayerId::One));
    let bears = card(20_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    thorn.players[PlayerId::One.index()].hand.push(bears);
    thorn.players[PlayerId::One.index()].mana_pool.green = 1;
    thorn.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(
        castable(&thorn, PlayerId::One, bears_id),
        "Thorn leaves a creature spell at its printed cost",
    );
}

#[test]
fn heartless_summoning_and_daru_warchief_apply_both_clauses() {
    let mut heartless = ready();
    heartless
        .battlefield
        .push(creature(10_000, cards::HEARTLESS_SUMMONING, PlayerId::One));
    heartless
        .battlefield
        .push(creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One));
    let bears = heartless.battlefield[1].clone();
    assert_eq!(heartless.power(&bears), Some(1));
    assert_eq!(heartless.toughness(&bears), Some(1));

    let troll = card(20_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.id;
    heartless.players[PlayerId::One.index()].hand.push(troll);
    heartless.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(
        castable(&heartless, PlayerId::One, troll_id),
        "the creature spell's two generic mana are removed",
    );

    let mut daru = ready();
    daru.battlefield
        .push(creature(10_010, cards::DARU_WARCHIEF, PlayerId::One));
    daru.battlefield.push(creature(
        10_011,
        cards::THALIA_GUARDIAN_OF_THRABEN,
        PlayerId::One,
    ));
    let thalia = daru.battlefield[1].clone();
    assert_eq!(daru.power(&thalia), Some(3));
    assert_eq!(daru.toughness(&thalia), Some(3));
}

#[test]
fn edgewalker_removes_only_the_named_colored_requirements() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::EDGEWALKER, PlayerId::One));
    let second = card(20_000, cards::EDGEWALKER, PlayerId::One);
    let second_id = second.id;
    game.players[PlayerId::One.index()].hand.push(second);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(
        castable(&game, PlayerId::One, second_id),
        "{{W}}{{B}} is removed, while the generic mana remains",
    );

    game.battlefield
        .push(creature(10_001, cards::EDGEWALKER, PlayerId::Two));
    assert!(
        castable(&game, PlayerId::One, second_id),
        "an opponent's Edgewalker does not change your Cleric spell",
    );
}

#[test]
fn colored_reduction_assigns_each_hybrid_symbol_once() {
    let reduced = super::super::mana_planning::reduce_mana_symbols(
        crate::mana_cost!("{W/U}{W/B}"),
        crate::mana_cost!("{W}{B}"),
    );
    assert_eq!(
        reduced.hybrid_total(),
        0,
        "white pays one hybrid reduction and black the other",
    );

    let reduced = super::super::mana_planning::reduce_mana_symbols(
        crate::mana_cost!("{W/U}{W/B}"),
        crate::mana_cost!("{W}"),
    );
    assert_eq!(
        reduced.hybrid_total(),
        1,
        "one white reduction cannot erase both hybrid symbols",
    );
}

#[test]
fn kaerveks_torch_taxes_only_spells_that_target_it_on_the_stack() {
    let mut game = ready();
    let torch = card(20_000, cards::KAERVEK_S_TORCH, PlayerId::One);
    let torch_id = torch.id;
    game.players[PlayerId::One.index()].hand.push(torch);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.apply(
        PlayerId::One,
        cast_action(torch_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("the Torch is cast for zero");
    let stacked_torch = game.stack.last().expect("the Torch is on the stack").id;

    let counter = card(20_001, cards::COUNTERSPELL, PlayerId::Two);
    let counter_id = counter.id;
    game.players[PlayerId::Two.index()].hand.push(counter);
    game.players[PlayerId::Two.index()].mana_pool.blue = 2;
    game.priority = PlayerId::Two;
    assert!(
        !castable(&game, PlayerId::Two, counter_id),
        "two blue does not cover the Torch's two-mana tax",
    );

    game.players[PlayerId::Two.index()].mana_pool.colorless = 2;
    let counter_cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == counter_id
                && choices.iter_targets().any(|target| {
                    *target == Target::Spell(stacked_torch)
                }))
        });
    assert!(
        counter_cast.is_some(),
        "paying the tax makes the target legal"
    );
}

#[test]
fn damping_sphere_counts_the_casters_prior_spells_this_turn() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::DAMPING_SPHERE, PlayerId::One));
    let bolt = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("the first spell has no Sphere tax");
    pass_priority_pair(&mut game);

    let bears = card(20_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    game.players[PlayerId::One.index()].hand.push(bears);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(
        !castable(&game, PlayerId::One, bears_id),
        "the second spell costs one more",
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    assert!(castable(&game, PlayerId::One, bears_id));
}

#[test]
fn hum_of_the_radix_counts_only_artifacts_the_spell_caster_controls() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::HUM_OF_THE_RADIX, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::SOL_RING, PlayerId::Two));
    let ring = card(20_000, cards::SOL_RING, PlayerId::One);
    let ring_id = ring.id;
    game.players[PlayerId::One.index()].hand.push(ring);

    let increase = game.spell_cost_increase(PlayerId::One, ring_id, &[]);
    assert_eq!(
        increase.generic, 2,
        "Hum counts the caster's two artifacts and ignores the opponent's",
    );
}

#[test]
fn hinata_adds_and_subtracts_per_distinct_target() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::HINATA_DAWN_CROWNED, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two));
    let target_id = game.battlefield[1].card.id;

    let discount = card(20_000, cards::DOOM_BLADE, PlayerId::One);
    let discount_id = discount.id;
    game.players[PlayerId::One.index()].hand.push(discount);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    assert!(
        castable(&game, PlayerId::One, discount_id),
        "Hinata removes Doom Blade's one generic mana for its target",
    );

    let taxed = card(20_001, cards::DOOM_BLADE, PlayerId::Two);
    let taxed_id = taxed.id;
    game.players[PlayerId::Two.index()].hand.push(taxed);
    let duplicate_target = [
        TargetSelection::single(TargetSlotId(0), Target::Permanent(target_id)),
        TargetSelection::single(TargetSlotId(1), Target::Permanent(target_id)),
    ];
    assert_eq!(
        game.spell_cost_increase(PlayerId::Two, taxed_id, &duplicate_target)
            .generic,
        1,
        "the same object targeted twice counts once",
    );
}

#[test]
fn every_requested_cost_modifier_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::SPHERE_OF_RESISTANCE,
        cards::THORN_OF_AMETHYST,
        cards::HEARTLESS_SUMMONING,
        cards::AURA_OF_SILENCE,
        cards::CHILL,
        cards::DEFENSE_GRID,
        cards::DERELOR,
        cards::FEROZ_S_BAN,
        cards::GLOOM,
        cards::GLOWRIDER,
        cards::HIGH_SEAS,
        cards::IRINI_SENGIR,
        cards::JADE_LEECH,
        cards::KAERVEK_S_TORCH,
        cards::SAPPHIRE_LEECH,
        cards::RUBY_LEECH,
        cards::SQUEEZE,
        cards::THALIA_GUARDIAN_OF_THRABEN,
        cards::EMERALD_MEDALLION,
        cards::JET_MEDALLION,
        cards::PEARL_MEDALLION,
        cards::RUBY_MEDALLION,
        cards::SAPPHIRE_MEDALLION,
        cards::DARU_WARCHIEF,
        cards::EDGEWALKER,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}

#[test]
fn dynamic_generic_tax_cards_report_honest_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::HUM_OF_THE_RADIX, cards::HINATA_DAWN_CROWNED] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
    assert_eq!(
        catalog
            .get(cards::DAMPING_SPHERE)
            .expect("Damping Sphere is cataloged")
            .rules
            .implementation_status(),
        ImplementationStatus::Partial,
        "only Damping Sphere's mana-production replacement remains unsupported",
    );
}
