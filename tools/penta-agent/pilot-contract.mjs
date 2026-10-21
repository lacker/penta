import { readFileSync } from "node:fs";

export const pilotSchema = JSON.parse(readFileSync(new URL("./pilot.schema.json", import.meta.url), "utf8"));

function invalid(path, message, values, typeMismatch = false) {
  return Object.assign(new Error(`${path}: ${message}`), { path, values, typeMismatch });
}

// Preserve the most specific failure through wrapper/nullable unions. Tag
// mismatches describe the alternatives only when no branch matched that tag.
function unionFailure(errors, path) {
  const depth = error => error.path.split(/[.[\]]/).length;
  const deepest = Math.max(...errors.map(depth));
  const relevant = errors.filter(error => depth(error) === deepest);
  if (relevant.every(error => error.values && error.path === relevant[0].path)) {
    const values = [...new Set(relevant.flatMap(error => error.values))];
    return invalid(relevant[0].path, `expected one of ${values.map(value => JSON.stringify(value)).join(", ")}`, values);
  }
  return relevant.find(error => !error.values && !error.typeMismatch)
    ?? relevant.find(error => !error.values) ?? relevant[0] ?? invalid(path, "invalid union value");
}

/** Validate the deliberately small JSON Schema vocabulary used by this contract.
 * The same schema is consumable by standard draft-2020-12 validators. */
export function validateContract(value, definition = "PilotPacket", schemaDocument = pilotSchema) {
  const visit = (value, schema, path) => {
    if (schema === true) return;
    if (schema === false) throw invalid(path, "unexpected field");
    if (schema.$ref) return visit(value, schemaDocument.$defs[schema.$ref.split("/").at(-1)], path);
    const alternatives = schema.oneOf ?? schema.anyOf;
    if (alternatives) {
      let matches = 0;
      const errors = [];
      for (const alternative of alternatives) {
        try { visit(value, alternative, path); matches++; } catch (error) { errors.push(error); }
      }
      if (!matches) throw unionFailure(errors, path);
      if (schema.oneOf && matches !== 1) throw invalid(path, "matches multiple union alternatives");
      return;
    }
    if (Object.hasOwn(schema, "const") && value !== schema.const) throw invalid(path, `expected ${schema.const}`, [schema.const]);
    if (schema.enum && !schema.enum.includes(value)) throw invalid(path, `expected one of ${schema.enum.map(value => JSON.stringify(value)).join(", ")}`, schema.enum);
    const type = value === null ? "null" : Array.isArray(value) ? "array" : typeof value;
    if (schema.type && !(schema.type === "integer" ? Number.isInteger(value) : type === schema.type)) throw invalid(path, `expected ${schema.type}`, undefined, true);
    if (typeof value === "number" && (!Number.isFinite(value) || value < (schema.minimum ?? -Infinity) || value > (schema.maximum ?? Infinity))) throw invalid(path, "number out of bounds");
    if (typeof value === "string") {
      const length = [...value].length;
      if (length < (schema.minLength ?? 0)) throw invalid(path, "string too short");
      if (length > (schema.maxLength ?? Infinity)) throw invalid(path, "string too long");
    }
    if (Array.isArray(value)) {
      if (value.length < (schema.minItems ?? 0) || value.length > (schema.maxItems ?? Infinity)) throw invalid(path, "invalid item count");
      value.forEach((item, index) => visit(item, schema.items ?? true, `${path}[${index}]`));
    } else if (value && typeof value === "object") {
      // Check discriminants before unrelated required fields so an unknown
      // action reports its allowed tags rather than another action's inputs.
      for (const [key, property] of Object.entries(schema.properties ?? {})) {
        if (Object.hasOwn(property, "const") && Object.hasOwn(value, key)) visit(value[key], property, `${path}.${key}`);
      }
      for (const key of schema.required ?? []) if (!Object.hasOwn(value, key)) throw invalid(`${path}.${key}`, "required");
      for (const [key, item] of Object.entries(value)) visit(item, schema.properties?.[key] ?? schema.additionalProperties ?? true, `${path}.${key}`);
    }
  };
  if (!schemaDocument.$defs[definition]) throw new Error(`unknown contract ${definition}`);
  visit(value, schemaDocument.$defs[definition], definition);
  return value;
}

export function validatePilot(packet) {
  validateContract(packet);
  if (packet.host.status !== packet.status) throw new Error("host and pilot status disagree");
  if (packet.host.revision !== undefined && packet.host.revision !== packet.revision) throw new Error("host and pilot revision disagree");
  const position = packet.position;
  if (position) {
    const collections = [...Object.values(position.zones), position.updates, position.choices,
      position.decision?.options, position.rules].filter(Boolean);
    for (const collection of collections) {
      if (collection.reference === null ? collection.items.length !== collection.total : collection.items.length > collection.total) {
        throw new Error("collection contents disagree with total");
      }
    }
    const tickets = position.choices.items.map(choice => choice.ticket);
    if (new Set(tickets).size !== tickets.length) throw new Error("duplicate action tickets");
  }
  return packet;
}
