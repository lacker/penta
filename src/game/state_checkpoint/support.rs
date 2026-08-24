fn visible_decision_rebinding_ids(
    state: Option<&model::DecisionStateSnapshot>,
) -> Vec<GameObjectId> {
    state
        .map(|state| {
            state
                .card_origins
                .iter()
                .map(|origin| GameObjectId(origin.object_id))
                .collect()
        })
        .unwrap_or_default()
}

const fn zone_kind_snapshot(zone: ZoneKind) -> ZoneKindSnapshot {
    match zone {
        ZoneKind::Library => ZoneKindSnapshot::Library,
        ZoneKind::Hand => ZoneKindSnapshot::Hand,
        ZoneKind::Battlefield => ZoneKindSnapshot::Battlefield,
        ZoneKind::Graveyard => ZoneKindSnapshot::Graveyard,
        ZoneKind::Stack => ZoneKindSnapshot::Stack,
        ZoneKind::Exile => ZoneKindSnapshot::Exile,
        ZoneKind::Command => ZoneKindSnapshot::Command,
    }
}

const fn completion_snapshot(completion: EntryCompletion) -> EntryCompletionSnapshot {
    match completion {
        EntryCompletion::LandPlayed { player } => EntryCompletionSnapshot::LandPlayed {
            seat: player.index(),
        },
        EntryCompletion::SpellResolved { card, definition } => {
            EntryCompletionSnapshot::SpellResolved {
                card: card.0,
                definition,
            }
        }
        EntryCompletion::AttachSource { source } => EntryCompletionSnapshot::AttachSource {
            source: source.0,
        },
        EntryCompletion::AttachToHost { host } => {
            EntryCompletionSnapshot::AttachToHost { host: host.0 }
        }
        EntryCompletion::Attacking { defender } => EntryCompletionSnapshot::Attacking {
            defender: match defender {
                crate::AttackDefender::Player(player) => AttackDefenderSnapshot::Player {
                    seat: player.index(),
                },
                crate::AttackDefender::Planeswalker(object) => {
                    AttackDefenderSnapshot::Planeswalker { object_id: object.0 }
                }
            },
        },
        EntryCompletion::Setup => EntryCompletionSnapshot::Setup,
        EntryCompletion::None => EntryCompletionSnapshot::None,
    }
}

pub(in crate::game::state_checkpoint) fn mana_snapshot(
    catalog: &CardCatalog,
    mana: Mana,
) -> ManaSnapshot {
    ManaSnapshot {
        color: mana_color_snapshot(mana.color),
        source: mana.source.map(|source| ManaSourceSnapshot {
            object: source.object.0,
            ability: ability_origin_snapshot(source.ability),
        }),
        payload: mana_payload_locator(catalog, mana),
    }
}

const fn mana_color_snapshot(color: crate::ManaColor) -> ManaColorSnapshot {
    match color {
        crate::ManaColor::White => ManaColorSnapshot::White,
        crate::ManaColor::Blue => ManaColorSnapshot::Blue,
        crate::ManaColor::Black => ManaColorSnapshot::Black,
        crate::ManaColor::Red => ManaColorSnapshot::Red,
        crate::ManaColor::Green => ManaColorSnapshot::Green,
        crate::ManaColor::Colorless => ManaColorSnapshot::Colorless,
    }
}

const fn parse_mana_color(color: ManaColorSnapshot) -> crate::ManaColor {
    match color {
        ManaColorSnapshot::White => crate::ManaColor::White,
        ManaColorSnapshot::Blue => crate::ManaColor::Blue,
        ManaColorSnapshot::Black => crate::ManaColor::Black,
        ManaColorSnapshot::Red => crate::ManaColor::Red,
        ManaColorSnapshot::Green => crate::ManaColor::Green,
        ManaColorSnapshot::Colorless => crate::ManaColor::Colorless,
    }
}

