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
  "moduleType": {
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
