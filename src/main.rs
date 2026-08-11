mod cuenta;
mod ledger;
mod utils;
mod cli;

use ledger::Ledger;
use cli::menu::start_cli;


fn main() {
    let mut ledger: Ledger = Ledger::new();
    start_cli(&mut ledger);
}
