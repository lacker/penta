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
 * @param {number | null | undefined} [actionSource] Current object carrying
 * the ability. Inline token clause IDs are only positional within that object.
 */
export function abilityOriginKey(origin, actionSource) {
  if (!origin) return "no-ability-origin";
  const sourcePrefix = actionSource == null ? "" : `object:${actionSource}:`;
  switch (origin.kind) {
    case "printed":
      return `${sourcePrefix}printed:${origin.definition}:${origin.partId}:${origin.abilityId}`;
    case "token":
      return `${sourcePrefix}token:${origin.partId}:${origin.abilityId}`;
    case "emblem":
      return `${sourcePrefix}emblem:${origin.abilityId}`;
    case "faceDown":
      return `${sourcePrefix}face-down:${origin.abilityId}`;
    case "intrinsicBasicLand":
      return `${sourcePrefix}intrinsic-basic-land:${origin.landType}`;
    case "granted":
      return [
        `${sourcePrefix}granted`,
        origin.source,
        origin.sourceDefinition,
        origin.sourcePartId,
        origin.sourceAbilityId,
        origin.grantId,
      ].join(":");
    case "tokenGranted":
      return [
        `${sourcePrefix}token-granted`,
        origin.source,
        origin.sourcePartId,
        origin.sourceAbilityId,
        origin.grantId,
      ].join(":");
    case "emblemGranted":
      return [
        `${sourcePrefix}emblem-granted`,
        origin.source,
        origin.sourceAbilityId,
        origin.grantId,
      ].join(":");
    case "faceDownGranted":
      return [
        `${sourcePrefix}face-down-granted`,
        origin.source,
        origin.sourceAbilityId,
        origin.grantId,
      ].join(":");
  }
}

/** @param {Action} action */
export function targetedActionOriginKey(action) {
  return abilityOriginKey(action.ability, action.cardId);
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
 * Whether dropping on a target identifies exactly one action. A drag cannot
 * expose a second picker after the drop, so payment, mode, or cost choices
 * that share a target have to stay in the click-through action flow.
 *
 * @template T
 * @param {T[]} actions
 * @param {(action: T) => string[]} targetKeys
 */
export function targetActionsAreUnambiguous(actions, targetKeys) {
  const claimedTargets = new Set();
  for (const action of actions) {
    for (const target of targetKeys(action)) {
      if (claimedTargets.has(target)) return false;
      claimedTargets.add(target);
    }
  }
  return true;
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
