use crate::cli::colors::*;
use std::process::Command;
use std::io::{self, Write};

pub fn pause() {
    input(&format!(
        "\n{}Press ENTER to continue...{}",
        DIM, RESET
    ));
}

pub fn menu() {
    println!("{BOLD}{CYAN}Aurum API{RESET}");
    println!("{DIM}Ledger account management{RESET}\n");

    println!("  {BOLD}1{RESET}  View accounts");
    println!("  {BOLD}2{RESET}  Create account");
    println!("  {BOLD}3{RESET}  Deposit to an account");
    println!("  {BOLD}0{RESET}  Exit");

    println!();
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

pub fn clean_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}