use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr,
    BankQuery::{self, Balance},
    StdResult, Uint128,
};
use cw_storage_plus::{Item, Map};

// #[cw_serde]
// pub struct OwnerCap {
//     pub id: u64,

// }

#[cw_serde]
pub struct TicketNFT {
    pub id: u64,
    pub ticket_type: String,
    pub owner: Addr,
    pub description: String,
    pub ticket_id: u64,
    pub ticket_claimed: bool,
}
#[cw_serde]
pub struct Tickets {
    pub tickets: Vec<TicketNFT>,
}
#[cw_serde]
pub struct UserTicketInfo {
    pub ticket_owner: Addr,
    pub ticket_id: u64,
    pub ticket_type: String,
    pub amount: Uint128,
}

#[cw_serde]
pub enum TicketStatus {
    InProgress,
    Assigned,
    Revoked,
}

#[cw_serde]
pub struct BmtPlatformDetails {
    pub id: u64,
    pub owner: Addr,
    pub sig_verify_pk: String,
    pub platform_fee: u64,
    pub profit: Uint128,
    pub user_tickets: Vec<UserTicketInfo>,
    pub current_ticket_index: u64,
    pub claim_nonce: u64,
    pub max_ticket_per_person: u64,
}

pub const BMT_PLATFORM_DETAILS: Item<BmtPlatformDetails> = Item::new("bmt_platform_info");
pub const USER_INFO_MAP: Map<Addr, UserTicketInfo> = Map::new("user_ticket_info");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const BLOCKED_USERS: Map<Addr, bool> = Map::new("blocked_users");
pub const TICKET_NFT: Item<Tickets> = Item::new("ticket_nft");

pub type UserTickets<'a> = Map<'a, Addr, Vec<UserTicketInfo>>;
pub type TicketTypes<'a> = Map<'a, String, u64>;
pub type UserBlackList<'a> = Map<'a, Addr, bool>;
