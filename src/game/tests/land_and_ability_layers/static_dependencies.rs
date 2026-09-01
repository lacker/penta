fn static_enchantment(
    id: CardDefinitionId,
    name: &'static str,
    abilities: &'static [AbilityDef],
) -> CardDefinition {
    let mut definition = CardDefinition::new(
        id,
        name,
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_enchantment(ManaCost::new(0, 0)).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    definition
}

/// A board carrying "creatures have flying" plus whatever else is handed in,
/// with a vanilla 2/1 last so tests can read it back.
fn game_granting_flying(extra: Vec<CardDefinition>) -> Game {
    static FLYING: AbilityDef = abilities::flying();
    static GRANT_FLYING: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures have flying.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&FLYING),
        },
    )];

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let grant = static_enchantment(
        CardDefinitionId::new(10_090),
        "Static flying grant test",
        &GRANT_FLYING,
    );
    let ids = std::iter::once(grant.id)
        .chain(extra.iter().map(|definition| definition.id))
        .collect::<Vec<_>>();
    definitions.push(grant);
    definitions.extend(extra);
    game.catalog = CardCatalog::new(definitions).unwrap();
    for (index, id) in ids.into_iter().enumerate() {
        let object = 10_000 + u32::try_from(index).unwrap();
        game.battlefield.push(creature(object, id, PlayerId::One));
    }
    game.battlefield
        .push(creature(10_100, cards::SAVANNAH_LIONS, PlayerId::One));
    game
}

static FLIERS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
]);

fn lions_have_granted_flying(game: &Game) {
    let lions = game.battlefield.last().unwrap();
    assert!(
        game.trigger_object_matches(
            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            &game.trigger_event_object(lions),
            lions.card.id,
            false,
        ),
        "read from outside the layer-6 walk the Lions have the granted flying"
    );
}

/// Where the answer still stratifies, pinned so it cannot drift silently.
///
/// Gathering a permanent's abilities is the one query that cannot read itself,
/// so a static ability that grants or removes abilities picks its recipients
/// from the layer below: it sees printed, copied, and resolved keywords, not
/// ones another static ability hands out. Closing this needs the CR 613.8
/// dependency evaluator.
#[test]
fn a_static_ability_grant_picks_recipients_from_the_layer_below_itself() {
    static TRAMPLE: AbilityDef = abilities::trample();
    static GRANT_TRAMPLE: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures with flying have trample.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                FLIERS,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&TRAMPLE),
        },
    )];

    let game = game_granting_flying(vec![static_enchantment(
        CardDefinitionId::new(10_091),
        "Static trample grant test",
        &GRANT_TRAMPLE,
    )]);
    lions_have_granted_flying(&game);
    assert!(
        !game.has_trample(game.battlefield.last().unwrap()),
        "but a grant keyed on flying picks its recipients from the layer below it"
    );
}

/// The stratification is confined to the ability layer. A static power and
/// toughness effect keyed on a keyword sits outside that walk, so One-Eyed
/// Scarecrow's shape reads flying another static effect granted.
#[test]
fn a_static_power_effect_keyed_on_a_keyword_sees_a_static_grant() {
    static SHRINK: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures with flying get -1/-0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                FLIERS,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(0),
            ),
        },
    )];

    let game = game_granting_flying(vec![static_enchantment(
        CardDefinitionId::new(10_092),
        "Static flier penalty test",
        &SHRINK,
    )]);
    lions_have_granted_flying(&game);
    assert_eq!(
        game.power(game.battlefield.last().unwrap()),
        Some(1),
        "the penalty applies to a creature only a static grant made a flier"
    );
}
