// Which object predicates a static ability may read.
//
// Split out of `program_context.rs` to keep that file inside the source-size
// limit. These two lists answer one question -- what the layer walk can see
// about an object -- and the runtime keeps its own copies of them.

/// Which predicates a static animation's own query may read.
///
/// The rule is stratification, not a list of favourites. A static animation
/// may ask whether an object is already a creature because CR 613.6 pins a
/// compound effect's recipient set when its layer-4 component starts; its
/// later components do not reselect after the effect supplies Creature. It
/// still may not ask about colour, which an animation can repaint without an
/// earlier component pinning the selection. Everything else below is another
/// card type, a non-land subtype, attachment, or which object is the source.
/// A basic land subtype remains excluded because layer-4 operations supply it.
/// The two `Game::static_*animation_predicate_is_supported` methods are the
/// runtime's copies of this list; both sides are meant to say the same thing.
fn static_animation_predicate_supported(predicate: ObjectPredicateDef, creature: bool) -> bool {
    match predicate {
        ObjectPredicateDef::Subtype(name) => !crate::card::BasicLandType::ALL
            .iter()
            .any(|land_type| land_type.subtype() == name),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::HasType(
            CardType::Land | CardType::Enchantment | CardType::Artifact,
        ) => true,
        ObjectPredicateDef::HasType(CardType::Creature) => creature,
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .all(|predicate| static_animation_predicate_supported(predicate, creature)),
        ObjectPredicateDef::Not(predicate) => {
            static_animation_predicate_supported(*predicate, creature)
        }
        _ => false,
    }
}

fn static_object_predicate_supported(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .all(static_object_predicate_supported),
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            static_object_predicate_supported(*predicate)
        }
        ObjectPredicateDef::ControlledBy(relation) | ObjectPredicateDef::OwnedBy(relation) => {
            static_player_relation_supported(relation)
        }
        ObjectPredicateDef::SharesNameWithAny(objects) => {
            static_condition_object_set_supported(*objects)
        }
        ObjectPredicateDef::ManaValueEqualTo(value)
        | ObjectPredicateDef::ManaValueAtMostValue(value)
        | ObjectPredicateDef::ToughnessLessThan(value)
        | ObjectPredicateDef::PowerGreaterThan(value)
        | ObjectPredicateDef::ToughnessGreaterThan(value)
        | ObjectPredicateDef::PowerLessThan(value) => static_source_value_supported(value),
        ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::HasName(
            ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::CreatingSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::ResolvingObject
            | ObjectRefDef::AttachedToSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_),
        )
        // A printed cost shape is only readable where the card's own
        // definition is in hand, which a static continuous effect does not
        // have; the zone-search path answers it instead.
        | ObjectPredicateDef::GenericManaCostAtMost(_)
        | ObjectPredicateDef::Special(_) => false,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameIsBasicLandName
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::HasAnyCounter
        | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasName(ObjectRefDef::Source)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        // A comparison of the object with itself reads nothing the walk
        // supplies, so there is no value to gate on.
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn => true,
    }
}
