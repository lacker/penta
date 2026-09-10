/// Flattens groups of ordinary abilities into one constant array, in order.
/// Mechanic constructors can return several clauses without requiring their
/// callers to select each clause separately.
///
/// ```
/// use penta::{ability_list, card::abilities, CostDef, mana_cost};
/// const ABILITIES: [penta::AbilityDef; 3] = ability_list![
///     [abilities::flying()],
///     abilities::evoke(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
/// ];
/// ```
#[macro_export]
macro_rules! ability_list {
    ($($group:expr),* $(,)?) => {{
        const GROUPS: &[&[$crate::AbilityDef]] = &[$(&$group),*];
        const LENGTH: usize = {
            let mut length = 0;
            let mut group = 0;
            while group < GROUPS.len() {
                length += GROUPS[group].len();
                group += 1;
            }
            length
        };
        const RESULT: [$crate::AbilityDef; LENGTH] = {
            let mut result = [$crate::AbilityDef::spell("", $crate::EffectDef::None); LENGTH];
            let mut position = 0;
            let mut group = 0;
            while group < GROUPS.len() {
                let mut clause = 0;
                while clause < GROUPS[group].len() {
                    result[position] = GROUPS[group][clause];
                    position += 1;
                    clause += 1;
                }
                group += 1;
            }
            result
        };
        RESULT
    }};
}

#[cfg(test)]
mod tests {
    use crate::card::abilities;
    use crate::{AbilityDef, CardRules, CostDef, mana_cost};

    #[test]
    fn evoke_ability_groups_preserve_order_and_empty_groups() {
        const EMPTY: [AbilityDef; 0] = crate::ability_list![];
        const RULES: CardRules = CardRules::new_creature(mana_cost!("{4}{U}"), &[], 2, 2)
            .with_abilities(&crate::ability_list![
                [abilities::flying()],
                EMPTY,
                abilities::evoke(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
                [abilities::haste()],
            ]);
        assert_eq!(
            RULES.rules_text(),
            "Flying\nEvoke {2}{U}\nWhen this creature enters, if it was evoked, sacrifice it.\nHaste",
        );
        assert_eq!(RULES.ability_clauses().len(), 4);
    }
}
