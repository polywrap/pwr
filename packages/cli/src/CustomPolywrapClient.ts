import { InvokeResult, InvokerOptions, PolywrapClient, PolywrapClientConfig, Uri } from "@polywrap/client-js";

export class CustomPolywrapClient extends PolywrapClient {
  constructor(config?: Partial<PolywrapClientConfig<string | Uri>>, options?: {
    noDefaults?: boolean;
  }) {
    super(config, options);
  }

  async invoke<TData = unknown, TUri extends Uri | string = string>(options: InvokerOptions<TUri, PolywrapClientConfig>): Promise<InvokeResult<TData>> {
    try {
      // console.log(`Invoking ${Uri.from(options.uri).uri}`);
      const result = await super.invoke<TData, TUri>(options);
      return result;
    }
    catch (error) {
      // console.log(`Error invoking ${Uri.from(options.uri).uri}`);
      throw error;
    }
  }
}