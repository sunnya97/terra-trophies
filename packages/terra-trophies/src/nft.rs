use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Binary;
use cw721::Expiration;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },

    /// Create a batch of NFTs to a list of owners; can only be called by the contract minter
    CreateBatch {
        /// Identifies the asset to which this NFT represents
        name: String,
        /// Describes the asset to which this NFT represents (may be empty)
        description: String,
        /// A URI pointing to an image representing the asset
        image: String,
    },
    /// Edit info of a batch
    EditBatch {
        /// Identifier of the batch which NFTs is to be edited
        batch_id: u64,
        /// Name of the batch; None if not to change
        name: Option<String>,
        /// Description of the batch; None if not to change
        description: Option<String>,
        /// Image of the batch; None if not to change
        image: Option<String>,
    },
    /// Mint multiple NFTs within a batch
    Mint {
        /// Identifier of the batch of which NFTs are to be minted
        batch_id: u64,
        /// The owners of the newly minted NFTs
        owners: Vec<String>,
    },
}
