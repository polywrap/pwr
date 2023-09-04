/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "http-server".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "enumTypes": [
    {
      "constants": [
        "GET",
        "POST",
        "PUT",
        "PATCH",
        "DELETE",
        "OPTIONS"
      ],
      "kind": 8,
      "type": "HttpMethod"
    }
  ],
  "moduleType": {
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "port",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "port",
              "required": true,
              "type": "UInt16"
            },
            "type": "UInt16"
          },
          {
            "kind": 34,
            "name": "requestTimeout",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "requestTimeout",
              "required": true,
              "type": "UInt32"
            },
            "type": "UInt32"
          },
          {
            "array": {
              "item": {
                "kind": 8192,
                "name": "routes",
                "required": true,
                "type": "Route"
              },
              "kind": 18,
              "name": "routes",
              "object": {
                "kind": 8192,
                "name": "routes",
                "required": true,
                "type": "Route"
              },
              "required": true,
              "type": "[Route]"
            },
            "kind": 34,
            "name": "routes",
            "required": true,
            "type": "[Route]"
          },
          {
            "kind": 34,
            "name": "onStart",
            "object": {
              "kind": 8192,
              "name": "onStart",
              "type": "WrapperCallback"
            },
            "type": "WrapperCallback"
          }
        ],
        "kind": 64,
        "name": "start",
        "required": true,
        "return": {
          "kind": 34,
          "name": "start",
          "object": {
            "kind": 8192,
            "name": "start",
            "required": true,
            "type": "StartResult"
          },
          "required": true,
          "type": "StartResult"
        },
        "type": "Method"
      }
    ],
    "type": "Module"
  },
  "objectTypes": [
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "uri",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "uri",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "method",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "method",
            "required": true,
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "WrapperCallback"
    },
    {
      "kind": 1,
      "properties": [
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "headers",
              "required": true,
              "type": "KeyValuePair"
            },
            "kind": 18,
            "name": "headers",
            "object": {
              "kind": 8192,
              "name": "headers",
              "required": true,
              "type": "KeyValuePair"
            },
            "required": true,
            "type": "[KeyValuePair]"
          },
          "kind": 34,
          "name": "headers",
          "required": true,
          "type": "[KeyValuePair]"
        },
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "params",
              "required": true,
              "type": "KeyValuePair"
            },
            "kind": 18,
            "name": "params",
            "object": {
              "kind": 8192,
              "name": "params",
              "required": true,
              "type": "KeyValuePair"
            },
            "required": true,
            "type": "[KeyValuePair]"
          },
          "kind": 34,
          "name": "params",
          "required": true,
          "type": "[KeyValuePair]"
        },
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "query",
              "required": true,
              "type": "KeyValuePair"
            },
            "kind": 18,
            "name": "query",
            "object": {
              "kind": 8192,
              "name": "query",
              "required": true,
              "type": "KeyValuePair"
            },
            "required": true,
            "type": "[KeyValuePair]"
          },
          "kind": 34,
          "name": "query",
          "required": true,
          "type": "[KeyValuePair]"
        },
        {
          "kind": 34,
          "name": "body",
          "scalar": {
            "kind": 4,
            "name": "body",
            "type": "Bytes"
          },
          "type": "Bytes"
        }
      ],
      "type": "Request"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "key",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "key",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "value",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "value",
            "required": true,
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "KeyValuePair"
    },
    {
      "kind": 1,
      "properties": [
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "headers",
              "required": true,
              "type": "KeyValuePair"
            },
            "kind": 18,
            "name": "headers",
            "object": {
              "kind": 8192,
              "name": "headers",
              "required": true,
              "type": "KeyValuePair"
            },
            "type": "[KeyValuePair]"
          },
          "kind": 34,
          "name": "headers",
          "type": "[KeyValuePair]"
        },
        {
          "kind": 34,
          "name": "body",
          "scalar": {
            "kind": 4,
            "name": "body",
            "type": "Bytes"
          },
          "type": "Bytes"
        },
        {
          "kind": 34,
          "name": "statusCode",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "statusCode",
            "required": true,
            "type": "UInt16"
          },
          "type": "UInt16"
        }
      ],
      "type": "Response"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "path",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "path",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "enum": {
            "kind": 16384,
            "name": "httpMethod",
            "required": true,
            "type": "HttpMethod"
          },
          "kind": 34,
          "name": "httpMethod",
          "required": true,
          "type": "HttpMethod"
        },
        {
          "kind": 34,
          "name": "handler",
          "object": {
            "kind": 8192,
            "name": "handler",
            "required": true,
            "type": "WrapperCallback"
          },
          "required": true,
          "type": "WrapperCallback"
        }
      ],
      "type": "Route"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "ok",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "ok",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        }
      ],
      "type": "StartResult"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
