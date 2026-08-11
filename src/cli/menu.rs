use crate::Ledger;

use super::colors::{GREEN, RESET, BOLD, YELLOW};
use crate::utils::crear_cuenta;

use crate::utils::*;

pub fn start_cli(ledger: &mut Ledger) {  
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
                crear_cuenta(ledger);
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