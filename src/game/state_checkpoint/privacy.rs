use super::{Game, PlayerId, ReplaceableEvent, Value};

impl Game {
    /// The hypothesis format can replace hidden-zone cards, but cannot yet
    /// bind hidden identities beneath public stack or battlefield objects.
    /// Withhold the entire executable checkpoint: object kinds, copy values,
    /// signatures, and detached continuations can all repeat that identity.
    pub(super) fn checkpoint_has_private_face_down_objects(&self, viewer: PlayerId) -> bool {
        self.battlefield
            .iter()
            .chain(&self.phased_out)
            .any(|permanent| permanent.face_down.is_some() && permanent.controller != viewer)
            || self
                .stack
                .iter()
                .any(|object| object.face_down.is_some() && object.controller != viewer)
            || self.pending_events.iter().any(|event| {
                let ReplaceableEvent::BattlefieldEntry(entry) = &event.event;
                entry.permanent.face_down.is_some() && entry.permanent.controller != viewer
            })
    }

    /// Nested continuations can retain an object after it has left the live
    /// zones and retired-object map. Only a live object the viewer controls
    /// supplies an unambiguous authorization to export its masked identity.
    pub(super) fn checkpoint_contains_detached_face_down(
        &self,
        value: &Value,
        viewer: PlayerId,
    ) -> bool {
        match value {
            Value::Array(values) => values
                .iter()
                .any(|value| self.checkpoint_contains_detached_face_down(value, viewer)),
            Value::Object(fields) => {
                let masked_object = fields.contains_key("objectKind")
                    && (fields.get("faceDown").is_some_and(Value::is_string)
                        || fields.get("hasRuntimeOverrides") == Some(&Value::Bool(true))
                        || fields.get("hasDynamicCharacteristics") == Some(&Value::Bool(true)));
                let visible_live_object = masked_object
                    && fields
                        .get("objectId")
                        .and_then(Value::as_u64)
                        .is_some_and(|id| {
                            self.battlefield
                                .iter()
                                .chain(&self.phased_out)
                                .any(|permanent| {
                                    u64::from(permanent.card.id.0) == id
                                        && permanent.controller == viewer
                                })
                                || self.stack.iter().any(|object| {
                                    u64::from(object.id.0) == id && object.controller == viewer
                                })
                        });
                (masked_object && !visible_live_object)
                    || fields
                        .values()
                        .any(|value| self.checkpoint_contains_detached_face_down(value, viewer))
            }
            _ => false,
        }
    }
}

pub(super) fn private_face_down_checkpoint(viewer: PlayerId) -> Value {
    serde_json::json!({
        "version": crate::protocol::CHECKPOINT_VERSION,
        "simulationFingerprint": crate::protocol::SIMULATION_FINGERPRINT,
        "viewer": viewer.index(),
        "hasDeferredState": true,
        "unavailableReason": "hiddenFaceDownObjects",
    })
}
