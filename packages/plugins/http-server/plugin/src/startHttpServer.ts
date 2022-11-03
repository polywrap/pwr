import { Client, UriResolutionContext } from "@polywrap/core-js";
import express, { Request, Response, NextFunction } from "express";
import cors from "cors";
import timeout from "connect-timeout";
import http from "http";
import { handleError } from "./handleError";
import { HttpServer_HttpMethodEnum, HttpServer_Response, HttpServer_Route, HttpServer_WrapperCallback } from "./wrap";
import { buildCleanUriHistory } from "@polywrap/uri-resolvers-js";

export const startHttpServer = async (port: number, requestTimeout: number, routes: HttpServer_Route[], onStart: HttpServer_WrapperCallback | undefined, client: Client): Promise<http.Server> => {
  const app = express();
  app.use(timeout(requestTimeout));
  app.use(express.json());

  app.all('*', handleError(async (req: Request, res: Response, next: NextFunction) => {
    res.header('Access-Control-Allow-Origin', '*');
    res.header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS, PUT, PATCH, DELETE');

    if (req.method !== 'OPTIONS') {
      console.log(`Request:  ${req.method} --- ${req.url}`);
    }
    next();
  }));

  app.use((req: Request, res: Response, next: NextFunction) => {
    res.on('finish', () => {
      console.log(`Response: ${req.method} ${res.statusCode} ${req.url}`);
    });
    next();
  });

  app.use(cors({
    origin: "*",
  }));

  for (const route of routes) {
    const handler = handleError(async (req: Request, res: Response) => {
      const result = await client.invoke<HttpServer_Response>({
        uri: route.handler.uri,
        method: route.handler.method,
        args: {
          request: {
            params: Object.keys(req.params).map(x => ({ key: x, value: req.params[x] })),
            query: Object.keys(req.query).map(x => ({ key: x, value: req.query[x] })),
            body: req.body
              ? JSON.stringify(req.body)
              : undefined,
          }
        }
      });

      if (result.error) {
        throw result.error;
      }

      if (!result.data) {
        throw new Error("No response data from http server wrapper");
      }

      const response = result.data;

      if (response.headers && response.headers.length > 0) {
        const headers: Record<string, string> = {};

        for (const header of response.headers) {
          headers[header.key] = header.value;
        }

        res.writeHead(response.statusCode, headers);
      }

      res.end(response.data);
    });

    switch (route.httpMethod) {
      case HttpServer_HttpMethodEnum.GET:
        app.get(route.path, handler);
        break;
      case HttpServer_HttpMethodEnum.POST:
        app.post(route.path, handler);
        break;
      case HttpServer_HttpMethodEnum.PUT:
        app.put(route.path, handler);
        break;
      case HttpServer_HttpMethodEnum.PATCH:
        app.patch(route.path, handler);
        break;
      case HttpServer_HttpMethodEnum.DELETE:
        app.delete(route.path, handler);
        break;
      case HttpServer_HttpMethodEnum.OPTIONS:
        app.options(route.path, handler);
        break;
    }
  }

  const server = http.createServer({}, app);
  
  return new Promise<http.Server>((resolve) => {
    server.listen(port, async () => {
      if (onStart) {
        await client.invoke({
          uri: onStart.uri,
          method: onStart.method,
          args: {}
        });
      }

      resolve(server);
    });
  });
};
