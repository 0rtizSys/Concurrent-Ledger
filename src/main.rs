#[derive(Debug)]
struct Cuenta {
    id: u64,
    nombre: String,
    saldo: u64,
}

impl Cuenta {
    fn depositar(&mut self, monto: u64) {
        self.saldo += monto;
    }
    fn retirar(&mut self, monto: u64) -> bool {
        if self.saldo >= monto {
            self.saldo -= monto;
            true
        } else {
            false
        }
    }
    fn saldo(&self) -> u64 {
        self.saldo
    }
}

struct Ledger {
    cuentas: Vec<Cuenta>
}

impl Ledger {
    fn agregar_cuenta(&mut self, cuenta: Cuenta) {
        self.cuentas.push(cuenta);
    }

    fn crear_cuenta(&mut self, nombre: &str) -> bool {
        let mut save: u64 = 1;
        self.agregar_cuenta(Cuenta { id: save, nombre: String::from(nombre), saldo: 0 });
        save += 1;
        true
    }
}

fn main(){
    let mut ledger = Ledger {
        cuentas: Vec::new()
    };

    ledger.crear_cuenta("jose");

    println!("{:#?}", ledger.cuentas[0]);
}