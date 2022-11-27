import { ClientConfig, Uri } from "@polywrap/client-js";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { ensAddresses, providers } from "@polywrap/test-env-js";

import { concurrentPromisePlugin } from "..";
import { PluginModule, PluginFactory } from "@polywrap/core-js";

export interface TestEnvironment {
  ipfs: string;
  ethereum: string;
  ensAddress: string;
}

export const testEnv: TestEnvironment = {
  ipfs: providers.ipfs,
  ethereum: providers.ethereum,
  ensAddress: ensAddresses.ensAddress,
};

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export interface GreetConfig extends Record<string, never> {}

export class GreetPlugin extends PluginModule<GreetConfig> {
  constructor(config: GreetConfig) {
    super(config);
  }

  public async hello(args: { greet: string }): Promise<string> {
    console.log(args.greet);
    await sleep(1000);
    return args.greet;
  }
}

// @ts-ignore
export const greetPlugin: PluginFactory<GreetConfig> = (
  config: GreetConfig
) => {
  return {
    factory: () => new GreetPlugin(config),
    manifest: undefined,
  };
};

export function getClientConfig(): ClientConfig<Uri> {
  return new ClientConfigBuilder()
    .addDefaults()
    .addPlugin(
      "wrap://ens/interface.concurrent.polywrap.eth",
      concurrentPromisePlugin({})
    )
    .addPlugin("wrap://ens/greet.eth", greetPlugin({}))
    .build();
}
