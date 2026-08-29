//! Participant-scoped blocking restrictions and declaration costs.

use super::*;
use crate::card::{BlockRestrictionDef, BlockRestrictionMatchDef, BlockRestrictionSubjectDef};

const TAXED_BLOCKER: CardDefinitionId = CardDefinitionId::new(10_170);
const TAXED_MANA_BLOCKER: CardDefinitionId = CardDefinitionId::new(10_171);
const MULTI_BLOCKER: CardDefinitionId = CardDefinitionId::new(10_172);
const TAXED_ATTACKER: CardDefinitionId = CardDefinitionId::new(10_173);
const REQUIRED_TAXED_BLOCKER: CardDefinitionId = CardDefinitionId::new(10_174);
const TAXED_MULTI_BLOCKER: CardDefinitionId = CardDefinitionId::new(10_175);

const BLOCKER_TAX: AppliedRuleDef =
    AppliedRuleDef::BlockRestriction(BlockRestrictionDef::unless_paid(
        BlockRestrictionSubjectDef::Blocker,
        BlockRestrictionMatchDef::Any,
        mana_cost!("{1}"),
    ));
const ATTACKER_TAX: AppliedRuleDef =
    AppliedRuleDef::BlockRestriction(BlockRestrictionDef::unless_paid(
        BlockRestrictionSubjectDef::Attacker,
        BlockRestrictionMatchDef::Any,
        mana_cost!("{1}"),
    ));

static TAXED_BLOCKER_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This creature can't block unless its controller pays {1}.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(BLOCKER_TAX),
    },
)];
static TAXED_MANA_BLOCKER_ABILITIES: [AbilityDef; 2] = [
    abilities::tap_for(ManaColor::Green),
    TAXED_BLOCKER_ABILITIES[0],
];
static MULTI_BLOCKER_EFFECTS: [AppliedEffectDef; 1] = [AppliedEffectDef::Rule(
    AppliedRuleDef::MayBlockAdditionalCreatures(1),
)];
static MULTI_BLOCKER_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This creature can block an additional creature each combat.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&MULTI_BLOCKER_EFFECTS),
    },
)];
static TAXED_MULTI_BLOCKER_EFFECTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(BLOCKER_TAX),
    AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
];
static TAXED_MULTI_BLOCKER_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This creature can't block unless its controller pays {1}, and it can block an additional creature each combat.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&TAXED_MULTI_BLOCKER_EFFECTS),
    },
)];
static TAXED_ATTACKER_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This creature can't be blocked unless the blocking creature's controller pays {1}.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(ATTACKER_TAX),
    },
)];
static REQUIRED_TAXED_BLOCKER_EFFECTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(BLOCKER_TAX),
    AppliedEffectDef::Rule(AppliedRuleDef::MustBlockEachAttackerIfAble),
];
static REQUIRED_TAXED_BLOCKER_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This creature blocks each creature if able and can't block unless its controller pays {1}.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&REQUIRED_TAXED_BLOCKER_EFFECTS),
    },
)];

fn creature_definition(
    id: CardDefinitionId,
    name: &'static str,
    abilities: &'static [AbilityDef],
) -> CardDefinition {
    let mut definition = CardDefinition::new(
        id,
        name,
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &["Test"], 2, 2).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    definition
}

fn restriction_catalog(game: &Game) -> CardCatalog {
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([
        creature_definition(TAXED_BLOCKER, "Taxed blocker", &TAXED_BLOCKER_ABILITIES),
        creature_definition(
            TAXED_MANA_BLOCKER,
            "Taxed mana blocker",
            &TAXED_MANA_BLOCKER_ABILITIES,
        ),
        creature_definition(MULTI_BLOCKER, "Multi-blocker", &MULTI_BLOCKER_ABILITIES),
        creature_definition(TAXED_ATTACKER, "Taxed attacker", &TAXED_ATTACKER_ABILITIES),
        creature_definition(
            REQUIRED_TAXED_BLOCKER,
            "Required taxed blocker",
            &REQUIRED_TAXED_BLOCKER_ABILITIES,
        ),
    ]);
    definitions.push(creature_definition(
        TAXED_MULTI_BLOCKER,
        "Taxed multi-blocker",
        &TAXED_MULTI_BLOCKER_ABILITIES,
    ));
    CardCatalog::new(definitions).expect("the blocking fixtures are valid")
}

