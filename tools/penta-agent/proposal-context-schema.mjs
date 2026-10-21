const number = { type: "integer", minimum: 0 };
const byte = { ...number, maximum: 255 };
const word = { ...number, maximum: 65535 };
const id = { ...number, maximum: 4294967295 };
const text = { type: "string" };
const boolean = { type: "boolean" };
const ref = name => ({ $ref: `#/$defs/${name}` });
const array = items => ({ type: "array", items });
const object = (properties, required = Object.keys(properties)) => ({ type: "object", properties, required, additionalProperties: false });

export const proposalContextDefinitions = {
  ProposalDecisionDefaults: object({ abilityText: { type: "null" }, card: { type: "null" },
    members: { type: "array", items: ref("JsonValue"), maxItems: 0 }, triggerId: { type: "null" }, zone: { const: "None" } }, []),
  ProposalDecisionOptions: object({ items: array(ref("ExactObject")), total: number,
    reference: { type: "null" }, defaults: ref("ProposalDecisionDefaults") }),
  ProposalActionContext: object({
    kind: { enum: ["complete", "waiting", "priority", "decision", "mulligan", "bottomCards", "discardCards", "untap", "declareAttackers", "declareBlockers", "combatDamage"] },
    ordinaryPriority: boolean, minimum: number, maximum: number, attacker: id,
    castOffers: array(ref("ProposalCastOffer")),
  }, ["kind", "ordinaryPriority"]),
  ProposalCastOffer: object({ card: id,
    sourceZone: { enum: ["hand", "command", "graveyard", "exile", "libraryTop"] },
    cost: { oneOf: [object({ kind: { const: "any" } }),
      object({ kind: { const: "printedAlternative" }, ability: ref("ProposalAbility") }, ["kind"]),
      object({ kind: { const: "grantedAlternative" }, index: number })] },
    mandatory: boolean, decision: id,
  }),
  ProposalAbilitySlot: object({ slotId: byte, minimum: byte, maximum: byte,
    countFromRules: boolean, another: boolean, excludesSource: boolean,
    dividedTotal: { anyOf: [{ type: "null" }, object({ kind: { const: "fixed" }, amount: word }), object({ kind: { const: "chosenX" } })] },
  }),
  ProposalAbilityMode: object({ modeId: byte, text, targetSlots: array(ref("ProposalAbilitySlot")) }),
  ProposalAbilityModes: object({ minimum: byte, maximum: byte, mayRepeat: boolean, choices: array(ref("ProposalAbilityMode")) }),
  ProposalAbilityDescription: object({ ability: ref("ProposalAbility"),
    kind: { enum: ["spell", "activated", "mana", "triggered", "triggeredMana", "static", "replacement", "alternativeCast", "additionalCost", "special", "pregame", "keyword", "deckConstruction", "companion"] },
    text, targetSlots: array(ref("ProposalAbilitySlot")),
    modes: { anyOf: [{ type: "null" }, ref("ProposalAbilityModes")] },
  }),
  ProposalGrantedCastCost: object({ playOptionId: byte, alternativeCostId: byte,
    kind: { enum: ["granted", "battlefield"] }, text, manaCost: text }),
  ProposalPlayPermission: object({ sourceZone: { enum: ["exile", "graveyard"] },
    cost: object({ kind: { enum: ["printed", "alternativeMana", "free", "energyEqualToManaValue", "foretell"] },
      manaCost: { anyOf: [text, { type: "null" }] } }),
    surcharge: text, landsMayBePlayed: boolean, adventureReturnOnly: boolean,
    maximumSpellManaValue: { anyOf: [number, { type: "null" }] },
    spendAnyColor: boolean, spendAnyType: boolean, grantsHaste: boolean }),
  ProposalObjectAbilities: { oneOf: [
    object({ source: id, abilities: array(ref("ProposalAbilityDescription")),
      playPermission: { anyOf: [ref("ProposalPlayPermission"), { type: "null" }] },
      grantedCastCosts: array(ref("ProposalGrantedCastCost")) }, ["source", "abilities"]),
    object({ source: id, abilitiesPreviouslySent: { const: true } }),
  ] },
};
