use crate::cli::colors::*;
use crate::Ledger;
use crate::utils::input;

pub fn crear_cuenta(ledger: &mut Ledger) {
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

pub fn mostrar_cuentas(ledger: &Ledger) {
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