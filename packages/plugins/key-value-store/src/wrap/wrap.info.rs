/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "key-value-store".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "importedModuleTypes": [
    {
      "isInterface": false,
      "kind": 256,
      "methods": [
        {
          "kind": 64,
          "name": "getOwnContext",
          "required": true,
          "return": {
            "kind": 34,
            "name": "getOwnContext",
            "object": {
              "kind": 8192,
              "name": "getOwnContext",
              "required": true,
              "type": "InvocationContext_ResolutionContext"
            },
            "required": true,
            "type": "InvocationContext_ResolutionContext"
          },
          "type": "Method"
        },
        {
          "kind": 64,
          "name": "getCallerContext",
          "required": true,
          "return": {
            "kind": 34,
            "name": "getCallerContext",
            "object": {
              "kind": 8192,
              "name": "getCallerContext",
              "type": "InvocationContext_ResolutionContext"
            },
            "type": "InvocationContext_ResolutionContext"
          },
          "type": "Method"
        }
      ],
      "namespace": "InvocationContext",
      "nativeType": "Module",
      "type": "InvocationContext_Module",
      "uri": "https/http.wrappers.dev/u/test/invocation-context"
    }
  ],
  "importedObjectTypes": [
    {
      "kind": 1025,
      "namespace": "InvocationContext",
      "nativeType": "ResolutionContext",
      "properties": [
        {
          "kind": 34,
          "name": "originUri",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "originUri",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "finalUri",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "finalUri",
            "required": true,
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "InvocationContext_ResolutionContext",
      "uri": "https/http.wrappers.dev/u/test/invocation-context"
    }
  ],
  "moduleType": {
    "imports": [
      {
        "type": "InvocationContext_ResolutionContext"
      },
      {
        "type": "InvocationContext_Module"
      }
    ],
    "kind": 128,
    "methods": [
      {
        "arguments": [
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
              "type": "Bytes"
            },
            "type": "Bytes"
          }
        ],
        "kind": 64,
        "name": "set",
        "required": true,
        "return": {
          "kind": 34,
          "name": "set",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "set",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
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
          }
        ],
        "kind": 64,
        "name": "get",
        "required": true,
        "return": {
          "kind": 34,
          "name": "get",
          "scalar": {
            "kind": 4,
            "name": "get",
            "type": "Bytes"
          },
          "type": "Bytes"
        },
        "type": "Method"
      },
      {
        "arguments": [
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
          }
        ],
        "kind": 64,
        "name": "remove",
        "required": true,
        "return": {
          "kind": 34,
          "name": "remove",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "remove",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
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
          }
        ],
        "kind": 64,
        "name": "has",
        "required": true,
        "return": {
          "kind": 34,
          "name": "has",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "has",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "kind": 64,
        "name": "keys",
        "required": true,
        "return": {
          "array": {
            "item": {
              "kind": 4,
              "name": "keys",
              "required": true,
              "type": "String"
            },
            "kind": 18,
            "name": "keys",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "keys",
              "required": true,
              "type": "String"
            },
            "type": "[String]"
          },
          "kind": 34,
          "name": "keys",
          "required": true,
          "type": "[String]"
        },
        "type": "Method"
      },
      {
        "kind": 64,
        "name": "values",
        "required": true,
        "return": {
          "array": {
            "item": {
              "kind": 4,
              "name": "values",
              "required": true,
              "type": "Bytes"
            },
            "kind": 18,
            "name": "values",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "values",
              "required": true,
              "type": "Bytes"
            },
            "type": "[Bytes]"
          },
          "kind": 34,
          "name": "values",
          "required": true,
          "type": "[Bytes]"
        },
        "type": "Method"
      },
      {
        "kind": 64,
        "name": "entries",
        "required": true,
        "return": {
          "array": {
            "item": {
              "kind": 8192,
              "name": "entries",
              "required": true,
              "type": "KeyValuePair"
            },
            "kind": 18,
            "name": "entries",
            "object": {
              "kind": 8192,
              "name": "entries",
              "required": true,
              "type": "KeyValuePair"
            },
            "required": true,
            "type": "[KeyValuePair]"
          },
          "kind": 34,
          "name": "entries",
          "required": true,
          "type": "[KeyValuePair]"
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
            "type": "Bytes"
          },
          "type": "Bytes"
        }
      ],
      "type": "KeyValuePair"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
