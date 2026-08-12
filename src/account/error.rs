use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AccountError {
    #[error("Account with ID {0} not found")]
    NotFound(u64),

    #[error("Deposit amount must be greater than zero")]
    ZeroDeposit,

    #[error("Withdraw amount must be greater than zero")]
    ZeroWithdraw,

    #[error("Insufficient balance: {balance} < {amount}")]
    InsufficientBalance { balance: u64, amount: u64 },
}

pub type AccountResult<T> = Result<T, AccountError>;