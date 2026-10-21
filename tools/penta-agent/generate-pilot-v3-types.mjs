import { writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { declarations } from "./generate-pilot-types.mjs";
import { pilotV3Schema } from "./pilot-v3-contract.mjs";

export const pilotV3Declarations = () => declarations(pilotV3Schema, "composed pilot-v3-contract.mjs");
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  writeFileSync(new URL("./pilot-v3.d.ts", import.meta.url), pilotV3Declarations());
}
