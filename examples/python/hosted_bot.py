"""Put a bot online so people can play it.

The whole thing is one loop over one HTTP call. Heartbeating is what "online"
means -- stop, and the bot leaves the list -- and the heartbeat's reply is
where invitations arrive. No WebSocket, no framework, no penta module: this
plays entirely against the server's engine, so it runs anywhere `requests`
does.

    python3 hosted_bot.py --name Fizzbot           # your own server
    python3 hosted_bot.py --name Fizzbot \
        --server https://penta.lacker.workers.dev  # where people can play it

The default is the development server from `cd web && pnpm run dev`, which
is where a bot under construction belongs: no limits, no audience. A linked
worktree gets its own port, which `pnpm run dev:url` reports.

`choose` is the whole bot. Everything above and below it is plumbing you can
copy verbatim; see BOTS.md for what an observation holds.
"""

import argparse
import time

import requests

# Heartbeat well inside the server's 45-second presence window, so one lost
# packet does not read as "this bot went away".
HEARTBEAT_SECONDS = 10
# How long to wait between polls while it is the opponent's turn to move.
POLL_SECONDS = 0.25
# This bot consumes the protocol-23 indexed-action vocabulary, requires no
# optional server facilities, and implements no optional server-required
# vocabulary. Never copy the server's claims without implementing them.
COMPATIBILITY = {
    "protocolVersion": 23,
    "capabilities": [],
    "requiredCapabilities": [],
    # A trained bot can set this to the simulationFingerprint it requires:
    # "requiredSimulationFingerprint": "sha256-...",
}


def choose(observation):
    """Pick an action index from one observation. Replace me with a model."""
    actions = observation["legalActions"]
    for index, action in enumerate(actions):
        if action["type"] == "KeepHand":
            return index
    # Anything that develops the board beats passing, and passing beats
    # conceding -- which is legal at every priority and would end the game.
    for wanted in (
        "PlayLand",
        "CastSpell",
        "DeclareAttacker",
        "ActivateAbility",
        "TakeSpecialAction",
    ):
        for index, action in enumerate(actions):
            if action["type"] == wanted:
                return index
    for index, action in enumerate(actions):
        if action["type"] != "Concede":
            return index
    return 0


def play(server, room, token):
    """Drive the opponent seat of one room until the game ends.

    The token came with the invitation and is what authorises this seat; the
    room id alone is just a name, and the room will refuse without it.
    """
    print(f"playing {room}")
    headers = {"x-penta-token": token}
    while True:
        view = requests.get(
            f"{server}/_game/{room}/opponent", headers=headers, timeout=30
        ).json()
        if view.get("result"):
            print(f"  finished: {view['result']}")
            return
        if not view.get("deciding"):
            # The human is thinking, or the engine is resolving something.
            time.sleep(POLL_SECONDS)
            continue
        index = choose(view["observation"])
        reply = requests.post(
            f"{server}/_game/{room}/command",
            json={"t": "botAct", "index": index},
            headers=headers,
            timeout=30,
        )
        if reply.status_code != 200:
            # A refused action leaves the previous observation standing, so
            # the next poll simply asks again.
            print(f"  refused: {reply.text}")
            time.sleep(POLL_SECONDS)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--server",
        default="http://localhost:3000",
        help="your own server while building; "
        "https://penta.lacker.workers.dev to play other people",
    )
    parser.add_argument("--name", default="Fizzbot")
    parser.add_argument("--deck", default="Sligh")
    arguments = parser.parse_args()
    server = arguments.server.rstrip("/")

    registration_reply = requests.post(
        f"{server}/_bots/register",
        json={
            "name": arguments.name,
            "deck": arguments.deck,
            "compatibility": COMPATIBILITY,
        },
        timeout=30,
    )
    if registration_reply.status_code == 409:
        print(f"server rejected this bot's compatibility: {registration_reply.text}")
        return
    registration_reply.raise_for_status()
    registration = registration_reply.json()
    identifier, token = registration["id"], registration["token"]
    print(f"registered as {arguments.name} ({identifier}) playing {arguments.deck}")
    print("waiting for a challenger…")

    finished = []
    while True:
        # A server restart, a proxy hiccup, or a redeploy all show up here as
        # one bad reply. Missing a heartbeat costs at most a spell out of the
        # list; crashing costs the whole bot, so the loop rides it out.
        try:
            response = requests.post(
                f"{server}/_bots/{identifier}/heartbeat",
                json={
                    "token": token,
                    "done": finished,
                    "compatibility": COMPATIBILITY,
                },
                timeout=30,
            )
            if response.status_code == 409:
                print(f"server rejected this bot's compatibility: {response.text}")
                return
            response.raise_for_status()
            reply = response.json()
        except (requests.RequestException, ValueError) as problem:
            print(f"heartbeat failed ({problem}); retrying")
            time.sleep(HEARTBEAT_SECONDS)
            continue
        finished = []
        for invite in reply.get("invites", []):
            try:
                play(server, invite["room"], invite["token"])
            except (requests.RequestException, ValueError) as problem:
                print(f"lost the game in {invite['room']} ({problem})")
            # Reporting it finished is what frees the bot for the next game,
            # whether it ended in a result or in a dropped connection.
            finished.append(invite["room"])
        if not finished:
            time.sleep(HEARTBEAT_SECONDS)


if __name__ == "__main__":
    main()
