import { PolywrapClient, Uri } from "@polywrap/client-js";
import { msgpackEncode } from "@polywrap/msgpack-js";
import prompts from "prompts";
import { extractAccessControlledUris } from "./extractAccessControlledUris";
import { invokeWithAccessControl } from "./getPolywrapClient";

export const runApp = async (uri: string, args: string[], polywrapClient: PolywrapClient) => {
  const acessControlledUris: string[] = [];
  const visitedUris = new Set<string>();
  await extractAccessControlledUris(uri, polywrapClient, acessControlledUris, visitedUris);

  if (acessControlledUris.length > 0) {
    const response = await prompts({
      type: "confirm",
      name: 'isAllowed',
      message: `App requested access to: \n${acessControlledUris.join("\n")}. \nDo you want to grant access?`
    });
  
    if (!response.isAllowed) {
      console.log(`Denied access for ${uri}`);
      return;
    }
  }
  
  const { data, error: invokeError } = await invokeWithAccessControl(
    {
      uri: new Uri(uri),
      method: "main",
      args: msgpackEncode({
        args
      }),
    }, 
    acessControlledUris, 
    polywrapClient
  );

  if (invokeError) {
    console.error(invokeError);
  } else {
    if (data) {
      if (Number.isInteger(data)) {
        process.exit(data as number);
      } else {
        console.error(`Expected exit code, got: `, data);
      }
    }
  }
};
