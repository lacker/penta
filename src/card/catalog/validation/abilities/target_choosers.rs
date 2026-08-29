/// The activation declaration protocol can stop before costs are paid and
/// hand Arena's final target to the opponent. More general interleaving,
/// optional counts, and divided choices need richer continuations first.
fn validate_activated_target_choosers(
    targets: &[crate::card::AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    for (index, target) in targets.iter().enumerate() {
        if target.chooser == TargetChooserDef::Controller {
            continue;
        }
        let is_supported = target.chooser == TargetChooserDef::Opponent
            && index + 1 == targets.len()
            && target.minimum == 1
            && target.maximum == 1
            && target.divided_total.is_none();
        if !is_supported {
            return Err(
                GrantedAbilityValidationError::UnsupportedActivatedTargetChoice {
                    target: TargetIndex::from_index(index)
                        .expect("the target count is validated before chooser shapes"),
                },
            );
        }
    }
    Ok(())
}
