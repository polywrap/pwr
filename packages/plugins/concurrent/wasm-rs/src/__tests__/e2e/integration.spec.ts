import { PolywrapClient } from "@polywrap/client-js";
import { PluginModule, PluginFactory } from "@polywrap/core-js";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import * as App from "../types/wrap";
import { concurrentPromisePlugin } from "../../../../implementations/promise/src";
import path from "path";

jest.setTimeout(60000);

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
    await sleep(2000);
    return args.greet;
  }
}

// @ts-ignore
export const greetPlugin: PluginFactory<
  GreetConfig
> = (config: GreetConfig) => {
  return {
    factory: () => new GreetPlugin(config),
    manifest: undefined
  };
};

describe("e2e", () => {
  let client: PolywrapClient;
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;

    const config = new ClientConfigBuilder()
      .addDefaults()
      .addPlugin(
        "ens/interface.concurrent.polywrap.eth",
        concurrentPromisePlugin({})
      )
      .addPlugin(
        "ens/hello.eth",
        greetPlugin({})
      )
      .build();
    client = new PolywrapClient(config);
  });

  it("greet", async () => {
    const expected: string = "polywrap";

    const result = await client.invoke<App.Boolean>({
      uri: wrapperUri,
      method: "run",
    });

    console.log(result);
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value).toEqual(expected);
  });
});
