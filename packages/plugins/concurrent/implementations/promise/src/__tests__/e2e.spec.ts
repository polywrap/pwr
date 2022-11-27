import { PolywrapClient } from "@polywrap/client-js";
import { PluginModule } from "@polywrap/core-js";
// import {
//   buildWrapper
// } from "@polywrap/test-env-js";
import { getClientConfig } from "./utils";

jest.setTimeout(300000);

describe("e2e", () => {
  let client: PolywrapClient;
  let ensUri: string;

  beforeAll(async () => {
    // await buildWrapper(`${__dirname}/integration`);
    const clientConfig = getClientConfig();
    client = new PolywrapClient(clientConfig);
    ensUri = `fs/${__dirname}/integration/build`;
  });

  test("check", async () => {
    const prodArr = await Promise.all([0, 1, 2, 3, 4].map(async () => {
      return await client.invoke({
        uri: "wrap://ens/interface.concurrent.polywrap.eth",
        method: "schedule",
        args: {
          task: {
            uri: "ens/greet.eth",
            method: "hello",
            args: JSON.stringify({
              greet: "Hello World!"
            })
          }
        }
      })
    }));
    let resArr: any[] = [];

    do {
      for (const prod of prodArr) {
        console.log(prod)
        if (!prod.ok) throw new Error("scheduling failed!");
        const res = (await client.invoke({
          uri: "wrap://ens/interface.concurrent.polywrap.eth",
          method: "result",
          args: {
            taskId: prod.value,
            timeout: 200
          }
        // @ts-ignore
        }))
        
        console.log(res)

        if (res.ok && res.value) {
          resArr.push(res.value)
        }
      }
      // const res = (await client.invoke({
      //   uri: "wrap://ens/interface.concurrent.polywrap.eth",
      //   method: "result",
      //   args: {
      //     // @ts-ignore
      //     taskId: result.value,
      //     timeout: 500
      //   }
      // })).value

      // console.log(res)
    } while (resArr.length != 5)

    console.log(resArr)
  })

  test("asyncBatchFetch", async () => {
    const result = await client.invoke({
      uri: ensUri,
      method: "asyncBatchFetch",
      args: { delays: ["1", "2", "3"] },
    });
    expect(result.ok).toBeTruthy();
    if (!result.ok) throw Error;
    expect(result.value).toBeTruthy();
    expect(result.value).toHaveLength(3);
  });

  test("batchFetch", async () => {
    const result = await client.invoke({
      uri: ensUri,
      method: "batchFetch",
      args: { delays: ["1", "2", "3"] },
    });
    expect(result.ok).toBeTruthy();
    if (!result.ok) throw Error;
    expect(result.value).toBeTruthy();
    expect(result.value).toHaveLength(3);
  });
});
