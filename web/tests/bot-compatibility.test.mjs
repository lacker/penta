import assert from "node:assert/strict";
import test from "node:test";

import {
  incompatibility,
  incompatibilityBody,
  parseBotCompatibility,
  parseServerCompatibility,
  publicServerCompatibility,
} from "../worker/bot-compatibility.mjs";

const FIXTURE_EPOCH = 7;
const FIXTURE_FINGERPRINT = `sha256-${"a".repeat(64)}`;
const server = parseServerCompatibility({
  protocolVersion: FIXTURE_EPOCH,
  capabilities: ["observation.future.v1", "reconstruction.checkpoint.v1"],
  requiredCapabilities: ["observation.future.v1"],
  simulationFingerprint: FIXTURE_FINGERPRINT,
  legacyUndeclaredProtocolVersion: FIXTURE_EPOCH - 1,
});

test("the same epoch and a required-capability superset are compatible", () => {
  const bot = parseBotCompatibility(
    {
      protocolVersion: FIXTURE_EPOCH,
      capabilities: ["future.unknown.v9", "observation.future.v1"],
      requiredCapabilities: ["reconstruction.checkpoint.v1"],
    },
    server,
  );
  assert.deepEqual(incompatibility(server, bot), {
    compatible: true,
    missingCapabilities: [],
    missingServerCapabilities: [],
    simulationMismatch: false,
  });
});

test("an epoch mismatch and a missing required capability are incompatible", () => {
  assert.equal(
    incompatibility(
      server,
      parseBotCompatibility(
        {
          protocolVersion: FIXTURE_EPOCH - 1,
          capabilities: ["observation.future.v1"],
        },
        server,
      ),
    ).compatible,
    false,
  );
  assert.deepEqual(
    incompatibility(
      server,
      parseBotCompatibility(
        { protocolVersion: FIXTURE_EPOCH, capabilities: [] },
        server,
      ),
    ),
    {
      compatible: false,
      missingCapabilities: ["observation.future.v1"],
      missingServerCapabilities: [],
      simulationMismatch: false,
    },
  );
});

test("a bot can require an advertised optional server facility", () => {
  const supported = parseBotCompatibility(
    {
      protocolVersion: FIXTURE_EPOCH,
      capabilities: ["observation.future.v1"],
      requiredCapabilities: ["reconstruction.checkpoint.v1"],
    },
    server,
  );
  assert.equal(incompatibility(server, supported).compatible, true);
  assert.deepEqual(
    incompatibility(
      { ...server, capabilities: ["observation.future.v1"] },
      supported,
    ),
    {
      compatible: false,
      missingCapabilities: [],
      missingServerCapabilities: ["reconstruction.checkpoint.v1"],
      simulationMismatch: false,
    },
  );
});

test("a trained bot can require the server simulation it was built for", () => {
  const exact = parseBotCompatibility(
    {
      protocolVersion: FIXTURE_EPOCH,
      capabilities: ["observation.future.v1"],
      requiredSimulationFingerprint: FIXTURE_FINGERPRINT,
    },
    server,
  );
  assert.equal(incompatibility(server, exact).compatible, true);
  assert.deepEqual(
    incompatibility(server, {
      ...exact,
      requiredSimulationFingerprint: `sha256-${"b".repeat(64)}`,
    }),
    {
      compatible: false,
      missingCapabilities: [],
      missingServerCapabilities: [],
      simulationMismatch: true,
    },
  );
});

test("a challenge-safe mismatch reveals no bot-derived diagnostics", () => {
  const privateBot = parseBotCompatibility(
    {
      protocolVersion: FIXTURE_EPOCH,
      capabilities: [],
      requiredCapabilities: ["private.requirement.v1"],
      requiredSimulationFingerprint: `sha256-${"b".repeat(64)}`,
    },
    server,
  );
  assert.deepEqual(incompatibilityBody(server, privateBot), {
    code: "incompatible_bot",
    error: "bot is incompatible with this server",
    server: publicServerCompatibility(server),
  });
  const authenticated = incompatibilityBody(server, privateBot, true);
  assert.equal(authenticated.bot, privateBot);
  assert.deepEqual(authenticated.missingCapabilities, ["observation.future.v1"]);
  assert.deepEqual(authenticated.missingServerCapabilities, ["private.requirement.v1"]);
  assert.equal(authenticated.simulationMismatch, true);
});

test("undeclared legacy bots remain on the fixed pre-negotiation epoch", () => {
  const legacy = parseBotCompatibility(undefined, server);
  assert.deepEqual(legacy, {
    protocolVersion: FIXTURE_EPOCH - 1,
    capabilities: [],
    requiredCapabilities: [],
  });
  assert.equal(incompatibility(server, legacy).compatible, false);

  const legacyServer = {
    ...server,
    protocolVersion: FIXTURE_EPOCH - 1,
    requiredCapabilities: [],
  };
  assert.equal(incompatibility(legacyServer, legacy).compatible, true);
  assert.equal(
    incompatibility(
      { ...legacyServer, protocolVersion: FIXTURE_EPOCH },
      legacy,
    ).compatible,
    false,
    "the open-world epoch requires an explicit declaration",
  );
});

test("present malformed declarations fail instead of masquerading as legacy", () => {
  assert.throws(() => parseBotCompatibility({}, server), /protocolVersion/);
  assert.throws(
    () =>
      parseBotCompatibility(
        { protocolVersion: FIXTURE_EPOCH, capabilities: [1] },
        server,
      ),
    /names must be nonempty/,
  );
  assert.throws(
    () =>
      parseBotCompatibility(
        {
          protocolVersion: FIXTURE_EPOCH,
          capabilities: Array.from({ length: 65 }, (_, i) => `x.${i}`),
        },
        server,
      ),
    /at most 64/,
  );
  assert.throws(
    () =>
      parseBotCompatibility(
        { protocolVersion: FIXTURE_EPOCH, capabilities: [" padded"] },
        server,
      ),
    /unpadded strings/,
  );
});

test("public server metadata omits the private legacy transition", () => {
  assert.deepEqual(publicServerCompatibility(server), {
    protocolVersion: FIXTURE_EPOCH,
    capabilities: ["observation.future.v1", "reconstruction.checkpoint.v1"],
    requiredCapabilities: ["observation.future.v1"],
    simulationFingerprint: FIXTURE_FINGERPRINT,
  });
});

test("the authoritative server manifest cannot require an unadvertised capability", () => {
  assert.throws(
    () =>
      parseServerCompatibility({
        protocolVersion: FIXTURE_EPOCH,
        capabilities: [],
        requiredCapabilities: ["missing.v1"],
        simulationFingerprint: FIXTURE_FINGERPRINT,
        legacyUndeclaredProtocolVersion: FIXTURE_EPOCH - 1,
      }),
    /not advertised/,
  );
});

test("fingerprint requirements are validated as bounded opaque identities", () => {
  assert.throws(
    () =>
      parseBotCompatibility(
        {
          protocolVersion: FIXTURE_EPOCH,
          capabilities: ["observation.future.v1"],
          requiredSimulationFingerprint: " padded",
        },
        server,
      ),
    /unpadded fingerprint string/,
  );
});
