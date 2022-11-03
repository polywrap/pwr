import {
  Uri,
  UriPackageOrWrapper,
  UriResolutionResult,
} from "@polywrap/core-js";
import { Result } from "@polywrap/result";
import {
  ResolverWithHistory,
} from "@polywrap/uri-resolvers-js";
import fs from "fs";
import { paths } from "../main";

export class FileSystemCacheResolver extends ResolverWithHistory<unknown> {
  constructor() {
    super();
  }

  protected getStepDescription = (): string =>
    `FileSystemCacheResolver`;

  protected async _tryResolveUri(
    uri: Uri
  ): Promise<Result<UriPackageOrWrapper, unknown>> {
    if (!uri.uri.startsWith("wrap://ipfs/")) {
      return UriResolutionResult.ok(uri);
    }
    
    const ipfsCid = uri.uri.slice("wrap://ipfs/".length, uri.uri.length);
    if (!fs.existsSync(`${paths.cache.wrappers.ipfs}/${ipfsCid}`)) {
      return UriResolutionResult.ok(uri);
    }
    return UriResolutionResult.ok(new Uri(`wrap://file/${paths.cache.wrappers.ipfs}/${ipfsCid}`));
  }
}