const fn expiration_snapshot(
    expiration: ContinuousEffectExpiration,
) -> ContinuousEffectExpirationSnapshot {
    match expiration {
        ContinuousEffectExpiration::EndOfTurn => ContinuousEffectExpirationSnapshot::EndOfTurn,
        ContinuousEffectExpiration::EndOfCombat => ContinuousEffectExpirationSnapshot::EndOfCombat,
        ContinuousEffectExpiration::UpkeepOf(player) => {
            ContinuousEffectExpirationSnapshot::UpkeepOf {
                seat: player.index(),
            }
        }
        ContinuousEffectExpiration::TurnOf { player, turn } => {
            ContinuousEffectExpirationSnapshot::TurnOf {
                seat: player.index(),
                turn,
            }
        }
        ContinuousEffectExpiration::WhileSourceTapped => {
            ContinuousEffectExpirationSnapshot::WhileSourceTapped
        }
        ContinuousEffectExpiration::Never => ContinuousEffectExpirationSnapshot::Never,
    }
}

fn parse_expiration(
    expiration: ContinuousEffectExpirationSnapshot,
) -> Result<ContinuousEffectExpiration, String> {
    match expiration {
        ContinuousEffectExpirationSnapshot::EndOfTurn => Ok(ContinuousEffectExpiration::EndOfTurn),
        ContinuousEffectExpirationSnapshot::EndOfCombat => {
            Ok(ContinuousEffectExpiration::EndOfCombat)
        }
        ContinuousEffectExpirationSnapshot::UpkeepOf { seat } => Ok(
            ContinuousEffectExpiration::UpkeepOf(player_from_index(seat)?),
        ),
        ContinuousEffectExpirationSnapshot::TurnOf { seat, turn } => {
            Ok(ContinuousEffectExpiration::TurnOf {
                player: player_from_index(seat)?,
                turn,
            })
        }
        ContinuousEffectExpirationSnapshot::WhileSourceTapped => {
            Ok(ContinuousEffectExpiration::WhileSourceTapped)
        }
        ContinuousEffectExpirationSnapshot::Never => Ok(ContinuousEffectExpiration::Never),
    }
}

const fn ability_origin_snapshot(origin: AbilityOrigin) -> AbilityOriginSnapshot {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } => AbilityOriginSnapshot::Printed {
            definition,
            part_id: part.0,
            ability_id: ability.0,
        },
        AbilityOrigin::Token { part, ability } => AbilityOriginSnapshot::Token {
            part_id: part.0,
            ability_id: ability.0,
        },
        AbilityOrigin::Emblem { ability } => AbilityOriginSnapshot::Emblem {
            ability_id: ability.0,
        },
        AbilityOrigin::FaceDown { ability } => AbilityOriginSnapshot::FaceDown {
            ability_id: ability.0,
        },
        AbilityOrigin::IntrinsicBasicLand(land_type) => AbilityOriginSnapshot::IntrinsicBasicLand {
            land_type: basic_land_type_snapshot(land_type),
        },
        AbilityOrigin::IntrinsicCounter(kind) => AbilityOriginSnapshot::IntrinsicCounter {
            counter: CounterKindSnapshot(kind),
        },
        AbilityOrigin::Granted {
            source,
            source_definition,
            source_part,
            source_ability,
            grant,
        } => AbilityOriginSnapshot::Granted {
            source: source.0,
            source_definition,
            source_part_id: source_part.0,
            source_ability_id: source_ability.0,
            grant_id: grant.0,
        },
        AbilityOrigin::TokenGranted {
            source,
            source_part,
            source_ability,
            grant,
        } => AbilityOriginSnapshot::TokenGranted {
            source: source.0,
            source_part_id: source_part.0,
            source_ability_id: source_ability.0,
            grant_id: grant.0,
        },
        AbilityOrigin::EmblemGranted {
            source,
            source_ability,
            grant,
        } => AbilityOriginSnapshot::EmblemGranted {
            source: source.0,
            source_ability_id: source_ability.0,
            grant_id: grant.0,
        },
        AbilityOrigin::FaceDownGranted {
            source,
            source_ability,
            grant,
        } => AbilityOriginSnapshot::FaceDownGranted {
            source: source.0,
            source_ability_id: source_ability.0,
            grant_id: grant.0,
        },
    }
}

