import { PolywrapClient, Uri, UriResolutionContext, Wrapper } from "@polywrap/client-js";
import { allAccessControlledUris } from "./getPolywrapClient";
import { cacheWrapper } from "./cacheWrapper";

export const extractAccessControlledUris = async (
  uri: string, 
  polywrapClient: PolywrapClient,
  acessControlledUris: string[],
  visitedUris: Set<string>
): Promise<void> => {
  if (visitedUris.has(uri)) {
    return;
  }
  visitedUris.add(uri);

  const result = await polywrapClient.tryResolveUri({uri });
  const wrapper: Wrapper = await polywrapClient["_loadWrapper"]({ uri });
  if (!result.ok) {
    return;
  }

  const manifest = await wrapper.getManifest({ noValidate: false }, polywrapClient);
  const importedUris = (manifest.abi.importedModuleTypes || []).map((importedModuleType) => new Uri(importedModuleType.uri).uri);
  
  const requestedUris = importedUris.filter((importedUri) => allAccessControlledUris.includes(importedUri));
  const otherUris = importedUris.filter((importedUri) => !allAccessControlledUris.includes(importedUri));
  if (requestedUris.length > 0) {
    acessControlledUris.push(...requestedUris);
  }

  for (const otherUri of otherUris) {
    await extractAccessControlledUris(otherUri, polywrapClient, acessControlledUris, visitedUris);
  }
};
