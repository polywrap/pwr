import { Wrapper, Uri } from "@polywrap/client-js";
import { IWrapperCache } from "@polywrap/uri-resolvers-js";
import { cacheWrapper } from "./cacheWrapper";

export class CustomWrapperCache implements IWrapperCache {
  private _map: Map<string, Wrapper> = new Map();

  async get(uri: Uri): Promise<Wrapper | undefined> {
    return this._map.get(uri.uri);
  }

  async set(uri: Uri, wrapper: Wrapper): Promise<void> {
    if (uri.uri.startsWith("wrap://ipfs/")) {
      cacheWrapper(uri.uri, wrapper);  
    }
  
    this._map.set(uri.uri, wrapper);
  }
}
