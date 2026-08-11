#[derive(Debug)]
pub struct Account {
    pub id: u64,
    pub name: String,
    pub balance: u64,
}

impl Account {
    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) {
        if self.balance >= amount {
            self.balance -= amount;
        }
    }

    pub fn query_balance(&self) -> u64 {
        self.balance
    }
}