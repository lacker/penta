/** Exact compatibility for a persisted command journal. */

/**
 * @param {{ replayVersion?: unknown, simulationFingerprint?: unknown }} stored
 * @param {{ replayVersion: number, simulationFingerprint: string }} current
 * @returns {string | null}
 */
export function replayCompatibilityError(stored, current) {
  if (stored.replayVersion === undefined || stored.simulationFingerprint === undefined) {
    return "game was recorded before exact replay compatibility metadata was stored";
  }
  if (
    stored.replayVersion === current.replayVersion &&
    stored.simulationFingerprint === current.simulationFingerprint
  ) {
    return null;
  }
  return (
    `game was recorded with replay ${String(stored.replayVersion)} simulation ` +
    `${String(stored.simulationFingerprint)}, this is replay ${current.replayVersion} ` +
    `simulation ${current.simulationFingerprint}`
  );
}
