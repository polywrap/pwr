import { Client, coreInterfaceUris, InvokerOptions, PolywrapClient, PolywrapClientConfig, Uri } from "@polywrap/client-js";
import { Connection, Connections, ethereumPlugin } from "@polywrap/ethereum-plugin-js";
import { ipfsPlugin } from "@polywrap/ipfs-plugin-js";
import { ipfsResolverPlugin } from "@polywrap/ipfs-resolver-plugin-js";
import { fileSystemPlugin } from "@polywrap/fs-plugin-js";
import { fileSystemResolverPlugin } from "@polywrap/fs-resolver-plugin-js";
import { ocrResolverPlugin } from "@nerfzael/ocr-resolver-plugin-wrapper";
import { ensContenthashResolverPlugin } from "@nerfzael/ens-contenthash-resolver-plugin-wrapper";
import { ipfsEnsContenthashResolverPlugin } from "@nerfzael/ipfs-ens-contenthash-resolver-plugin-wrapper";
import { ocrEnsContenthashResolverPlugin } from "@nerfzael/ocr-ens-contenthash-resolver-plugin-wrapper";
import { wrapClientPlugin } from "@nerfzael/wrap-client-plugin-wrapper";
import { subprocessPlugin } from "@wraplib/subprocess-plugin-js";
import { concurrentPromisePlugin } from "@wraplib/concurrent-promise-plugin";
import {
  PackageToWrapperCacheResolver,
  RecursiveResolver,
  buildUriResolver,
  LegacyRedirectsResolver,
  LegacyPluginsResolver,
} from "@polywrap/uri-resolvers-js";
import { ExtendableUriResolver } from "@polywrap/uri-resolver-extensions-js";
import { FileSystemCacheResolver } from "./resolvers/FileSystemCacheResolver";
import { loggerPlugin } from "@polywrap/logger-plugin-js";
import { CustomWrapperCache } from "./CustomWrapperCache";
import { PwrCommandsResolver } from "./resolvers/PwrCommandsResolver";
import { httpServerPlugin } from "@wraplib/http-server-plugin-wrapper";
import { ipfsPlugin as wrapIpfsPlugin } from "@wraplib/ipfs-plugin-wrapper";
import { CustomPolywrapClient } from "./CustomPolywrapClient";

export const allAccessControlledUris = [
  "wrap://ens/ens-resolver.polywrap.eth",
  "wrap://ens/ethereum.polywrap.eth",
  "wrap://ens/http.polywrap.eth",
  "wrap://ens/ipfs-resolver.polywrap.eth",
  "wrap://ens/ens-contenthash-resolver.eth",
  "wrap://ens/ipfs-ens-contenthash-resolver.eth",
  "wrap://ens/ocr-ens-contenthash-resolver.eth",
  "wrap://ens/ocr-resolver.eth",
  "wrap://ens/wrap-client.eth",
  "wrap://ens/ipfs.polywrap.eth",
  "wrap://ens/fs.polywrap.eth",
  "wrap://ens/fs-resolver.polywrap.eth",
  "wrap://ens/http-server.eth",
  "wrap://ens/wrap-ipfs.eth",
  "wrap://ens/goerli/v2.interface.concurrent.polywrap.eth",
  "wrap://ens/goerli/interface.subprocess.polywrap.eth",
];

export let accessControlledUris: string[] = [
];

export let allAllowedUris: string[] = [];

export const invokeAsAdmin = async (
  options: InvokerOptions<string, PolywrapClientConfig>,
  polywrapClient: PolywrapClient
) => {
  accessControlledUris = [];

  return polywrapClient.invoke(options);
};

export const invokeWithAccessControl = async (
  options: InvokerOptions<Uri, PolywrapClientConfig>, 
  allowedUris: string[], 
  polywrapClient: PolywrapClient
) => {
  accessControlledUris = allAccessControlledUris;
  allAllowedUris = allowedUris;

  return polywrapClient.invoke(options);
};

export const defaultIpfsProviders = [
  "https://ipfs.wrappers.io",
  "https://ipfs.io",
];

