"""A first penta bot: play a land, cast the biggest spell you can, attack.

This is the whole shape of a bot — a function from observation to action
index — plus a harness that runs a series and prints win rates. Swap
`choose` for your model's forward pass and you have a training loop.

Run from a directory containing the built `penta` module (see BOTS.md):

    python3 first_bot.py
"""

import json

import penta

CATALOG = {card["definition"]: card for card in json.loads(penta.catalog())["cards"]}


def spell_score(action):
    """Bigger creatures first, then anything else castable."""
    card = CATALOG.get(action.get("card"))
    if card is None:
        return 0
    power = card.get("power") or 0
    return 10 + power


def choose(observation):
    """Pick an action index from one observation. Replace me with a model."""
    actions = observation["legalActions"]
    by_type = {}
    for action in actions:
        by_type.setdefault(action["type"], []).append(action)

    # Opening hand: keep anything with 2-5 lands.
    if "KeepHand" in by_type:
        lands = sum(
            1
            for card in observation["hand"]
            if CATALOG[card["definition"]]["kind"] == "Land"
        )
        if 2 <= lands <= 5 or not by_type.get("TakeMulligan"):
            return by_type["KeepHand"][0]["index"]
        return by_type["TakeMulligan"][0]["index"]

    # One land every turn.
    if "PlayLand" in by_type:
        return by_type["PlayLand"][0]["index"]

    # Cast the beefiest thing on offer. Actions whose costs cannot be paid
    # are simply absent from legalActions, so no mana math is needed.
    if "CastSpell" in by_type:
        best = max(by_type["CastSpell"], key=spell_score)
        if spell_score(best) > 0:
            return best["index"]

    # Turn every creature sideways.
    if "DeclareAttacker" in by_type:
        return by_type["DeclareAttacker"][0]["index"]

    # Otherwise take whichever quiet option moves the game along.
    for kind in (
        "FinishDeclaringAttackers",
        "FinishDeclaringBlockers",
        "AssignCombatDamage",
        "ChooseDecision",
        "DiscardCards",
        "BottomCards",
        "ChooseUntap",
        "PassPriority",
    ):
        if kind in by_type:
            return by_type[kind][0]["index"]
    return actions[0]["index"]


def play_one(opponent, seed):
    """Returns True when this bot (seat p1) wins."""
    game = penta.Game("Sligh", "The Deck", opponent=opponent, seed=seed)
    while game.result() is None:
        observation = json.loads(game.observe())
        game.act(choose(observation))
    return game.result() == "p1"


def main():
    print(
        f"penta engine {penta.engine_version()}, protocol {penta.protocol_version()}, "
        f"simulation {penta.simulation_fingerprint()}"
    )
    for opponent in ("random", "handcrafted"):
        wins = sum(play_one(opponent, seed) for seed in range(100))
        print(f"vs {opponent}: won {wins}/100")


if __name__ == "__main__":
    main()
