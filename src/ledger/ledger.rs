use crate::account::Account;

pub struct Ledger {
    pub accounts: Vec<Account>,
    history: History,
}

struct History {
    next_id: u64,
    t_accounts: u64,
}

impl History {
    fn new() -> Self {
        Self {
            next_id: 1,
            t_accounts: 0,
        }
    }
}

impl Ledger {
    fn add_account(&mut self, acc: Account){
        self.accounts.push(acc);
    }

    pub fn create_account(&mut self, name: &str) {
        let id = self.history.next_id;

        let acc: Account = Account {
            id: id,
            name: String::from(name),
            balance: 0,
        };

        self.history.next_id += 1;
        self.history.t_accounts += 1;

        self.add_account(acc);
    }
    
    pub fn mut_find_account(&mut self, target: u64) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|account| account.id == target)
    }

    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            history: History::new(),
        }
    }
}