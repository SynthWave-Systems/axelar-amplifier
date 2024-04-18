use connection_router_api::ChainName;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use multisig::worker_set::WorkerSet;

#[cw_serde]
pub struct Config {
    pub governance: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

type ProverAddress = Addr;
type ProverAddresses = Vec<ProverAddress>;

pub const PROVERS_PER_CHAIN: Map<ChainName, ProverAddresses> = Map::new("provers_per_chain");

pub const ACTIVE_WORKERSET_FOR_PROVER: Map<ProverAddress, WorkerSet> =
    Map::new("active_prover_workerset");
