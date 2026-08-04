mod cuenta;
mod ledger;
mod utils;

use ledger::Ledger;
use utils::clean_terminal;

use std::io::{self, Write};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

fn pause() {
    input(&format!(
        "\n{}Presiona ENTER para continuar...{}",
        DIM, RESET
    ));
}

fn menu() {
    println!("{BOLD}{CYAN}Aurum API{RESET}");
    println!("{DIM}Gestion de cuentas del ledger{RESET}\n");

    println!("  {BOLD}1{RESET}  Ver cuentas");
    println!("  {BOLD}2{RESET}  Crear cuenta");
    println!("  {BOLD}0{RESET}  Salir");

    println!();
}

fn mostrar_cuentas(ledger: &Ledger) {
    println!("{BOLD}{CYAN}Cuentas{RESET}\n");

    if ledger.cuentas.is_empty() {
        println!("{DIM}Todavia no hay cuentas registradas.{RESET}");
        println!("{DIM}Crea una cuenta desde la opcion 2 para empezar.{RESET}");
        return;
    }

    println!("{BOLD}{:<6} {:<24} {:>12}{RESET}", "ID", "Nombre", "Saldo");
    println!("{DIM}{:-<6} {:-<24} {:->12}{RESET}", "", "", "");

    for cuenta in &ledger.cuentas {
        println!(
            "{:<6} {:<24} {:>12}",
            cuenta.id, cuenta.nombre, cuenta.saldo
        );
    }
}

fn crear_cuenta(ledger: &mut Ledger) {
    println!("{BOLD}{CYAN}Nueva cuenta{RESET}\n");

    let nombre = input("Nombre: ");

    if nombre.is_empty() {
        println!("\n{RED}No se creo la cuenta: el nombre no puede estar vacio.{RESET}");
        return;
    }

    ledger.crear_cuenta(&nombre);
    println!(
        "\n{GREEN}Cuenta \"{}\" creada correctamente.{RESET}",
        nombre
    );
}

fn main() {
    let mut ledger: Ledger = Ledger::new();

    clean_terminal();

    loop {
        menu();

        let comando: String = input(&format!("{BOLD}> {RESET}"));
        match comando.as_str() {
            "1" => {
                clean_terminal();
                mostrar_cuentas(&ledger);
                pause();
                clean_terminal();
            }

            "2" => {
                clean_terminal();
                crear_cuenta(&mut ledger);
                pause();
                clean_terminal();
            }

            "0" | "salir" | "Salir" | "SALIR" => {
                clean_terminal();
                println!("{GREEN}Hasta luego.{RESET}");
                break;
            }

            "" => {
                clean_terminal();
            }

            _ => {
                clean_terminal();
                println!("{YELLOW}Opcion invalida: \"{}\".{RESET}\n", comando);
            }
        }
    }
}
