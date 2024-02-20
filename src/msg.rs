use core::str;

use cosmwasm_schema::{cw_serde, schemars, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{TicketNFT, UserTicketInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub sig_verify_pk: String,
    pub platform_fee: u64,
    pub max_ticket_per_person: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub enum ExecuteMsg {
    BuyTicket {
        ticket_id: u64,
        amount: Vec<Coin>,
        ticket_type: String,
    },
    ClaimTicket {
        ticket_owner: String,
        ticket_id: u64,
        ticket_type: String,
    },
    SetVerifyPkMsg {
        verify_pk_str: String,
    },
    BlockUserMsg {
        user_addr: Addr,
        description: String,
    },
    UnblockUserMsg {
        user_addr: Addr,
    },
    ChangeOwnerMsg {
        new_owner: Addr,
        ticket_id: u64,
        ticket_type: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TicketInfoResponse)]
    GetTicketInfo { ticket_id: u64 },

    #[returns(UserTicketsResponse)]
    GetUserTickets { user: Addr },
}

// #[cw_serde]
// pub enum TicketPurchaseResponse {
//     TicketPurchase {
//     id: u64,
//     ticket_type: String,
//     description: String,
//     ticket_id: u64,
//     ticket_claimed: bool,
//     }
// }

#[cw_serde]
pub struct TicketInfoResponse {
    pub ticket: TicketNFT,
}
#[cw_serde]
pub struct UserTicketsResponse {
    pub tickets: Vec<UserTicketInfo>,
}
#[cw_serde]
pub struct BlockUserResponse {
    pub blocked: bool,
    pub user_addr: Addr,
    pub description: String,
}
#[cw_serde]
pub struct ClaimTicketResponse {
    pub claim_status: bool,
}
