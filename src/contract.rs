use std::{ops::Add, process::Command};

use crate::state::{
    BmtPlatformDetails, OwnerCap, TicketNFT, TicketStatus, TicketTypes, UserBlackList, UserTickets,
    BMT_PLATFORM_DETAILS, OWNER_CAP, USER_INFO_MAP,
};
use crate::{
    error::ContractError,
    msg::{
        ClaimTicketResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TicketInfoResponse,
        UserTicketsResponse,
    },
    state::{UserTicketInfo, BLOCKED_USERS, TICKET_NFT},
};
use cosmwasm_std::{
    attr, coin, entry_point, to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, Deps, DepsMut,
    Env, MessageInfo, Response, StdError, StdResult, Storage, SubMsg, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let platform = BmtPlatformDetails {
        id: 1,
        owner: msg.owner,
        sig_verify_pk: msg.sig_verify_pk,
        platform_fee: msg.platform_fee,
        profit: Uint128::zero(),
        user_tickets: Vec::new(),
        current_ticket_index: 0,
        claim_nonce: 0,
        max_ticket_per_person: msg.max_ticket_per_person,
    };
    // Save the state in the storage
    BMT_PLATFORM_DETAILS.save(deps.storage, &platform)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        ExecuteMsg::BuyTicket {
            ticket_id,
            amount,
            ticket_type,
        } => buy_ticket(deps, env, info, ticket_id, amount, ticket_type),

        ExecuteMsg::ClaimTicket {
            ticket_owner,
            ticket_type,
            ticket_id,
        } => claim_ticket(deps, env, info, ticket_id, ticket_type),

        ExecuteMsg::BlockUserMsg {
            user_addr,
            description,
        } => block_user(deps, env, info, user_addr, description),

        ExecuteMsg::SetVerifyPkMsg { verify_pk_str } => {
            set_verify_pk(deps, env, info, verify_pk_str)
        }

        ExecuteMsg::UnblockUserMsg { user_addr } => unblock_user(deps, env, info, user_addr),

        ExecuteMsg::ChangeOwnerMsg {
            new_owner,
            ticket_id,
            ticket_type,
        } => change_owner_msg(deps, env, info, new_owner, ticket_id, ticket_type),
    }
}
fn check_ticket_availability(
    deps: DepsMut,
    ticket_id: u64,
    ticket_type: &str,
) -> Result<bool, ContractError> {
    // Example logic for checking ticket availability
    // This could involve loading ticket data from storage and checking its status
    let tickets = BMT_PLATFORM_DETAILS.load(deps.storage)?;
    // For demonstration, let's assume all tickets are available
    Ok(true)
}

fn buy_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ticket_id: u64,
    amount: Vec<Coin>,
    ticket_type: String,
) -> Result<Response, ContractError> {
    // Simplified logic to check if the ticket can be purchased
    // This would include checking ticket availability, matching ticket type, and user eligibility
    let mut platform = BMT_PLATFORM_DETAILS.load(deps.storage)?;

    // Assuming a function checks if the ticket is available for purchase
    let ticket_available = check_ticket_availability(deps, ticket_id, &ticket_type)?;
    if !ticket_available {
        return Err(ContractError::InvalidTicketType {});
    }

    // Assuming a function to check if user has not exceeded the ticket limit
    let user_tickets = USER_INFO_MAP.load(deps.storage, info.sender).unwrap();
    if user_tickets.ticket_type.len() + amount.len()
        > platform.max_ticket_per_person.try_into().unwrap()
    {
        return Err(ContractError::TicketLimitExceeded {});
    }

    // Add the message sender (buyer) to the list of owners with their respective number of tokens
    let mut owner_map = OWNER_CAP.load(deps.storage)?;
    // let existing_owner = owner_map.(&info.sender).cloned();
    // let new_owner = existing_owner.add_amount(&info.sender, amount.clone());
    // owner_map.insert(&info.sender, new_owner);
    OWNER_CAP.save(deps.storage, &owner_map)?;

    // Save updated user information
    let coins: Vec<Coin> = amount.clone();
    let total_amount: Uint128 = coins.iter().map(|coin| coin.amount).sum();

    let user_info = UserTicketInfo {
        ticket_owner: info.sender,
        ticket_id: ticket_id,
        ticket_type: ticket_type,
        amount: total_amount,
    };
    USER_INFO_MAP.save(deps.storage, info.sender, &user_info)?;

    if user_tickets.ticket_type.len() as u64 >= platform.max_ticket_per_person {
        return Err(ContractError::TicketLimitExceeded {});
    }

    let ticket_cost = Uint128::new(10);
    let total_payment: Uint128 = amount.iter().map(|coin| coin.amount).sum();

    if total_payment < ticket_cost {
        return Err(ContractError::InsufficientAmount {});
    }

    platform.profit += total_payment;
    BMT_PLATFORM_DETAILS.save(deps.storage, &platform)?;

    Ok(Response::new()
        .add_attribute("method", "buy_ticket")
        .add_attribute("ticket_id", ticket_id.to_string())
        .add_attribute("ticket_type", ticket_type.to_string())
        .add_attribute("amount", total_payment.to_string()))
}

