/**
 * Pure compatibility rules for hosted bots. The engine supplies the server
 * manifest; keeping validation here makes registration, listing, heartbeat,
 * and challenge use exactly the same predicate.
 */

/** @typedef {{ protocolVersion: number, capabilities: string[],
 *   requiredCapabilities: string[], requiredSimulationFingerprint?: string }} BotCompatibility */
/** @typedef {BotCompatibility & { requiredCapabilities: string[],
 *   simulationFingerprint: string,
 *   legacyUndeclaredProtocolVersion: number }} ServerCompatibility */

/** Public declarations are persisted, so keep one request from growing without bound. */
const MAX_CAPABILITIES = 64;
const MAX_CAPABILITY_LENGTH = 128;

function version(value, field) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error(`${field} must be a nonnegative integer`);
  }
  return value;
}

function capabilities(value, field) {
  if (!Array.isArray(value)) {
    throw new Error(`${field} must be an array of nonempty strings`);
  }
  if (value.length > MAX_CAPABILITIES) {
    throw new Error(`${field} may contain at most ${MAX_CAPABILITIES} names`);
  }
  if (
    value.some(
      (item) =>
        typeof item !== "string" ||
        !item ||
        item.length > MAX_CAPABILITY_LENGTH ||
        item.trim() !== item,
    )
  ) {
    throw new Error(
      `${field} names must be nonempty, unpadded strings of at most ` +
        `${MAX_CAPABILITY_LENGTH} characters`,
    );
  }
  return [...new Set(value)].sort();
}

function fingerprint(value, field) {
  if (
    typeof value !== "string" ||
    !value ||
    value.length > 256 ||
    value.trim() !== value
  ) {
    throw new Error(`${field} must be a nonempty, unpadded fingerprint string`);
  }
  return value;
}

/** @param {unknown} value @returns {ServerCompatibility} */
export function parseServerCompatibility(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("server compatibility manifest must be an object");
  }
  const manifest = /** @type {Record<string, unknown>} */ (value);
  const parsed = {
    protocolVersion: version(manifest.protocolVersion, "protocolVersion"),
    capabilities: capabilities(manifest.capabilities, "capabilities"),
    requiredCapabilities: capabilities(
      manifest.requiredCapabilities,
      "requiredCapabilities",
    ),
    simulationFingerprint: fingerprint(
      manifest.simulationFingerprint,
      "simulationFingerprint",
    ),
    legacyUndeclaredProtocolVersion: version(
      manifest.legacyUndeclaredProtocolVersion,
      "legacyUndeclaredProtocolVersion",
    ),
  };
  for (const required of parsed.requiredCapabilities) {
    if (!parsed.capabilities.includes(required)) {
      throw new Error(`required capability ${required} is not advertised`);
    }
  }
  return parsed;
}

/**
 * Missing metadata is a deliberate transition case, fixed to the epoch that
 * introduced negotiation. A present but malformed declaration is an error.
 *
 * @param {unknown} value
 * @param {ServerCompatibility} server
 * @returns {BotCompatibility}
 */
export function parseBotCompatibility(value, server) {
  if (value === undefined) {
    return {
      protocolVersion: server.legacyUndeclaredProtocolVersion,
      capabilities: [],
      requiredCapabilities: [],
    };
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("compatibility must be an object");
  }
  const declaration = /** @type {Record<string, unknown>} */ (value);
  const parsed = {
    protocolVersion: version(declaration.protocolVersion, "compatibility.protocolVersion"),
    capabilities: capabilities(
      declaration.capabilities,
      "compatibility.capabilities",
    ),
    requiredCapabilities:
      declaration.requiredCapabilities === undefined
        ? []
        : capabilities(
            declaration.requiredCapabilities,
            "compatibility.requiredCapabilities",
          ),
    ...(declaration.requiredSimulationFingerprint === undefined
      ? {}
      : {
          requiredSimulationFingerprint: fingerprint(
            declaration.requiredSimulationFingerprint,
            "compatibility.requiredSimulationFingerprint",
          ),
        }),
  };
  return parsed;
}

/** @param {ServerCompatibility} server @param {BotCompatibility} bot */
export function incompatibility(server, bot) {
  const missingCapabilities = server.requiredCapabilities.filter(
    (required) => !bot.capabilities.includes(required),
  );
  const missingServerCapabilities = bot.requiredCapabilities.filter(
    (required) => !server.capabilities.includes(required),
  );
  const simulationMismatch =
    bot.requiredSimulationFingerprint !== undefined &&
    bot.requiredSimulationFingerprint !== server.simulationFingerprint;
  return {
    compatible:
      bot.protocolVersion === server.protocolVersion &&
      missingCapabilities.length === 0 &&
      missingServerCapabilities.length === 0 &&
      !simulationMismatch,
    missingCapabilities,
    missingServerCapabilities,
    simulationMismatch,
  };
}

/** @param {ServerCompatibility} server */
export function publicServerCompatibility(server) {
  return {
    protocolVersion: server.protocolVersion,
    capabilities: [...server.capabilities],
    requiredCapabilities: [...server.requiredCapabilities],
    simulationFingerprint: server.simulationFingerprint,
  };
}

/**
 * A challenger is not authenticated to the bot, so only registration and
 * heartbeat may receive diagnostics derived from the bot's declaration.
 *
 * @param {ServerCompatibility} server
 * @param {BotCompatibility} bot
 * @param {boolean} exposeBot
 */
export function incompatibilityBody(server, bot, exposeBot = false) {
  const body = {
    code: "incompatible_bot",
    error: "bot is incompatible with this server",
    server: publicServerCompatibility(server),
  };
  if (!exposeBot) return body;
  const result = incompatibility(server, bot);
  return {
    ...body,
    bot,
    missingCapabilities: result.missingCapabilities,
    missingServerCapabilities: result.missingServerCapabilities,
    simulationMismatch: result.simulationMismatch,
  };
}
