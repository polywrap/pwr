import {
  Client,
  Module,
  manifest,
  Args_start,
  HttpServer_Result,
} from "./wrap";

import { PluginFactory } from "@polywrap/core-js";
import { startHttpServer } from "./startHttpServer";

type NoConfig = Record<string, never>;

export class HttpServerPlugin extends Module<NoConfig> {
  async start(args: Args_start, client: Client): Promise<HttpServer_Result> {
    startHttpServer(args.port, args.requestTimeout, args.routes, args.onStart ?? undefined, client);

    return {
      ok: true
    };
  }
}
export const httpServerPlugin: PluginFactory<NoConfig> = () => {
  return {
    factory: () => new HttpServerPlugin({}),
    manifest,
  };
};

export const plugin = httpServerPlugin;