export const getPolywrapClient = (): PolywrapClient => {
  const config = {
    ethereum: {
      providers: {
        mainnet: `https://mainnet.infura.io/v3/${process.env.INFURA_PROJECT_ID}`,
        ropsten: `https://ropsten.infura.io/v3/${process.env.INFURA_PROJECT_ID}`,
        rinkeby: `https://rinkeby.infura.io/v3/${process.env.INFURA_PROJECT_ID}`,
        goerli: `https://goerli.infura.io/v3/${process.env.INFURA_PROJECT_ID}`,
        polygon: `https://polygon-mainnet.infura.io/v3/${process.env.INFURA_PROJECT_ID}`
      },
    }
  };

  const envs = [
    {
      uri: "wrap://ens/http.polywrap.eth",
      env: {
        urlPrefixWhitelist: [
        ],
        urlPrefixBlocklist: [
          "file",
          "localhost",
          "http://localhost",
          "https://localhost",
          "127.0.0.1",
          "http://127.0.0.1",
          "https://127.0.0.1",
        ]
      }
    },
    {
      uri: new Uri("wrap://ens/ipfs.polywrap.eth"),
      env: {
        provider: defaultIpfsProviders[0],
        fallbackProviders: defaultIpfsProviders.slice(1),
      },
    },
  ];

  const interfaces = [
    {
      interface: coreInterfaceUris.uriResolver.uri,
      implementations: [
        "wrap://ens/fs-resolver.polywrap.eth",
        "wrap://ens/ipfs-resolver.polywrap.eth",
        "wrap://ens/ens-contenthash-resolver.eth",
        "wrap://ens/ipfs-ens-contenthash-resolver.eth",
        "wrap://ens/ocr-ens-contenthash-resolver.eth",
        "wrap://ens/ocr-resolver.eth"
      ],
    },
  ];

  const plugins = [
    {
      uri: "wrap://ens/ipfs-resolver.polywrap.eth",
      plugin: ipfsResolverPlugin({}),
    },
    {
      uri: "wrap://ens/ens-contenthash-resolver.eth",
      plugin: ensContenthashResolverPlugin({})
    },
    {
      uri: "wrap://ens/ipfs-ens-contenthash-resolver.eth",
      plugin: ipfsEnsContenthashResolverPlugin({})
    },
    {
      uri: "wrap://ens/ocr-ens-contenthash-resolver.eth",
      plugin: ocrEnsContenthashResolverPlugin({})
    },
    {
      uri: "wrap://ens/ocr-resolver.eth",
      plugin: ocrResolverPlugin({})
    },
    {
      uri: "wrap://ens/wrap-client.eth",
      plugin: wrapClientPlugin({})
    },
    {
      uri: "wrap://ens/ipfs.polywrap.eth",
      plugin: ipfsPlugin({}),
    },
    {
      uri: "wrap://ens/fs.polywrap.eth",
      plugin: fileSystemPlugin({}),
    },
    {
      uri: "wrap://ens/fs-resolver.polywrap.eth",
      plugin: fileSystemResolverPlugin({}),
    },
    {
      uri: "wrap://ens/goerli/logger.polywrap.eth",
      plugin: loggerPlugin({})
    },
    {
      uri: "wrap://ens/http-server.eth",
      plugin: httpServerPlugin({})
    },
    {
      uri: "wrap://ens/wrap-ipfs.eth",
      plugin: wrapIpfsPlugin({})
    },
    {
      uri: "wrap://ens/goerli/interface.subprocess.polywrap.eth",
      plugin: subprocessPlugin({})
    },
    {
      uri: "wrap://ens/goerli/v2.interface.concurrent.polywrap.eth",
      plugin: concurrentPromisePlugin({ clientFactory: getPolywrapClient })
    }
  ];

  const resolver = new RecursiveResolver(
    new PackageToWrapperCacheResolver(
      new CustomWrapperCache(),
      buildUriResolver([
        new LegacyRedirectsResolver(),
        new LegacyPluginsResolver(),
        new FileSystemCacheResolver(),
        new PwrCommandsResolver(),
        new ExtendableUriResolver(),
      ])
    )
  );

  const redirects = [
    {
      from: "wrap://ens/monowrap.eth",
      to: "fs/Users/niraj/Documents/projects/polywrap/monowrap/build"
    }
  ]

  const client = process.env.INFURA_PROJECT_ID
    ? new CustomPolywrapClient(
      {
        envs,
        interfaces,
        redirects,
        plugins: [
          ...plugins,
          {
            uri: "wrap://ens/ethereum.polywrap.eth",
            plugin: ethereumPlugin({
              connections: new Connections({
                networks: {
                  mainnet: new Connection({
                    provider: config.ethereum.providers.mainnet
                  }),
                  ropsten: new Connection({
                    provider: config.ethereum.providers.ropsten
                  }),
                  rinkeby: new Connection({
                    provider: config.ethereum.providers.rinkeby
                  }),
                  goerli: new Connection({
                    provider: config.ethereum.providers.goerli
                  }),
                  polygon: new Connection({
                    provider: config.ethereum.providers.polygon
                  }),
                }
              })
            }),
          },
        ],
        resolver,
      }, { noDefaults: true})
    : new CustomPolywrapClient({
        envs,
        interfaces,
        redirects,
        plugins: [
          ...plugins,
          {
            uri: "wrap://ens/ethereum.polywrap.eth",
            plugin: ethereumPlugin({
              connections: new Connections({
                networks: {
                  mainnet: new Connection({
                    provider:
                      "https://mainnet.infura.io/v3/b00b2c2cc09c487685e9fb061256d6a6",
                  }),
                  goerli: new Connection({
                    provider:
                      "https://goerli.infura.io/v3/b00b2c2cc09c487685e9fb061256d6a6",
                  }),
                },
              }),
            }),
          }
        ],
        resolver,
      }, { noDefaults: true});

  return client;
};