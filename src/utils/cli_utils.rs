use crate::cli::colors::*;
use std::process::Command;
use std::io::{self, Write};

pub fn pause() {
    input(&format!(
        "\n{}Presiona ENTER para continuar...{}",
        DIM, RESET
    ));
}

pub fn menu() {
    println!("{BOLD}{CYAN}Aurum API{RESET}");
    println!("{DIM}Gestion de cuentas del ledger{RESET}\n");

    println!("  {BOLD}1{RESET}  Ver cuentas");
    println!("  {BOLD}2{RESET}  Crear cuenta");
    println!("  {BOLD}0{RESET}  Salir");

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