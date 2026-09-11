// Durable labels on entry-time scalar choices.
fn parse_choice_binding(label: Option<&str>) -> Result<Option<crate::Binding>, String> {
    label
        .map(|label| {
            crate::Binding::try_from_label(label)
                .ok_or_else(|| format!("unknown choice binding label {label:?}"))
        })
        .transpose()
}
