fn entry_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_)
        | ValueDef::SourceCastX
        | ValueDef::AdditionalCostPayments(_)
        | ValueDef::ColorsOfManaSpent => true,
        ValueDef::CountObjects(objects) | ValueDef::CardTypesAmongObjects(objects) => {
            entry_object_set_supported(*objects)
        }
        ValueDef::Negate(value) => entry_value_supported(*value),
        ValueDef::Scaled(scaled) => entry_value_supported(scaled.value),
        ValueDef::Sum(sum) => entry_value_supported(sum.left) && entry_value_supported(sum.right),
        ValueDef::IfAdditionalCostPaid(conditional) => {
            entry_value_supported(conditional.if_paid)
                && entry_value_supported(conditional.otherwise)
        }
        ValueDef::Halved(halved) => entry_value_supported(halved.value),
        _ => false,
    }
}

fn entry_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::LinkedExiles => true,
        ObjectSetDef::Matching { objects, .. } => entry_object_set_supported(*objects),
        _ => false,
    }
}
