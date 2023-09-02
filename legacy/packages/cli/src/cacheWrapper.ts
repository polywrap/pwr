import { Wrapper } from "@polywrap/client-js";
import { WasmWrapper } from "@polywrap/wasm-js";
import { paths } from "./main";
import fs from "fs";

export const cacheWrapper = async (uri: string, wrapper: Wrapper) => {
  if (!(wrapper instanceof WasmWrapper)) {
    return;
  }

  const manifestBuffer = await (wrapper as WasmWrapper).getFile({ path: "wrap.info" });
  const wrapManifest = await (wrapper as WasmWrapper).getManifest();

  const ipfsCid = uri.replace("wrap://ipfs/", "");

  if (!fs.existsSync(`${paths.cache.wrappers.ipfs}/${ipfsCid}`)) {
    fs.mkdirSync(`${paths.cache.wrappers.ipfs}/${ipfsCid}`);
  
    fs.writeFileSync(`${paths.cache.wrappers.ipfs}/${ipfsCid}/wrap.info`, manifestBuffer);

    if (wrapManifest && wrapManifest.type === "wasm") {
      const wasmModule = await (wrapper as WasmWrapper).getFile({ path: "wrap.wasm" });
      fs.writeFileSync(`${paths.cache.wrappers.ipfs}/${ipfsCid}/wrap.wasm`, wasmModule);
    }
  }
};
