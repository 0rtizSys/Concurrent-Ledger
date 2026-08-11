use crate::Ledger;

use super::colors::{GREEN, RESET, BOLD, YELLOW};
use crate::utils::create_account;

use crate::utils::*;

pub fn start_cli(ledger: &mut Ledger) {  
    loop {
        menu();

        let command: String = input(&format!("{BOLD}> {RESET}"));
        match command.as_str() {
            "1" => {
                clean_terminal();
                show_accounts(&ledger);
                pause();
                clean_terminal();
            }

            "2" => {
                clean_terminal();
                create_account(ledger);
                pause();
                clean_terminal();
            }

            "0" | "exit" | "Exit" | "EXIT" => {
                clean_terminal();
                println!("{GREEN}Goodbye.{RESET}");
                break;
            }

            "" => {
                clean_terminal();
            }

            _ => {
                clean_terminal();
                println!("{YELLOW}Invalid option: \"{}\".{RESET}\n", command);
            }
        }
    }
}
