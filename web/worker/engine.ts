/**
 * The generated WASM engine, loaded lazily for Worker requests that need it.
 * The server-render test evaluates the Worker entry point under Node, where a
 * bundled `.wasm` import is not a compiled module, so initialization must not
 * happen merely because the entry point was imported.
 */
export type EngineModule = typeof import("../app/wasm/penta_wasm.js");

let engineReady: Promise<EngineModule> | null = null;

export function engine(): Promise<EngineModule> {
  engineReady ??= (async () => {
    const [module, wasm] = await Promise.all([
      import("../app/wasm/penta_wasm.js"),
      // Workers cannot fetch a local file, so hand wasm-bindgen the compiled
      // module supplied by the bundler instead of asking it to fetch a URL.
      import("../app/wasm/penta_wasm_bg.wasm"),
    ]);
    await module.default({ module_or_path: wasm.default });
    return module;
  })();
  return engineReady;
}
