use std::num::FpCategory::Nan;
use std::time::Duration;

use crate::account::error::AccountError;
use crate::cli::colors::*;

use crate::{Ledger, account};
use crate::utils::{clean_terminal, input, pause};

use crate::account::{Account, deposit_account};

pub fn create_account(ledger: &mut Ledger) {
    println!("{BOLD}{CYAN}New Account{RESET}\n");

    let name = input("Name: ");

    if name.is_empty() {
        println!("\n{RED}Account not created: name cannot be empty.{RESET}");
        return;
    }

    ledger.create_account(&name);
    println!(
        "\n{GREEN}Account \"{}\" created successfully.{RESET}",
        name
    );
}

pub fn show_accounts(ledger: &Ledger) {
    println!("{BOLD}{CYAN}Accounts{RESET}\n");

    if ledger.accounts.is_empty() {
        println!("{DIM}💡 No accounts yet. Create one to get started!{RESET}");
        return;
    }

    println!("{BOLD}{:<6} {:<24} {:>12}{RESET}", "ID", "Name", "Balance");
    println!("{DIM}{:-<6} {:-<24} {:->12}{RESET}", "", "", "");

    for acc in &ledger.accounts {
        println!(
            "{:<6} {:<24} {:>12}",
            acc.id, acc.name, acc.balance
        );
    }
}

pub fn ledger_deposit_account(ledger: &mut Ledger) {
    println!("{BOLD}{CYAN}Deposit{RESET}\n");
    let account_id_str = input("Target accound ID: ").to_string();
    let account_id: u64 = account_id_str.parse().unwrap();

    let amount_str = input("Amount: ").to_string();
    let amount: u64 = amount_str.parse().unwrap();

    println!("{DIM}{YELLOW}Procesing...{RESET}");
    std::thread::sleep(Duration::from_secs(1));
    clean_terminal();
    println!("{BOLD}{CYAN}Deposit{RESET}\n");

    match deposit_account(ledger, account_id, amount) {
        Ok(()) => {
            println!("{DIM}{GREEN}\nSuccess...\n{RESET}");
            return;        }
        Err(AccountError::NotFound(account_id)) => {
            print!("{DIM}{RED}\nAccount with ID {account_id} doesn't exist, please try again...\n{RESET}");
        }
        Err(AccountError::ZeroDeposit) => {
            print!("{DIM}{RED}\nThe amount must be higher than 0...\n{RESET}");
        }
        Err(e) => {
            println!("{DIM}{RED}Error inesperado...\nError: {e}{RESET}");
        }
    }
}