fn claim_ticket(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    ticket_id: u64,
    ticket_type: String,
) -> Result<Response, ContractError> {
    let mut ticket = TICKET_NFT.load(deps.storage)?;

    if ticket.ticket_id == ticket_id && !ticket.ticket_claimed {
        ticket.ticket_claimed = true;
        TICKET_NFT.save(deps.storage, &ticket)?;
    }

    if ticket.ticket_type != ticket_type {
        return Err(ContractError::InvalidTicketType {});
    }

    Ok(Response::new()
        .add_attribute("method", "claim_ticket")
        .add_attribute("ticket_id", ticket_id.to_string())
        .add_attribute("ticket_type", ticket_type))
}

fn block_user(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_addr: Addr,
    description: String,
) -> Result<Response, ContractError> {
    let platform = BMT_PLATFORM_DETAILS.load(deps.storage)?;
    if info.sender != platform.owner {
        return Err(ContractError::Unauthorized {});
    }

    let is_already_blocked = BLOCKED_USERS
        .load(deps.storage, user_addr)
        .unwrap_or_default();
    if is_already_blocked {
        return Err(ContractError::UserAlreadyBlackListed {});
    }

    BLOCKED_USERS.save(deps.storage, user_addr, &true)?;

    Ok(Response::new()
        .add_attribute("method", "block_user")
        .add_attribute("user", user_addr)
        .add_attribute("status", "blocked"))
}

fn set_verify_pk(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verify_pk_str: String,
) -> Result<Response, ContractError> {
    // Only allow the contract owner to set the verification public key
    let owner = OWNER_CAP.load(deps.storage)?;
    if info.sender != msg.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Logic to update the verification public key
    let mut platform = BMT_PLATFORM_DETAILS.load(deps.storage)?;
    platform.sig_verify_pk = verify_pk_str;
    BMT_PLATFORM_DETAILS.save(deps.storage, &platform)?;

    Ok(Response::new().add_attribute("method", "set_verify_pk"))
}

fn unblock_user(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_addr: Addr,
) -> Result<Response, ContractError> {
    let owner = OWNER_CAP.load(deps.storage)?;
    // if OwnerCap::(&info,.sender, &owner) {
    //     return Err(ContractError::Unauthorized {  });
    // }

    let is_blocked = BLOCKED_USERS
        .may_load(deps.storage, user_addr.clone())?
        .unwrap();

    if !is_blocked {
        return Err(ContractError::UserNotBlocked {});
    }

    BLOCKED_USERS.remove(deps.storage, user_addr.clone());

    Ok(Response::new()
        .add_attribute("method", "Unblock_user")
        .add_attribute("user", user_addr))
}

fn change_owner_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: Addr,
    ticket_id: u64,
    ticket_type: String,
) -> Result<Response, ContractError> {
    let owner = OWNER_CAP.load(deps.storage)?;
}
