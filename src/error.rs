use cosmwasm_std::{OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("invalid claimable Ticket")]
    InvalidClaimableTicket {},

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid length")]
    InvalidLength {},

    #[error("ticket limit exceeded")]
    TicketLimitExceeded {},

    #[error("invalid ticket type")]
    InvalidTicketType {},

    #[error("insufficient amount")]
    InsufficientAmount {},

    #[error("invalid signature")]
    InvalidSignature {},

    #[error("user already blacklisted")]
    UserAlreadyBlackListed {},

    #[error("user not blocked")]
    UserNotBlocked {},

    #[error("user blacklisted")]
    UserBlackListed {},
}
