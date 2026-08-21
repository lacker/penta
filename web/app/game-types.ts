import type { FormatId } from "./game-config";

export type Owner = "human" | "opponent";

export type CardArtMetadata = {
  scryfallId: string;
  artist: string;
};

export type StackObjectKind = "Spell" | "ActivatedAbility" | "TriggeredAbility";

export type ImplementationStatus = "complete" | "partial" | "metadataOnly";

export type AbilityOriginMetadata =
  | { kind: "printed"; definition: number; partId: number; abilityId: number }
  | { kind: "token"; partId: number; abilityId: number }
  | { kind: "emblem"; abilityId: number }
  | {
      kind: "intrinsicBasicLand";
      landType: "plains" | "island" | "swamp" | "mountain" | "forest";
    }
  | {
      kind: "granted";
      source: number;
      sourceDefinition: number;
      sourcePartId: number;
      sourceAbilityId: number;
      grantId: number;
    }
  | {
      kind: "tokenGranted";
      source: number;
      sourcePartId: number;
      sourceAbilityId: number;
      grantId: number;
    }
  | {
      kind: "emblemGranted";
      source: number;
      sourceAbilityId: number;
      grantId: number;
    };

export type DecisionKind = "Choice" | "TriggerOrder" | "TriggerPlacement";

export type SpellFormMetadata =
  | { kind: "part"; partId: number }
  | { kind: "combined"; partIds: number[] };

export type TargetSelectionMetadata = {
  slotId: number;
  amounts: number[];
  targetCardIds: number[];
  targetPlayers: Owner[];
  targetStackIds: number[];
};

export type CastTargetSelectionMetadata = TargetSelectionMetadata;

export type AttackDefenderMetadata =
  | { kind: "player"; player: Owner }
  | { kind: "planeswalker"; cardId: number };

export type CastSignatureMetadata = {
  playOptionId: number;
  form: SpellFormMetadata;
  modeIds: number[];
  alternativeCostId?: number | null;
  additionalCostIds: number[];
  x: number;
  targetSelections: TargetSelectionMetadata[];
};

export type Emblem = {
  id: number;
  owner: Owner;
  name: string;
  rulesText: string;
  abilityTexts: string[];
  sourceAbility: AbilityOriginMetadata;
};

export type Card = {
  /** Current-zone GameObjectId used by actions, targets, and DOM identity. */
  id: number;
  /** Logical card part supplying this permanent's visible characteristics. */
  partId?: number;
  /** Physical double-faced topology, independent of copied characteristics. */
  physicalFace?: {
    kind: "transforming" | "modal";
    side: "front" | "back";
  };
  name: string;
  art: CardArtMetadata | null;
  kind: string;
  typeLine?: string;
  implementationStatus: ImplementationStatus;
  isLand?: boolean;
  rulesText: string;
  manaCost?: {
    generic: number;
    white: number;
    blue: number;
    black: number;
    red: number;
    green: number;
    /** One entry per hybrid pair the cost carries, such as `{ symbol: "R/W", count: 3 }`. */
    hybrid: { symbol: string; count: number }[];
    x: boolean;
  } | null;
  owner?: Owner;
  chosenCardName?: string | null;
  chosenCreatureType?: string | null;
  tapped?: boolean;
  power?: number | null;
  toughness?: number | null;
  damage?: number;
  loyalty?: number | null;
  loyaltyAbilityUsedThisTurn?: boolean;
  attacking?: boolean;
  attackDefender?: AttackDefenderMetadata | null;
  blockedThisCombat?: boolean;
  /** Every attacker this creature is blocking, since a band is blocked as
   * a group and one creature may be allowed several blocks. Emptied as those
   * attackers leave combat, which does not stop this creature blocking. */
  blocking?: number[];
  /** Whether it has blocked at all this combat. This is the one to read to
   * ask whether it is a blocking creature. */
  blockingThisCombat?: boolean;
  flying?: boolean;
  canAttack?: boolean;
  enteredThisTurn?: boolean;
  xValue?: number | null;
};

