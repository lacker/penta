"""Smoke test for the Python bindings: full games, determinism, self-play.

Run via scripts/check-bindings.sh, which builds the module and puts it on
the path first.
"""

import json

import penta

fingerprint = penta.simulation_fingerprint()
assert fingerprint.startswith("sha256-")
assert len(fingerprint) == 71
assert all(character in "0123456789abcdef" for character in fingerprint[7:])
print(
    "engine",
    penta.engine_version(),
    "protocol",
    penta.protocol_version(),
    "simulation",
    fingerprint,
)
assert "Sligh" in penta.deck_names()
catalog_payload = json.loads(penta.catalog())
assert catalog_payload["simulationFingerprint"] == fingerprint
catalog = {c["definition"]: c for c in catalog_payload["cards"]}
assert any(c["name"] == "Lightning Bolt" for c in catalog.values())

standard_decks = penta.deck_names(format="isd-m14-standard")
assert "Briksza Naya Midrange" in standard_decks
standard_catalog_payload = json.loads(penta.catalog(format="isd-m14-standard"))
assert standard_catalog_payload["format"] == "isd-m14-standard"
assert standard_catalog_payload["simulationFingerprint"] == fingerprint
assert any(
    card["name"] == "Huntmaster of the Fells // Ravager of the Fells"
    for card in standard_catalog_payload["cards"]
)

standard_game = penta.Game(
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    opponent="external",
    format="isd-m14-standard",
    seed=17,
)
standard_observation = json.loads(standard_game.observe())
assert standard_observation["format"] == "isd-m14-standard"
assert standard_observation["simulationFingerprint"] == fingerprint

try:
    penta.deck_names(format="not-a-format")
except ValueError:
    pass
else:
    raise AssertionError("bad format accepted")

def pass_bot(obs):
    prefer = ["KeepHand", "ChooseDecision", "PassPriority", "FinishDeclaringAttackers",
              "FinishDeclaringBlockers", "AssignCombatDamage", "DiscardCards",
              "BottomCards", "ChooseUntap"]
    actions = obs["legalActions"]
    for kind in prefer:
        for action in actions:
            if action["type"] == kind:
                return action["index"]
    return 0

# vs handcrafted
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=7)
steps = 0
while game.result() is None:
    obs = json.loads(game.observe())
    game.act(pass_bot(obs))
    steps += 1
    assert steps < 100000
print("vs handcrafted:", game.result(), "in", steps, "decisions")

# determinism
def run(seed):
    g = penta.Game("Goblins", "Sligh", opponent="random", seed=seed)
    trace = []
    while g.result() is None:
        obs = json.loads(g.observe())
        trace.append(len(obs["legalActions"]))
        g.act(pass_bot(obs))
    return g.result(), trace
a, b = run(99), run(99)
assert a == b, "same seed, same game"
print("determinism ok:", a[0], "over", len(a[1]), "decisions")

# clone: a fork replays identically and diverges independently
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=7)
for _ in range(30):
    game.act(pass_bot(json.loads(game.observe())))
replay = game.clone()
assert game.observe() == replay.observe(), "a clone starts byte-identical"
for _ in range(20):
    choice = pass_bot(json.loads(game.observe()))
    game.act(choice)
    replay.act(choice)
    assert game.observe("p1") == replay.observe("p1"), "same actions, same bytes"
# Diverge: the fork plays a different legal action than the original, the
# two games stop matching, and the original never notices. Walk to a
# decision with at least two options first.
while len(json.loads(game.observe())["legalActions"]) < 2:
    game.act(0)
obs = json.loads(game.observe())
choice = pass_bot(obs)
other = (choice + 1) % len(obs["legalActions"])
before = game.observe()
fork = game.clone()
fork.act(other)
assert game.observe() == before, "the original is untouched"
game.act(choice)
assert game.observe("p1") != fork.observe("p1"), \
    "different actions, different games"
for _ in range(10):  # a fork is a live game, not a snapshot: it plays on
    if fork.result() is not None:
        break
    fork.act(pass_bot(json.loads(fork.observe())))
print("clone: forks replay identically and diverge independently")

# self-play: one loop drives both seats
game = penta.Game("Goblins", "White Weenie", opponent="external", seed=13)
steps = 0
while game.result() is None:
    seat = game.decision_seat()
    obs = json.loads(game.observe(seat))
    assert obs["seat"] == seat
    game.act(pass_bot(obs))
    steps += 1
    assert steps < 200000
print("self-play:", game.result(), "in", steps, "decisions")

# A hosted observation can become a live local determinization without the
# host seed. Here the hypotheses happen to use a local game's true hidden
# hand and libraries only so the smoke test can obtain correctly sized lists
# cheaply. Outside-game lists are explicit hypotheses and may be empty.
game = penta.Game("Sligh", "The Deck", opponent="external", seed=31)
view_json = game.observe("p1")
view = json.loads(view_json)
zone_definitions = lambda zone: [card["definition"] for card in json.loads(zone)]
hidden = {
    "hands": {"p2": zone_definitions(game.hand("p2"))},
    "libraries": {
        "p1": zone_definitions(game.library("p1")),
        "p2": zone_definitions(game.library("p2")),
    },
    "outsideGame": {"p1": [], "p2": []},
}
world = penta.Game.from_observation(view_json, json.dumps(hidden), rollout_seed=999)
rebuilt = json.loads(world.observe("p1"))
assert rebuilt["hand"] == view["hand"], "public object ids survive reconstruction"
assert rebuilt["legalActions"] == view["legalActions"]
assert "seed" not in rebuilt and "rng" not in rebuilt["checkpoint"]
world.act(0)
print("checkpoint: redacted observation rebuilt as a live local world")


# hypothetical worlds: you do not know what the opponent holds, so build the
# worlds you think are plausible and roll each one out. The engine supplies no
# distribution -- naming the cards is the whole API.
by_name = {card["name"]: definition for definition, card in catalog.items()}

game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=3)
game.act(pass_bot(json.loads(game.observe())))
truth = json.loads(game.hand("p2"))
assert truth, "the simulation view shows the opponent's real hand"

worlds = []
for guess in ("Lightning Bolt", "Counterspell"):
    world = game.clone()
    world.set_hand("p2", [by_name["Mountain"], by_name[guess]])
    held = [c["definition"] for c in json.loads(world.hand("p2"))]
    assert held == [by_name["Mountain"], by_name[guess]]
    world.act(pass_bot(json.loads(world.observe())))
    worlds.append(held)

assert worlds[0] != worlds[1], "two different worlds from one position"
assert json.loads(game.hand("p2")) == truth, "the real game is untouched"

# A library can be stacked outright, including down to nothing.
world = game.clone()
world.set_library("p2", [by_name["Black Lotus"]])
assert len(json.loads(world.library("p2"))) == 1
world.set_library("p2", [])
assert json.loads(world.library("p2")) == []

try:
    world.set_hand("p2", [60000])
    raise AssertionError("a card outside the catalog must raise")
except ValueError:
    pass
print("simulate: opponent hands rewritten, worlds diverge, real game untouched")

# hidden info: p1 never sees p2's hand
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=3)
obs = json.loads(game.observe())
assert "opponentHandSize" in obs and isinstance(obs["opponentHandSize"], int)
print("smoke test passed")
