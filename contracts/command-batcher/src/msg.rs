use std::collections::HashMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint256, Uint64};

use crate::{encoding::Data, types::ProofID};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String,
    pub gateway_address: String,
    pub multisig_address: String,
    pub service_registry_address: String,
    pub destination_chain_id: Uint256,
    pub signing_threshold: (Uint64, Uint64),
    pub service_name: String,
    pub chain_name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Start building a proof that includes specified messages
    // Queries the gateway for actual message contents
    ConstructProof {
        message_ids: Vec<String>,
    },
    RotateSnapshot {
        pub_keys: HashMap<String, HexBinary>,
        key_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetProofResponse)]
    GetProof { proof_id: String },
}

#[cw_serde]
pub enum ProofStatus {
    Pending,
    Completed { execute_data: HexBinary }, // encoded data and proof sent to destination gateway
}

#[cw_serde]
pub struct GetProofResponse {
    pub proof_id: ProofID,
    pub message_ids: Vec<String>,
    pub data: Data,
    pub status: ProofStatus,
}
