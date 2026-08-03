mod cuenta;
use cuenta::Cuenta;

mod ledger;
use ledger::Ledger;

fn main(){
    let mut ledg = Ledger::new();


    println!("{:#?}", ledg.cuentas);
}