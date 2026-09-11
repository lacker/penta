/** Presentation only: exact JSON changes, with reconstruction fetched separately. */
export function playingObservation(observation) {
  const result = structuredClone(observation);
  delete result.checkpoint;
  if (result.legalActions?.length > 100 || JSON.stringify(result.legalActions ?? []).length > 12_000) {
    const types = {};
    for (const action of result.legalActions) types[action.type] = (types[action.type] ?? 0) + 1;
    result.legalActions = { count: result.legalActions.length, types, inspect: "legalActions" };
  }
  if (result.decision?.options?.length > 100 || JSON.stringify(result.decision?.options ?? []).length > 12_000) {
    result.decision.options = { count: result.decision.options.length, inspect: "decision" };
  }
  return result;
}

/** Bound ordinary inspect pages without losing IDs or making the cursor stall. */
export function page(items, offset, limit) {
  const selected = [];
  let size = 0;
  for (const item of items.slice(offset, offset + limit)) {
    const length = JSON.stringify(item).length;
    if (selected.length && size + length > 24_000) break;
    selected.push(item);
    size += length;
  }
  return { items: selected, total: items.length, offset,
    nextOffset: offset + selected.length < items.length ? offset + selected.length : null };
}

// Paths are arrays of literal property names, not executable expressions. A
// same-length array can use numeric-string index paths. Length changes replace
// the whole array, so applying edits never requires splicing or leaves holes.
export function changes(before, after, path = []) {
  if (JSON.stringify(before) === JSON.stringify(after)) return [];
  const sameLengthArrays = Array.isArray(before) && Array.isArray(after)
    && before.length === after.length;
  if (before && after && typeof before === "object" && typeof after === "object"
      && (sameLengthArrays || (!Array.isArray(before) && !Array.isArray(after)))) {
    const edits = [];
    for (const key of Object.keys(before)) {
      if (!Object.hasOwn(after, key)) edits.push({ path: [...path, key], remove: true });
    }
    for (const [key, value] of Object.entries(after)) {
      edits.push(...changes(before[key], value, [...path, key]));
    }
    const replacement = [{ path, value: after }];
    return JSON.stringify(edits).length < JSON.stringify(replacement).length ? edits : replacement;
  }
  return [{ path, value: after }];
}

export function present(view, previous, full = false) {
  if (!view.observation) return { result: view, previous };
  const observation = playingObservation(view.observation);
  const envelope = { ...view };
  delete envelope.observation;
  let result = { ...envelope, observation };
  if (previous && !full) {
    const delta = { ...envelope, baseRevision: previous.revision,
      changes: changes(previous.observation, observation) };
    if (JSON.stringify(delta).length < JSON.stringify(result).length) result = delta;
  }
  return { result, previous: { revision: view.revision, observation } };
}
