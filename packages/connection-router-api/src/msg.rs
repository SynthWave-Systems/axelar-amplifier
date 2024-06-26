use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::primitives::*;

#[cw_serde]
pub enum ExecuteMsg {
    /*
     * Governance Methods
     * All the below messages should only be called by governance
     */
    // Registers a new chain with the router
    RegisterChain {
        chain: ChainName,
        gateway_address: Address,
    },
    // Changes the gateway address associated with a particular chain
    UpgradeGateway {
        chain: ChainName,
        contract_address: Address,
    },

    /*
     * Router Admin Methods
     * All the below messages should only be called by the router admin
     */
    // Freezes a chain, in the specified direction.
    FreezeChain {
        chain: ChainName,
        direction: GatewayDirection,
    },
    // Unfreezes a chain, in the specified direction.
    UnfreezeChain {
        chain: ChainName,
        direction: GatewayDirection,
    },

    /*
     * Gateway Messages
     * The below messages can only be called by registered gateways
     */
    // Routes a message to all outgoing gateways registered to the destination domain.
    // Called by an incoming gateway
    RouteMessages(Vec<Message>),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChainEndpoint)]
    GetChainInfo(ChainName),

    // Returns a list of chains registered with the router
    // The list is paginated by:
    // - start_after: the chain name to start after, which the next page of results should start.
    // - limit: limit the number of chains returned, default is u32::MAX.
    #[returns(Vec<ChainEndpoint>)]
    Chains {
        start_after: Option<ChainName>,
        limit: Option<u32>,
    },
}
