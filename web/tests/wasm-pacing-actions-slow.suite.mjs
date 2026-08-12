import test from "node:test";

import {
  creaturelessSecondMainsWaitForUsableActions,
  passButtonLabelMatchesDestination,
} from "./wasm-pacing-slow-scenarios.mjs";

test(
  "[slow] the pass button label matches where the click actually lands",
  passButtonLabelMatchesDestination,
);

test(
  "[slow] creatureless second mains wait exactly for usable card actions",
  creaturelessSecondMainsWaitForUsableActions,
);