fn ability_origin_from_snapshot(origin: AbilityOriginSnapshot) -> AbilityOrigin {
    match origin {
        AbilityOriginSnapshot::Printed {
            definition,
            part_id,
            ability_id,
        } => AbilityOrigin::Printed {
            definition,
            part: CardPartId(part_id),
            ability: AbilityId(ability_id),
        },
        AbilityOriginSnapshot::Token {
            part_id,
            ability_id,
        } => AbilityOrigin::Token {
            part: CardPartId(part_id),
            ability: AbilityId(ability_id),
        },
        AbilityOriginSnapshot::Emblem { ability_id } => AbilityOrigin::Emblem {
            ability: AbilityId(ability_id),
        },
        AbilityOriginSnapshot::FaceDown { ability_id } => AbilityOrigin::FaceDown {
            ability: AbilityId(ability_id),
        },
        AbilityOriginSnapshot::IntrinsicBasicLand { land_type } => {
            AbilityOrigin::IntrinsicBasicLand(parse_basic_land_type(land_type))
        }
        AbilityOriginSnapshot::IntrinsicCounter { counter } => {
            AbilityOrigin::IntrinsicCounter(counter.0)
        }
        AbilityOriginSnapshot::Granted {
            source,
            source_definition,
            source_part_id,
            source_ability_id,
            grant_id,
        } => AbilityOrigin::Granted {
            source: GameObjectId(source),
            source_definition,
            source_part: CardPartId(source_part_id),
            source_ability: AbilityId(source_ability_id),
            grant: GrantId(grant_id),
        },
        AbilityOriginSnapshot::TokenGranted {
            source,
            source_part_id,
            source_ability_id,
            grant_id,
        } => AbilityOrigin::TokenGranted {
            source: GameObjectId(source),
            source_part: CardPartId(source_part_id),
            source_ability: AbilityId(source_ability_id),
            grant: GrantId(grant_id),
        },
        AbilityOriginSnapshot::EmblemGranted {
            source,
            source_ability_id,
            grant_id,
        } => AbilityOrigin::EmblemGranted {
            source: GameObjectId(source),
            source_ability: AbilityId(source_ability_id),
            grant: GrantId(grant_id),
        },
        AbilityOriginSnapshot::FaceDownGranted {
            source,
            source_ability_id,
            grant_id,
        } => AbilityOrigin::FaceDownGranted {
            source: GameObjectId(source),
            source_ability: AbilityId(source_ability_id),
            grant: GrantId(grant_id),
        },
    }
}

const fn object_kind_snapshot(kind: ObjectKind) -> ObjectKindSnapshot {
    match kind {
        ObjectKind::Card(definition) => ObjectKindSnapshot::Card { definition },
        ObjectKind::Token => ObjectKindSnapshot::Token,
        ObjectKind::Emblem => ObjectKindSnapshot::Emblem,
        ObjectKind::Ability => ObjectKindSnapshot::Ability,
    }
}

fn object_kind_from_snapshot(
    snapshot: ObjectKindSnapshot,
    catalog: &CardCatalog,
) -> Result<ObjectKind, String> {
    match snapshot {
        ObjectKindSnapshot::Card { definition } => {
            catalog
                .get(definition)
                .ok_or("checkpoint object card definition is absent from this catalog")?;
            Ok(ObjectKind::Card(definition))
        }
        ObjectKindSnapshot::Token => Ok(ObjectKind::Token),
        ObjectKindSnapshot::Emblem => Ok(ObjectKind::Emblem),
        ObjectKindSnapshot::Ability => Ok(ObjectKind::Ability),
    }
}

const fn basic_land_type_snapshot(value: BasicLandType) -> BasicLandTypeSnapshot {
    match value {
        BasicLandType::Plains => BasicLandTypeSnapshot::Plains,
        BasicLandType::Island => BasicLandTypeSnapshot::Island,
        BasicLandType::Swamp => BasicLandTypeSnapshot::Swamp,
        BasicLandType::Mountain => BasicLandTypeSnapshot::Mountain,
        BasicLandType::Forest => BasicLandTypeSnapshot::Forest,
    }
}

