import assert from "node:assert/strict";
import test from "node:test";

import {
  BOT_MOVE_MS,
  FINISHED_ROOM_MS,
  HUMAN_MOVE_MS,
  INVITE_MS,
  PRESENCE_MS,
  REGISTRATION_MS,
  isOnline,
  liveInvites,
  moveBudgetMs,
  publicBot,
  worthKeeping,
} from "../worker/bot-presence.mjs";

const NOW = 1_000_000;

function bot(overrides = {}) {
  return {
    id: "abc",
    name: "Fizzbot",
    deck: "Sligh",
    lastSeen: NOW,
    invites: [],
    ...overrides,
  };
}

test("a bot is online for exactly as long as its heartbeat lease", () => {
  assert.equal(isOnline(NOW, NOW), true);
  assert.equal(isOnline(NOW - PRESENCE_MS, NOW), true, "the boundary is inclusive");
  assert.equal(isOnline(NOW - PRESENCE_MS - 1, NOW), false);
});

test("a bot that never heartbeated is offline, so registering is not being online", () => {
  assert.equal(isOnline(0, NOW), false);
  assert.equal(publicBot(bot({ lastSeen: 0 }), NOW).online, false);
});

test("an invitation nobody picked up expires, freeing the bot for the next challenger", () => {
  const fresh = { room: "r1", reason: "challenge", at: NOW - 1_000 };
  const stale = { room: "r2", reason: "challenge", at: NOW - INVITE_MS };
  assert.deepEqual(liveInvites([fresh, stale], NOW), [fresh]);
  assert.equal(publicBot(bot({ invites: [stale] }), NOW).busy, false);
  assert.equal(publicBot(bot({ invites: [fresh] }), NOW).busy, true);
});

test("the public view carries no token, whatever else the record holds", () => {
  const view = publicBot(
    bot({
      token: "secret",
      compatibility: {
        protocolVersion: 7,
        capabilities: ["private.claim.v1"],
        requiredCapabilities: [],
      },
    }),
    NOW,
  );
  assert.deepEqual(Object.keys(view).sort(), ["busy", "deck", "id", "name", "online"]);
  assert.equal(JSON.stringify(view).includes("secret"), false);
  assert.equal(JSON.stringify(view).includes("private.claim.v1"), false);
});

test("a bot's move clock is far shorter than a person's", () => {
  assert.equal(moveBudgetMs("bot"), BOT_MOVE_MS);
  assert.equal(moveBudgetMs("human"), HUMAN_MOVE_MS);
  assert.ok(
    BOT_MOVE_MS < HUMAN_MOVE_MS,
    "a program that has not answered in a minute is not thinking",
  );
});

test("a registration outlives the presence lease, so a bot can come back", () => {
  const yesterday = NOW - REGISTRATION_MS + 1;
  assert.equal(isOnline(yesterday, NOW), false, "long offline");
  assert.equal(worthKeeping(bot({ lastSeen: yesterday }), NOW), true, "still its name");
  assert.equal(
    worthKeeping(bot({ lastSeen: NOW - REGISTRATION_MS }), NOW),
    false,
    "past the retention window it is deleted",
  );
});

test("a bot holding a game is kept however long it has been quiet", () => {
  // Past the retention window, so only the invitation can save it.
  const quiet = NOW - REGISTRATION_MS;
  assert.equal(
    worthKeeping(bot({ lastSeen: quiet, invites: [{ at: NOW - 1_000 }] }), NOW),
    true,
    "deleting it would strand the room it owes a game",
  );
  assert.equal(
    worthKeeping(bot({ lastSeen: quiet, invites: [{ at: NOW - INVITE_MS }] }), NOW),
    false,
    "an expired invitation strands nobody",
  );
});

test("a finished room is kept long enough to read, not forever", () => {
  assert.ok(FINISHED_ROOM_MS > HUMAN_MOVE_MS, "outlast the clock that ended it");
  assert.ok(FINISHED_ROOM_MS < REGISTRATION_MS, "but do not keep games for a day");
});
