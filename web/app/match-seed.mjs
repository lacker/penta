// @ts-check

/** The engine takes a 32-bit seed, so this is the largest one it can deal. */
export const maxSeed = 0xffff_ffff;

/**
 * Reads a seed the player typed or put on the address bar. The engine wants a
 * whole number that fits in 32 bits, so anything else is `null`: the caller
 * decides whether that means "roll one" (an empty field) or "fix the typo".
 *
 * @param {string | null | undefined} text
 * @returns {number | null}
 */
export function parseSeed(text) {
  if (text == null) return null;
  const trimmed = text.trim();
  if (!/^\d+$/.test(trimmed)) return null;
  const parsed = Number(trimmed);
  return Number.isSafeInteger(parsed) && parsed <= maxSeed ? parsed : null;
}

/**
 * Whether typed text is a seed the engine cannot deal. Blank is not a mistake,
 * it is a request for a random deal, so only non-empty unparseable text counts.
 *
 * @param {string} text
 */
export function seedTextIsInvalid(text) {
  return text.trim() !== "" && parseSeed(text) === null;
}
