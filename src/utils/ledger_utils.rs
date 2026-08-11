use crate::cli::colors::*;
use crate::Ledger;
use crate::utils::input;

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