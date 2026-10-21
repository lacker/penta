import { createHash } from "node:crypto";
import { PilotView } from "./pilot-view.mjs";
import { validatePilotV3 } from "./pilot-v3-contract.mjs";

/** Proposal input is complete seat-visible state, never an action menu. */
export class PilotV3View extends PilotView {
  constructor(view, catalog) {
    if (view.apiVersion !== 2) throw new Error("pilot-v3 requires native API 2");
    if (view.observation && view.observation.actionInterface !== "proposal-v1") {
      throw new Error("pilot-v3 requires a proposal-v1 observation");
    }
    // Reuse the state partition and references without asking the engine to
    // enumerate actions. The temporary empty list exists only in this adapter.
    super(view.observation ? { ...view, apiVersion: 1, observation: { ...view.observation, legalActions: [] } } : { ...view, apiVersion: 1 }, catalog);
    const packet = this.packet;
    packet.presentation = "pilot-v3";
    packet.host.apiVersion = 2;
    if (Object.hasOwn(packet.host, "yield")) {
      packet.host.currentYield = packet.host.yield;
      delete packet.host.yield;
    }
    if (packet.host.receipt && Object.hasOwn(packet.host.receipt, "yield")) {
      packet.host.receipt.yieldAtAcceptance = packet.host.receipt.yield;
      delete packet.host.receipt.yield;
    }
    if (packet.position) {
      const position = packet.position;
      delete position.choices;
      delete position.facts.actionInterface;
      position.actionInterface = "proposal-v1";
      for (const field of ["actionContext", "actionAbilities"]) {
        position[field] = position.facts[field];
        delete position.facts[field];
      }
      position.reference = this.reference("proposal observation", view.observation);
      if (position.decision) {
        const options = position.decision.options;
        const defaults = {};
        // Factor only established empty metadata, never identities, labels, or
        // unknown fields. Presence checks preserve absent versus explicit null.
        for (const [key, empty] of Object.entries({ abilityText: null, card: null, members: [], triggerId: null, zone: "None" })) {
          if (options.items.length && options.items.every(item => Object.hasOwn(item, key))
              && options.items.some(item => JSON.stringify(item[key]) === JSON.stringify(empty))) defaults[key] = empty;
        }
        position.decision.options = { ...options, defaults, items: options.items.map(item =>
          Object.fromEntries(Object.entries(item).filter(([key, value]) =>
            !Object.hasOwn(defaults, key) || JSON.stringify(value) !== JSON.stringify(defaults[key])))) };
      }
      position.rules.items = position.rules.items.map(rule => ({ ...rule,
        printed: rule.reference ? inlinePrintedRules(this.references.get(rule.reference).value) : null }));
    }
    validatePilotV3(packet);
  }

  collection(_kind, items) {
    if (!Array.isArray(items)) throw new Error("collection must be an array");
    return { items, total: items.length, reference: null };
  }

  reference(kind, value) {
    const suffix = kind === "printed card reference" ? `:${value.definition}` : "";
    const reference = `p3r:${createHash("sha256").update(JSON.stringify([this.id, kind, suffix])).digest("base64url")}`;
    this.references.set(reference, { kind, value: structuredClone(value) });
    return reference;
  }
}

function inlinePrintedRules(card) {
  // Omit only catalog presentation metadata. Keep unknown fields and nested
  // gameplay records intact; the reference still resolves to the full card.
  const { printings: _printings, art: _art, ...rules } = card;
  return rules;
}
