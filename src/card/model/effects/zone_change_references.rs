/// An exact object reference whose one zone-change successor may be named by
/// another part of the same resolving instruction (CR 400.7j).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneChangeReferenceDef {
    Source,
    AbilityGrantSource,
    ResolvingObject,
    Binding(ObjectBindingIndex),
    AdditionalCostObject(AdditionalCostObjectIndex),
    AttachedToSource,
    Target(TargetIndex),
    TriggeringObject,
    SourceOfTargetedStackObject(TargetIndex),
}

impl ZoneChangeReferenceDef {
    pub const fn exact(self) -> ObjectRefDef {
        match self {
            Self::Source => ObjectRefDef::Source,
            Self::AbilityGrantSource => ObjectRefDef::AbilityGrantSource,
            Self::ResolvingObject => ObjectRefDef::ResolvingObject,
            Self::Binding(binding) => ObjectRefDef::Binding(binding),
            Self::AdditionalCostObject(index) => ObjectRefDef::AdditionalCostObject(index),
            Self::AttachedToSource => ObjectRefDef::AttachedToSource,
            Self::Target(target) => ObjectRefDef::Target(target),
            Self::TriggeringObject => ObjectRefDef::TriggeringObject,
            Self::SourceOfTargetedStackObject(target) => {
                ObjectRefDef::SourceOfTargetedStackObject(target)
            }
        }
    }
}
