/** Lossless field factoring, apart from explicitly identified art metadata. */
export function withoutArt(value) {
  if (Array.isArray(value)) return value.map(withoutArt);
  if (!value || typeof value !== "object") return value;
  return Object.fromEntries(Object.entries(value).filter(([key, child]) => {
    // Only the protocol's known CardArt shape is presentation metadata.
    // Preserve unknown fields, including future fields also named "art".
    return !(key === "art" && child && typeof child === "object"
      && Object.keys(child).length === 2 && Object.hasOwn(child, "scryfallId") && Object.hasOwn(child, "artist"));
  }).map(([key, child]) => [key, withoutArt(child)]));
}

/** Shared fields are present with exactly the same value in EVERY row.
 * Missing, null, false, zero, and empty collections therefore stay distinct.
 * Rows override shared fields; no earlier observation is needed. */
export function factorRows(rows) {
  if (rows.length < 2 || rows.some(row => !row || Array.isArray(row) || typeof row !== "object")) return rows;
  const shared = Object.fromEntries(Object.entries(rows[0]).filter(([key, value]) =>
    rows.every(row => Object.hasOwn(row, key) && JSON.stringify(row[key]) === JSON.stringify(value))));
  if (!Object.keys(shared).length) return rows;
  const result = { shared, rows: rows.map(row => Object.fromEntries(Object.entries(row).filter(([key]) => !Object.hasOwn(shared, key)))) };
  return JSON.stringify(result).length < JSON.stringify(rows).length ? result : rows;
}

export function namedObject(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) return withoutArt(value);
  const result = withoutArt(value);
  if (value.objectId !== undefined && typeof value.name === "string" && !Object.hasOwn(value, "label")) {
    result.label = `#${value.objectId} ${value.name}`;
    if (typeof value.power === "number" && typeof value.toughness === "number") result.label += ` ${value.power}/${value.toughness}`;
    if (typeof value.tapped === "boolean") result.label += value.tapped ? "; tapped" : "; untapped";
  }
  return result;
}

export function visibleDefinitions(observation) {
  const ids = new Set();
  function visit(value) {
    if (!value || typeof value !== "object") return;
    for (const [key, child] of Object.entries(value)) {
      if (key === "checkpoint") continue;
      if ((key === "definition" || key === "sourceDefinition") && typeof child === "string") ids.add(child);
      else visit(child);
    }
  }
  visit(observation);
  return [...ids];
}

export function printedReference(card) {
  return {
    name: card.name, provenance: "Printed reference; current observed characteristics are in the position.",
    rulesText: card.rulesText, manaCost: card.manaCost,
    parts: card.parts?.map(part => ({ id: part.id, name: part.name, typeLine: part.typeLine, rulesText: part.rulesText, manaCost: part.manaCost })),
  };
}
