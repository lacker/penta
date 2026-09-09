use super::ObjectPredicateDef;

/// Restart without a game conclusion. Matching cards linked in exile stay
/// there through pregame, then enter under the effect controller's control.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RestartGameDef {
    pub retained_exiles: ObjectPredicateDef,
}
