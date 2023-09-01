import {
  Client,
  Module,
  manifest,
  Args_create,
  Args_cat,
  Args_get,
  Args_isOnline,
  Args_resolve,
  Args_stop,
  Args_start,
} from "./wrap";

import { PluginFactory } from "@polywrap/core-js";
import { create, IPFS } from "ipfs-core";
import { ipfsCat } from "./ipfsCat";
import { ipfsGet } from "./ipfsGet";

type NoConfig = Record<string, never>;

export class IpfsPlugin extends Module<NoConfig> {
  node: IPFS | undefined;

  async create(args: Args_create, client: Client): Promise<boolean> {
    this.node = await create({});

    return true;
  }

  async cat(args: Args_cat, client: Client): Promise<Uint8Array> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    return ipfsCat(
      this.node, 
      args.ipfsPath, 
      args.options 
        ? { timeout: args.options?.timeout ?? undefined } 
        : undefined
    );
  }

  async get(args: Args_get, client: Client): Promise<Uint8Array> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    return ipfsGet(
      this.node, 
      args.ipfsPath, 
      args.options 
        ? { timeout: args.options?.timeout ?? undefined } 
        : undefined
    );
  }

  async start(_: Args_start, client: Client): Promise<boolean> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    await this.node.start();

    return true;
  }

  async stop(_: Args_stop, client: Client): Promise<boolean> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    await this.node.stop();

    return true;
  }

  async resolve(args: Args_resolve, client: Client): Promise<string> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    const result = await this.node.resolve(
      args.name, 
      args.options 
        ? { timeout: args.options?.timeout ?? undefined } 
        : undefined
    );

    return result;
  }

  async isOnline(_: Args_isOnline, client: Client): Promise<boolean> {
    if (!this.node) {
      throw new Error("IPFS node is not initialized");
    }

    const online = await this.node.isOnline();

    return online;
  }
}
export const ipfsPlugin: PluginFactory<NoConfig> = () => {
  return {
    factory: () => new IpfsPlugin({}),
    manifest,
  };
};

export const plugin = ipfsPlugin;
