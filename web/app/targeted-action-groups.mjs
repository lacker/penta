// @ts-check

/** @typedef {import("./game-types").AbilityOriginMetadata} AbilityOriginMetadata */
/** @typedef {import("./game-types").Action} Action */

/**
 * @typedef TargetedActionGroup
 * @property {string} key Stable identity for one exact ability origin.
 * @property {AbilityOriginMetadata | null | undefined} origin
 * @property {string} label Backend-authored, target-independent ability label.
 * @property {Action[]} actions Complete legal actions belonging to the origin.
 */

/**
 * Canonicalizes the tagged origin metadata without relying on JSON property order.
 *
 * @param {AbilityOriginMetadata | null | undefined} origin
 */
export function abilityOriginKey(origin) {
  if (!origin) return "no-ability-origin";
  switch (origin.kind) {
    case "printed":
      return `printed:${origin.definition}:${origin.partId}:${origin.abilityId}`;
    case "intrinsicBasicLand":
      return `intrinsic-basic-land:${origin.landType}`;
    case "granted":
      return [
        "granted",
        origin.source,
        origin.sourceDefinition,
        origin.sourcePartId,
        origin.sourceAbilityId,
        origin.grantId,
      ].join(":");
  }
}

/** @param {Action} action */
export function targetedActionOriginKey(action) {
  return abilityOriginKey(action.ability);
}

/**
 * Keeps unrelated panel actions visible while scoping actions from the
 * selected source to the ability origin the player chose.
 *
 * @param {Action} action
 * @param {number | null} selectedSource
 * @param {string | null} selectedOrigin
 */
export function actionMatchesTargetedOrigin(action, selectedSource, selectedOrigin) {
  return selectedOrigin === null ||
    action.cardId !== selectedSource ||
    targetedActionOriginKey(action) === selectedOrigin;
}

/**
 * Groups already-filtered targeted actions by the complete serialized origin.
 * A target-independent backend label names the ability picker; the complete
 * target-bearing actions remain available for the next interaction.
 *
 * @param {Action[]} actions
 * @returns {TargetedActionGroup[]}
 */
export function groupTargetedActionsByOrigin(actions) {
  /** @type {Map<string, TargetedActionGroup>} */
  const groups = new Map();
  for (const action of actions) {
    const key = targetedActionOriginKey(action);
    const group = groups.get(key);
    if (group) {
      group.actions.push(action);
      continue;
    }
    groups.set(key, {
      key,
      origin: action.ability,
      label: action.ability ? (action.abilityLabel ?? action.label) : "Choose a target",
      actions: [action],
    });
  }
  return [...groups.values()];
}
