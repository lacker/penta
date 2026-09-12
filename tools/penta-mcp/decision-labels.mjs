/** Deterministic labels. Exact action fields remain beside every label. */
export function objectNames(observation) {
  const names = new Map();
  function visit(value) {
    if (!value || typeof value !== "object") return;
    if (value.objectId !== undefined && typeof value.name === "string" && !names.has(value.objectId)) names.set(value.objectId, value.name);
    for (const [key, child] of Object.entries(value)) if (key !== "checkpoint") visit(child);
  }
  // Historical disclosures must not overwrite a current copy or changed name.
  for (const key of ["battlefield", "stack", "hand", "graveyards", "exiles"]) visit(observation[key]);
  visit(observation);
  return names;
}

export function objectLabel(id, names) {
  return `#${id}${names.has(id) ? ` ${names.get(id)}` : ""}`;
}

export function actionLabel(action, names) {
  const object = id => objectLabel(id, names);
  let label;
  switch (action.type) {
    case "PassPriority": label = "Pass priority"; break;
    case "Concede": label = "Concede"; break;
    case "KeepHand": label = "Keep hand"; break;
    case "TakeMulligan": label = "Take mulligan"; break;
    case "PlayLand": label = `Play ${object(action.card)}`; break;
    case "CastSpell": label = `Cast ${object(action.card)}`; break;
    case "ActivateManaAbility": label = `Activate mana from ${object(action.source)} (${action.color})`; break;
    case "ActivateAbility": label = `Activate ability of ${object(action.source)}`; break;
    case "DeclareAttacker": label = `Attack with ${object(action.attacker)}`; break;
    case "DeclareBlocker": label = `Block with ${object(action.blocker)}`; break;
    case "FinishDeclaringAttackers": label = "Finish declaring attackers"; break;
    case "FinishDeclaringBlockers": label = "Finish declaring blockers"; break;
    case "ChooseDecision": label = `Select options for decision ${action.decision}`; break;
    case "CancelDecision": label = `Cancel decision ${action.decision}`; break;
    default: label = action.type;
  }
  // Include names for all referenced objects, without rewriting nested targets,
  // payment plans, mode IDs, or ability origins into an inferred command.
  const ids = new Set();
  function visit(value, key) {
    if (Array.isArray(value)) value.forEach(item => visit(item, key));
    else if (value && typeof value === "object") Object.entries(value).forEach(([k, v]) => visit(v, k));
    else if (["card", "source", "permanent", "attacker", "blocker", "objectId", "sacrifices", "costObjects", "cards", "permanents"].includes(key) && names.has(value)) ids.add(value);
  }
  visit(action);
  const objects = [...ids].map(object).filter(name => !label.includes(name));
  return objects.length ? `${label}; ${objects.join("; ")}` : label;
}
