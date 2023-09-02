import * as IPFS from "ipfs-core";

export const ipfsGet = async (
  ipfs: IPFS.IPFS, 
  ipfsPath: string,
  options?: { timeout?: number }
): Promise<Uint8Array> => {
  const stream = ipfs.get(ipfsPath, options);

  let data: Uint8Array = new Uint8Array();

  for await (const chunk of stream) {
    const temp = new Uint8Array(data.length + chunk.length);
    temp.set(data);
    temp.set(chunk, data.length);
    data = temp;
  }

  return data;
};
