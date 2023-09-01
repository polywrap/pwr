import {
  Uri,
  UriPackageOrWrapper,
  UriResolutionResult,
} from "@polywrap/core-js";
import { Result } from "@polywrap/result";
import {
  ResolverWithHistory,
} from "@polywrap/uri-resolvers-js";

export class PwrCommandsResolver extends ResolverWithHistory<unknown> {
  constructor() {
    super();
  }

  protected getStepDescription = (): string =>
    `PwrResolver`;

  protected async _tryResolveUri(
    uri: Uri
  ): Promise<Result<UriPackageOrWrapper, unknown>> {
    if (!uri.uri.startsWith("wrap://pwr/")) {
      return UriResolutionResult.ok(uri);
    }
    
    const path = uri.uri.slice("wrap://pwr/".length, uri.uri.length);

    switch (path) {
      case "echo":
        return UriResolutionResult.ok(new Uri(`wrap://ens/wrap-echo.eth`));
      default:
        return UriResolutionResult.ok(new Uri(`wrap://ens/${path}.pwr-app.eth`));
    }
  }
}
