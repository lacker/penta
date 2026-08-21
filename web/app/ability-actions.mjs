// @ts-check

/** @typedef {import("./game-types").AbilityOriginMetadata} AbilityOriginMetadata */
/** @typedef {import("./game-types").Action} Action */

/**
 * A stable key for one exact ability origin. Printed ability IDs are only
 * positional within a card part, so the definition and part remain part of
 * the identity. Granted abilities likewise keep their complete provenance.
 *
 * @param {AbilityOriginMetadata | null | undefined} origin
 * @param {number | null | undefined} [actionSource] Current object carrying
 * the ability. Inline token clause IDs are only positional within that object.
 * @returns {string | null}
 */
export function abilityOriginKey(origin, actionSource) {
  if (!origin) return null;
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
export function actionHasTargets(action) {
  return (
    (action.targetCount ?? 0) > 0 ||
    action.targetCardId != null ||
    action.targetPlayer != null ||
    action.targetStackId != null ||
    (action.targetCardIds?.length ?? 0) > 0 ||
    (action.targetPlayers?.length ?? 0) > 0 ||
    (action.targetStackIds?.length ?? 0) > 0
  );
}

/**
 * Groups the complete legal variants of each activated ability without
 * conflating abilities that happen to share a source or printed text.
 * Optional-target abilities deliberately retain their zero-target action as
 * a separate member of the same group.
 *
 * @param {Action[]} actions
 */
export function buildAbilityActionGroups(actions) {
  /** @type {Map<string, {key: string, actions: Action[], targeted: Action[], targetless: Action[]}>} */
  const groups = new Map();
  for (const action of actions) {
    const key = abilityOriginKey(action.ability, action.cardId);
    if (key === null) continue;
    let group = groups.get(key);
    if (!group) {
      group = { key, actions: [], targeted: [], targetless: [] };
      groups.set(key, group);
    }
    group.actions.push(action);
    (actionHasTargets(action) ? group.targeted : group.targetless).push(action);
  }
  return [...groups.values()];
}
