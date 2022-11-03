use std::{collections::HashMap};

use serde::{Serialize, Deserialize};
use itertools::Itertools;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbiItem {
    pub constant: bool,
    pub inputs: Vec<AbiItem>,
    pub name: String,
    pub outputs: Vec<AbiItem>,
    pub payable: bool,
    pub state_mutability: String,
    #[serde(rename = "type")]
    pub type_of: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_of: String,
}

pub type EthereumAbi = Vec<AbiItem>;

pub struct TemplateModel {
    functions: FunctionsModel,
}

pub struct FunctionsModel {
    view: Vec<FunctionModel>,
}

#[derive(Clone)]
pub struct FunctionModel {
    state_mutability: String,
    args: Vec<ArgModel>,
    return_args: Vec<ReturnArgModel>,
}

#[derive(Clone)]
pub struct ArgModel {
    schema_type: String,
    string_parse_schema_type: String,
    input_to_string: String,
    assembly_script_type: String,
}

#[derive(Clone)]
pub struct ReturnArgModel {
    schema_type: String,
    string_parse_schema_type: String,
    assembly_script_type: String,
} 

pub fn solidity_type_to_schema_type(type_name: &String) -> String {
    match type_name.as_str() { 
        "int8" => "Int8".to_string(),
        "int16" => "Int16".to_string(),
        "int32" => "Int32".to_string(),
        "int" => "BigInt".to_string(),
        "bool" => "Boolean".to_string(),
        "bytes" => "Bytes".to_string(),
        "bytes32" => "String".to_string(),
        "string" => "String".to_string(),
        "address" => "String".to_string(),
        "uint8" => "UInt8".to_string(),
        "uint16" => "UInt16".to_string(),
        "uint32" => "UInt32".to_string(),
        "uint64" => "BigInt".to_string(),
        "uint128" => "BigInt".to_string(),
        "uint256" => "BigInt".to_string(),
        _ => "Undefined".to_string(),
    }
}
pub fn schema_type_to_from_string_var(type_name: &String, var_name: &String) -> String {
    match type_name.as_str() { 
        "Int8" => format!("parseInt({}) as i8", var_name),
        "Int16" => format!("parseInt({}) as i16", var_name),
        "Int32" => format!("parseInt({}) as i32", var_name),
        "String" => var_name.clone(),
        "UInt8" => format!("parseInt({}) as u8", var_name),
        "UInt16" => format!("parseInt({}) as u16", var_name),
        "UInt32" => format!("parseInt({}) as u32", var_name),
        "BigInt" => format!("BigInt.fromString({})", var_name),
        _ => "Undefined".to_string(),
    }
}

pub fn schema_type_to_from_var(type_name: &String, name: &String, var_name: &String) -> String {
    match type_name.as_str() { 
        "Int8" => format!("{}.{}.toString()", var_name, name),
        "Int16" => format!("{}.{}.toString()", var_name, name),
        "Int32" => format!("{}.{}.toString()", var_name, name),
        "String" => format!("{}.{}", var_name, name),
        "UInt8" => format!("{}.{}.toString()", var_name, name),
        "UInt16" => format!("{}.{}.toString()", var_name, name),
        "UInt32" => format!("{}.{}.toString()", var_name, name),
        "BigInt" => format!("{}.{}.toString()", var_name, name),
        _ => "Undefined".to_string(),
    }
}

pub fn schema_type_to_assembly_script_type(type_name: &String) -> String {
    match type_name.as_str() { 
        "Int8" => "i8".to_string(),
        "Int16" => "i16".to_string(),
        "Int32" => "i32".to_string(),
        "Boolean" => "bool".to_string(),
        "Bytes" => "Bytes".to_string(),
        "String" => "string".to_string(),
        "UInt8" => "u8".to_string(),
        "UInt16" => "u16".to_string(),
        "UInt32" => "u32".to_string(),
        "BigInt" => "BigInt".to_string(),
        _ => "Undefined".to_string(),
    }
}

