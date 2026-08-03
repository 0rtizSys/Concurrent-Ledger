#[derive(Debug)]
pub struct Cuenta {
    pub id: u64,
    pub nombre: String,
    pub saldo: u64,
}

impl Cuenta {
    pub fn depositar(&mut self, monto: u64) {
        self.saldo += monto;
    }

    pub fn retirar(&mut self, monto: u64) {
        if self.saldo >= monto {
            self.saldo -= monto;
        }
    }

    pub fn ver_saldo(&self) -> u64 {
        self.saldo
    }
}