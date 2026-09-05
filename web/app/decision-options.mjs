// @ts-check

/** @typedef {import("./game-types").DecisionOption} DecisionOption */

export const LARGE_DECISION_THRESHOLD = 20;
export const MAX_VISIBLE_DECISION_OPTIONS = 20;

/**
 * Turns a large decision into a type-ahead list. Large choices begin empty so
 * naming a card never means scrolling through the catalog. Already-selected
 * options remain visible while refining a multi-select search.
 *
 * @param {DecisionOption[]} options
 * @param {string} query
 * @param {number[]} selectedIds
 */
export function searchableDecisionOptions(options, query, selectedIds = []) {
  if (options.length < LARGE_DECISION_THRESHOLD) {
    return {
      searchable: false,
      matches: options,
      matchCount: options.length,
      truncated: false,
    };
  }

  const selected = new Set(selectedIds);
  const selectedOptions = options.filter((option) => selected.has(option.id));
  const normalizedQuery = query.trim().toLocaleLowerCase();
  if (normalizedQuery.length === 0) {
    return {
      searchable: true,
      matches: selectedOptions,
      matchCount: options.length,
      truncated: options.length > selectedOptions.length,
    };
  }

  const ranked = options
    .map((option, index) => {
      const label = option.label.toLocaleLowerCase();
      const wordPrefix = label
        .split(/[^\p{L}\p{N}]+/u)
        .some((word) => word.startsWith(normalizedQuery));
      const rank = label === normalizedQuery ? 0 : label.startsWith(normalizedQuery) ? 1 : wordPrefix ? 2 : 3;
      return { option, index, label, rank };
    })
    .filter(({ label }) => label.includes(normalizedQuery))
    .sort((left, right) => left.rank - right.rank || left.index - right.index);
  const unselectedMatches = ranked
    .map(({ option }) => option)
    .filter((option) => !selected.has(option.id));
  const remainingSlots = Math.max(0, MAX_VISIBLE_DECISION_OPTIONS - selectedOptions.length);

  return {
    searchable: true,
    matches: [...selectedOptions, ...unselectedMatches.slice(0, remainingSlots)],
    matchCount: ranked.length,
    truncated: ranked.length > remainingSlots,
  };
}
