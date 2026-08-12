use super::{error::{AccountError, AccountResult}};
use crate::Ledger;

pub fn deposit_account(ledger: &mut Ledger, account_id: u64, amount: u64) -> AccountResult<()> {
    if amount <= 0 {
        return Err(AccountError::ZeroDeposit);
    }

    let account = ledger.mut_find_account(account_id)
    .ok_or(AccountError::NotFound(account_id))?;

    account.deposit(amount);
    Ok(())
}