const fn parse_basic_land_type(value: BasicLandTypeSnapshot) -> BasicLandType {
    match value {
        BasicLandTypeSnapshot::Plains => BasicLandType::Plains,
        BasicLandTypeSnapshot::Island => BasicLandType::Island,
        BasicLandTypeSnapshot::Swamp => BasicLandType::Swamp,
        BasicLandTypeSnapshot::Mountain => BasicLandType::Mountain,
        BasicLandTypeSnapshot::Forest => BasicLandType::Forest,
    }
}

const fn turn_phase_snapshot(value: TurnPhaseDef) -> TurnPhaseSnapshot {
    match value {
        TurnPhaseDef::Combat => TurnPhaseSnapshot::Combat,
        TurnPhaseDef::PostcombatMain => TurnPhaseSnapshot::PostcombatMain,
    }
}

const fn parse_turn_phase(value: TurnPhaseSnapshot) -> TurnPhaseDef {
    match value {
        TurnPhaseSnapshot::Combat => TurnPhaseDef::Combat,
        TurnPhaseSnapshot::PostcombatMain => TurnPhaseDef::PostcombatMain,
    }
}

fn turn_phase_resume_snapshot(value: TurnPhaseResume) -> TurnPhaseResumeSnapshot {
    match value {
        TurnPhaseResume::Step(Step::PrecombatMain) => TurnPhaseResumeSnapshot::PrecombatMain,
        TurnPhaseResume::Step(Step::BeginningOfCombat) => {
            TurnPhaseResumeSnapshot::BeginningOfCombat
        }
        TurnPhaseResume::Step(Step::PostcombatMain) => TurnPhaseResumeSnapshot::PostcombatMain,
        TurnPhaseResume::Step(Step::End) => TurnPhaseResumeSnapshot::End,
        TurnPhaseResume::NextTurn => TurnPhaseResumeSnapshot::NextTurn,
        TurnPhaseResume::Step(
            Step::Upkeep
            | Step::Draw
            | Step::DeclareAttackers
            | Step::DeclareBlockers
            | Step::CombatDamage
            | Step::EndOfCombat
            | Step::Cleanup,
        ) => unreachable!("turn phase scheduler stored a non-phase continuation"),
    }
}

const fn parse_turn_phase_resume(value: TurnPhaseResumeSnapshot) -> TurnPhaseResume {
    match value {
        TurnPhaseResumeSnapshot::PrecombatMain => TurnPhaseResume::Step(Step::PrecombatMain),
        TurnPhaseResumeSnapshot::BeginningOfCombat => {
            TurnPhaseResume::Step(Step::BeginningOfCombat)
        }
        TurnPhaseResumeSnapshot::PostcombatMain => TurnPhaseResume::Step(Step::PostcombatMain),
        TurnPhaseResumeSnapshot::End => TurnPhaseResume::Step(Step::End),
        TurnPhaseResumeSnapshot::NextTurn => TurnPhaseResume::NextTurn,
    }
}

/// One copied ability written down, or `None` when the catalog no longer
/// holds it -- which is what marks a checkpoint as carrying state it cannot
/// write.
fn copiable_ability_snapshot(
    catalog: &CardCatalog,
    ability: &CopiableAbility,
) -> Option<CopiableAbilitySnapshot> {
    Some(CopiableAbilitySnapshot {
        origin: ability_origin_snapshot(ability.origin),
        ability: ability_locator(catalog, |candidate| *candidate == ability.definition)?,
    })
}

/// The inverse of [`copiable_ability_snapshot`].
fn parse_copiable_ability(
    snapshot: &CopiableAbilitySnapshot,
    catalog: &CardCatalog,
) -> Result<CopiableAbility, String> {
    Ok(CopiableAbility {
        origin: ability_origin_from_snapshot(snapshot.origin),
        definition: catalog_ability(catalog, &snapshot.ability)
            .ok_or_else(|| "checkpoint copied ability locator is absent from this catalog".to_owned())?,
    })
}
