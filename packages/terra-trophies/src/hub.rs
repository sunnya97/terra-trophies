// use cosmwasm_std::{Addr, Api, StdResult};
// use cw721::Expiration;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// use crate::metadata::Metadata;

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// pub struct InstantiateMsg {
//     /// Code ID of the `trophy-nft` contract
//     pub nft_code_id: u64,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     /// Create a new trophy with the specified metadata
//     CreateTrophy {
//         /// Metadata of this trophy
//         metadata: Metadata,
//         /// The deadline before which instances of this trophy can be minted
//         expiry: Option<Expiration>,
//         /// The maximum number of trophy instances can be minted
//         max_supply: Option<u64>,
//     },
//     /// Update metadata an existing trophy. Only the creator of the collection can call
//     EditTrophy {
//         /// Identifier of the trophy
//         trophy_id: u64,
//         /// The new metadata for the trophy
//         metadata: Metadata,
//     },
//     /// Mint new instances of a specified trophy to a list of addresses. Called only if the trophy's
//     /// minting rule is set to `ByOwner` and if caller if owner
//     MintByMinter {
//         /// Idnetifier of the trophy
//         trophy_id: u64,
//         /// A list of owners to receive instances of the trophy
//         owners: Vec<String>,
//     },
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     /// Total number of existing trophies. Returns `ContractInfoResponse`
//     ContractInfo {},
//     /// Info about a trophy. Returns `TrophyInfo<String>`
//     TrophyInfo {
//         trophy_id: u64,
//     },
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// pub struct ContractInfoResponse {
//     pub nft: String,
//     pub trophy_count: u64,
// }

// pub mod helpers {
//     use super::{Metadata, QueryMsg, TrophyInfo};
//     use cosmwasm_std::{to_binary, Addr, QuerierWrapper, QueryRequest, StdResult, WasmQuery};

//     pub fn query_trophy_metadata(
//         querier: &QuerierWrapper,
//         hub_addr: &Addr,
//         trophy_id: u64,
//     ) -> StdResult<Metadata> {
//         let trophy_info: TrophyInfo<String> =
//             querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//                 contract_addr: hub_addr.to_string(),
//                 msg: to_binary(&QueryMsg::TrophyInfo {
//                     trophy_id,
//                 })?,
//             }))?;
//         Ok(trophy_info.metadata)
//     }
// }
