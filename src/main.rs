#[derive(Debug)]
struct Cuenta {
    id: u64,
    nombre: String,
    saldo: u64,
}

fn main(){
    let cuenta_1: Cuenta = Cuenta { id: 1, nombre: String::from("testing"), saldo: 999 };

    println!("{:#?}", cuenta_1);
}