fn blocking_game(
    attacker_definitions: &[CardDefinitionId],
    blocker_definition: CardDefinitionId,
) -> (Game, Vec<GameObjectId>, GameObjectId) {
    let mut game = ready_game();
    game.catalog = restriction_catalog(&game);
    game.step = Step::DeclareBlockers;
    game.turns_started[PlayerId::Two.index()] = 1;
    let attackers = attacker_definitions
        .iter()
        .enumerate()
        .map(|(index, definition)| {
            let mut attacker = creature(
                20_000 + u32::try_from(index).expect("a small fixture"),
                *definition,
                PlayerId::One,
            );
            attacker.attacking = true;
            attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
            let id = attacker.card.id;
            game.battlefield.push(attacker);
            id
        })
        .collect();
    let blocker = creature(20_100, blocker_definition, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    (game, attackers, blocker_id)
}

fn block_action(blocker: GameObjectId, attacker: GameObjectId) -> Action {
    Action::DeclareBlocker { blocker, attacker }
}

#[test]
fn a_blocker_can_tap_for_mana_to_pay_its_own_declaration_cost() {
    let (mut game, attackers, blocker) =
        blocking_game(&[cards::SAVANNAH_LIONS], TAXED_MANA_BLOCKER);
    let block = block_action(blocker, attackers[0]);
    assert!(game.legal_actions(PlayerId::Two).contains(&block));
    game.apply(PlayerId::Two, block).unwrap();
    game.apply(PlayerId::Two, Action::FinishDeclaringBlockers)
        .unwrap();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == blocker)
            .is_some_and(|permanent| permanent.tapped),
        "blocking does not tap the creature, so its mana ability can pay"
    );
}

#[test]
fn one_blocker_scoped_cost_covers_every_block_that_creature_makes() {
    let (mut game, attackers, blocker) = blocking_game(
        &[cards::SAVANNAH_LIONS, cards::GRIZZLY_BEARS],
        TAXED_MULTI_BLOCKER,
    );
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;

    game.apply(PlayerId::Two, block_action(blocker, attackers[0]))
        .unwrap();
    let second = block_action(blocker, attackers[1]);
    assert!(
        game.legal_actions(PlayerId::Two).contains(&second),
        "the same blocker restriction is charged once, not once per attacker"
    );
    game.apply(PlayerId::Two, second).unwrap();
    game.apply(PlayerId::Two, Action::FinishDeclaringBlockers)
        .unwrap();
    assert_eq!(game.players[PlayerId::Two.index()].mana_pool.colorless, 0);
}

#[test]
fn attacker_scoped_costs_add_for_a_creature_blocking_several_attackers() {
    let (mut game, attackers, blocker) =
        blocking_game(&[TAXED_ATTACKER, TAXED_ATTACKER], MULTI_BLOCKER);
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    game.apply(PlayerId::Two, block_action(blocker, attackers[0]))
        .unwrap();
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&block_action(blocker, attackers[1])),
        "each protected attacker contributes its own cost"
    );
}

#[test]
fn a_must_block_requirement_never_forces_a_declaration_cost() {
    let (mut game, _attackers, _blocker) =
        blocking_game(&[cards::SAVANNAH_LIONS], REQUIRED_TAXED_BLOCKER);
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::FinishDeclaringBlockers),
        "being able to afford the optional cost does not make the block mandatory"
    );
}

fn offers_pair(game: &Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two)
        .contains(&block_action(blocker, attacker))
}

fn cast_untargeted_spell(
    game: &mut Game,
    player: PlayerId,
    instance: u32,
    definition: CardDefinitionId,
) {
    let spell = card(instance, definition, player);
    let spell_id = spell.id;
    game.players[player.index()].hand.push(spell);
    game.players[player.index()].mana_pool.red = 1;
    game.players[player.index()].mana_pool.colorless = 5;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the test spell is castable");
    game.apply(player, action).unwrap();
    drain_pending(game);
}

