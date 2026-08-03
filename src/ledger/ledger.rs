use crate::cuenta::Cuenta;

pub struct Ledger {
    pub cuentas: Vec<Cuenta>,
    historial: Historial,
}

struct Historial {
    siguiente_id: u64,
    total_cuentas: u64,
}

impl Historial {
    fn new() -> Self {
        Self {
            siguiente_id: 1,
            total_cuentas: 0,
        }
    }
}

impl Ledger {
    pub fn agregar_cuenta(&mut self, cuenta: Cuenta){
        self.cuentas.push(cuenta);
    }

    pub fn crear_cuenta(&mut self, nombre: &str) {
        let id = self.historial.siguiente_id;

        let cuenta: Cuenta = Cuenta {
            id: id,
            nombre: String::from(nombre),
            saldo: 0,
        };

        self.historial.siguiente_id += 1;
        self.historial.total_cuentas += 1;

        self.agregar_cuenta(cuenta);
    }

    pub fn new() -> Self {
        Self {
            cuentas: Vec::new(),
            historial: Historial::new(),
        }
    }
}