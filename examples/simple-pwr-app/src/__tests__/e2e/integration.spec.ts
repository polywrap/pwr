import { PolywrapClient } from "@polywrap/client-js";
import path from "path";

jest.setTimeout(120000);

type MaybeUriOrManifest = {
  uri: string;
  manifest: Uint8Array;
};

describe("wrapscan-uri-resolver e2e tests", () => {
  const client: PolywrapClient = new PolywrapClient();

  const testWrapPath = "wrap/test-wrap@1.0.0";
  const wrapscanUrl = "http/wraps.wrapscan.io";
  const wrapscanDevUrl = "http/dev.wraps.wrapscan.io";
  const resolvePath = "/r/";

  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  });

  it("incorrect authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "foo",
        path: testWrapPath,
      },
    });

    expect(result.ok).toBeTruthy();
    if (result.ok) {
      expect(result.value).toBe(null);
    }
  });

  it("correct authority", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "wrapscan.io",
        path: testWrapPath,
      },
    });

    expect(result.ok).toBeTruthy();

    if (result.ok) {
      expect(result.value?.uri).toBe(wrapscanUrl + resolvePath + testWrapPath);
      expect(result.value?.manifest).toBeNull();
    }
  });

  it("custom provider", async () => {
    const result = await client.invoke<MaybeUriOrManifest | null>({
      uri: wrapperUri,
      method: "tryResolveUri",
      args: {
        authority: "wrapscan.io",
        path: testWrapPath,
      },
      env: {
        providerUrl: wrapscanDevUrl,
      },
    });

    expect(result.ok).toBeTruthy();

    if (result.ok) {
      expect(result.value?.uri).toBe(wrapscanDevUrl + resolvePath + testWrapPath);
      expect(result.value?.manifest).toBeNull();
    }
  });

  it("getFile", async () => {
    const result = await client.invoke<Uint8Array>({
      uri: wrapperUri,
      method: "getFile",
      args: {
        path: testWrapPath + "/wrap.info",
      },
    });

    if (!result.ok) fail(result.error);
    expect(result.value).toStrictEqual(null);
  });
});
