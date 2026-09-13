# Legacy and Vintage

Both formats use the existing constructed rules: 20 life, a 60-card main-deck
minimum, up to 15 sideboard cards, London mulligans, and modern mana rules.
Legacy permits four copies except basic lands. Vintage applies its restricted
list to the main deck and sideboard combined, with four copies of other
nonbasic cards.

The profiles use the tournament-eligible paper sets already represented in
Penta's catalog. This is a growing subset of the real card pools; registering a
format does not require importing complete sets. Card availability, format
legality, and executable rules support are separate questions. Each imported
card resolves to a catalog identity, and incomplete rules remain whole-card
`CardRules::unsupported()` declarations with an inline capability-gap audit.

Ban and restriction policy was checked on September 12, 2026 against the
[Wizards list](https://magic.wizards.com/en/banned-restricted-list). Category
bans for ante, Conspiracy, offensive cards, stickers, and Attractions are
expanded into names using the local Scryfall snapshot. Future card/set imports
and ban announcements require reviewing this policy; it is not fetched at
runtime. `src/formats/eternal.rs` owns both profiles.

## Initial event corpus

- [Legacy MTGO Challenge 32, September 8, 2026](https://mtgtop8.com/event?e=90648&f=LE):
  all sixteen published lists in `decks/legacy/`.
- [The 31th God of Vintage Tournament, Tokyo, September 3, 2026](https://mtgtop8.com/event?e=90586&f=VI):
  all eight published lists in `decks/vintage/`.

Each YAML file records its exact deck URL, player, placing, and published
counts. Lists come from the event's `.dec` export. Both Legacy Death & Taxes
lists have 80 main-deck cards; The_shallow_grave's Reanimator list has 61.
Every other main deck has 60, and every sideboard has 15. Companion cards remain
in their published sideboards. They are available as companions only when the
card's complete companion condition and gameplay are implemented.

Names resolve to existing canonical identities wherever possible. Multifaced
names are expanded to `front // back`, except the existing Brazen Borrower
adventure identity, which retains its established catalog name. The source's
`Rough / Tumble` is normalized to `Rough // Tumble`. No card counts or choices
are substituted to increase playability.

Native, protocol, and browser setup use `legacy` and `vintage`. The browser
menus include all imported lists and disclose that some card effects are
unavailable. Run `make deck-report` for each deck's current unsupported names
and `make catalog-report` for format coverage. Supporting additional cards
updates these reports without a manually maintained readiness list.

The import contains 313 distinct card identities: 287 were already cataloged
and 26 were added. Of those additions, 17 have complete declarative rules using
existing mechanics; nine are whole-card unsupported entries with the missing
capability documented beside each declaration. Three existing unsupported
entries are also implemented: Deafening Silence, Karn, the Great Creator, and
Kozilek's Command. Consign to Memory also gains its missing replicate-copy
trigger.
