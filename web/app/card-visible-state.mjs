// @ts-check

/** @typedef {import("./game-types").Card} Card */

/**
 * Public choices attached to a permanent must be part of its visual pile
 * identity. Otherwise two Needles naming different cards (or two Caverns
 * naming different creature types) collapse into one misleading pile.
 *
 * @param {Card} card
 */
export function cardChoiceStateKey(card) {
  return `${card.chosenCardName ?? ""}\u0000${card.chosenCreatureType ?? ""}\u0000${card.chosenColor ?? ""}`;
}

/** @param {Card} card */
export function cardChoiceLabel(card) {
  if (card.chosenCardName) return `Named card: ${card.chosenCardName}`;
  if (card.chosenCreatureType) return `Chosen type: ${card.chosenCreatureType}`;
  if (card.chosenColor) return `Chosen color: ${card.chosenColor}`;
  return null;
}

/**
 * Everything the battlefield currently exposes about whether two permanents
 * are interchangeable in one visual pile. Object identity is deliberately
 * omitted for ordinary permanents, but retained when the engine says an
 * object carries state that the presentation cannot safely compare (an
 * attachment or an active object-specific effect, for example).
 *
 * @param {Card} card
 */
export function cardPileStateKey(card) {
  return JSON.stringify([
    card.name,
    card.partId ?? null,
    card.physicalFace ?? null,
    card.kind,
    card.typeLine ?? null,
    card.token ?? false,
    card.faceDown ?? false,
    card.phasedOut ?? false,
    card.tapped ?? false,
    card.power ?? null,
    card.toughness ?? null,
    card.damage ?? 0,
    card.loyalty ?? null,
    card.loyaltyAbilityUsedThisTurn ?? false,
    card.attacking ?? false,
    card.attackDefender ?? null,
    card.blockedThisCombat ?? false,
    card.blocking ?? [],
    card.blockingThisCombat ?? false,
    card.attackingBand ?? null,
    card.flying ?? false,
    card.canAttack ?? false,
    card.enteredThisTurn ?? false,
    cardChoiceStateKey(card),
    card.chosenBasicLandType ?? null,
    card.hasIndividualState ? card.id : null,
  ]);
}

/**
 * The held turn-banner frame should preview the untap result the engine
 * actually reached. Blindly straightening every permanent invents untaps
 * through Mana Vault, skipped untap steps, and other restrictions.
 *
 * @param {Card[]} held
 * @param {Card[]} incoming
 */
export function battlefieldWithObservedUntap(held, incoming) {
  const incomingById = new Map(incoming.map((card) => [card.id, card]));
  return held.map((card) => {
    const observed = incomingById.get(card.id);
    return observed && observed.tapped !== card.tapped
      ? { ...card, tapped: observed.tapped }
      : card;
  });
}

/**
 * Number same-name permanents on each side of the table so their abilities
 * and decisions can point back to one unambiguous game object.
 *
 * @param {Card[]} cards
 */
export function duplicatePermanentMarkers(cards) {
  /** @type {Map<string, number[]>} */
  const groups = new Map();
  for (const card of cards) {
    const key = `${card.owner ?? "human"}\u0000${card.name}`;
    const group = groups.get(key) ?? [];
    group.push(card.id);
    groups.set(key, group);
  }
  /** @type {Map<number, string>} */
  const markers = new Map();
  for (const ids of groups.values()) {
    if (ids.length < 2) continue;
    ids.forEach((id, index) => markers.set(id, String(index + 1)));
  }
  return markers;
}
