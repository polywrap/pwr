use std::collections::HashSet;

use polywrap_client::core::uri::Uri;
use wrap_manifest_schemas::deserialize::{deserialize_wrap_manifest, DeserializeManifestOptions};

use crate::client::CoreClient;
use crate::prompter::Prompter;
use crate::logger::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] 
struct AppArgs {
    args: Vec<String>,
}

pub struct AppManager;
impl AppManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_app(
        &self,
        uri: &Uri, 
        args: &[String], 
        all_access_controlled_uris: Vec<String>,
        client: &dyn CoreClient, 
        prompter: &dyn Prompter, 
        logger: &dyn Logger,
    ) -> i32 {
        let access_controlled_uris: Vec<String> = vec![];
        let mut visited_uris: HashSet<String> = HashSet::new();
        
        // extract_access_controlled_uris(
        //     uri, 
        //     &all_access_controlled_uris, 
        //     &access_controlled_uris, 
        //     &mut visited_uris,
        //     client, 
        //     logger,
        // );
    
        if access_controlled_uris.len() > 0 {
            let response = prompter.confirm(
                format!("App requested access to: \n{}. \nDo you want to grant access?", access_controlled_uris.join("\n"))
            );

            let response = match response {
                Ok(response) => response,
                Err(promptError) => {
                    logger.error(format!("{}", promptError));
                    return 1;
                }
            };
    
            if !response {
                logger.error(format!("Denied access for {}", uri.to_string()));
                return 1;
            }
        }
    
        let serialization_result = polywrap_msgpack::serialize(&AppArgs {
            args: args.to_vec(),
        });
        let args = match serialization_result {
            Ok(args) => args,
            Err(serializeError) => {
                logger.error(format!("{}", serializeError));
                return 1;
            }
        };

        println!("args: {:?}", args);

        let result = invoke_with_access_control(
            uri,
            "main",
            Some(&args),
            client,
            access_controlled_uris.clone(), 
            access_controlled_uris, 
        );
    
        match result {
            Ok(data) => {
                let exit_code = polywrap_msgpack::decode::<i32>(&data);

                match exit_code {
                    Ok(exit_code) => {
                        return exit_code;
                    },
                    Err(decodeError) => {
                        logger.error(format!("{:?}", decodeError));
                        return 1;
                    }
                }
            },
            Err(invokeError) => {
                logger.error(format!("{}", invokeError));
                return 1;
            }
        }
    }
}

fn extract_access_controlled_uris(
    uri: &Uri, 
    _all_access_controlled_uris: &[String],
    _acess_controlled_uris: &[String],
    visited_uris: &mut HashSet<String>,
    client: &dyn CoreClient,
    logger: &dyn Logger,
) {
    if visited_uris.contains(&uri.to_string()) {
        return;
    }
    visited_uris.insert(uri.to_string());

    let serialized_manifest = client.get_manifest(uri);
    let serialized_manifest = match serialized_manifest {
        Ok(serialized_manifest) => serialized_manifest,
        Err(getManifestError) => {
            logger.error(format!("{}", getManifestError));
            return;
        }
    };

    let manifest = deserialize_wrap_manifest(&serialized_manifest, Some(DeserializeManifestOptions {
        no_validate: false,
        ext_schema: None
    }));

    let manifest = match manifest {
        Ok(manifest) => manifest,
        Err(deserializeError) => {
            logger.error(format!("{}", deserializeError));
            return;
        }
    };

    let imported_module_types = if manifest.abi.imported_module_types != None {
        manifest.abi.imported_module_types.unwrap() 
    } else {
        [].to_vec()
    };

    // TODO: figure out how to read the manifest and get the imported uris
    // let imported_uris = imported_module_types.map(|imported_module_type| Uri::try_from(imported_module_type.uri).unwrap().to_string());

    // let requested_uris = imported_uris.filter(|imported_uri| all_access_controlled_uris.contains(imported_uri));
    // let other_uris = imported_uris.filter(|imported_uri| !all_access_controlled_uris.contains(imported_uri));
    // if requested_uris.len() > 0 {
    //     acess_controlled_uris.extend_from_slice(requested_uris);
    // }

    // for other_uri in other_uris {
    //     extract_access_controlled_uris(other_uri, client, all_access_controlled_uris, acess_controlled_uris, visited_uris);
    // }
}

fn invoke_with_access_control(
    uri: &Uri,
    method: &str,
    args: Option<&[u8]>,
    client: &dyn CoreClient,
    _allowed_uris: Vec<String>,
    _all_access_controlled_uris: Vec<String>,
) -> Result<Vec<u8>, String> {
    match client.invoke_raw(
        uri, 
        method, 
        args, 
        None,
        None
    ) {
        Ok(data) => {
            return Ok(data);
        },
        Err(invoke_error) => {
            return Err(format!("{}", invoke_error));
        }   
    }
}
