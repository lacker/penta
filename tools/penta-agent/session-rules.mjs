// JSON object member order is not semantic; array order and all values are.
function semanticKey(value) {
  if (Array.isArray(value)) return `[${value.map(semanticKey).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${semanticKey(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

/** Rules knowledge belongs only to this live session and successful writes. */
export class SessionRules {
  #printed = new Map();
  #abilities = new Map();

  reset() { this.#printed.clear(); this.#abilities.clear(); }

  prepare(packet) {
    const collection = ["pilot-v2", "pilot-v3"].includes(packet?.presentation) ? packet.position?.rules : null;
    const abilities = packet?.presentation === "pilot-v3" ? packet.position?.actionAbilities : null;
    if (!Array.isArray(collection?.items) && !Array.isArray(abilities)) return { packet, commit() {} };
    const updates = new Map();
    const items = collection?.items?.map(rule => {
      if (!rule.available || rule.printed == null) {
        // Unavailable or omitted rules do not teach the session their contents.
        // Forget unavailable definitions so recovery explicitly prints them.
        if (!rule.available) updates.set(rule.definition, null);
        return rule;
      }
      const { reference: _reference, ...known } = rule;
      const key = semanticKey(known);
      updates.set(rule.definition, key);
      if (this.#printed.get(rule.definition) !== key) return rule;
      return { definition: rule.definition, printed: null, printedPreviouslySent: true, reference: rule.reference };
    });
    // The next map contains only currently visible sources. A disappeared source
    // must teach its metadata again if it later becomes visible.
    const nextAbilities = new Map();
    const actionAbilities = abilities?.map(record => {
      if (!Array.isArray(record.abilities)) return record;
      const key = semanticKey(record);
      nextAbilities.set(record.source, key);
      return this.#abilities.get(record.source) === key
        ? { source: record.source, abilitiesPreviouslySent: true } : record;
    });
    return {
      packet: { ...packet, position: { ...packet.position,
        ...(items ? { rules: { ...collection, items } } : {}),
        ...(actionAbilities ? { actionAbilities } : {}),
      } },
      // Call only after stdout accepted the complete response. Merely preparing
      // output (including a response whose write failed) establishes no knowledge.
      commit: () => {
        for (const [definition, key] of updates) {
          if (key === null) this.#printed.delete(definition);
          else this.#printed.set(definition, key);
        }
        if (actionAbilities) this.#abilities = nextAbilities;
      },
    };
  }
}
