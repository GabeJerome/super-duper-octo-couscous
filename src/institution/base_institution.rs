use super::institution::Institution;
use crate::banking::account::Account;

pub struct BaseInstitution {
    pub name: String,
    pub address: String,
    pub accounts: Vec<Box<dyn Account>>,
}

impl BaseInstitution {
    pub fn new(name: String, address: String) -> Self {
        BaseInstitution {
            name,
            address,
            accounts: Vec::new(),
        }
    }
}

impl Institution for BaseInstitution {
    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn accounts(&self) -> &Vec<Box<dyn Account>> {
        &self.accounts
    }

    fn add_account(&mut self, account: Box<dyn Account>) {
        self.accounts.push(account);
    }

    fn remove_account(&mut self, account: Box<dyn Account>) {
        if let Some(pos) = self
            .accounts
            .iter()
            .position(|a| a.get_id() == account.get_id())
        {
            self.accounts.remove(pos);
        }
    }

    fn apply_annual_fee(&mut self) {
        let fee = 10.00;
        for account in &mut self.accounts {
            if account.get_balance() >= fee {
                account.withdraw(fee).ok();
            } else {
                account.withdraw(account.get_balance()).ok();
            }
        }
    }
}
