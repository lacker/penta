// Native proposals share canonical identifiers, but omit redundant action labels.
const id = { type: "integer", minimum: 0, maximum: 4294967295 };
const byte = { type: "integer", minimum: 0, maximum: 255 };
const word = { type: "integer", minimum: 0, maximum: 65535 };
const unit = { type: "integer", minimum: 0, maximum: Number.MAX_SAFE_INTEGER };
const str = { type: "string" };
const ref = name => ({ $ref: `#/$defs/${name}` });
const array = items => ({ type: "array", items, maxItems: 4096 });
const nullable = schema => ({ anyOf: [schema, { type: "null" }] });
const object = (properties, required = Object.keys(properties)) => ({ type: "object",
  properties: Object.fromEntries(Object.entries(properties).map(([key, value]) => [key, required.includes(key) ? value : nullable(value)])),
  required, additionalProperties: false });
const action = (type, properties = {}, required = Object.keys(properties)) => object({ type: { const: type }, ...properties }, ["type", ...required]);
const origin = (kind, fields) => object({ kind: { const: kind }, ...fields });
const ids = array(id);
const bytes = array(byte);
const units = array(unit);
const selections = array(ref("ProposalTargetSelection"));
const colors = ["white", "blue", "black", "red", "green", "colorless"];
const split = { type: "object", properties: Object.fromEntries(colors.map(color => [color, word])), required: [], additionalProperties: false };

export const proposalDefinitions = {
  ProposalTarget: { oneOf: [object({ type: { const: "player" }, seat: ref("Seat") }),
    ...["permanent", "card", "spell"].map(type => object({ type: { const: type }, objectId: id }))] },
  ProposalTargetSelection: object({ slotId: byte, targets: array(ref("ProposalTarget")), amounts: nullable(array(word)) }, ["slotId"]),
  ProposalAbility: { oneOf: [
    origin("printed", { definition: str, partId: byte, abilityId: byte }),
    origin("token", { partId: byte, abilityId: byte }),
    ...["emblem", "faceDown"].map(kind => origin(kind, { abilityId: byte })),
    origin("intrinsicBasicLand", { landType: { enum: ["plains", "island", "swamp", "mountain", "forest"] } }),
    origin("intrinsicCounter", { counter: str }),
    origin("granted", { source: id, sourceDefinition: str, sourcePartId: byte, sourceAbilityId: byte, grantId: byte }),
    origin("tokenGranted", { source: id, sourcePartId: byte, sourceAbilityId: byte, grantId: byte }),
    ...["emblemGranted", "faceDownGranted"].map(kind => origin(kind, { source: id, sourceAbilityId: byte, grantId: byte })),
  ] },
  ProposalManaPayment: array(object({ symbol: str, count: word })),
  ProposalCastChoices: object({ playOptionId: byte, modeIds: bytes, targetSelections: selections, x: word,
    alternativeCostId: nullable(byte), additionalCostIds: bytes, chosenCreatureType: str, permissionSource: id,
    splicedCards: ids, manaPayment: ref("ProposalManaPayment") }, []),
  ProposalManaAction: { anyOf: ["color", "combination"].map(required => action("ActivateManaAbility", {
    source: id, ability: ref("ProposalAbility"), color: { enum: colors }, countersRemoved: word,
    costObject: id, combination: split, triggeredManaChoices: array(split) }, ["source", "ability", required])) },
  ProposalAction: { oneOf: [
    ...["KeepHand", "TakeMulligan", "PassPriority", "BeginPayment", "FinishDeclaringAttackers", "FinishDeclaringBlockers", "Concede"].map(type => action(type)),
    ...["BottomCards", "DiscardCards"].map(type => action(type, { cards: ids }, [])),
    action("ChooseDecision", { decision: id, options: ids }, ["decision"]), action("CancelDecision", { decision: id }),
    action("ChooseUntap", { permanents: ids }, []),
    ...["Foretell", "Plot", "TakeCompanion"].map(type => action(type, { card: id })),
    action("Suspend", { card: id, ability: ref("ProposalAbility"), x: word }, ["card", "ability"]),
    action("UnlockDoor", { room: id, door: byte }), action("TurnFaceUp", { permanent: id }),
    action("PlayLand", { card: id, playOptionId: byte }, ["card"]), ref("ProposalManaAction"),
    action("CastSpell", { card: id, choices: ref("ProposalCastChoices"), costObjects: ids }, ["card"]),
    action("ActivateAbility", { source: id, ability: ref("ProposalAbility"), targetSelections: selections,
      costObjects: ids, x: word, modeIds: bytes, manaPayment: nullable(ref("ProposalManaPayment")),
      alternativeCost: object({ source: id, ability: ref("ProposalAbility") }) }, ["source", "ability"]),
    action("DeclareAttacker", { attacker: id, defender: { oneOf: [object({ type: { const: "player" }, seat: ref("Seat") }), object({ type: { const: "planeswalker" }, objectId: id })] } }),
    action("ExertAttacker", { attacker: id }), action("BandAttackers", { first: id, second: id }),
    action("DeclareBlocker", { blocker: id, attacker: id }),
    action("AssignCombatDamage", { attacker: id, assignments: array(object({ recipient: ref("ProposalTarget"), amount: word })) }, ["attacker"]),
  ] },
  ProposalFunding: object({ action: ref("ProposalManaAction"), manaUnits: units, answers: array(ids) }, ["action"]),
  ProposalPayment: object({ funding: array(ref("ProposalFunding")), contributions: array(object({ source: id, kind: { enum: ["convoke", "delve", "improvise", "waterbend"] }, symbol: str })), announcements: array(ids), manaUnits: units }, []),
  NativeProposal: object({ action: ref("ProposalAction"), payment: ref("ProposalPayment") }, ["action"]),
};
export const proposalChoice = object({ proposal: ref("NativeProposal") });