pub fn transform(model: EthereumAbi) -> TemplateModel {
    let item_type_groups: HashMap<String, Vec<AbiItem>> = model
        .into_iter()
        .map(|x| (x.type_of.clone(), x))
        .into_iter()
        .into_group_map();

    let model_functions: FunctionsModel;

    if let Some(functions) = item_type_groups.get("function") {
        let all_functions: Vec<FunctionModel> = functions
            .into_iter()
            .map(|x| {
                FunctionModel {
                    state_mutability: x.state_mutability.clone(),
                    args: x.inputs
                        .iter()
                        .map(|input| {
                            let schema_type = solidity_type_to_schema_type(&input.type_of);

                            ArgModel {
                                schema_type: schema_type.clone(),
                                string_parse_schema_type: schema_type_to_from_string_var(&schema_type, &"result".to_string()),
                                input_to_string: schema_type_to_from_var(&schema_type, &input.name, &"input".to_string()),
                                assembly_script_type: schema_type_to_assembly_script_type(&schema_type),
                            }
                        })
                        .collect(),
                    return_args: x.outputs
                        .iter()
                        .map(|input| {
                            let schema_type = solidity_type_to_schema_type(&input.type_of);

                            ReturnArgModel {
                                schema_type: schema_type.clone(),
                                string_parse_schema_type: schema_type_to_from_string_var(&schema_type, &"result".to_string()),
                                assembly_script_type: schema_type_to_assembly_script_type(&schema_type),
                            }
                        })
                        .collect(),
                }
                })
            .collect();

        let functions_by_state_mut: HashMap<String, Vec<FunctionModel>> = all_functions.into_iter()
            .map(|x| (x.state_mutability.clone(), x))
            .into_iter()
            .into_group_map();

        model_functions = FunctionsModel {
            view: if let Some(view_functions) = functions_by_state_mut.get("view") {
                view_functions.to_vec()
            } else {
                vec![]
            },
        };
    } else {
        model_functions = FunctionsModel {
            view: vec![],
        };
    }

    TemplateModel {
        functions: model_functions,
    }
  // if(func) {
  //   func._arr.forEach((x: JSON.Value) => {
  //     const inputs = (x as JSON.Obj).getArr("inputs");

  //     if(!inputs) {
  //       return;
  //     }
  //     inputs._arr
  //       .map<JSON.Obj>((x: JSON.Value) => {
  //         return <JSON.Obj>x;
  //       })
  //       .forEach((input: JSON.Obj) => {
  //         const type = strOrEmpty(input, "type");
  //         input.set("schemaType", solidityTypeToSchemaType(type));
  //         const schemaType = strOrEmpty(input, "schemaType");
  //         const name = strOrEmpty(input, "name");

  //         input.set("stringParseSchemaType", schemaTypeToFromStringVar(schemaType, "result"));
  //         input.set("inputToString", schemaTypeToFromVar(schemaType, name, "input"));
  //         input.set("assemblyScriptType", schemaTypeToAssemblyScriptType(schemaType));
  //       });

  //     const outputs = (x as JSON.Obj).getArr("outputs");

  //       if(!outputs) {
  //         return;
  //       }
  //     outputs._arr
  //       .map<JSON.Obj>((x: JSON.Value) => {
  //         return <JSON.Obj>x;
  //       })
  //       .forEach((input: JSON.Obj) => {
  //         const type = strOrEmpty(input, "type");
  //         input.set("schemaType", solidityTypeToSchemaType(type));

  //         const schemaType = strOrEmpty(input, "schemaType");

  //         input.set("stringParseSchemaType", schemaTypeToFromStringVar(schemaType, "result"));
  //         input.set("assemblyScriptType", schemaTypeToAssemblyScriptType(schemaType));
  //       });
  //   });
  //   grouped.set("function", groupByStateMut(func._arr.map<JSON.Obj>((x: JSON.Value) => x as JSON.Obj)));

  //   addFirstLast(grouped);
  // }

  // return grouped;
}
