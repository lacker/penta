import { pilotSchema, validateContract } from "./pilot-contract.mjs";
import { pilotV2Schema } from "./pilot-v2-contract.mjs";

import { proposalContextDefinitions } from "./proposal-context-schema.mjs";
import { proposalDefinitions, proposalChoice } from "./proposal-schema.mjs";

export function composePilotV3Schema() {
  const schema = structuredClone(pilotV2Schema);
  schema.$id = "https://penta.local/schemas/pilot-v3.json";
  schema.title = "Penta native pilot-v3 proposals";
  schema.$defs.Host.properties.apiVersion = { const: 2 };
  for (const variant of schema.$defs.PilotPacket.oneOf) variant.properties.presentation.const = "pilot-v3";
  const position = schema.$defs.Position;
  delete position.properties.choices;
  delete position.properties.choiceGroups;
  position.required = position.required.filter(key => key !== "choices");
  position.properties.actionInterface = { const: "proposal-v1" };
  position.properties.actionContext = { $ref: "#/$defs/ProposalActionContext" };
  position.properties.actionAbilities = { type: "array", items: { $ref: "#/$defs/ProposalObjectAbilities" } };
  position.required.push("actionInterface", "actionContext", "actionAbilities");
  position.properties.zones = structuredClone(pilotSchema.$defs.Position.properties.zones);
  schema.$defs.Decision.properties.options = { $ref: "#/$defs/ProposalDecisionOptions" };
  schema.$defs.CompleteRule.properties.printed = { anyOf: [{ $ref: "#/$defs/ExactObject" }, { type: "null" }] };
  Object.assign(schema.$defs, proposalDefinitions, proposalContextDefinitions);
  schema.$defs.NativePlayChoice.oneOf.push(proposalChoice);
  schema.$defs.NativeInteractionChoice.oneOf.push(proposalChoice);
  schema.$defs.Receipt.properties.rejection = { type: "object", properties: {
    code: { type: "string" }, path: { type: "string" }, message: { type: "string" },
  }, required: ["code", "path", "message"], additionalProperties: false };
  return schema;
}

export const pilotV3Schema = composePilotV3Schema();
export function validatePilotV3(packet) {
  validateContract(packet, "PilotPacket", pilotV3Schema);
  if (packet.host.status !== packet.status) throw new Error("host and pilot status disagree");
  if (packet.position) {
    const position = packet.position;
    for (const collection of [...Object.values(position.zones), position.updates, position.decision?.options, position.rules].filter(Boolean)) {
      if (collection.reference !== null || collection.items.length !== collection.total) throw new Error("proposal state must be complete inline");
    }
  }
  return packet;
}
