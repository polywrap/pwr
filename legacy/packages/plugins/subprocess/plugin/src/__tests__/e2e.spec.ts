import path from "path";
import { Subprocess_Module } from "./types";

import { PolywrapClient } from "@polywrap/client-js";
import { ClientConfigBuilder } from "@polywrap/client-config-builder-js";
import { subprocessPlugin } from "..";

jest.setTimeout(360000);

describe("Subprocess plugin", () => {
  let wrapperUri = "ens/goerli/subprocess.polywrap.eth";
  let client: PolywrapClient;

  beforeAll(() => {
    const config = new ClientConfigBuilder()
      .addDefaults()
      .addPlugin(wrapperUri, subprocessPlugin({}))
      .build();
    client = new PolywrapClient(config);
  });

  test("exec", async () => {
    const result = await Subprocess_Module.exec(
      {
        command: `echo "Hello World!"`,
      },
      client,
      wrapperUri
    );
    expect(result.data).toMatchObject({stdout: "Hello World!\n"});
  });
});