#[test]
fn tower_of_coireall_excludes_walls_and_no_other_blockers() {
    let mut game = ready_game();
    let tower = creature(21_100, cards::TOWER_OF_COIREALL, PlayerId::One);
    let tower_id = tower.card.id;
    game.battlefield.push(tower);
    let attacker = creature(21_101, cards::SAVANNAH_LIONS, PlayerId::One);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    ..
                } if *source == tower_id
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(attacker_id))
            )
        })
        .expect("the Tower can target the prospective attacker");
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);

    game.step = Step::DeclareBlockers;
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("the target remains");
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let wall = creature(21_102, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    let bear = creature(21_103, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert!(!offers_pair(&game, wall_id, attacker_id));
    assert!(offers_pair(&game, bear_id, attacker_id));
}

#[test]
fn awe_for_the_guilds_only_silences_monocolored_blockers() {
    let mut game = ready_game();
    let white = creature(21_110, cards::SAVANNAH_LIONS, PlayerId::Two);
    let white_id = white.card.id;
    game.battlefield.push(white);
    let colorless = creature(21_111, cards::JUGGERNAUT, PlayerId::Two);
    let colorless_id = colorless.card.id;
    game.battlefield.push(colorless);
    cast_untargeted_spell(&mut game, PlayerId::One, 21_112, cards::AWE_FOR_THE_GUILDS);

    game.step = Step::DeclareBlockers;
    let mut attacker = creature(21_113, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    assert!(!offers_pair(&game, white_id, attacker_id));
    assert!(offers_pair(&game, colorless_id, attacker_id));
}

#[test]
fn seismic_stomp_reads_flying_granted_before_it_resolves() {
    let mut game = ready_game();
    let ground = creature(21_120, cards::GRIZZLY_BEARS, PlayerId::Two);
    let ground_id = ground.card.id;
    game.battlefield.push(ground);
    let mut flier = creature(21_121, cards::GRIZZLY_BEARS, PlayerId::Two);
    flier.set_counters(CounterKind::PlusOnePlusOne, 1);
    let flier_id = flier.card.id;
    game.battlefield.push(flier);
    game.battlefield
        .push(creature(21_124, cards::SAPPHIRE_DRAKE, PlayerId::Two));
    cast_untargeted_spell(&mut game, PlayerId::One, 21_122, cards::SEISMIC_STOMP);

    game.step = Step::DeclareBlockers;
    let mut attacker = creature(21_123, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    assert!(!offers_pair(&game, ground_id, attacker_id));
    assert!(offers_pair(&game, flier_id, attacker_id));
}

#[test]
fn champion_of_lambholt_compares_each_blocker_with_its_current_power() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut champion = creature(21_000, cards::CHAMPION_OF_LAMBHOLT, PlayerId::One);
    champion.attacking = true;
    champion.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    champion.set_counters(CounterKind::PlusOnePlusOne, 1);
    let champion_id = champion.card.id;
    game.battlefield.push(champion);
    let mut ally = creature(21_001, cards::SAVANNAH_LIONS, PlayerId::One);
    ally.attacking = true;
    ally.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let ally_id = ally.card.id;
    game.battlefield.push(ally);
    let small = creature(21_002, cards::LLANOWAR_ELVES, PlayerId::Two);
    let small_id = small.card.id;
    game.battlefield.push(small);
    let equal = creature(21_003, cards::GRIZZLY_BEARS, PlayerId::Two);
    let equal_id = equal.card.id;
    game.battlefield.push(equal);

    for attacker in [champion_id, ally_id] {
        assert!(!offers_pair(&game, small_id, attacker));
        assert!(offers_pair(&game, equal_id, attacker));
    }
}

#[test]
fn cyclops_tyrant_reads_the_prospective_attackers_effective_power() {
    let (game, attackers, tyrant) = blocking_game(
        &[cards::SAVANNAH_LIONS, cards::CRAW_WURM],
        cards::CYCLOPS_TYRANT,
    );
    assert!(!offers_pair(&game, tyrant, attackers[0]));
    assert!(offers_pair(&game, tyrant, attackers[1]));
}

#[test]
fn amrou_kithkin_reads_a_blockers_statically_modified_power() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(21_130, cards::AMROU_KITHKIN, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let blocker = creature(21_131, cards::SAVANNAH_LIONS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    assert!(offers_pair(&game, blocker_id, attacker_id));

    game.battlefield
        .push(creature(21_132, cards::CRUSADE, PlayerId::Two));
    assert!(
        !offers_pair(&game, blocker_id, attacker_id),
        "Crusade raises the prospective blocker's power to three"
    );
}

#[test]
fn prophecy_brawlers_switch_both_restrictions_with_untapped_lands() {
    for definition in [cards::BRANDED_BRAWLERS, cards::VETERAN_BRAWLERS] {
        let (mut game, attackers, blocker) = blocking_game(&[cards::SAVANNAH_LIONS], definition);
        assert!(offers_pair(&game, blocker, attackers[0]));
        let land = creature(21_140, cards::MOUNTAIN, PlayerId::Two);
        let land_id = land.card.id;
        game.battlefield.push(land);
        assert!(!offers_pair(&game, blocker, attackers[0]));
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == land_id)
            .expect("the land remains")
            .tapped = true;
        assert!(offers_pair(&game, blocker, attackers[0]));

        let mut game = ready_game();
        game.step = Step::DeclareAttackers;
        game.turns_started[PlayerId::One.index()] = 1;
        let brawler = creature(21_141, definition, PlayerId::One);
        let brawler_id = brawler.card.id;
        game.battlefield.push(brawler);
        let land = creature(21_142, cards::MOUNTAIN, PlayerId::Two);
        let land_id = land.card.id;
        game.battlefield.push(land);
        let attack = Action::DeclareAttacker {
            attacker: brawler_id,
            defender: AttackDefender::Player(PlayerId::Two),
        };
        assert!(!game.legal_actions(PlayerId::One).contains(&attack));
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == land_id)
            .expect("the land remains")
            .tapped = true;
        assert!(game.legal_actions(PlayerId::One).contains(&attack));
    }
}
