# Worlds 32 Top 8 — Standard: WOE–TRK

These are the eight published lists from [Worlds 32, Atlanta, November 14,
2026](https://mtgtop8.com/event?e=93781&f=ST). Each YAML preserves the published
60-card main deck and 15-card sideboard. Alternate-spell names are expanded to
the catalog's full identity; no card substitutions are made.

The fixed `woe-trk-standard` profile covers the 21 paper Standard sets from
Wilds of Eldraine through Star Trek, including Foundations and The Big Score.
It uses the paper ban list checked on November 19, 2026. It does not change the
older WOE–HOB snapshot. Commander products, bonus sheets, and Arena-only bans
do not expand or alter this pool.

| Finish | Player | Deck | Remaining unsupported identities |
| --- | --- | --- | --- |
| 1 | Stefan Schütz | [Izzet Mission](https://mtgtop8.com/event?e=93781&d=912141&f=ST) | None |
| 2 | Eduardo Sajgalik | [Selesnya Aggro](https://mtgtop8.com/event?e=93781&d=912137&f=ST) | Michelangelo, Weirdness to 11; Pawpatch Recruit |
| 3–4 | Kellen Pastore | [Boros Pixie](https://mtgtop8.com/event?e=93781&d=912126&f=ST) | Captain James T. Kirk |
| 3–4 | James Drake | [Selesnya Landfall](https://mtgtop8.com/event?e=93781&d=912161&f=ST) | Elven Passage |
| 5–8 | Shuhei Nakamura | [Azorius Momo](https://mtgtop8.com/event?e=93781&d=912138&f=ST) | None |
| 5–8 | Charis Kikidis | [Izzet Prowess](https://mtgtop8.com/event?e=93781&d=912123&f=ST) | None |
| 5–8 | Abe Schnake | [Jund Sacrifice](https://mtgtop8.com/event?e=93781&d=912128&f=ST) | Q, Stacking the Deck; Wolverine, Fierce Fighter |
| 5–8 | Arne Huschenbeth | [UW Control](https://mtgtop8.com/event?e=93781&d=912122&f=ST) | Captain Marvel, Earth's Protector; Countersculpt; Jace's Machinations; Theorist's Sanctum |

Three lists have complete main-deck and sideboard coverage. Across all eight,
131 of 141 distinct identities are executable. Unsupported cards remain
whole-card unsupported declarations; importing a list does not imply complete
rules support. Run `make deck-report` for current catalog-derived coverage.

The remaining implementations need these rules boundaries:

- **Michelangelo:** replace counter placement, including counters placed as a
  creature enters, adding one counter per affected creature.
- **Pawpatch Recruit:** retain the creature targeted by the triggering spell or
  ability and exclude that identity from the new target choice.
- **Kirk:** choose multiple modes for a triggered ability when its controller's
  hand is empty, preserving target/continuation and checkpoint semantics.
- **Elven Passage:** compose the optional behold choice between a controlled
  Elf and a revealed Elf in hand, retaining the fetched land for the untap.
- **Q:** grant an activated ability to the exiled nonland card and create a
  castable copy from exile after paying its hand-exile cost.
- **Wolverine:** replace incoming damage by first clearing previously marked
  damage, then dealing the new damage normally.
- **Captain Marvel:** power-up's once-per-object activation and entry-turn
  discount of the source's complete mana cost, including colored symbols.
- **Countersculpt / Theorist's Sanctum:** behold-Jace in additional-cost and
  battlefield-entry contexts, plus empower's choice or creation of a Jace
  planeswalker token and loyalty counters.
- **Jace's Machinations:** empower Jace and temporary permission to activate
  loyalty abilities at instant speed on any player's turn.

The full Reality Fracture and Star Trek printing inventories are cataloged.
This change implements 47 new-set identities and six existing-set identities
(including the debut identities of three FRA reprints); the wider inventory
continues to report unsupported rules explicitly. Card data was checked against
the November 19, 2026 Scryfall reference snapshot.
