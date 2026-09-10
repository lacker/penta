// Payment obligations reconstructed against their authored program.

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub(super) enum ResolvedEffectPaymentSnapshot {
    Action { source: u32, amount: u16 },
    All(Vec<Self>),
    Choice(Vec<Self>),
    Mana(ManaCostSnapshot),
    LabeledMana {
        label: String,
        source: u32,
        cost: ManaCostSnapshot,
    },
    SnowMana {
        label: Option<String>,
        source: u32,
        amount: u16,
    },
    Life(u16),
    DrawCards(u16),
    DiscardCards(u16),
    PutCounters {
        object: u32,
        kind: CounterKindSnapshot,
        amount: u16,
        times: u16,
    },
    SacrificePermanents(u16),
    ExileTopCards(u16),
    AddMana {
        color: ManaColorSnapshot,
        amount: u16,
    },
    OpponentGainsLife(u16),
    OpponentCreatesTokens(u16),
    // Reserved legacy format-15 tag; new obligations use Action.
    GainControlPermanents {
        source: u32,
        amount: u16,
    },
    FlipCoins(u16),
    Energy(u16),
    /// Appended after the first two, so a checkpoint written before this
    /// payment existed still reads as one of them.
    Mill(u16),
    Discard(u16),
    /// Which cards match is read back from the authored effect rather than
    /// carried here: the predicate is a static definition, and the payment
    /// this describes is only ever restored beside the ability that named it.
    DiscardMatching,
    /// Likewise: how much can be paid is read off the payer's mana rather
    /// than written down, because the options are rebuilt from it.
    ChosenGenericMana,
    /// The same for energy: how much can be paid is read off the payer's
    /// counters rather than written down.
    ChosenEnergy,
    RemoveAnyNumberOfCounters {
        object: u32,
        kind: CounterKindSnapshot,
    },
    /// Compatibility spelling for the matching-permanent move payment. The
    /// authored effect restores its destination, so the old hand-only tag can
    /// represent the generalized internal cost without changing wire data.
    ReturnPermanentMatching,
    /// The same, for the one it sacrifices.
    SacrificePermanentMatching,
    SacrificeCreaturesWithTotalPower(u16),
}