export type Action = {
  index: number;
  label: string;
  kind: "primary" | "combat" | "pass" | "danger";
  /** Current source GameObjectId, despite the legacy JSON field name. */
  cardId?: number | null;
  targetCardId?: number | null;
  targetPlayer?: Owner | null;
  targetStackId?: number | null;
  targetCardIds?: number[];
  targetPlayers?: Owner[];
  targetStackIds?: number[];
  targetCount?: number;
  targetSelections?: TargetSelectionMetadata[];
  attackDefender?: AttackDefenderMetadata | null;
  ability?: AbilityOriginMetadata | null;
  /** Target-independent activation label; includes exact ability text when disambiguation is needed. */
  abilityLabel?: string | null;
  manaAbility?: boolean;
  spellAction?: boolean;
  playOptionId?: number | null;
  modeIds?: number[] | null;
  sacrificeCardIds?: number[];
  combatDamageAttacker?: number | null;
  x?: number | null;
  paymentAction?: boolean;
  manaSourceIds?: number[];
  decisionId?: number | null;
  decisionOptionIds?: number[];
  /** Hand-card IDs in one legal London-mulligan bottom combination. */
  bottomCardIds: number[];
};

export type OpponentAction = {
  label: string;
  // "turn" beats carry no action of their own — they exist so a turn nobody
  // acted on is still announced.
  kind: "land" | "spell" | "ability" | "combat" | "choice" | "turn" | "draw";
  card?: string | null;
  cardId?: number | null;
  manaSources?: string[];
  state: GameState;
};

export type PlayerState = {
  life: number;
  library: number;
  mana: {
    white: number;
    blue: number;
    black: number;
    red: number;
    green: number;
    colorless: number;
  };
  hand?: Card[];
  handSize?: number;
  graveyard: Card[];
};

export type DecisionOption = {
  id: number;
  label: string;
  cardId?: number | null;
  cardName?: string | null;
  members?: Array<{ id: number; name: string }>;
  /** Pending trigger identity, when this option represents a trigger. */
  triggerId?: number | null;
  /** The exact ability text, when it is narrower than the source card text. */
  abilityText?: string | null;
  zone: string;
};

export type DecisionState = {
  id: number;
  kind: DecisionKind;
  /** Trigger-order submissions are always listed in desired resolution order. */
  orderSemantics?: "resolution";
  prompt: string;
  minimum: number;
  maximum: number;
  cancellable: boolean;
  visibility: string;
  options: DecisionOption[];
};

/**
 * A hosted room's move clock. Absent for a local game, which has no clock:
 * nobody else is waiting on you.
 */
export type MoveClock = {
  seat: "human" | "bot";
  /** Epoch milliseconds, from the server's clock. */
  deadline: number;
};

export type GameState = {
  format: FormatId;
  /** Only in a hosted game, and only while it is live. */
  moveClock?: MoveClock;
  turn: number;
  gameTurn: number;
  step: string;
  regularCombatDamagePending: boolean;
  /// Opening hands are still being settled, so no turn or step has begun.
  pregame: boolean;
  active: string;
  priority: string;
  human: PlayerState & { hand: Card[] };
  opponent: PlayerState & { handSize: number };
  battlefield: Card[];
  emblems: Emblem[];
  stack: Array<{
    id: number;
    /** Deprecated JSON compatibility alias for `id`; never use for targeting. */
    cardId?: number;
    /** Historical source game-object ID for an ability. */
    sourceId?: number | null;
    /** Stable positional identifier of the printed ability that created this object. */
    abilityId?: number | null;
    /** Full frozen origin, including printed/token clauses, intrinsic subtype, and grants. */
    ability?: AbilityOriginMetadata | null;
    signature?: CastSignatureMetadata | null;
    name: string;
    art: CardArtMetadata | null;
    owner: Owner;
    kind: StackObjectKind;
    counterable?: boolean;
    cardKind: string;
    typeLine?: string;
    implementationStatus: ImplementationStatus;
    isLand?: boolean;
    manaCost?: Card["manaCost"];
    rulesText: string;
    /** Exact stack ability text, when available separately from source rules. */
    abilityText?: string | null;
    power?: number | null;
    toughness?: number | null;
    x: number;
    targetCardIds: number[];
    targetPlayers: Owner[];
    targetStackIds: number[];
  }>;
  actions: Action[];
  passLabel: string | null;
  decision: DecisionState | null;
  opponentActions?: OpponentAction[];
  /// The board the moment your own action landed, before the game answered.
  /// Only sent when there is a replay to run; the replay starts from here.
  afterYourAction?: GameState | null;
  canUndoMana: boolean;
  canCancelAttackers: boolean;
  phaseStops: string[];
  autopassEnabled: boolean;
  result: null | { outcome: "win" | "loss" | "draw"; message: string };
  events: string[];
};
